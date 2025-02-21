# 0771-sokol-加载角色图片

## 目标

实现加载角色图片帧，根据顺序加载多个。

## 环境

- Time 2025-02-21
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

除了实现加载图片和缓存的功能，修复了 StringHashMap 的 key 被释放的问题。

## cache.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");

var allocator: std.mem.Allocator = undefined;

pub fn init(alloc: std.mem.Allocator) void {
    allocator = alloc;
    TextureCache.init();
}

pub fn deinit() void {
    TextureCache.deinit();
}

pub const TextureCache = struct {
    const stbi = @import("stbi");
    const Cache = std.StringHashMap(gfx.Texture);

    var cache: Cache = undefined;

    pub fn init() void {
        cache = Cache.init(allocator);
        stbi.init(allocator);
    }

    pub fn load(path: [:0]const u8) ?gfx.Texture {
        const entry = cache.getOrPut(path) catch |e| {
            std.log.err("texture cache allocate error: {}", .{e});
            return null;
        };
        if (entry.found_existing) return entry.value_ptr.*;

        std.log.info("loading texture from: {s}", .{path});
        var image = stbi.Image.loadFromFile(path, 4) catch |e| {
            std.log.err("loading image error: {}", .{e});
            return null;
        };

        defer image.deinit();

        const texture = gfx.Texture.init(image.width, image.height, image.data);
        entry.value_ptr.* = texture;
        entry.key_ptr.* = allocator.dupe(u8, path) catch unreachable;
        return texture;
    }

    pub fn deinit() void {
        stbi.deinit();
        var keyIter = cache.keyIterator();
        while (keyIter.next()) |key| {
            allocator.free(key.*);
        }
        cache.deinit();
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

const playerAnimationNumber = 6;

var background: gfx.Texture = undefined;
var playerLeft: [playerAnimationNumber]gfx.Texture = undefined;
var playerRight: [playerAnimationNumber]gfx.Texture = undefined;

const stbi = @import("stbi");

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

fn frame() void {
    var encoder = gfx.CommandEncoder{};
    defer encoder.submit();

    var batch = gfx.TextureBatch.begin(background);
    batch.draw(0, 0);
    batch.end();
}

fn event(evt: ?*const window.Event) void {
    _ = evt;
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

![加载角色图片][1]

[1]: images/sokol034.png

## 附录
