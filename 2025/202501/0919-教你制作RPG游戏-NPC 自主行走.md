# 0919-教你制作RPG游戏-NPC 自主行走

## 目标

还是在第二张地图，增加一个 PNC，这个 NPC 可以在地图上自主游荡。

## 环境

- Time 2025-05-06

## 参考

1. <https://www.bilibili.com/video/BV1eB4y197zi>

## 想法

因为只有一个 NPC 有这个功能，就没有封装到 NPC 本身中，直接放到 map 地图模块里，简单一些。

## NPC 属性

定义了 NPC 的速度和四个方向的动画，一个计时器来控制 NPC 行走的时间，增加了 NPC 的类型。
感觉可以新增一个待机的动态，NPC 不一定要一直走，可以走一下停一下。

```zig
const NPC_SPEED = 80;

const FrameAnimation = gfx.FixedFrameAnimation(4, 0.25);

var upAnimation: FrameAnimation = undefined;
var downAnimation: FrameAnimation = undefined;
var leftAnimation: FrameAnimation = undefined;
var rightAnimation: FrameAnimation = undefined;
var facing: math.FourDirection = .down;
var timer: window.Timer = .init(1.5);

const NPCType = enum { fixed, walk, fly };
```

## 初始化 NPC

初始化了一个可以行走的 NPC，定义了她的动画信息。

```zig
    const npc4 = assets.loadTexture("assets/npc4.png", .init(960, 960));
    const size: math.Vector = .init(960, 240);
    upAnimation = .init(npc4.subTexture(.init(.{ .y = 720 }, size)));
    downAnimation = .init(npc4.subTexture(.init(.{ .y = 0 }, size)));
    leftAnimation = .init(npc4.subTexture(.init(.{ .y = 240 }, size)));
    rightAnimation = .init(npc4.subTexture(.init(.{ .y = 480 }, size)));

    maps[1].npcArray[1].animation = downAnimation;
    maps[1].npcArray[1].type = .walk;
```

## 更新 NPC

每帧需要对 NPC 的动画更新，计算移动的距离。如果计时器结束，还需要改变方向。
可以优化，如果某个方向已经碰撞不能行走了，就不生成某个方向。

```zig
pub fn updateNpc(npc: *NPC, delta: f32) void {
    if (npc.animation) |*animation| animation.update(delta);

    if (npc.type == .fixed) return;

    if (timer.isFinishedAfterUpdate(delta)) {
        facing = math.random().enumValue(math.FourDirection);
        npc.animation = switch (facing) {
            .up => upAnimation,
            .down => downAnimation,
            .left => leftAnimation,
            .right => rightAnimation,
        };
        timer.reset();
    }

    const velocity = facing.toVector().scale(delta * NPC_SPEED);
    const position = npc.position.add(velocity);
    if (npc.type == .walk and canWalk(position)) npc.position = position;
    if (npc.type == .fly) npc.position = position;

    npc.area = .init(npc.position.sub(.init(40, 60)), NPC_AREA);
}
```

## 效果

![NPC 自主行走][1]

[1]: images/教你制作RPG游戏23.webp

## 附录

### map.zig

