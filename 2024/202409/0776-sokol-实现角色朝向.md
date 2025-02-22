# 0776-sokol-实现角色朝向

## 目标

根据按键的左右，来实现游戏角色的面向。

## 环境

- Time 2025-02-22
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

将状态信息都放到了一个 Player 的结构体中。

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
            return null;
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

    pub fn play(self: *FrameAnimation, delta: f32) void {
        self.timer += delta;
        if (self.timer >= self.interval) {
            self.current = (self.current + 1) % self.count;
            self.timer = 0;
        }
    }

    pub fn currentTexture(self: FrameAnimation) gfx.Texture {
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

var background: gfx.Texture = undefined;

const Player = struct {
    x: f32 = 500,
    y: f32 = 500,
    speed: f32 = 3,
    faceLeft: bool = true,
    leftAnimation: animation.FrameAnimation,
    rightAnimation: animation.FrameAnimation,
    moveUp: bool = false,
    moveDown: bool = false,
    moveLeft: bool = false,
    moveRight: bool = false,

    pub fn init() Player {
        const leftFmt: []const u8 = "assets/img/player_left_{}.png";
        const left = animation.FrameAnimation.load(leftFmt, 6, 50).?;

        const rightFmt = "assets/img/player_right_{}.png";
        const right = animation.FrameAnimation.load(rightFmt, 6, 50).?;

        return .{ .leftAnimation = left, .rightAnimation = right };
    }

    pub fn update(self: *Player, delta: f32) void {
        if (self.moveUp) self.y -= self.speed;
        if (self.moveDown) self.y += self.speed;
        if (self.moveLeft) self.x -= self.speed;
        if (self.moveRight) self.x += self.speed;

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
};

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
}

var player: Player = undefined;

fn frame() void {
    const delta = window.deltaMillisecond();
    player.update(delta);

    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);
    defer renderPass.submit();

    var single = gfx.TextureSingle.begin(renderPass);

    single.draw(0, 0, background);
    single.draw(player.x, player.y, player.currentTexture());

    // var batch = gfx.TextureBatch.begin(renderPass, playerLeft[playerAnimationIndex]);
    // batch.draw(0, 0);
    // batch.end();
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

![实现角色朝向][1]

[1]: images/sokol039.webp

## 附录
