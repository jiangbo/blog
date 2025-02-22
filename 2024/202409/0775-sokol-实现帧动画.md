# 0775-sokol-实现帧动画

## 目标

之前的动画是基于帧数的，现在实现的帧动画是基于时间。

## 环境

- Time 2025-02-22
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

基于时间的帧动画，不管帧率高和低，看到的效果应该都是一样的。

## window.zig

新增了一个方法来获取每帧的间隔时间。

```zig
const std = @import("std");
const sk = @import("sokol");

const context = @import("context.zig");

pub const Event = sk.app.Event;
pub const RunInfo = struct {
    init: *const fn () void,
    frame: *const fn () void,
    event: *const fn (?*const Event) void,
    deinit: *const fn () void,
};

pub fn deltaMillisecond() f32 {
    return @floatCast(sk.app.frameDuration() * 1000);
}

var runInfo: RunInfo = undefined;
pub fn run(info: RunInfo) void {
    runInfo = info;
    sk.app.run(.{
        .width = @as(i32, @intFromFloat(context.width)),
        .height = @as(i32, @intFromFloat(context.height)),
        .window_title = context.title,
        .logger = .{ .func = sk.log.func },
        .win32_console_attach = true,
        .init_cb = init,
        .event_cb = event,
        .frame_cb = frame,
        .cleanup_cb = cleanup,
    });
}

export fn init() void {
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });
    runInfo.init();
}

export fn event(evt: ?*const Event) void {
    runInfo.event(evt);
}

export fn frame() void {
    runInfo.frame();
}

export fn cleanup() void {
    sk.gfx.shutdown();
    runInfo.deinit();
}
```

## animation.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const cache = @import("cache.zig");

pub const FrameAnimation = struct {
    interval: f32,
    frames: [maxFrame]gfx.Texture = undefined,
    count: u32,
    current: u32 = 0,
    timer: f32 = 0,

    const maxFrame = 10;

    pub fn load(comptime pathFmt: []const u8, count: u32, interval: f32) ?FrameAnimation {
        if (count <= 0 or count > maxFrame) {
            std.log.warn("frame count must be (0, {}], actual: {}", .{ maxFrame, count });
        }

        var self = FrameAnimation{ .interval = interval, .count = count };

        var buffer: [64]u8 = undefined;
        for (0..count) |index| {
            const path = std.fmt.bufPrintZ(&buffer, pathFmt, .{index}) catch |e| {
                std.log.warn("frame animation path error: {}", .{e});
                return null;
            };

            const texture = cache.TextureCache.load(path);
            self.frames[index] = texture orelse return null;
        }

        return self;
    }

    pub fn currentOrNext(self: *FrameAnimation, delta: f32) gfx.Texture {
        self.timer += delta;
        if (self.timer >= self.interval) {
            self.current = (self.current + 1) % self.count;
            self.timer = 0;
        }
        return self.frames[self.current];
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

const playerAnimationNumber = 6;

var background: gfx.Texture = undefined;
var playerLeft: animation.FrameAnimation = undefined;
var playerRight: animation.FrameAnimation = undefined;

fn init() void {
    const allocator = context.allocator;
    cache.init(allocator);

    context.camera = gfx.Camera.init(context.width, context.height);
    context.textureSampler = gfx.Sampler.liner();

    context.batchBuffer = gfx.BatchBuffer.init(allocator) catch unreachable;

    // 加载背景
    background = cache.TextureCache.load("assets/img/background.png").?;

    // 加载角色
    const leftFmt: []const u8 = "assets/img/player_left_{}.png";
    playerLeft = animation.FrameAnimation.load(leftFmt, 6, 50).?;

    const rightFmt = "assets/img/player_right_{}.png";
    playerRight = animation.FrameAnimation.load(rightFmt, 6, 50).?;
}

const Vector2 = struct { x: f32 = 0, y: f32 = 0 };

var playerPosition: Vector2 = .{ .x = 500, .y = 500 }; // 角色初始位置
const playerSpeed: f32 = 3; // 角色移动速度

fn frame() void {
    if (moveUp) playerPosition.y -= playerSpeed;
    if (moveDown) playerPosition.y += playerSpeed;
    if (moveLeft) playerPosition.x -= playerSpeed;
    if (moveRight) playerPosition.x += playerSpeed;

    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);
    defer renderPass.submit();

    var single = gfx.TextureSingle.begin(renderPass);

    single.draw(0, 0, background);

    const delta = window.deltaMillisecond();
    single.draw(playerPosition.x, playerPosition.y, playerRight.currentOrNext(delta));

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

![帧动画][1]

[1]: images/sokol038.webp

## 附录
