# 0779-sokol-加载敌人动画

## 目标

仿照之前的角色的定义，定义一个敌人结构体，并且进行渲染。

## 环境

- Time 2025-02-22
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

将角色相关的代码放到 player.zig 中，在 main.zig 定义 Enemy 结构体。

## player.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const animation = @import("animation.zig");
const cache = @import("cache.zig");
const context = @import("context.zig");

pub const Player = struct {
    x: f32 = 500,
    y: f32 = 500,
    speed: f32 = 0.4,
    faceLeft: bool = true,
    leftAnimation: animation.FrameAnimation,
    rightAnimation: animation.FrameAnimation,
    shadow: gfx.Texture,
    moveUp: bool = false,
    moveDown: bool = false,
    moveLeft: bool = false,
    moveRight: bool = false,

    pub fn init() Player {
        const leftFmt: []const u8 = "assets/img/player_left_{}.png";
        const left = animation.FrameAnimation.load(leftFmt, 6, 50).?;

        const rightFmt = "assets/img/player_right_{}.png";
        const right = animation.FrameAnimation.load(rightFmt, 6, 50).?;

        return .{
            .leftAnimation = left,
            .rightAnimation = right,
            .shadow = cache.TextureCache.load("assets/img/shadow_player.png").?,
        };
    }

    pub fn update(self: *Player, delta: f32) void {
        var vector2: Vector2 = .{};
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
            self.leftAnimation.play(delta)
        else
            self.rightAnimation.play(delta);
    }

    pub fn currentTexture(self: Player) gfx.Texture {
        if (self.faceLeft) {
            return self.leftAnimation.currentTexture();
        } else {
            return self.rightAnimation.currentTexture();
        }
    }

    pub fn shadowX(self: Player) f32 {
        const w = self.currentTexture().width - self.shadow.width;
        return self.x + w / 2;
    }

    pub fn shadowY(self: Player) f32 {
        return self.y + self.currentTexture().height - 8;
    }
};

const Vector2 = struct {
    x: f32 = 0,
    y: f32 = 0,

    pub fn normalize(self: Vector2) Vector2 {
        if (self.x == 0 and self.y == 0) return .{};
        const length = std.math.sqrt(self.x * self.x + self.y * self.y);
        return .{ .x = self.x / length, .y = self.y / length };
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

    // 加载敌人
    enemy = Enemy.init();
}

const Enemy = struct {
    x: f32 = 400,
    y: f32 = 400,
    leftAnimation: animation.FrameAnimation,
    rightAnimation: animation.FrameAnimation,
    shadow: gfx.Texture,
    faceLeft: bool = true,

    pub fn init() Enemy {
        const leftFmt: []const u8 = "assets/img/enemy_left_{}.png";
        const left = animation.FrameAnimation.load(leftFmt, 6, 50).?;

        const rightFmt = "assets/img/enemy_right_{}.png";
        const right = animation.FrameAnimation.load(rightFmt, 6, 50).?;

        return .{
            .leftAnimation = left,
            .rightAnimation = right,
            .shadow = cache.TextureCache.load("assets/img/shadow_enemy.png").?,
        };
    }

    pub fn update(self: *Enemy, delta: f32) void {
        if (self.faceLeft)
            self.leftAnimation.play(delta)
        else
            self.rightAnimation.play(delta);
    }

    pub fn currentTexture(self: Enemy) gfx.Texture {
        if (self.faceLeft) {
            return self.leftAnimation.currentTexture();
        } else {
            return self.rightAnimation.currentTexture();
        }
    }

    pub fn shadowX(self: Enemy) f32 {
        const w = self.currentTexture().width - self.shadow.width;
        return self.x + w / 2;
    }

    pub fn shadowY(self: Enemy) f32 {
        return self.y + self.currentTexture().height - 25;
    }
};

var player: Player = undefined;
var enemy: Enemy = undefined;

fn frame() void {
    const delta = window.deltaMillisecond();
    player.update(delta);
    enemy.update(delta);

    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);
    defer renderPass.submit();

    var single = gfx.TextureSingle.begin(renderPass);

    single.draw(0, 0, background);

    single.draw(enemy.shadowX(), enemy.shadowY(), enemy.shadow);
    single.draw(enemy.x, enemy.y, enemy.currentTexture());

    single.draw(player.shadowX(), player.shadowY(), player.shadow);
    single.draw(player.x, player.y, player.currentTexture());
}

fn event(evt: ?*const window.Event) void {
    if (evt) |e| if (e.type == .KEY_DOWN) switch (e.key_code) {
        .W => player.moveUp = true,
        .S => player.moveDown = true,
        .A => player.moveLeft = true,
        .D => player.moveRight = true,
        else => {},
    } else if (e.type == .KEY_UP) switch (e.key_code) {
        .W => player.moveUp = false,
        .S => player.moveDown = false,
        .A => player.moveLeft = false,
        .D => player.moveRight = false,
        else => {},
    };
}

fn deinit() void {
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

![加载敌人动画][1]

[1]: images/sokol042.webp

## 附录
