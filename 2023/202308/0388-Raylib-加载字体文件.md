# 0388-Raylib-加载字体文件

## 环境

- Time 2024-03-01
- Zig 0.12.0-dev.3076+6e078883e
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

加载 TTF 字体文件并显示。

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

    var parrots = ray.LoadImage("res/parrots.png"); // Load image in CPU memory (RAM)

    // TTF Font loading with custom generation parameters
    const font = ray.LoadFontEx("resources/KAISG.ttf", 64, 0, 0);
    defer ray.UnloadFont(font);

    // Draw over image using custom font
    const size: f32 = 48;
    ray.ImageDrawTextEx(&parrots, font, "[Parrots font drawing]", .{ .x = 20.0, .y = 20.0 }, size, 0.0, ray.RED);

    const texture = ray.LoadTextureFromImage(parrots); // Image converted to texture, uploaded to GPU memory (VRAM)
    defer ray.UnloadTexture(texture);
    ray.UnloadImage(parrots); // Once image has been converted to texture and uploaded to VRAM, it can be unloaded from RAM

    const position = ray.Vector2{
        .x = @floatFromInt(screenWidth / 2 - @divTrunc(texture.width, 2)),
        .y = @floatFromInt(screenHeight / 2 - @divTrunc(texture.height, 2) - 20),
    };

    var showFont = false;
    while (!ray.WindowShouldClose()) {

        // Update
        showFont = ray.IsKeyDown(ray.KEY_SPACE);

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        if (!showFont) {
            // Draw texture with text already drawn inside
            ray.DrawTextureV(texture, position, ray.WHITE);

            // Draw text directly using sprite font
            ray.DrawTextEx(font, "[Parrots font drawing]", .{
                .x = position.x + 20,
                .y = position.y + 20 + 280,
            }, size, 0.0, ray.WHITE);
        } else {
            const x = screenWidth / 2 - @divTrunc(font.texture.width, 2);
            ray.DrawTexture(font.texture, x, 50, ray.BLACK);
        }

        ray.DrawText("PRESS SPACE to SHOW FONT ATLAS USED", 290, 420, 10, ray.DARKGRAY);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![程序生成图片][1]

## 总结

加载 TTF 字体文件并显示。

[1]: images/raylib-texture-ttf.png

## 附录
