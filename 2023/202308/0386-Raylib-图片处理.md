# 0386-Raylib-图片处理

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

图片的各种变换处理。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [texture] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)

    var cat = ray.LoadImage("res/cat.png"); // Load image in CPU memory (RAM)
    ray.ImageCrop(&cat, .{ .x = 100, .y = 10, .width = 280, .height = 380 }); // Crop an image piece
    ray.ImageFlipHorizontal(&cat); // Flip cropped image horizontally
    ray.ImageResize(&cat, 150, 200); // Resize flipped-cropped image

    var parrots = ray.LoadImage("res/parrots.png"); // Load image in CPU memory (RAM)

    // Draw one image over the other with a scaling of 1.5f
    var width: f32 = @floatFromInt(cat.width);
    var height: f32 = @floatFromInt(cat.height);
    ray.ImageDraw(
        &parrots,
        cat,
        .{ .width = width, .height = height },
        .{ .x = 30, .y = 40, .width = width * 1.5, .height = height * 1.5 },
        ray.WHITE,
    );

    width = @floatFromInt(parrots.width);
    height = @floatFromInt(parrots.height);
    ray.ImageCrop(&parrots, .{ .y = 50, .width = width, .height = height - 100 }); // Crop resulting image

    // Draw on the image with a few image draw methods
    ray.ImageDrawPixel(&parrots, 10, 10, ray.RAYWHITE);
    ray.ImageDrawCircleLines(&parrots, 10, 10, 5, ray.RAYWHITE);
    ray.ImageDrawRectangle(&parrots, 5, 20, 10, 10, ray.RAYWHITE);

    ray.UnloadImage(cat); // Unload image from RAM

    // Load custom font for frawing on image
    const font = ray.LoadFont("res/custom_jupiter_crash.png");

    const size: f32 = @floatFromInt(font.baseSize);
    // Draw over image using custom font
    ray.ImageDrawTextEx(&parrots, font, "PARROTS & CAT", .{ .x = 300, .y = 230 }, size, -2, ray.WHITE);

    ray.UnloadFont(font); // Unload custom font (already drawn used on image)

    // Image converted to texture, uploaded to GPU memory (VRAM)
    const texture = ray.LoadTextureFromImage(parrots);
    // Once image has been converted to texture and uploaded to VRAM, it can be unloaded from RAM
    ray.UnloadImage(parrots);

    while (!ray.WindowShouldClose()) {

        // Update

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        const x = @divTrunc(screenWidth, 2) - @divTrunc(texture.width, 2);
        const y = @divTrunc(screenHeight, 2) - @divTrunc(texture.height, 2);
        ray.DrawTexture(texture, x, y - 40, ray.WHITE);
        ray.DrawRectangleLines(x, y - 40, texture.width, texture.height, ray.DARKGRAY);

        ray.DrawText("We are drawing only one texture from various images composed!", 240, 350, 10, ray.DARKGRAY);
        ray.DrawText("Source images have been cropped, scaled, flipped and copied one over the other.", 190, 370, 10, ray.DARKGRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

图片：<https://github.com/raysan5/raylib/blob/master/examples/textures/resources/>

## 效果

![图片处理][1]

## 总结

图片的各种处理。

[1]: images/raylib-texture-image.png

## 附录
