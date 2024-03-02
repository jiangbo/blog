# 0392-Raylib-图片的旋转

## 环境

- Time 2024-03-02
- Zig 0.12.0-dev.3076+6e078883e
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

实现图片的旋转。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [textures] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)

    const scarfy = ray.LoadTexture("res/scarfy.png"); // Texture loading
    defer ray.UnloadTexture(scarfy);

    const frameWidth: f32 = @floatFromInt(@divTrunc(scarfy.width, 6));
    const frameHeight: f32 = @floatFromInt(scarfy.height);

    // Source rectangle (part of the texture to use for drawing)
    const sourceRec = ray.Rectangle{ .width = frameWidth, .height = frameHeight };

    // Destination rectangle (screen rectangle where drawing part of texture)
    const destRec = ray.Rectangle{
        .x = @as(f32, @floatFromInt(screenWidth)) / 2.0,
        .y = @as(f32, @floatFromInt(screenHeight)) / 2.0,
        .width = frameWidth * 2.0,
        .height = frameHeight * 2.0,
    };

    // Origin of the texture (rotation/scale point), it's relative to destination rectangle size
    const origin = ray.Vector2{ .x = frameWidth, .y = frameHeight };

    var rotation: f32 = 0;

    while (!ray.WindowShouldClose()) {

        // Update
        rotation += 1;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        // NOTE: Using DrawTexturePro() we can easily rotate and scale the part of the texture we draw
        // sourceRec defines the part of the texture we use for drawing
        // destRec defines the rectangle where our texture part will fit (scaling it to fit)
        // origin defines the point of the texture used as reference for rotation and scaling
        // rotation defines the texture rotation (using origin as rotation point)
        ray.DrawTexturePro(scarfy, sourceRec, destRec, origin, rotation, ray.WHITE);

        const x: c_int = @intFromFloat(destRec.x);
        const y: c_int = @intFromFloat(destRec.y);
        ray.DrawLine(x, 0, x, screenHeight, ray.GRAY);
        ray.DrawLine(0, y, screenWidth, y, ray.GRAY);

        ray.DrawText("(c) Scarfy sprite by Eiden Marsal", screenWidth - 200, screenHeight - 20, 10, ray.GRAY);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![图片旋转][1]

## 总结

实现图片的旋转。

[1]: images/raylib-texture-rotation.png

## 附录
