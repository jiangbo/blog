# 0499-OpenGL-渲染小球

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

渲染小球，可以跟着挡板移动，碰到屏幕边缘后反弹。

## sprite.zig

```zig
const std = @import("std");
const zlm = @import("zlm");
const Texture2D = @import("texture.zig").Texture2D;
// const SpriteRenderer = @import("renderer.zig").SpriteRenderer;

pub const Sprite = struct {
    texture: Texture2D,
    position: zlm.Vec2 = zlm.Vec2.zero,
    size: zlm.Vec2 = zlm.Vec2.new(10, 10),
    rotate: f32 = 0,
    color: zlm.Vec3 = zlm.Vec3.one,
    solid: bool = true,
    destroyed: bool = false,
};

pub const Ball = struct {
    sprite: Sprite,
    radius: f32,
    stuck: bool = true,
    velocity: zlm.Vec2 = zlm.Vec2.new(100, -350),

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

        self.player = sprite.Sprite{
            .texture = resource.getTexture(.paddle),
            .position = zlm.Vec2.new(self.width / 2 - 50, self.height - 20),
            .size = zlm.Vec2.new(100, 20),
        };

        self.ball = sprite.Ball{ .sprite = sprite.Sprite{
            .size = zlm.Vec2.new(ballRadius * 2, ballRadius * 2),
            .texture = resource.getTexture(.face),
        }, .radius = ballRadius };
        self.ball.sprite.position = self.ballPositionWithPlayer();
    }

    fn ballPositionWithPlayer(self: Game) zlm.Vec2 {
        if (!self.ball.stuck) return self.ball.sprite.position;
        const x = self.player.size.x / 2 - ballRadius;
        return self.player.position.add(zlm.Vec2.new(x, -ballRadius * 2));
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

    pub fn deinit(self: Game) void {
        for (self.levels) |level| level.deinit();
        resource.deinit();
    }
};

const GameLevel = struct {
    bricks: std.ArrayList(sprite.Sprite) = undefined,
    width: f32 = 0,
    height: f32 = 0,

    fn draw(self: GameLevel, renderer: SpriteRenderer) void {
        for (self.bricks.items) |brick| {
            renderer.draw(brick);
        }
    }

    fn deinit(self: GameLevel) void {
        self.bricks.deinit();
    }
    // fn isCompleted() bool{
    //     return false;
    // };
    fn init(self: *GameLevel, allocator: std.mem.Allocator, path: []const u8) !void {
        try self.doInit(allocator, path);
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

    fn parse(self: *GameLevel, allocator: std.mem.Allocator, level: FileLevel) !void {
        const size = level.width * level.height;
        self.bricks = try std.ArrayList(sprite.Sprite).initCapacity(allocator, size);

        const unitWidth = self.width / @as(f32, @floatFromInt(level.width));
        const unitHeight = self.height / @as(f32, @floatFromInt(level.height));

        for (level.level, 0..) |unit, index| {
            const x: f32 = @floatFromInt(index % level.width);
            const y: f32 = @floatFromInt(index / level.width);
            if (unit == 1) {
                try self.bricks.append(sprite.Sprite{
                    .position = zlm.Vec2.new(x * unitWidth, y * unitHeight),
                    .size = zlm.Vec2.new(unitWidth, unitHeight),
                    .texture = resource.getTexture(.solid_block),
                    .solid = true,
                });
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

            try self.bricks.append(sprite.Sprite{
                .position = zlm.Vec2.new(x * unitWidth, y * unitHeight),
                .size = zlm.Vec2.new(unitWidth, unitHeight),
                .texture = resource.getTexture(.block),
                .color = color,
            });
        }
    }
};

const FileLevel = struct {
    level: []const u8,
    width: usize,
    height: usize,
};
```

## resource.zig

对资源加载部分进行了修改，不用再提前进行加载。

```zig
const std = @import("std");
const zstbi = @import("zstbi");
const Texture2D = @import("texture.zig").Texture2D;
const Shader = @import("shader.zig").Shader;

pub const Texture2DEnum = enum { face, block, solid_block, background, paddle };
pub const ShaderEnum = enum { shader };

var textures: std.EnumMap(Texture2DEnum, Texture2D) = undefined;
var shaders: std.EnumMap(ShaderEnum, Shader) = undefined;

pub fn init(allocator: std.mem.Allocator) void {
    zstbi.init(allocator);

    textures = std.EnumMap(Texture2DEnum, Texture2D){};
    shaders = std.EnumMap(ShaderEnum, Shader){};
}

const cstr = [:0]const u8;
pub fn loadShader(name: ShaderEnum, vs: cstr, fs: cstr) Shader {
    const shader = Shader.init(vs, fs);
    shaders.put(name, shader);
    return shader;
}

pub fn getShader(name: ShaderEnum) Shader {
    return shaders.get(name).?;
}

fn loadTexture(name: Texture2DEnum, file: cstr) Texture2D {
    var image = zstbi.Image.loadFromFile(file, 4) catch unreachable;
    defer image.deinit();

    var texture = Texture2D{};
    texture.generate(image.width, image.height, image.data);

    textures.put(name, texture);
    return texture;
}

pub fn getTexture(name: Texture2DEnum) Texture2D {
    return textures.get(name) orelse loadTexture(name, switch (name) {
        .face => "assets/awesomeface.png",
        .block => "assets/block.png",
        .solid_block => "assets/block_solid.png",
        .background => "assets/background.jpg",
        .paddle => "assets/paddle.png",
    });
}

pub fn deinit() void {
    var textureIterator = textures.iterator();
    while (textureIterator.next()) |texture| texture.value.deinit();
    var shaderIterator = shaders.iterator();
    while (shaderIterator.next()) |shader| shader.value.deinit();
    zstbi.deinit();
}
```

## 效果

![一个小球][1]

## 总结

渲染了一个小球。

[1]: images/opengl24.gif

## 附录
