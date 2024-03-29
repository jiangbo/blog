# 0394-Raylib-加载原始数据

## 环境

- Time 2024-03-03
- Zig 0.12.0-dev.3076+6e078883e
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

加载原始数据到显卡。

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

    // Load RAW image data (512x512, 32bit RGBA, no file header)
    const fudesumiRaw = ray.LoadImageRaw("res/fudesumi.raw", 384, 512, ray.PIXELFORMAT_UNCOMPRESSED_R8G8B8A8, 0);
    const fudesumi = ray.LoadTextureFromImage(fudesumiRaw); // Upload CPU (RAM) image to GPU (VRAM)
    defer ray.UnloadTexture(fudesumi);
    ray.UnloadImage(fudesumiRaw); // Unload CPU (RAM) image data

    // Generate a checked texture by code
    const width = 960;
    const height = 480;

    // Dynamic memory allocation to store pixels data (Color type)
    const allocator = std.heap.c_allocator;
    var pixels = try allocator.alloc(ray.Color, width * height);

    for (0..height) |y| {
        for (0..width) |x| {
            if (((x / 32 + y / 32) / 1) % 2 == 0)
                pixels[y * width + x] = ray.ORANGE
            else
                pixels[y * width + x] = ray.GOLD;
        }
    }

    // Load pixels data into an image structure and create texture
    const checkedIm = ray.Image{
        .data = pixels.ptr, // We can assign pixels directly to data
        .width = width,
        .height = height,
        .format = ray.PIXELFORMAT_UNCOMPRESSED_R8G8B8A8,
        .mipmaps = 1,
    };

    const checked = ray.LoadTextureFromImage(checkedIm);
    defer ray.UnloadTexture(checked);
    ray.UnloadImage(checkedIm); // Unload CPU (RAM) image data (pixels)
    while (!ray.WindowShouldClose()) {

        // Update

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawTexture(checked, screenWidth / 2 - @divTrunc(checked.width, 2), screenHeight / 2 - @divTrunc(checked.height, 2), ray.Fade(ray.WHITE, 0.5));
        ray.DrawTexture(fudesumi, 430, -30, ray.WHITE);

        ray.DrawText("CHECKED TEXTURE ", 84, 85, 30, ray.BROWN);
        ray.DrawText("GENERATED by CODE", 72, 148, 30, ray.BROWN);
        ray.DrawText("and RAW IMAGE LOADING", 46, 210, 30, ray.BROWN);

        ray.DrawText("(c) Fudesumi sprite by Eiden Marsal", 310, screenHeight - 20, 10, ray.BROWN);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![原始数据][1]

## 总结

加载原始数据到显卡。

[1]: images/raylib-texture-rawdata.png

## 附录
