# 0913-教你制作RPG游戏-创建 NPC

## 目标

地图上现在没有任何东西，新增两个 NPC 放到地图上。

## 环境

- Time 2025-04-30

## 参考

1. <https://www.bilibili.com/video/BV1eB4y197zi>

## 想法

看了后面的地图切换的，也是使用的一个不可见的 NPC 来实现的。

## map.zig

```zig
const std = @import("std");

const window = @import("window.zig");
const gfx = @import("graphics.zig");
const math = @import("math.zig");
const audio = @import("audio.zig");
const assets = @import("assets.zig");
const c = @import("c.zig");

pub const SIZE: math.Vector = .init(1000, 800);
const PLAYER_OFFSET: math.Vector = .init(120, 220);
const PLAYER_SIZE: math.Vector = .init(240, 240);

const NPC = struct { position: math.Vector, texture: ?gfx.Texture = null };

const Map = struct {
    map: gfx.Texture,
    mapShade: gfx.Texture,
    mapBack: ?gfx.Texture = null,
    mapBlock: ?std.StaticBitSet(SIZE.x * SIZE.y) = null,
    npcArray: [2]NPC = undefined,
};

var index: usize = maps.len - 1;
var maps: [2]Map = undefined;

pub fn init() void {
    maps[0] = Map{
        .map = assets.loadTexture("assets/map1.png", SIZE),
        .mapShade = assets.loadTexture("assets/map1_shade.png", SIZE),
        .mapBack = assets.loadTexture("assets/map1_back.png", SIZE),
    };

    maps[0].npcArray = .{
        .{
            .position = .init(800, 300),
            .texture = assets.loadTexture("assets/npc1.png", PLAYER_SIZE),
        },
        .{
            .position = .init(700, 280),
            .texture = assets.loadTexture("assets/npc2.png", PLAYER_SIZE),
        },
    };

    maps[1] = Map{
        .map = assets.loadTexture("assets/map2.png", SIZE),
        .mapShade = assets.loadTexture("assets/map2_shade.png", SIZE),
    };

    const file = assets.File.load("assets/map1_block.png", callback);
    if (file.data.len != 0) initMapBlock(file.data);

    changeMap();
}

pub fn changeMap() void {
    index = (index + 1) % maps.len;
    switch (index) {
        0 => audio.playMusic("assets/1.ogg"),
        1 => audio.playMusic("assets/2.ogg"),
        else => unreachable,
    }

    if (maps[index].mapBlock == null and index == 0) {
        const file = assets.File.load("assets/map1_block.png", callback);
        if (file.data.len != 0) initMapBlock(file.data);
    }

    if (maps[index].mapBlock == null and index == 1) {
        const file = assets.File.load("assets/map2_block.png", callback);
        if (file.data.len != 0) initMapBlock(file.data);
    }
}

pub fn canWalk(pos: math.Vector) bool {
    const x, const y = .{ @round(pos.x), @round(pos.y) };

    if (x < 0 or x >= SIZE.x or y < 0 or y >= SIZE.y) return false;
    if (maps[index].mapBlock) |block| {
        return !block.isSet(@intFromFloat(x + y * SIZE.x));
    } else return false;
}

fn callback(allocator: std.mem.Allocator, buffer: *[]const u8) void {
    const image = c.stbImage.loadFromMemory(buffer.*) catch unreachable;
    defer c.stbImage.unload(image);

    buffer.* = allocator.dupe(u8, image.data) catch unreachable;
    initMapBlock(buffer.*);
}

fn initMapBlock(buffer: []const u8) void {
    const data: []const u32 = @ptrCast(@alignCast(buffer));
    std.debug.assert(data.len == SIZE.x * SIZE.y);

    var blocks: std.StaticBitSet(SIZE.x * SIZE.y) = .initEmpty();
    for (data, 0..) |color, i| if (color == 0xFF000000) blocks.set(i);

    maps[index].mapBlock = blocks;
}

pub fn drawBackground() void {
    if (maps[index].mapBack) |back| gfx.draw(back, .zero);
    gfx.draw(maps[index].map, .zero);

    for (maps[index].npcArray) |npc| {
        if (npc.texture) |texture| {
            gfx.draw(texture, npc.position.sub(PLAYER_OFFSET));
        }
    }
}

pub fn drawForeground() void {
    gfx.draw(maps[index].mapShade, .zero);
}
```

## 效果

![创建 NPC][1]

[1]: images/教你制作RPG游戏17.webp

## 附录
