# 0780-sokol-封装左右动画

## 目标

现在左右动画没有在一起，将其放到一起。

## 环境

- Time 2025-02-22
- Zig 0.14.0-dev.3271+bd237bced

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

每次定义都要定义左和右两边的动画，直接将其放到一起。

## animation.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const cache = @import("cache.zig");

pub const FixedSizeFrameAnimation = struct {
    interval: f32,
    current: u32 = 0,
    timer: f32 = 0,
    frames: [maxFrame]gfx.Texture,

    const maxFrame = 6;

    pub fn load(comptime pathFmt: []const u8, interval: f32) ?@This() {
        var self = @This(){ .frames = undefined, .interval = interval };
        var buffer: [64]u8 = undefined;
        for (0..maxFrame) |index| {
            const path = std.fmt.bufPrintZ(&buffer, pathFmt, .{index}) catch |e| {
                std.log.warn("frame animation path error: {}", .{e});
                return null;
            };

            const texture = cache.TextureCache.load(path);
            self.frames[index] = texture orelse return null;
        }

        return self;
    }

    pub fn play(self: *@This(), delta: f32) void {
        self.timer += delta;
        if (self.timer >= self.interval) {
            self.current = (self.current + 1) % @as(u32, @intCast(self.frames.len));
            self.timer = 0;
        }
    }

    pub fn currentTexture(self: @This()) gfx.Texture {
        return self.frames[self.current];
    }
};

pub const FrameAnimation = struct {
    left: FixedSizeFrameAnimation,
    right: FixedSizeFrameAnimation,
};
```

## player.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const animation = @import("animation.zig");
const cache = @import("cache.zig");
const context = @import("context.zig");
const window = @import("window.zig");

pub const Player = struct {
    x: f32 = 500,
    y: f32 = 500,
    speed: f32 = 0.4,
    faceLeft: bool = true,
    animation: animation.FrameAnimation,
    shadow: gfx.Texture,
    moveUp: bool = false,
    moveDown: bool = false,
    moveLeft: bool = false,
    moveRight: bool = false,

    pub fn init() Player {
        const leftFmt: []const u8 = "assets/img/player_left_{}.png";
        const left = animation.FixedSizeFrameAnimation.load(leftFmt, 50).?;

        const rightFmt = "assets/img/player_right_{}.png";
        const right = animation.FixedSizeFrameAnimation.load(rightFmt, 50).?;

        return .{
            .animation = .{ .left = left, .right = right },
            .shadow = cache.TextureCache.load("assets/img/shadow_player.png").?,
        };
    }

    pub fn processEvent(self: *Player, event: *const window.Event) void {
        if (event.type == .KEY_DOWN) switch (event.key_code) {
            .W => self.moveUp = true,
            .S => self.moveDown = true,
            .A => self.moveLeft = true,
            .D => self.moveRight = true,
            else => {},
        } else if (event.type == .KEY_UP) switch (event.key_code) {
            .W => self.moveUp = false,
            .S => self.moveDown = false,
            .A => self.moveLeft = false,
            .D => self.moveRight = false,
            else => {},
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
            self.animation.left.play(delta)
        else
            self.animation.right.play(delta);
    }

    pub fn currentTexture(self: Player) gfx.Texture {
        return if (self.faceLeft)
            self.animation.left.currentTexture()
        else
            self.animation.right.currentTexture();
    }

    pub fn shadowX(self: *Player) f32 {
        const w = self.currentTexture().width - self.shadow.width;
        return self.x + w / 2;
    }

    pub fn shadowY(self: *Player) f32 {
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
```

## main.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const cache = @import("cache.zig");
const context = @import("context.zig");
const window = @import("window.zig");
const animation = @import("animation.zig");

const Player = @import("player.zig").Player;

var background: gfx.Texture = undefined;

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

    // 加载敌人
    enemy = Enemy.init();
}

const Enemy = struct {
    x: f32 = 400,
    y: f32 = 400,
    animation: animation.FrameAnimation,
    shadow: gfx.Texture,
    faceLeft: bool = true,

    pub fn init() Enemy {
        const leftFmt: []const u8 = "assets/img/enemy_left_{}.png";
        const left = animation.FixedSizeFrameAnimation.load(leftFmt, 50).?;

        const rightFmt = "assets/img/enemy_right_{}.png";
        const right = animation.FixedSizeFrameAnimation.load(rightFmt, 50).?;

        return Enemy{
            .animation = .{ .left = left, .right = right },
            .shadow = cache.TextureCache.load("assets/img/shadow_enemy.png").?,
        };
    }

    pub fn update(self: *Enemy, delta: f32) void {
        if (self.faceLeft)
            self.animation.left.play(delta)
        else
            self.animation.right.play(delta);
    }

    pub fn currentTexture(self: Enemy) gfx.Texture {
        return if (self.faceLeft)
            self.animation.left.currentTexture()
        else
            self.animation.right.currentTexture();
    }

    pub fn shadowX(self: *Enemy) f32 {
        const width = self.currentTexture().width - self.shadow.width;
        return self.x + width / 2;
    }

    pub fn shadowY(self: *Enemy) f32 {
        return self.y + self.currentTexture().height - 25;
    }
};

var player: Player = undefined;
var enemy: Enemy = undefined;

fn frame() void {
    const delta = window.deltaMillisecond();
    player.update(delta);
    enemy.update(delta);

    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);
    defer renderPass.submit();

    var single = gfx.TextureSingle.begin(renderPass);

    single.draw(0, 0, background);

    single.draw(enemy.shadowX(), enemy.shadowY(), enemy.shadow);
    single.draw(enemy.x, enemy.y, enemy.currentTexture());

    single.draw(player.shadowX(), player.shadowY(), player.shadow);
    single.draw(player.x, player.y, player.currentTexture());
}

fn event(evt: ?*const window.Event) void {
    if (evt) |e| player.processEvent(e);
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

![封装左右动画][1]

[1]: images/sokol043.webp

## 附录
