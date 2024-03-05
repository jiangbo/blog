# 0402-Raylib-加载精灵字体

## 环境

- Time 2024-03-05
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

使用自定义的精灵图片字体来显示文字。

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

    const msg1: [*:0]const u8 = "THIS IS A custom SPRITE FONT...";
    const msg2: [*:0]const u8 = "...and this is ANOTHER CUSTOM font...";
    const msg3: [*:0]const u8 = "...and a THIRD one! GREAT! :D";

    // NOTE: Textures/Fonts MUST be loaded after Window initialization (OpenGL context is required)
    const font1 = ray.LoadFont("res/custom_mecha.png"); // Font loading
    defer ray.UnloadFont(font1);
    const font2 = ray.LoadFont("res/custom_alagard.png"); // Font loading
    defer ray.UnloadFont(font2);
    const font3 = ray.LoadFont("res/custom_jupiter_crash.png"); // Font loading
    defer ray.UnloadFont(font3);

    const screenWidthF: f32 = @floatFromInt(screenWidth);
    const screenHeightF: f32 = @floatFromInt(screenHeight);
    const size1: f32 = @floatFromInt(font1.baseSize);
    const fontPosition1 = ray.Vector2{
        .x = screenWidthF / 2.0 - ray.MeasureTextEx(font1, msg1, size1, -3).x / 2,
        .y = screenHeightF / 2.0 - size1 / 2.0 - 80.0,
    };

    const size2: f32 = @floatFromInt(font2.baseSize);
    const fontPosition2 = ray.Vector2{
        .x = screenWidthF / 2.0 - ray.MeasureTextEx(font2, msg2, size2, -2.0).x / 2.0,
        .y = screenHeightF / 2.0 - size2 / 2.0 - 10.0,
    };

    const size3: f32 = @floatFromInt(font3.baseSize);
    const fontPosition3 = ray.Vector2{
        .x = screenWidthF / 2.0 - ray.MeasureTextEx(font3, msg3, size3, 2.0).x / 2.0,
        .y = screenHeightF / 2.0 - size3 / 2.0 + 50.0,
    };

    while (!ray.WindowShouldClose()) {

        // Update

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawTextEx(font1, msg1, fontPosition1, size1, -3, ray.WHITE);
        ray.DrawTextEx(font2, msg2, fontPosition2, size2, -2, ray.WHITE);
        ray.DrawTextEx(font3, msg3, fontPosition3, size3, 2, ray.WHITE);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![精灵字体][1]

## 总结

使用自定义的精灵图片字体来显示文字。

[1]: images/raylib-text-sprite.png

## 附录
