# 0405-Raylib-字体过滤

## 环境

- Time 2024-03-06
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

对显示的字体进行放大，移动和过滤。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [text] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    const msg: [:0]const u8 = "Loaded Font";

    // NOTE: Textures/Fonts MUST be loaded after Window initialization (OpenGL context is required)

    // TTF Font loading with custom generation parameters
    var font = ray.LoadFontEx("res/KAISG.ttf", 96, 0, 0);
    defer ray.UnloadFont(font);

    // Generate mipmap levels to use trilinear filtering
    // NOTE: On 2D drawing it won't be noticeable, it looks like FILTER_BILINEAR
    ray.GenTextureMipmaps(&font.texture);

    var fontSize: f32 = @floatFromInt(font.baseSize);
    var fontPosition = ray.Vector2{
        .x = 40.0,
        .y = @as(f32, @floatFromInt(screenHeight)) / 2.0 - 80.0,
    };
    var textSize = ray.Vector2{};

    // Setup texture scaling filter
    ray.SetTextureFilter(font.texture, ray.TEXTURE_FILTER_POINT);
    var currentFontFilter: c_int = 0; // TEXTURE_FILTER_POINT

    while (!ray.WindowShouldClose()) {

        // Update
        fontSize += ray.GetMouseWheelMove() * 4.0;

        // Choose font texture filter method
        if (ray.IsKeyPressed(ray.KEY_ONE)) {
            ray.SetTextureFilter(font.texture, ray.TEXTURE_FILTER_POINT);
            currentFontFilter = 0;
        } else if (ray.IsKeyPressed(ray.KEY_TWO)) {
            ray.SetTextureFilter(font.texture, ray.TEXTURE_FILTER_BILINEAR);
            currentFontFilter = 1;
        } else if (ray.IsKeyPressed(ray.KEY_THREE)) {
            // NOTE: Trilinear filter won't be noticed on 2D drawing
            ray.SetTextureFilter(font.texture, ray.TEXTURE_FILTER_TRILINEAR);
            currentFontFilter = 2;
        }

        textSize = ray.MeasureTextEx(font, msg, fontSize, 0);

        if (ray.IsKeyDown(ray.KEY_LEFT))
            fontPosition.x -= 10
        else if (ray.IsKeyDown(ray.KEY_RIGHT)) fontPosition.x += 10;

        // Load a dropped TTF file dynamically (at current fontSize)
        if (ray.IsFileDropped()) {
            const droppedFiles = ray.LoadDroppedFiles();

            // NOTE: We only support first ttf file dropped
            if (ray.IsFileExtension(droppedFiles.paths[0], ".ttf")) {
                ray.UnloadFont(font);
                const size: c_int = @intFromFloat(fontSize);
                font = ray.LoadFontEx(droppedFiles.paths[0], size, 0, 0);
            }

            ray.UnloadDroppedFiles(droppedFiles); // Unload filepaths from memory
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("Use mouse wheel to change font size", 20, 20, 10, ray.GRAY);
        ray.DrawText("Use KEY_RIGHT and KEY_LEFT to move text", 20, 40, 10, ray.GRAY);
        ray.DrawText("Use 1, 2, 3 to change texture filter", 20, 60, 10, ray.GRAY);
        ray.DrawText("Drop a new TTF font for dynamic loading", 20, 80, 10, ray.DARKGRAY);

        ray.DrawTextEx(font, msg, fontPosition, fontSize, 0, ray.BLACK);

        // TODO: It seems texSize measurement is not accurate due to chars offsets...
        //DrawRectangleLines(fontPosition.x, fontPosition.y, textSize.x, textSize.y, RED);

        ray.DrawRectangle(0, screenHeight - 80, screenWidth, 80, ray.LIGHTGRAY);
        ray.DrawText(ray.TextFormat("Font size: %02.02f", fontSize), 20, screenHeight - 50, 10, ray.DARKGRAY);
        ray.DrawText(ray.TextFormat("Text size: [%02.02f, %02.02f]", textSize.x, textSize.y), 20, screenHeight - 30, 10, ray.DARKGRAY);
        ray.DrawText("CURRENT TEXTURE FILTER:", 250, 400, 20, ray.GRAY);

        if (currentFontFilter == 0)
            ray.DrawText("POINT", 570, 400, 20, ray.BLACK)
        else if (currentFontFilter == 1)
            ray.DrawText("BILINEAR", 570, 400, 20, ray.BLACK)
        else if (currentFontFilter == 2)
            ray.DrawText("TRILINEAR", 570, 400, 20, ray.BLACK);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![字体过滤][1]

## 总结

对显示的字体进行放大，移动和过滤。

[1]: images/raylib-text-filter.png

## 附录
