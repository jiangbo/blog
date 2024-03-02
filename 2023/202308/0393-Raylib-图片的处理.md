# 0393-Raylib-图片的处理

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

实现图片的处理。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const NUM_PROCESSES = 9;

const ImageProcess = enum {
    NONE,
    COLOR_GRAYSCALE,
    COLOR_TINT,
    COLOR_INVERT,
    COLOR_CONTRAST,
    COLOR_BRIGHTNESS,
    GAUSSIAN_BLUR,
    FLIP_VERTICAL,
    FLIP_HORIZONTAL,
};

const processText = [_][*c]const u8{
    "NO PROCESSING",
    "COLOR GRAYSCALE",
    "COLOR TINT",
    "COLOR INVERT",
    "COLOR CONTRAST",
    "COLOR BRIGHTNESS",
    "GAUSSIAN BLUR",
    "FLIP VERTICAL",
    "FLIP HORIZONTAL",
};

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [textures] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)
    var imOrigin = ray.LoadImage("res/parrots.png"); // Loaded in CPU memory (RAM)
    // Format image to RGBA 32bit (required for texture update) <-- ISSUE
    ray.ImageFormat(&imOrigin, ray.PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);
    // Image converted to texture, GPU memory (VRAM)
    const texture = ray.LoadTextureFromImage(imOrigin);
    defer ray.UnloadTexture(texture);

    var imCopy = ray.ImageCopy(imOrigin);

    var currentProcess: c_int = 0;
    var textureReload = false;

    var toggleRecs: [NUM_PROCESSES]ray.Rectangle = undefined;
    var mouseHoverRec: c_int = -1;

    for (0..NUM_PROCESSES) |i|
        toggleRecs[i] = .{
            .x = 40.0,
            .y = (50 + 32 * @as(f32, @floatFromInt(i))),
            .width = 150.0,
            .height = 30.0,
        };
    while (!ray.WindowShouldClose()) {

        // Update

        // Mouse toggle group logic
        for (0..NUM_PROCESSES) |i| {
            if (ray.CheckCollisionPointRec(ray.GetMousePosition(), toggleRecs[i])) {
                mouseHoverRec = @intCast(i);

                if (ray.IsMouseButtonReleased(ray.MOUSE_BUTTON_LEFT)) {
                    currentProcess = @intCast(i);
                    textureReload = true;
                }
                break;
            } else mouseHoverRec = -1;
        }

        // Keyboard toggle group logic
        if (ray.IsKeyPressed(ray.KEY_DOWN)) {
            currentProcess += 1;
            if (currentProcess > (NUM_PROCESSES - 1)) currentProcess = 0;
            textureReload = true;
        } else if (ray.IsKeyPressed(ray.KEY_UP)) {
            currentProcess -= 1;
            if (currentProcess < 0) currentProcess = 7;
            textureReload = true;
        }

        // Reload texture when required
        if (textureReload) {
            ray.UnloadImage(imCopy); // Unload image-copy data
            imCopy = ray.ImageCopy(imOrigin); // Restore image-copy from image-origin

            // NOTE: Image processing is a costly CPU process to be done every frame,
            // If image processing is required in a frame-basis, it should be done
            // with a texture and by shaders
            const process: ImageProcess = @enumFromInt(currentProcess);
            switch (process) {
                ImageProcess.COLOR_GRAYSCALE => ray.ImageColorGrayscale(&imCopy),
                ImageProcess.COLOR_TINT => ray.ImageColorTint(&imCopy, ray.GREEN),
                ImageProcess.COLOR_INVERT => ray.ImageColorInvert(&imCopy),
                ImageProcess.COLOR_CONTRAST => ray.ImageColorContrast(&imCopy, -40),
                ImageProcess.COLOR_BRIGHTNESS => ray.ImageColorBrightness(&imCopy, -80),
                ImageProcess.GAUSSIAN_BLUR => ray.ImageBlurGaussian(&imCopy, 10),
                ImageProcess.FLIP_VERTICAL => ray.ImageFlipVertical(&imCopy),
                ImageProcess.FLIP_HORIZONTAL => ray.ImageFlipHorizontal(&imCopy),
                ImageProcess.NONE => {},
            }

            const pixels = ray.LoadImageColors(imCopy); // Load pixel data from image (RGBA 32bit)
            ray.UpdateTexture(texture, pixels); // Update texture with new image data
            ray.UnloadImageColors(pixels); // Unload pixels data from RAM

            textureReload = false;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("IMAGE PROCESSING:", 40, 30, 10, ray.DARKGRAY);

        // Draw rectangles
        for (0..NUM_PROCESSES) |i| {
            ray.DrawRectangleRec(toggleRecs[i], if ((i == currentProcess) or (i == mouseHoverRec)) ray.SKYBLUE else ray.LIGHTGRAY);

            const x: c_int = @intFromFloat(toggleRecs[i].x);
            const y: c_int = @intFromFloat(toggleRecs[i].y);
            const width: c_int = @intFromFloat(toggleRecs[i].width);
            const height: c_int = @intFromFloat(toggleRecs[i].height);
            ray.DrawRectangleLines(x, y, width, height, if ((i == currentProcess) or (i == mouseHoverRec)) ray.BLUE else ray.GRAY);
            ray.DrawText(processText[i], (x + @divTrunc(width, 2) - @divTrunc(ray.MeasureText(processText[i], 10), 2)), y + 11, 10, if ((i == currentProcess) or (i == mouseHoverRec)) ray.DARKBLUE else ray.DARKGRAY);
        }

        ray.DrawTexture(texture, screenWidth - texture.width - 60, screenHeight / 2 - @divTrunc(texture.height, 2), ray.WHITE);
        ray.DrawRectangleLines(screenWidth - texture.width - 60, screenHeight / 2 - @divTrunc(texture.height, 2), texture.width, texture.height, ray.BLACK);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![图片处理][1]

## 总结

实现图片的处理。

[1]: images/raylib-texture-process.png

## 附录
