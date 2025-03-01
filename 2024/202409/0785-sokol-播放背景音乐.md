# 0785-sokol-播放背景音乐

## 目标

游戏运行过程中，播放背景音乐。

## 环境

- Time 2025-03-01
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>
3. <https://github.com/hosackm/minimp3>

## 想法

感觉播放背景音乐有点不对，不过先就这样。另外，对于音效部分，暂时不弄，因为不清楚怎么混合这两者。

## build.zig.zon

```zig
.minimp3 = .{
    .url = "git+https://github.com/hosackm/minimp3#393350f9f37432d1469db5e9546b2a2f6587f41c",
    .hash = "1220d60b71edfd7389f396d1629552d975e569cb4be1d6e8609da95fb0ccca7ea199",
},
```

## build.zig

```zig
    const minimp3 = b.dependency("minimp3", .{
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addImport("mp3", minimp3.module("decoder"));
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
const Enemy = @import("player.zig").Enemy;

var background: gfx.Texture = undefined;

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

    const sk = @import("sokol");

    sk.audio.setup(.{
        .stream_cb = stream_cb,
        .sample_rate = 24000,
        .buffer_frames = 1152,
        .logger = .{ .func = sk.log.func },
    });

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

var scoreBuffer: [64:0]u8 = undefined;
fn frame() void {
    const delta = window.deltaMillisecond();
    player.update(delta);
    tryGenerateEnemy();
    for (enemies.items) |*enemy| {
        enemy.update(delta, player);
    }

    // 碰撞检测
    checkBulletEnemyCollision();
    checkPlayerEnemyCollision();

    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);
    defer renderPass.submit();

    defer {
        const score = std.fmt.bufPrintZ(&scoreBuffer, "score: {d}", .{player.score});
        window.displayText(1, 2, score catch unreachable);
    }

    var single = gfx.TextureSingle.begin(renderPass);

    single.draw(0, 2.0, background);

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
    if (evt) |e| player.processEvent(e);
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

由于是背景音乐，所以就不截图了。

## 附录
