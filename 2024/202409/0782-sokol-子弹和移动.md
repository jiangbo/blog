# 0782-sokol-子弹和移动

## 目标

生成子弹，围绕角色进行旋转。

## 环境

- Time 2025-02-24
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

教程中的子弹是画圆圈实现，现在还不清楚怎么实现，先使用图片代替。

## player.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const animation = @import("animation.zig");
const cache = @import("cache.zig");
const context = @import("context.zig");
const window = @import("window.zig");
const math = @import("math.zig");

pub const Bullet = struct {
    x: f32 = 0,
    y: f32 = 0,
    texture: gfx.Texture,

    pub const radialSpeed: f32 = 0.025;
    pub const tangentSpeed: f32 = 0.025;
};

pub const Player = struct {
    x: f32 = 500,
    y: f32 = 500,
    speed: f32 = 0.4,
    faceLeft: bool = true,
    animation: animation.FrameAnimation,
    shadow: gfx.Texture,
    moveUp: bool = false,
    moveDown: bool = false,
    moveLeft: bool = false,
    moveRight: bool = false,

    bullets: [3]Bullet = undefined,

    pub fn init() Player {
        const leftFmt: []const u8 = "assets/img/player_left_{}.png";
        const left = animation.FixedSizeFrameAnimation.load(leftFmt, 50).?;

        const rightFmt = "assets/img/player_right_{}.png";
        const right = animation.FixedSizeFrameAnimation.load(rightFmt, 50).?;

        var self = Player{
            .animation = .{ .left = left, .right = right },
            .shadow = cache.TextureCache.load("assets/img/shadow_player.png").?,
        };

        const tex = cache.TextureCache.load("assets/img/bullet.png").?;
        for (&self.bullets) |*bullet| {
            bullet.* = Bullet{ .x = -tex.width, .y = -tex.height, .texture = tex };
        }

        return self;
    }

    pub fn processEvent(self: *Player, event: *const window.Event) void {
        if (event.type == .KEY_DOWN) switch (event.key_code) {
            .W => self.moveUp = true,
            .S => self.moveDown = true,
            .A => self.moveLeft = true,
            .D => self.moveRight = true,
            else => {},
        } else if (event.type == .KEY_UP) switch (event.key_code) {
            .W => self.moveUp = false,
            .S => self.moveDown = false,
            .A => self.moveLeft = false,
            .D => self.moveRight = false,
            else => {},
        };
    }

    pub fn update(self: *Player, delta: f32) void {
        var vector2: math.Vector2 = .{};
        if (self.moveUp) vector2.y -= 1;
        if (self.moveDown) vector2.y += 1;
        if (self.moveLeft) vector2.x -= 1;
        if (self.moveRight) vector2.x += 1;

        const normalized = vector2.normalize();
        self.x += normalized.x * delta * self.speed;
        self.y += normalized.y * delta * self.speed;

        self.x = std.math.clamp(self.x, 0, context.width - self.currentTexture().width);
        self.y = std.math.clamp(self.y, 0, context.height - self.currentTexture().height);

        if (self.moveLeft) self.faceLeft = true;
        if (self.moveRight) self.faceLeft = false;

        if (self.faceLeft)
            self.animation.left.play(delta)
        else
            self.animation.right.play(delta);

        self.updateBullets();
    }

    fn updateBullets(self: *Player) void {
        const len: f32 = @floatFromInt(self.bullets.len);
        const radianInterval = 2 * std.math.pi / len;

        const total = window.totalMillisecond();
        const radius = 100 + 25 * @sin(total * Bullet.radialSpeed);

        const playerCenterX = self.x + self.currentTexture().width / 2;
        const playerCenterY = self.y + self.currentTexture().height / 2;

        for (0..self.bullets.len) |i| {
            const pos = radianInterval * @as(f32, @floatFromInt(i));
            const radian = pos + total * Bullet.tangentSpeed;
            self.bullets[i].x = playerCenterX + radius * @sin(radian);
            self.bullets[i].y = playerCenterY + radius * @cos(radian);
        }
    }

    pub fn currentTexture(self: Player) gfx.Texture {
        return if (self.faceLeft)
            self.animation.left.currentTexture()
        else
            self.animation.right.currentTexture();
    }

    pub fn shadowX(self: *Player) f32 {
        const w = self.currentTexture().width - self.shadow.width;
        return self.x + w / 2;
    }

    pub fn shadowY(self: *Player) f32 {
        return self.y + self.currentTexture().height - 8;
    }
};
```

## main.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const cache = @import("cache.zig");
const context = @import("context.zig");
const window = @import("window.zig");
const animation = @import("animation.zig");

const math = @import("math.zig");
const Player = @import("player.zig").Player;

var background: gfx.Texture = undefined;

fn init() void {
    const allocator = context.allocator;
    cache.init(allocator);

    context.camera = gfx.Camera.init(context.width, context.height);
    context.textureSampler = gfx.Sampler.liner();

    context.batchBuffer = gfx.BatchBuffer.init(allocator) catch unreachable;

    // 加载背景
    background = cache.TextureCache.load("assets/img/background.png").?;

    // 加载角色
    player = Player.init();

    enemies = std.ArrayList(Enemy).init(allocator);
    // 加载敌人动画资源
    _ = Enemy.init();
}

const Direction = enum { left, right, up, down };

fn initEnemyPosition(enemy: *Enemy) void {
    const direction = context.rand.enumValue(Direction);
    switch (direction) {
        .left => {
            enemy.x = -enemy.currentTexture().width;
            enemy.y = context.rand.float(f32) * context.height;
        },
        .right => {
            enemy.x = context.width;
            enemy.y = context.rand.float(f32) * context.height;
        },
        .up => {
            enemy.x = context.rand.float(f32) * context.width;
            enemy.y = -enemy.currentTexture().height;
        },
        .down => {
            enemy.x = context.rand.float(f32) * context.width;
            enemy.y = context.height;
        },
    }
}

const Enemy = struct {
    x: f32 = 0,
    y: f32 = 0,
    animation: animation.FrameAnimation,
    shadow: gfx.Texture,
    faceLeft: bool = true,
    speed: f32 = 0.1,

    pub fn init() Enemy {
        const leftFmt: []const u8 = "assets/img/enemy_left_{}.png";
        const left = animation.FixedSizeFrameAnimation.load(leftFmt, 50).?;

        const rightFmt = "assets/img/enemy_right_{}.png";
        const right = animation.FixedSizeFrameAnimation.load(rightFmt, 50).?;

        return Enemy{
            .animation = .{ .left = left, .right = right },
            .shadow = cache.TextureCache.load("assets/img/shadow_enemy.png").?,
        };
    }

    pub fn update(self: *Enemy, delta: f32) void {
        const playerPos = math.Vector2{ .x = player.x, .y = player.y };
        const enemyPos = math.Vector2{ .x = self.x, .y = self.y };
        const normalized = playerPos.sub(enemyPos).normalize();

        self.x += normalized.x * delta * self.speed;
        self.y += normalized.y * delta * self.speed;

        self.faceLeft = normalized.x < 0;

        if (self.faceLeft)
            self.animation.left.play(delta)
        else
            self.animation.right.play(delta);
    }

    pub fn currentTexture(self: Enemy) gfx.Texture {
        return if (self.faceLeft)
            self.animation.left.currentTexture()
        else
            self.animation.right.currentTexture();
    }

    pub fn shadowX(self: Enemy) f32 {
        const width = self.currentTexture().width - self.shadow.width;
        return self.x + width / 2;
    }

    pub fn shadowY(self: Enemy) f32 {
        return self.y + self.currentTexture().height - 25;
    }
};

var player: Player = undefined;
var enemies: std.ArrayList(Enemy) = undefined;

const enemyGenerateInterval: f32 = 2000;

var enemyGenerateTimer: f32 = 0;
fn tryGenerateEnemy() void {
    enemyGenerateTimer += window.deltaMillisecond();
    if (enemyGenerateTimer >= enemyGenerateInterval) {
        enemyGenerateTimer = 0;
        enemies.append(Enemy.init()) catch unreachable;
        initEnemyPosition(&enemies.items[enemies.items.len - 1]);
    }
}

fn frame() void {
    const delta = window.deltaMillisecond();
    player.update(delta);
    tryGenerateEnemy();
    for (enemies.items) |*enemy| {
        enemy.update(delta);
    }

    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);
    defer renderPass.submit();

    var single = gfx.TextureSingle.begin(renderPass);

    single.draw(0, 0, background);

    // 敌人
    for (enemies.items) |enemy| {
        single.draw(enemy.shadowX(), enemy.shadowY(), enemy.shadow);
        single.draw(enemy.x, enemy.y, enemy.currentTexture());
    }

    // 玩家
    single.draw(player.shadowX(), player.shadowY(), player.shadow);
    single.draw(player.x, player.y, player.currentTexture());

    // 子弹
    for (&player.bullets) |bullet| {
        single.draw(bullet.x, bullet.y, bullet.texture);
    }
}

fn event(evt: ?*const window.Event) void {
    if (evt) |e| player.processEvent(e);
}

fn deinit() void {
    enemies.deinit();
    context.batchBuffer.deinit(context.allocator);
    cache.deinit();
}

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    context.allocator = gpa.allocator();

    context.width = 1280;
    context.height = 720;

    var prng = std.Random.DefaultPrng.init(@intCast(std.time.timestamp()));
    context.rand = prng.random();
    window.run(.{ .init = init, .event = event, .frame = frame, .deinit = deinit });
}
```

## 效果

![子弹和移动][1]

[1]: images/sokol045.webp

## 附录
