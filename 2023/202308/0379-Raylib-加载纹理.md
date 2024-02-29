# 0379-Raylib-加载纹理

## 环境

- Time 2024-02-29
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

使用纹理来进行加载和显示。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [shapes] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)
    const texture = ray.LoadTexture("res/raylib_logo.png"); // Texture loading
    defer ray.UnloadTexture(texture);

    while (!ray.WindowShouldClose()) {

        // Update

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        const x = screenWidth / 2 - @divTrunc(texture.width, 2);
        const y = screenHeight / 2 - @divTrunc(texture.height, 2);
        ray.DrawTexture(texture, x, y, ray.WHITE);
        ray.DrawText("this IS a texture!", 360, 370, 10, ray.GRAY);

        ray.DrawFPS(10, 10);
    }
}
```

图片来源：<https://github.com/raysan5/raylib/blob/master/examples/textures/resources/raylib_logo.png>

## 效果

![纹理的加载][1]

## 总结

加载和使用纹理。

[1]: images/raylib-texture-logo.png

## 附录
