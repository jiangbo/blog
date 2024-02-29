# 0380-Raylib-图片转纹理

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

先将图片加载到内存中，然后转换到显存。

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

    // Loaded in CPU memory (RAM)
    const image = ray.LoadImage("res/raylib_logo.png");
    // Image converted to texture, GPU memory (VRAM)
    const texture = ray.LoadTextureFromImage(image);
    defer ray.UnloadTexture(texture);
    // Once image has been converted to texture and uploaded to VRAM, it can be unloaded from RAM
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

图片来源：<https://github.com/raysan5/raylib/blob/master/examples/textures/resources/raylib_logo.png>

## 效果

![图片转纹理][1]

应该和上一节的效果一致。

## 总结

先将图片加载到内存中，然后转换到显存。

[1]: images/raylib-texture-logo.png

## 附录
