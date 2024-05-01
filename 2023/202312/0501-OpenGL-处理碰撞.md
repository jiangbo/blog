# 0501-OpenGL-处理碰撞

## 环境

- Time 2024-05-01
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://learnopengl-cn.github.io/06%20In%20Practice/2D-Game>
2. <https://learnopengl.com/In-Practice/2D-Game/>

### 目标

实现小球和方块，小球和挡板的碰撞处理，如果碰撞底边，就重置当前关卡。

## sprite.zig

```zig
const std = @import("std");
const zlm = @import("zlm");
const Texture2D = @import("texture.zig").Texture2D;

pub const Sprite = struct {
    texture: Texture2D,
    position: zlm.Vec2 = zlm.Vec2.zero,
    size: zlm.Vec2 = zlm.Vec2.new(10, 10),
    rotate: f32 = 0,
    color: zlm.Vec3 = zlm.Vec3.one,
    solid: bool = false,
    destroyed: bool = false,

    pub fn checkCollision(s1: Sprite, s2: Sprite) bool {
        const collisionX = s1.position.x + s1.size.x >= s2.position.x //
        and s2.position.x + s2.size.x >= s1.position.x;

        const collisionY = s1.position.y + s1.size.y >= s2.position.y //
        and s2.position.y + s2.size.y >= s1.position.y;

        return collisionX and collisionY;
    }
};

pub const Ball = struct {
    sprite: Sprite,
    radius: f32,
    stuck: bool = true,
    velocity: zlm.Vec2,

    pub fn move(self: *Ball, deltaTime: f32, width: f32) zlm.Vec2 {
        if (self.stuck) return self.sprite.position;

        const delta = self.velocity.scale(deltaTime);
        self.sprite.position = self.sprite.position.add(delta);

        if (self.sprite.position.x <= 0) {
            self.velocity.x = -self.velocity.x;
            self.sprite.position.x = 0;
        }

        if (self.sprite.position.x + self.sprite.size.x >= width) {
            self.velocity.x = -self.velocity.x;
            self.sprite.position.x = width - self.sprite.size.x;
        }

        if (self.sprite.position.y <= 0) {
            self.velocity.y = -self.velocity.y;
            self.sprite.position.y = 0;
        }

        return self.sprite.position;
    }

    pub fn checkCollision(self: Ball, s2: Sprite) Collision {
        const center = self.sprite.position.add(zlm.Vec2.all(self.radius));

        const aabbHalf = s2.size.scale(0.5);
        const aabbCenter = s2.position.add(aabbHalf);

        var difference = center.sub(aabbCenter);
        const clamped = difference.componentClamp(aabbHalf.neg(), aabbHalf);
        const closest = aabbCenter.add(clamped);
        difference = closest.sub(center);
        if (difference.length() > self.radius) return Collision{};

        return Collision.collisioned(difference);
    }
};

pub const Collision = struct {
    collisioned: bool = false,
    direction: enum { up, right, down, left } = .up,
    vector: zlm.Vec2 = zlm.Vec2.zero,

    fn collisioned(target: zlm.Vec2) Collision {
        const compass = [_]zlm.Vec2{
            zlm.Vec2.new(0.0, 1.0),
            zlm.Vec2.new(1.0, 0.0),
            zlm.Vec2.new(0.0, -1.0),
            zlm.Vec2.new(-1.0, 0.0),
        };
        var max: f32 = 0.0;
        var bestMatch: usize = 0;
        for (compass, 0..) |value, i| {
            const dot = target.normalize().dot(value);
            if (dot > max) {
                max = dot;
                bestMatch = i;
            }
        }
        return Collision{
            .collisioned = true,
            .direction = @enumFromInt(bestMatch),
            .vector = compass[bestMatch],
        };
    }
};
```

## game.zig

