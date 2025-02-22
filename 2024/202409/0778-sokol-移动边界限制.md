# 0778-sokol-移动边界限制

## 目标

角色在屏幕上移动的时候，限制它不能移动到屏幕外。

## 环境

- Time 2025-02-22
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

实时修正角色的坐标就可以了，同时修正了角色斜向移动变快的问题。

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
    speed: f32 = 0.003,
    faceLeft: bool = true,
    leftAnimation: animation.FrameAnimation,
    rightAnimation: animation.FrameAnimation,
    shadow: gfx.Texture,
    moveUp: bool = false,
    moveDown: bool = false,
    moveLeft: bool = false,
    moveRight: bool = false,

    pub fn init() Player {
        const leftFmt: []const u8 = "assets/img/player_left_{}.png";
        const left = animation.FrameAnimation.load(leftFmt, 6, 50).?;

        const rightFmt = "assets/img/player_right_{}.png";
        const right = animation.FrameAnimation.load(rightFmt, 6, 50).?;

        return .{
            .leftAnimation = left,
            .rightAnimation = right,
            .shadow = cache.TextureCache.load("assets/img/shadow_player.png").?,
        };
    }

    pub fn update(self: *Player, delta: f32) void {
        var vector2: Vector2 = .{};
        if (self.moveUp) vector2.y -= 1;
        if (self.moveDown) vector2.y += 1;
        if (self.moveLeft) vector2.x -= 1;
        if (self.moveRight) vector2.x += 1;

        const normalized = vector2.normalize();
        self.x += normalized.x * delta * self.speed;
        self.y += normalized.y * delta * self.speed;

        self.x = std.math.clamp(self.x, 0, context.width - self.currentTexture().width);
        self.y = std.math.clamp(self.y, 0, context.height - self.currentTexture().height);

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

    pub fn shadowX(self: Player) f32 {
        const w = self.currentTexture().width - self.shadow.width;
        return self.x + w / 2;
    }

    pub fn shadowY(self: Player) f32 {
        return self.y + self.currentTexture().height - 8;
    }
};

const Vector2 = struct {
    x: f32 = 0,
    y: f32 = 0,

    pub fn normalize(self: Vector2) Vector2 {
        if (self.x == 0 and self.y == 0) return .{};
        const length = std.math.sqrt(self.x * self.x + self.y * self.y);
        return .{ .x = self.x / length, .y = self.y / length };
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
    single.draw(player.shadowX(), player.shadowY(), player.shadow);
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

![移动边界限制][1]

[1]: images/sokol041.webp

## 附录
