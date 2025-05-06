# 0918-教你制作RPG游戏-NPC 互动

## 目标

在第二张地图新增一个可以互动的 NPC，互动时播放 NPC 的动画。

## 环境

- Time 2025-05-06

## 参考

1. <https://www.bilibili.com/video/BV1eB4y197zi>

## 想法

给的这个动画的图片感觉不规则，要不就四个动画，要不三个动画，但是全在一行。
但是给的图片是三个动画，然后两行，留出一个空位，这种最好处理成标准的动画图片。

## 动画数量计数

在动画文件中，新增了一个传递动画数量的方法，不使用动画帧切片的长度来存储动画的长度。

```zig
pub fn initWithCount(texture: Texture, count: u8) Animation {
    var frames: [maxSize]math.Rectangle = undefined;

    const floatCount: f32 = @floatFromInt(count);
    const width = @divExact(texture.width(), floatCount);
    const size: math.Vector = .{ .x = width, .y = texture.height() };

    for (0..count) |index| {
        const x = @as(f32, @floatFromInt(index)) * width;
        frames[index] = .init(.init(x, texture.area.min.y), size);
    }

    return .{ .texture = texture, .frames = frames, .count = count };
}
```

## 新增 NPC

新增了一个 NPC，可以看到动画处理部分很难看。

```zig
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
```

## 渲染动画

渲染的时候，以动画为准，有动画并且未结束，就渲染动画，否则渲染纹理。

```zig
        const npcPosition = npc.position.sub(PLAYER_OFFSET);

        if (npc.animation != null and !npc.animation.?.finished()) {
            gfx.draw(npc.animation.?.currentTexture(), npcPosition);
        } else if (npc.texture) |texture| {
            gfx.draw(texture, npcPosition);
        }
```

## 效果

![NPC 互动][1]

[1]: images/教你制作RPG游戏22.webp

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

const FrameAnimation = gfx.FixedFrameAnimation(4, 0.25);

const Action = *const fn () void;
pub const NPC = struct {
    position: math.Vector,
    texture: ?gfx.Texture = null,
    animation: ?FrameAnimation = null,
    area: math.Rectangle = .{},
    keyTrigger: bool = true,
    action: *const fn () void = undefined,

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

    std.mem.sort(NPC, &maps[0].npcArray, {}, struct {
        fn lessThan(_: void, a: NPC, b: NPC) bool {
            return a.position.y < b.position.y;
        }
    }.lessThan);

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
    maps[1].npcArray[2].area = .init(.init(980, 400), .init(20, 600));
    maps[1].npcArray[2].keyTrigger = false;

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

### scene.zig

```zig
const std = @import("std");

const window = @import("window.zig");
const gfx = @import("graphics.zig");
const math = @import("math.zig");
const audio = @import("audio.zig");
const assets = @import("assets.zig");

const Player = @import("Player.zig");
const map = @import("map.zig");
const PLAYER_SPEED = 150;
const PLAYER_OFFSET: math.Vector = .init(120, 220);

var players: [3]Player = undefined;
var currentPlayer: *Player = &players[0];
pub var position: math.Vector = .init(100, 500);
var facing: math.FourDirection = .down;
var keyPressed: bool = false;
var velocity: math.Vector = .zero;

pub fn init() void {
    gfx.camera = .{ .rect = .init(.zero, window.size), .border = map.SIZE };
    gfx.camera.lookAt(position);

    players[0] = .init("assets/r1.png", 0);
    players[1] = .init("assets/r2.png", 1);
    players[2] = .init("assets/r3.png", 2);

    map.init();
}

pub fn update(delta: f32) void {
    velocity = .zero;
    keyPressed = false;

    if (window.isAnyKeyDown(&.{ .UP, .W })) updatePlayer(.up);
    if (window.isAnyKeyDown(&.{ .DOWN, .S })) updatePlayer(.down);
    if (window.isAnyKeyDown(&.{ .LEFT, .A })) updatePlayer(.left);
    if (window.isAnyKeyDown(&.{ .RIGHT, .D })) updatePlayer(.right);

    if (window.isRelease(.TAB)) {
        currentPlayer = &players[(currentPlayer.index + 1) % players.len];
    }

    if (velocity.approx(.zero)) {
        currentPlayer.current(facing).reset();
    } else {
        velocity = velocity.normalize().scale(delta * PLAYER_SPEED);
        const tempPosition = position.add(velocity);
        if (map.canWalk(tempPosition)) position = tempPosition;
        gfx.camera.lookAt(position);
    }

    if (keyPressed) currentPlayer.current(facing).update(delta);

    for (map.npcSlice()) |*npc| {
        if (npc.area.contains(position)) {
            if (npc.keyTrigger) {
                if (window.isRelease(.SPACE)) npc.action();
            } else npc.action();
        }

        if (npc.animation) |*animation| animation.update(delta);
    }
}

fn updatePlayer(direction: math.FourDirection) void {
    facing = direction;
    keyPressed = true;
    velocity = velocity.add(direction.toVector());
}

pub fn render() void {
    gfx.beginDraw();
    defer gfx.endDraw();

    map.drawBackground();

    var playerNotDraw: bool = true;
    for (map.npcSlice()) |npc| {
        if (npc.position.y > position.y and playerNotDraw) {
            drawPlayer();
            playerNotDraw = false;
        }

        const npcPosition = npc.position.sub(PLAYER_OFFSET);

        if (npc.animation != null and !npc.animation.?.finished()) {
            gfx.draw(npc.animation.?.currentTexture(), npcPosition);
        } else if (npc.texture) |texture| {
            gfx.draw(texture, npcPosition);
        }

        gfx.drawRectangle(npc.area);
    }

    if (playerNotDraw) drawPlayer();

    map.drawForeground();

    window.showFrameRate();
}

fn drawPlayer() void {
    const playerTexture = currentPlayer.current(facing).currentTexture();
    gfx.draw(playerTexture, position.sub(PLAYER_OFFSET));
}
```
