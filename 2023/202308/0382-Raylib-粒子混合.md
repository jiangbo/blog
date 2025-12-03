# 0382-Raylib-粒子混合

## 环境

- Time 2024-02-29
- Zig 0.12.0-dev.3076+6e078883e
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

实现粒子混合的效果。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

// Particle structure with basic data
const Particle = struct {
    position: ray.Vector2 = .{},
    color: ray.Color,
    alpha: f32,
    size: f32,
    rotation: f32,
    active: bool = false, // NOTE: Use it to activate/deactive particle
};

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [shapes] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // Particles pool, reuse them!
    var mouseTail: [200]Particle = undefined;

    // Initialize particles
    for (&mouseTail) |*article| {
        article.color = .{
            .r = @intCast(ray.GetRandomValue(0, 255)),
            .g = @intCast(ray.GetRandomValue(0, 255)),
            .b = @intCast(ray.GetRandomValue(0, 255)),
            .a = 255,
        };
        article.alpha = 1.0;
        article.size = @as(f32, @floatFromInt(ray.GetRandomValue(1, 30))) / 20.0;
        article.rotation = @floatFromInt(ray.GetRandomValue(0, 360));
    }

    const gravity: f32 = 3.0;

    const smoke = ray.LoadTexture("res/spark_flame.png");

    var blending: c_int = ray.BLEND_ALPHA;

    while (!ray.WindowShouldClose()) {

        // Update
        // Activate one particle every frame and Update active particles
        // NOTE: Particles initial position should be mouse position when activated
        // NOTE: Particles fall down with gravity and rotation... and disappear after 2 seconds (alpha = 0)
        // NOTE: When a particle disappears, active = false and it can be reused.
        for (&mouseTail) |*article| {
            if (!article.active) {
                article.active = true;
                article.alpha = 1.0;
                article.position = ray.GetMousePosition();
                break;
            }
        }

        for (&mouseTail) |*article| {
            if (article.active) {
                article.position.y += gravity / 2;
                article.alpha -= 0.005;

                if (article.alpha <= 0.0) article.active = false;

                article.rotation += 2.0;
            }
        }

        if (ray.IsKeyPressed(ray.KEY_SPACE)) {
            if (blending == ray.BLEND_ALPHA)
                blending = ray.BLEND_ADDITIVE
            else
                blending = ray.BLEND_ALPHA;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.DARKGRAY);

        ray.BeginBlendMode(blending);

        // Draw active particles
        const width: f32 = @floatFromInt(smoke.width);
        const height: f32 = @floatFromInt(smoke.height);
        for (mouseTail) |article| {
            if (article.active) ray.DrawTexturePro(
                smoke,
                .{ .width = width, .height = height },
                .{
                    .x = article.position.x,
                    .y = article.position.y,
                    .width = width * article.size,
                    .height = height * article.size,
                },
                .{ .x = width * article.size / 2.0, .y = height * article.size / 2.0 },
                article.rotation,
                ray.Fade(article.color, article.alpha),
            );
        }

        ray.EndBlendMode();

        ray.DrawText("PRESS SPACE to CHANGE BLENDING MODE", 180, 20, 20, ray.BLACK);

        if (blending == ray.BLEND_ALPHA)
            ray.DrawText("ALPHA BLENDING", 290, screenHeight - 40, 20, ray.BLACK)
        else
            ray.DrawText("ADDITIVE BLENDING", 280, screenHeight - 40, 20, ray.RAYWHITE);

        ray.DrawFPS(10, 10);
    }
}
```

图片：<https://github.com/raysan5/raylib/blob/master/examples/textures/resources/spark_flame.png>

## 效果

![粒子混合][1]

## 总结

实现了粒子混合。

[1]: images/raylib-texture-particles.png

## 附录
