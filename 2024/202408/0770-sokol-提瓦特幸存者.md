# 0770-sokol-提瓦特幸存者

## 目标

在 B 站上看到从零开始的游戏开发，跟着这个学习一下看看。

## 环境

- Time 2025-02-21
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

这一节主要是加载一张背景图片。

## main.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const cache = @import("cache.zig");
const context = @import("context.zig");
const window = @import("window.zig");

fn init() void {
    const allocator = context.allocator;
    cache.init(allocator);

    context.camera = gfx.Camera.init(context.width, context.height);
    context.textureSampler = gfx.Sampler.liner();

    context.batchBuffer = gfx.BatchBuffer.init(allocator) catch unreachable;
}

fn frame() void {
    const texture = cache.TextureCache.load("assets/img/background.png").?;

    var batch = gfx.TextureBatch.begin(texture);
    defer batch.end();

    batch.draw(0, 0);
}

fn event(evt: ?*const window.Event) void {
    _ = evt;
}

fn deinit() void {
    cache.deinit();
    context.batchBuffer.deinit(context.allocator);
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

![加载背景图片][1]

[1]: images/sokol033.png

## 附录
