# 0498-OpenGL-玩家挡板控制

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

控制最底部的挡板，可以左右移动。

## Game

```zig
const std = @import("std");
const zlm = @import("zlm");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const resource = @import("resource.zig");
const SpriteRenderer = @import("renderer.zig").SpriteRenderer;
const Sprite = @import("sprite.zig").Sprite;

const Allocator = std.mem.Allocator;
const GameState = enum { active, menu, win };
const playerSpeed: f32 = 500;
pub const Game = struct {
    state: GameState = .active,
    width: f32,
    height: f32,
    keys: [1024]bool = [_]bool{false} ** 1024,
    spriteRenderer: SpriteRenderer = undefined,
    levels: [4]GameLevel = undefined,
    level: usize = 0,
    player: Sprite = undefined,

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

        _ = try resource.loadTexture(.face, "assets/awesomeface.png");
        _ = try resource.loadTexture(.block, "assets/block.png");
        _ = try resource.loadTexture(.solid_block, "assets/block_solid.png");
        _ = try resource.loadTexture(.background, "assets/background.jpg");
        _ = try resource.loadTexture(.paddle, "assets/paddle.png");

        self.levels[0] = .{ .width = self.width, .height = self.height / 2 };
        try self.levels[0].init(allocator, "assets/lv1.json");
        self.levels[1] = .{ .width = self.width, .height = self.height / 2 };
        try self.levels[1].init(allocator, "assets/lv2.json");
        self.levels[2] = .{ .width = self.width, .height = self.height / 2 };
        try self.levels[2].init(allocator, "assets/lv3.json");
        self.levels[3] = .{ .width = self.width, .height = self.height / 2 };
        try self.levels[3].init(allocator, "assets/lv4.json");

        self.player = Sprite{
            .texture = resource.getTexture(.paddle),
            .position = zlm.Vec2.new(self.width / 2 - 50, self.height - 20),
            .size = zlm.Vec2.new(100, 20),
        };
    }

    pub fn processInput(self: *Game, deltaTime: f64) void {
        if (self.state != .active) return;

        const speed = playerSpeed * @as(f32, @floatCast(deltaTime));

        if (self.keys[@as(usize, @intCast(glfw.Key.a.getScancode()))]) {
            self.player.position.x -= speed;
            if (self.player.position.x < 0) self.player.position.x = 0;
        }

        if (self.keys[@as(usize, @intCast(glfw.Key.d.getScancode()))]) {
            self.player.position.x += speed;
            const maxX = self.width - self.player.size.x;
            if (self.player.position.x > maxX) self.player.position.x = maxX;
        }
    }
    pub fn update(self: Game, deltaTime: f64) void {
        _ = self;
        _ = deltaTime;
    }

    pub fn render(self: Game) void {
        if (self.state == .active) {
            const background = resource.getTexture(.background);
            self.spriteRenderer.draw(Sprite{
                .texture = background,
                .size = zlm.Vec2.new(self.width, self.height),
            });

            self.levels[self.level].draw(self.spriteRenderer);
            self.spriteRenderer.draw(self.player);
        }
    }

    pub fn deinit(self: Game) void {
        for (self.levels) |level| level.deinit();
        resource.deinit();
    }
};
```

## 效果

![玩家控制挡板][1]

## 总结

玩家可以控制挡板左右移动，但是不能超出屏幕。

[1]: images/opengl23.gif

## 附录
