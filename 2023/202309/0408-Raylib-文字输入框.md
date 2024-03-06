# 0408-Raylib-文字输入框

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

输入框可以输入英文。

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

    // NOTE: Textures/Fonts MUST be loaded after Window initialization (OpenGL context is required)

    const msg: [:0]const u8 = "Signed Distance Fields";

    // Loading file to memory
    var fileSize: c_int = 0;
    const fileData: [*c]u8 = ray.LoadFileData("res/anonymous_pro_bold.ttf", &fileSize);

    // Default font generation from TTF font
    var fontDefault: ray.Font = .{
        .baseSize = 16,
        .glyphCount = 95,
        .glyphs = ray.LoadFontData(fileData, fileSize, 16, 0, 95, ray.FONT_DEFAULT),
    };

    // Loading font data from memory data
    // Parameters > font size: 16, no glyphs array provided (0), glyphs count: 95 (autogenerate chars array)
    // Parameters > glyphs count: 95, font size: 16, glyphs padding in image: 4 px, pack method: 0 (default)
    var atlas = ray.GenImageFontAtlas(fontDefault.glyphs, &fontDefault.recs, 95, 16, 4, 0);
    fontDefault.texture = ray.LoadTextureFromImage(atlas);
    ray.UnloadImage(atlas);

    // SDF font generation from TTF font
    var fontSDF: ray.Font = .{
        .baseSize = 16,
        .glyphCount = 95,
        // Parameters > font size: 16, no glyphs array provided (0), glyphs count: 0 (defaults to 95)
        .glyphs = ray.LoadFontData(fileData, fileSize, 16, 0, 0, ray.FONT_SDF),
    };

    // Parameters > glyphs count: 95, font size: 16, glyphs padding in image: 0 px, pack method: 1 (Skyline algorythm)
    atlas = ray.GenImageFontAtlas(fontSDF.glyphs, &fontSDF.recs, 95, 16, 0, 1);
    fontSDF.texture = ray.LoadTextureFromImage(atlas);
    ray.UnloadImage(atlas);

    ray.UnloadFileData(fileData); // Free memory from loaded file

    // Load SDF required shader (we use default vertex shader)
    const shader = ray.LoadShader(0, "res/sdf.fs");
    ray.SetTextureFilter(fontSDF.texture, ray.TEXTURE_FILTER_BILINEAR); // Required for SDF font

    var fontPosition = ray.Vector2{
        .x = 40,
        .y = @as(f32, @floatFromInt(screenHeight)) / 2.0 - 50,
    };
    var textSize = ray.Vector2{};
    var fontSize: f32 = 16.0;
    var currentFont: c_int = 0; // 0 - fontDefault, 1 - fontSDF

    while (!ray.WindowShouldClose()) {

        // Update
        fontSize += ray.GetMouseWheelMove() * 8.0;

        if (fontSize < 6) fontSize = 6;

        currentFont = if (ray.IsKeyDown(ray.KEY_SPACE)) 1 else 0;

        if (currentFont == 0)
            textSize = ray.MeasureTextEx(fontDefault, msg, fontSize, 0)
        else
            textSize = ray.MeasureTextEx(fontSDF, msg, fontSize, 0);

        fontPosition.x = @as(f32, @floatFromInt(ray.GetScreenWidth())) / 2 - textSize.x / 2;
        fontPosition.y = @as(f32, @floatFromInt(ray.GetScreenHeight())) / 2 - textSize.y / 2 + 80;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        if (currentFont == 1) {
            // NOTE: SDF fonts require a custom SDf shader to compute fragment color
            ray.BeginShaderMode(shader); // Activate SDF font shader
            ray.DrawTextEx(fontSDF, msg, fontPosition, fontSize, 0, ray.BLACK);
            ray.EndShaderMode(); // Activate our default shader for next drawings

            ray.DrawTexture(fontSDF.texture, 10, 10, ray.BLACK);
        } else {
            ray.DrawTextEx(fontDefault, msg, fontPosition, fontSize, 0, ray.BLACK);
            ray.DrawTexture(fontDefault.texture, 10, 10, ray.BLACK);
        }

        if (currentFont == 1)
            ray.DrawText("SDF!", 320, 20, 80, ray.RED)
        else
            ray.DrawText("default font", 315, 40, 30, ray.GRAY);

        ray.DrawText("FONT SIZE: 16.0", ray.GetScreenWidth() - 240, 20, 20, ray.DARKGRAY);
        ray.DrawText(ray.TextFormat("RENDER SIZE: %02.02f", fontSize), ray.GetScreenWidth() - 240, 50, 20, ray.DARKGRAY);
        ray.DrawText("Use MOUSE WHEEL to SCALE TEXT!", ray.GetScreenWidth() - 240, 90, 10, ray.DARKGRAY);

        ray.DrawText("HOLD SPACE to USE SDF FONT VERSION!", 340, ray.GetScreenHeight() - 30, 20, ray.MAROON);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![输入框][1]

## 总结

输入框可以输入英文。

[1]: images/raylib-text-input.png

## 附录