```zig
const std = @import("std");

const window = @import("window.zig");
const gfx = @import("graphics.zig");
const math = @import("math.zig");
const audio = @import("audio.zig");
const assets = @import("assets.zig");
const c = @import("c.zig");
const scene = @import("scene.zig");

pub const SIZE: math.Vector = .init(1000, 800);
const PLAYER_OFFSET: math.Vector = .init(120, 220);
const NPC_SIZE: math.Vector = .init(240, 240);
const NPC_AREA: math.Vector = .init(80, 100);
const NPC_SPEED = 80;

const FrameAnimation = gfx.FixedFrameAnimation(4, 0.25);

var upAnimation: FrameAnimation = undefined;
var downAnimation: FrameAnimation = undefined;
var leftAnimation: FrameAnimation = undefined;
var rightAnimation: FrameAnimation = undefined;
var facing: math.FourDirection = .down;
var timer: window.Timer = .init(1.5);

const NPCType = enum { fixed, walk, fly };

const Action = *const fn () void;
pub const NPC = struct {
    position: math.Vector,
    texture: ?gfx.Texture = null,
    animation: ?FrameAnimation = null,
    area: math.Rectangle = .{},
    keyTrigger: bool = true,
    action: *const fn () void = undefined,
    type: NPCType = .fixed,

    pub fn init(x: f32, y: f32, path: ?[:0]const u8, action: Action) NPC {
        var self: NPC = .{ .position = .init(x, y), .action = action };

        if (path) |p| self.texture = assets.loadTexture(p, NPC_SIZE);
        self.area = .init(self.position.sub(.init(40, 60)), NPC_AREA);
        return self;
    }
};

const Map = struct {
    map: gfx.Texture,
    mapShade: gfx.Texture,
    mapBack: ?gfx.Texture = null,
    mapBlock: ?std.StaticBitSet(SIZE.x * SIZE.y) = null,
    npcArray: [3]NPC = undefined,
};

var index: usize = maps.len - 1;
var maps: [2]Map = undefined;

fn npc1Action() void {
    std.log.info("npc1 action", .{});
}

fn npc2Action() void {
    std.log.info("npc2 action", .{});
}

fn map2npc1Action() void {
    maps[1].npcArray[0].animation.?.reset();
}

pub fn init() void {
    maps[0] = Map{
        .map = assets.loadTexture("assets/map1.png", SIZE),
        .mapShade = assets.loadTexture("assets/map1_shade.png", SIZE),
        .mapBack = assets.loadTexture("assets/map1_back.png", SIZE),
        .npcArray = .{
            .init(800, 300, "assets/npc1.png", npc1Action),
            .init(700, 280, "assets/npc2.png", npc2Action),
            .init(0, 0, null, changeMap0),
        },
    };
    maps[0].npcArray[2].area = .init(.{ .y = 400 }, .init(20, 600));
    maps[0].npcArray[2].keyTrigger = false;

    sortNPC(&maps[1].npcArray);

    // 地图二的具有动画的 NPC
    const anim = assets.loadTexture("assets/Anm1.png", .init(480, 480));
    const animation = anim.subTexture(.init(.zero, .init(480, 240)));
    var anim2 = FrameAnimation.initWithCount(animation, 2);
    anim2.addFrame(.init(.init(0, 240), .init(240, 240)));
    anim2.stop();

    maps[1] = Map{
        .map = assets.loadTexture("assets/map2.png", SIZE),
        .mapShade = assets.loadTexture("assets/map2_shade.png", SIZE),
        .npcArray = .{
            .init(700, 300, "assets/npc3.png", map2npc1Action),
            .init(500, 280, null, npc2Action),
            .init(0, 0, null, changeMap1),
        },
    };
    maps[1].npcArray[0].animation = anim2;

    const npc4 = assets.loadTexture("assets/npc4.png", .init(960, 960));
    const size: math.Vector = .init(960, 240);
    upAnimation = .init(npc4.subTexture(.init(.{ .y = 720 }, size)));
    downAnimation = .init(npc4.subTexture(.init(.{ .y = 0 }, size)));
    leftAnimation = .init(npc4.subTexture(.init(.{ .y = 240 }, size)));
    rightAnimation = .init(npc4.subTexture(.init(.{ .y = 480 }, size)));

    maps[1].npcArray[1].animation = downAnimation;
    maps[1].npcArray[1].type = .walk;

    maps[1].npcArray[2].area = .init(.init(980, 400), .init(20, 600));
    maps[1].npcArray[2].keyTrigger = false;
    sortNPC(&maps[1].npcArray);

    const file = assets.File.load("assets/map1_block.png", callback);
    if (file.data.len != 0) initMapBlock(file.data);

    changeMap();
}

fn changeMap0() void {
    changeMap();
    scene.position.x = SIZE.x - 25;
}

fn changeMap1() void {
    changeMap();
    scene.position.x = 25;
}

fn sortNPC(npcArray: []NPC) void {
    std.mem.sort(NPC, npcArray, {}, struct {
        fn lessThan(_: void, a: NPC, b: NPC) bool {
            return a.position.y < b.position.y;
        }
    }.lessThan);
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

pub fn npcSlice() []NPC {
    return maps[index].npcArray[0..];
}

pub fn updateNpc(npc: *NPC, delta: f32) void {
    if (npc.animation) |*animation| animation.update(delta);

    if (npc.type == .fixed) return;

    if (timer.isFinishedAfterUpdate(delta)) {
        facing = math.random().enumValue(math.FourDirection);
        npc.animation = switch (facing) {
            .up => upAnimation,
            .down => downAnimation,
            .left => leftAnimation,
            .right => rightAnimation,
        };
        timer.reset();
    }

    const velocity = facing.toVector().scale(delta * NPC_SPEED);
    const position = npc.position.add(velocity);
    if (npc.type == .walk and canWalk(position)) npc.position = position;
    if (npc.type == .fly) npc.position = position;

    npc.area = .init(npc.position.sub(.init(40, 60)), NPC_AREA);
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
}

pub fn drawForeground() void {
    gfx.draw(maps[index].mapShade, .zero);
}
```
