# 0497-OpenGL-渲染关卡

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

将解析后的关卡数据渲染到屏幕。

## Game

```zig
pub const Game = struct {
    state: GameState = .active,
    width: f32,
    height: f32,
    keys: [1024]bool = [_]bool{false} ** 1024,
    spriteRenderer: SpriteRenderer = undefined,
    levels: [4]GameLevel = undefined,
    level: usize = 0,

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

        self.levels[0] = .{ .width = self.width, .height = self.height / 2 };
        try self.levels[0].init(allocator, "assets/lv1.json");
        self.levels[1] = .{ .width = self.width, .height = self.height / 2 };
        try self.levels[1].init(allocator, "assets/lv2.json");
        self.levels[2] = .{ .width = self.width, .height = self.height / 2 };
        try self.levels[2].init(allocator, "assets/lv3.json");
        self.levels[3] = .{ .width = self.width, .height = self.height / 2 };
        try self.levels[3].init(allocator, "assets/lv4.json");
    }
    // game loop
    pub fn processInput(self: *Game, deltaTime: f64) void {
        _ = deltaTime;
        _ = self;
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
        }
    }

    pub fn deinit(self: Game) void {
        for (self.levels) |level| level.deinit();
        resource.deinit();
    }
};
```

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
```

## 效果

![渲染关卡][1]

## 总结

将关卡数据渲染到屏幕。

[1]: images/opengl22.png

## 附录
