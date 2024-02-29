# 0381-Raylib-纹理转图片

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

从纹理中取出图片。

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

    // Load image data into CPU memory (RAM)
    var image = ray.LoadImage("res/raylib_logo.png");
    // Image converted to texture, GPU memory (RAM -> VRAM)
    var texture = ray.LoadTextureFromImage(image);
    // Unload image data from CPU memory (RAM)
    ray.UnloadImage(image);

    // Load image from GPU texture (VRAM -> RAM)
    image = ray.LoadImageFromTexture(texture);
    // Unload texture from GPU memory (VRAM)
    ray.UnloadTexture(texture);

    // Recreate texture from retrieved image data (RAM -> VRAM)
    texture = ray.LoadTextureFromImage(image);
    ray.UnloadImage(image);

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

## 效果

应该和上一节的效果一致。

## 总结

从纹理中获得图片。

## 附录