```zig
const std = @import("std");
const zlm = @import("zlm");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const resource = @import("resource.zig");
const SpriteRenderer = @import("renderer.zig").SpriteRenderer;
const sprite = @import("sprite.zig");

const Allocator = std.mem.Allocator;
const GameState = enum { active, menu, win };
const playerSpeed: f32 = 500;
const ballRadius: f32 = 12.5;
const ballVelocity: zlm.Vec2 = zlm.Vec2.new(100, -350);

pub const Game = struct {
    state: GameState = .active,
    width: f32,
    height: f32,
    keys: [1024]bool = [_]bool{false} ** 1024,
    spriteRenderer: SpriteRenderer = undefined,
    levels: [4]GameLevel = undefined,
    level: usize = 0,
    player: sprite.Sprite = undefined,
    ball: sprite.Ball = undefined,

    pub fn init(self: *Game, allocator: std.mem.Allocator) !void {
        resource.init(allocator);

        const vs: [:0]const u8 = @embedFile("shader/vertex.glsl");
        const fs: [:0]const u8 = @embedFile("shader/fragment.glsl");
        const shader = resource.loadShader(.shader, vs, fs);

        const projection = zlm.Mat4.createOrthogonal(0, self.width, self.height, 0, -1, 1);
        shader.setUniformMatrix4fv("projection", &projection.fields[0][0]);
        shader.setUniform1i("image", 0);

        self.spriteRenderer = SpriteRenderer{ .shader = shader };
        self.spriteRenderer.initRenderData();

        var buffer: [30]u8 = undefined;
        for (&self.levels, 1..) |*value, i| {
            value.* = .{ .width = self.width, .height = self.height / 2 };
            const path = std.fmt.bufPrint(&buffer, "assets/lv{}.json", .{i});
            try value.init(allocator, try path);
        }

        self.resetPlayer();
    }

    fn ballPositionWithPlayer(self: Game) zlm.Vec2 {
        if (!self.ball.stuck) return self.ball.sprite.position;
        const x = self.player.size.x / 2 - ballRadius;
        return self.player.position.add(zlm.Vec2.new(x, -ballRadius * 2));
    }

    fn doCollisions(self: *Game) void {
        for (self.levels[self.level].bricks.items) |*box| {
            if (box.destroyed) continue;

            const collision = self.ball.checkCollision(box.*);
            if (!collision.collisioned) continue;
            if (!box.solid) box.destroyed = true;

            if (collision.direction == .left or collision.direction == .right) {
                self.ball.velocity.x = -self.ball.velocity.x;
                var delta = self.ball.radius - @abs(collision.vector.x);
                delta = if (collision.direction == .left) -delta else delta;
                self.ball.sprite.position.x += delta;
            } else {
                self.ball.velocity.y = -self.ball.velocity.y;
                var delta = self.ball.radius - @abs(collision.vector.y);
                delta = if (collision.direction == .up) -delta else delta;
                self.ball.sprite.position.y += delta;
            }
        }

        const collision = self.ball.checkCollision(self.player);
        if (!collision.collisioned) return;

        const center = self.player.position.x + self.player.size.x / 2;
        const distance = (self.ball.sprite.position.x + self.ball.radius) - center;
        const percentage = distance / (self.player.size.x / 2);

        const old = self.ball.velocity;
        self.ball.velocity.x = ballVelocity.x * percentage * 2;
        self.ball.velocity.y = -@abs(self.ball.velocity.y);
        self.ball.velocity = self.ball.velocity.normalize().scale(old.length());
    }

    pub fn processInput(self: *Game, deltaTime: f32) void {
        if (self.state != .active) return;

        const distance = playerSpeed * deltaTime;

        if (self.keys[@as(usize, @intCast(glfw.Key.a.getScancode()))]) {
            self.player.position.x -= distance;
            if (self.player.position.x < 0) self.player.position.x = 0;
            self.ball.sprite.position = self.ballPositionWithPlayer();
        }

        if (self.keys[@as(usize, @intCast(glfw.Key.d.getScancode()))]) {
            self.player.position.x += distance;
            const maxX = self.width - self.player.size.x;
            if (self.player.position.x > maxX) self.player.position.x = maxX;
            self.ball.sprite.position = self.ballPositionWithPlayer();
        }

        if (self.keys[@as(usize, @intCast(glfw.Key.space.getScancode()))]) {
            self.ball.stuck = false;
        }
    }

    pub fn update(self: *Game, deltaTime: f32) void {
        _ = self.ball.move(deltaTime, self.width);

        if (!self.ball.stuck) self.doCollisions();

        if (self.ball.sprite.position.y >= self.height) {
            self.levels[self.level].reset();
            self.resetPlayer();
        }
    }

    pub fn render(self: Game) void {
        if (self.state != .active) return;
        const background = resource.getTexture(.background);
        self.spriteRenderer.draw(sprite.Sprite{
            .texture = background,
            .size = zlm.Vec2.new(self.width, self.height),
        });

        self.levels[self.level].draw(self.spriteRenderer);
        self.spriteRenderer.draw(self.player);
        self.spriteRenderer.draw(self.ball.sprite);
    }

    fn resetPlayer(self: *Game) void {
        self.player = sprite.Sprite{
            .texture = resource.getTexture(.paddle),
            .position = zlm.Vec2.new(self.width / 2 - 50, self.height - 20),
            .size = zlm.Vec2.new(100, 20),
        };
        self.ball = sprite.Ball{ .sprite = sprite.Sprite{
            .size = zlm.Vec2.new(ballRadius * 2, ballRadius * 2),
            .texture = resource.getTexture(.face),
        }, .velocity = ballVelocity, .radius = ballRadius };
        self.ball.sprite.position = self.ballPositionWithPlayer();
    }

    pub fn deinit(self: Game) void {
        for (self.levels) |level| level.deinit();
        resource.deinit();
    }
};

const GameLevel = struct {
    bricks: std.ArrayList(sprite.Sprite) = undefined,
    width: f32 = 0,
    height: f32 = 0,
    copy: std.ArrayList(sprite.Sprite) = undefined,

    fn draw(self: GameLevel, renderer: SpriteRenderer) void {
        for (self.bricks.items) |brick| {
            if (!brick.destroyed) renderer.draw(brick);
        }
    }

    fn deinit(self: GameLevel) void {
        self.bricks.deinit();
        self.copy.deinit();
    }
    // fn isCompleted() bool{
    //     return false;
    // };
    fn init(self: *GameLevel, allocator: std.mem.Allocator, path: []const u8) !void {
        try self.doInit(allocator, path);
        self.copy = try self.bricks.clone();
    }

    fn doInit(self: *GameLevel, allocator: std.mem.Allocator, path: []const u8) !void {
        std.log.info("load level: {s}", .{path});
        const file = try std.fs.cwd().openFile(path, .{});
        defer file.close();

        const text = try file.readToEndAlloc(allocator, 1024 * 4);
        defer allocator.free(text);

        const parsed = try std.json.parseFromSlice(FileLevel, allocator, text, .{});
        defer parsed.deinit();

        try self.parse(allocator, parsed.value);
    }

    fn parse(self: *GameLevel, allocator: std.mem.Allocator, file: FileLevel) !void {
        const size = file.width * file.height;
        self.bricks = try std.ArrayList(sprite.Sprite).initCapacity(allocator, size);
        const unitWidth = self.width / @as(f32, @floatFromInt(file.width));
        const unitHeight = self.height / @as(f32, @floatFromInt(file.height));

        for (file.level, 0..) |unit, index| {
            const x: f32 = @floatFromInt(index % file.width);
            const y: f32 = @floatFromInt(index / file.width);
            if (unit == 1) {
                self.bricks.append(sprite.Sprite{
                    .position = zlm.Vec2.new(x * unitWidth, y * unitHeight),
                    .size = zlm.Vec2.new(unitWidth, unitHeight),
                    .texture = resource.getTexture(.solid_block),
                    .solid = true,
                }) catch unreachable;
                continue;
            }

            const color = switch (unit) {
                0 => continue,
                2 => zlm.Vec3.new(0.2, 0.6, 1.0),
                3 => zlm.Vec3.new(0.0, 0.7, 0.0),
                4 => zlm.Vec3.new(0.8, 0.8, 0.4),
                5 => zlm.Vec3.new(1.0, 0.5, 0.0),
                else => zlm.Vec3.new(1.0, 1.0, 1.0),
            };

            self.bricks.append(sprite.Sprite{
                .position = zlm.Vec2.new(x * unitWidth, y * unitHeight),
                .size = zlm.Vec2.new(unitWidth, unitHeight),
                .texture = resource.getTexture(.block),
                .color = color,
            }) catch unreachable;
        }
    }

    fn reset(self: *GameLevel) void {
        @memcpy(self.bricks.items, self.copy.items);
    }
};

const FileLevel = struct {
    level: []const u8,
    width: usize,
    height: usize,
};
```

## 效果

![碰撞处理][1]

## 总结

实现了碰撞处理和重置关卡，对 game.zig 进行了重构。

[1]: images/opengl26.gif

## 附录
