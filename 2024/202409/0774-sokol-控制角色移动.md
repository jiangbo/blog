# 0774-sokol-控制角色移动

## 目标

使用键盘的 W S A D 键来控制角色的上下左右。

## 环境

- Time 2025-02-22
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

写了五篇文章了，才看完第一集，完成第一集的内容，不使用图形库和引擎，效率实在不高。

## main.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const cache = @import("cache.zig");
const context = @import("context.zig");
const window = @import("window.zig");

const playerAnimationNumber = 6;

var background: gfx.Texture = undefined;
var playerLeft: [playerAnimationNumber]gfx.Texture = undefined;
var playerRight: [playerAnimationNumber]gfx.Texture = undefined;

fn init() void {
    const allocator = context.allocator;
    cache.init(allocator);

    context.camera = gfx.Camera.init(context.width, context.height);
    context.textureSampler = gfx.Sampler.liner();

    context.batchBuffer = gfx.BatchBuffer.init(allocator) catch unreachable;

    // 加载背景
    background = cache.TextureCache.load("assets/img/background.png").?;

    // 加载角色
    var nameBuffer: [64]u8 = undefined;
    for (0..playerAnimationNumber) |index| {
        playerLeft[index] = loadTexture(&nameBuffer, "left", index).?;
    }
    for (0..playerAnimationNumber) |index| {
        playerRight[index] = loadTexture(&nameBuffer, "right", index).?;
    }
}

const pathFmt = "assets/img/player_{s}_{}.png";
fn loadTexture(buffer: []u8, direction: []const u8, index: usize) ?gfx.Texture {
    const path = std.fmt.bufPrintZ(buffer, pathFmt, .{ direction, index });
    return cache.TextureCache.load(path catch unreachable).?;
}

const Vector2 = struct { x: f32 = 0, y: f32 = 0 };

var frameCounter: usize = 0; // 每帧计数
var playerAnimationIndex: usize = 0; // 角色动画的索引

var playerPosition: Vector2 = .{ .x = 500, .y = 500 }; // 角色初始位置
const playerSpeed: f32 = 3; // 角色移动速度

fn frame() void {
    frameCounter += 1;
    if (frameCounter % 5 == 0) playerAnimationIndex += 1;

    playerAnimationIndex %= playerAnimationNumber;

    if (moveUp) playerPosition.y -= playerSpeed;
    if (moveDown) playerPosition.y += playerSpeed;
    if (moveLeft) playerPosition.x -= playerSpeed;
    if (moveRight) playerPosition.x += playerSpeed;

    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);
    defer renderPass.submit();

    var single = gfx.TextureSingle.begin(renderPass);

    single.draw(0, 0, background);
    single.draw(playerPosition.x, playerPosition.y, playerLeft[playerAnimationIndex]);

    // var batch = gfx.TextureBatch.begin(renderPass, playerLeft[playerAnimationIndex]);
    // batch.draw(0, 0);
    // batch.end();
}

var moveUp: bool = false;
var moveDown: bool = false;
var moveLeft: bool = false;
var moveRight: bool = false;

fn event(evt: ?*const window.Event) void {
    if (evt) |e| if (e.type == .KEY_DOWN) switch (e.key_code) {
        .W => moveUp = true,
        .S => moveDown = true,
        .A => moveLeft = true,
        .D => moveRight = true,
        else => {},
    } else if (e.type == .KEY_UP) switch (e.key_code) {
        .W => moveUp = false,
        .S => moveDown = false,
        .A => moveLeft = false,
        .D => moveRight = false,
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

![控制角色移动][1]

[1]: images/sokol037.webp

## 附录
