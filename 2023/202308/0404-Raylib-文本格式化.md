# 0404-Raylib-文本格式化

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

对需要显示的文本进行格式化。

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

    // Define characters to draw
    // NOTE: raylib supports UTF-8 encoding, following list is actually codified as UTF8 internally
    const msg: [*:0]const u8 =
        \\!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHI
        \\JKLMNOPQRSTUVWXYZ"[]^_`abcdefghijklmn
        \\opqrstuvwxyz{|}~¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓ
        \\ÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷
        \\øùúûüýþÿ
    ;
    // NOTE: Textures/Fonts MUST be loaded after Window initialization (OpenGL context is required)

    // BMFont (AngelCode) : Font data and image atlas have been generated using external program
    const fontBm = ray.LoadFont("res/pixantiqua.fnt");
    defer ray.UnloadFont(fontBm);

    // TTF font : Font data and atlas are generated directly from TTF
    // NOTE: We define a font base size of 32 pixels tall and up-to 250 characters
    const fontTtf = ray.LoadFontEx("res/pixantiqua.ttf", 32, 0, 250);
    defer ray.UnloadFont(fontTtf);
    ray.SetTextLineSpacing(48); // Set line spacing for multiline text (when line breaks are included '\n')
    var useTtf = false;

    while (!ray.WindowShouldClose()) {

        // Update
        //----------------------------------------------------------------------------------
        useTtf = ray.IsKeyDown(ray.KEY_SPACE);

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("Hold SPACE to use TTF generated font", 20, 20, 20, ray.LIGHTGRAY);

        if (!useTtf) {
            ray.DrawTextEx(fontBm, msg, .{ .x = 20.0, .y = 100.0 }, 32, 2, ray.MAROON);
            ray.DrawText("Using BMFont (Angelcode) imported", 20, ray.GetScreenHeight() - 30, 20, ray.GRAY);
        } else {
            const size: f32 = @floatFromInt(fontTtf.baseSize);
            ray.DrawTextEx(fontTtf, msg, .{ .x = 20.0, .y = 100.0 }, size, 2, ray.LIME);
            ray.DrawText("Using TTF font generated", 20, ray.GetScreenHeight() - 30, 20, ray.GRAY);
        }

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![格式化文本][1]

## 总结

对需要显示的文本进行格式化。

[1]: images/raylib-text-format.png

## 附录
