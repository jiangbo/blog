# 0788-sokol-游戏的进入与退出

## 目标

实现从游戏菜单进入游戏和退出游戏。

## 环境

- Time 2025-03-01
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

这个应该算完成了吧，因为很多都是从头写的，所以比较慢，期待下一个吧，源码见附录。

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
const Enemy = @import("player.zig").Enemy;

var background: gfx.Texture = undefined;
var menu: gfx.Texture = undefined;

const ImageButton = struct {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    idleTexture: gfx.Texture,
    hoveredTexture: gfx.Texture,
    pushedTexture: gfx.Texture,

    state: ButtonState = .idle,

    pub const ButtonState = enum { idle, hovered, pushed };

    pub fn hoverIfEnter(self: *ImageButton, mouseX: f32, mouseY: f32) void {
        if (self.state == .pushed) {
            return;
        }

        if (mouseX > self.x and mouseX < self.x + self.width and //
            mouseY > self.y and mouseY < self.y + self.height)
        {
            self.state = .hovered;
        } else {
            self.state = .idle;
        }
    }

    pub fn pushIfEnter(self: *ImageButton, mouseX: f32, mouseY: f32) void {
        if (mouseX > self.x and mouseX < self.x + self.width and //
            mouseY > self.y and mouseY < self.y + self.height)
        {
            self.state = .pushed;
        }
    }

    pub fn currentTexture(self: ImageButton) gfx.Texture {
        return switch (self.state) {
            .idle => self.idleTexture,
            .hovered => self.hoveredTexture,
            .pushed => self.pushedTexture,
        };
    }
};

var file: std.fs.File = undefined;
var reader: std.fs.File.Reader = undefined;
var decoder: ?mp3.Decoder = null;
export fn stream_cb(buffer: [*c]f32, _: i32, _: i32) void {
    if (decoder == null) {
        const path = "assets/mus/bgm.mp3";
        file = std.fs.cwd().openFile(path, .{}) catch unreachable;
        decoder = mp3.init();
        reader = file.reader();
    }

    var nextFrame = decoder.?.nextFrame(reader) catch unreachable;
    if (nextFrame == null) {
        file.seekTo(0) catch unreachable;
        reader = file.reader();
        nextFrame = decoder.?.nextFrame(reader) catch unreachable;
    }
    var temp: [1152]f32 = undefined;
    for (0..temp.len) |index| {
        const f: f32 = @floatFromInt(nextFrame.?.samples[index]);
        temp[index] = f / 32768;
    }

    @memcpy(buffer[0..temp.len], &temp);
}

fn init() void {
    const allocator = context.allocator;
    cache.init(allocator);

    context.camera = gfx.Camera.init(context.width, context.height);
    context.textureSampler = gfx.Sampler.liner();

    context.batchBuffer = gfx.BatchBuffer.init(allocator) catch unreachable;

    // 加载背景
    background = cache.TextureCache.load("assets/img/background.png").?;

    // 加载菜单
    menu = cache.TextureCache.load("assets/img/menu.png").?;

    // 按钮
    const idleTexture = cache.TextureCache.load("assets/img/ui_start_idle.png").?;

    const centerX = context.width / 2 - idleTexture.width / 2;
    startButton = ImageButton{
        .x = centerX,
        .y = 430,
        .width = idleTexture.width,
        .height = idleTexture.height,
        .idleTexture = idleTexture,
        .hoveredTexture = cache.TextureCache.load("assets/img/ui_start_hovered.png").?,
        .pushedTexture = cache.TextureCache.load("assets/img/ui_start_pushed.png").?,
    };

    quitButton = ImageButton{
        .x = centerX,
        .y = 550,
        .width = idleTexture.width,
        .height = idleTexture.height,
        .idleTexture = cache.TextureCache.load("assets/img/ui_quit_idle.png").?,
        .hoveredTexture = cache.TextureCache.load("assets/img/ui_quit_hovered.png").?,
        .pushedTexture = cache.TextureCache.load("assets/img/ui_quit_pushed.png").?,
    };

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

var start: bool = false;
var startButton: ImageButton = undefined;
var quitButton: ImageButton = undefined;

var scoreBuffer: [64:0]u8 = undefined;
fn frame() void {
    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);

    var single = gfx.TextureSingle.begin(renderPass);

    defer renderPass.submit();
    if (!start) {
        single.draw(-1, -1, menu);
        single.draw(startButton.x, startButton.y, startButton.currentTexture());
        single.draw(quitButton.x, quitButton.y, quitButton.currentTexture());
        if (startButton.state == .pushed) {
            const sk = @import("sokol");

            sk.audio.setup(.{
                .stream_cb = stream_cb,
                .sample_rate = 24000,
                .buffer_frames = 1152,
                .logger = .{ .func = sk.log.func },
            });
            start = true;
        }
        if (quitButton.state == .pushed) window.exit();
        return;
    }

    const delta = window.deltaMillisecond();
    player.update(delta);
    tryGenerateEnemy();
    for (enemies.items) |*enemy| {
        enemy.update(delta, player);
    }

    // 碰撞检测
    checkBulletEnemyCollision();
    checkPlayerEnemyCollision();

    defer {
        const score = std.fmt.bufPrintZ(&scoreBuffer, "score: {d}", .{player.score});
        window.displayText(1, 2, score catch unreachable);
    }

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

fn checkBulletEnemyCollision() void {
    for (&player.bullets) |bullet| {
        const bulletCenterX = bullet.x + bullet.texture.width / 2;
        const bulletCenterY = bullet.y + bullet.texture.height / 2;
        for (enemies.items, 0..) |enemy, index| {
            const enemyRectangle = math.Rectangle{
                .x = enemy.x,
                .y = enemy.y,
                .width = enemy.currentTexture().width,
                .height = enemy.currentTexture().height,
            };
            if (enemyRectangle.contains(bulletCenterX, bulletCenterY)) {
                _ = enemies.swapRemove(index);
                player.score += 1;
            }
        }
    }
}

fn checkPlayerEnemyCollision() void {
    for (enemies.items) |enemy| {
        const playerRect = math.Rectangle{
            .x = player.x,
            .y = player.y,
            .width = player.currentTexture().width,
            .height = player.currentTexture().height,
        };

        const enemyCenterX = enemy.x + enemy.currentTexture().width / 2;
        const enemyCenterY = enemy.y + enemy.currentTexture().height / 2;

        if (playerRect.contains(enemyCenterX, enemyCenterY)) {
            std.log.info("collision", .{});
            window.exit();
        }
    }
}

fn event(evt: ?*const window.Event) void {
    if (evt) |e| {
        player.processEvent(e);
        if (e.type == .MOUSE_MOVE) {
            startButton.hoverIfEnter(e.mouse_x, e.mouse_y);
            quitButton.hoverIfEnter(e.mouse_x, e.mouse_y);
        }
        if (e.type == .MOUSE_UP and e.mouse_button == .LEFT) {
            startButton.pushIfEnter(e.mouse_x, e.mouse_y);
            quitButton.pushIfEnter(e.mouse_x, e.mouse_y);
        }
    }
}

fn deinit() void {
    enemies.deinit();
    context.batchBuffer.deinit(context.allocator);
    cache.deinit();
}

const mp3 = @import("mp3");

pub fn main() !void {
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

![进入与退出游戏][1]

[1]: images/sokol050.webp

## 附录
