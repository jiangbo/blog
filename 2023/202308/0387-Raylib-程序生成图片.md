# 0387-Raylib-程序生成图片

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

通过程序来生成图片并显示。

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

    const verticalGradient = ray.GenImageGradientLinear(screenWidth, screenHeight, 0, ray.RED, ray.BLUE);
    const horizontalGradient = ray.GenImageGradientLinear(screenWidth, screenHeight, 90, ray.RED, ray.BLUE);
    const diagonalGradient = ray.GenImageGradientLinear(screenWidth, screenHeight, 45, ray.RED, ray.BLUE);
    const radialGradient = ray.GenImageGradientRadial(screenWidth, screenHeight, 0.0, ray.WHITE, ray.BLACK);
    const squareGradient = ray.GenImageGradientSquare(screenWidth, screenHeight, 0.0, ray.WHITE, ray.BLACK);
    const checked = ray.GenImageChecked(screenWidth, screenHeight, 32, 32, ray.RED, ray.BLUE);
    const whiteNoise = ray.GenImageWhiteNoise(screenWidth, screenHeight, 0.5);
    const perlinNoise = ray.GenImagePerlinNoise(screenWidth, screenHeight, 50, 50, 4.0);
    const cellular = ray.GenImageCellular(screenWidth, screenHeight, 32);

    const textures = [_]ray.Texture2D{
        ray.LoadTextureFromImage(verticalGradient),
        ray.LoadTextureFromImage(horizontalGradient),
        ray.LoadTextureFromImage(diagonalGradient),
        ray.LoadTextureFromImage(radialGradient),
        ray.LoadTextureFromImage(squareGradient),
        ray.LoadTextureFromImage(checked),
        ray.LoadTextureFromImage(whiteNoise),
        ray.LoadTextureFromImage(perlinNoise),
        ray.LoadTextureFromImage(cellular),
    };

    // Unload image data (CPU RAM)
    ray.UnloadImage(verticalGradient);
    ray.UnloadImage(horizontalGradient);
    ray.UnloadImage(diagonalGradient);
    ray.UnloadImage(radialGradient);
    ray.UnloadImage(squareGradient);
    ray.UnloadImage(checked);
    ray.UnloadImage(whiteNoise);
    ray.UnloadImage(perlinNoise);
    ray.UnloadImage(cellular);

    var currentTexture: usize = 0;
    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_LEFT) or ray.IsKeyPressed(ray.KEY_RIGHT)) {
            currentTexture = (currentTexture + 1) % textures.len; // Cycle between the textures
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawTexture(textures[currentTexture], 0, 0, ray.WHITE);

        ray.DrawRectangle(30, 400, 325, 30, ray.Fade(ray.SKYBLUE, 0.5));
        ray.DrawRectangleLines(30, 400, 325, 30, ray.Fade(ray.WHITE, 0.5));
        ray.DrawText("MOUSE LEFT BUTTON to CYCLE PROCEDURAL TEXTURES", 40, 410, 10, ray.WHITE);

        switch (currentTexture) {
            0 => ray.DrawText("VERTICAL GRADIENT", 560, 10, 20, ray.RAYWHITE),
            1 => ray.DrawText("HORIZONTAL GRADIENT", 540, 10, 20, ray.RAYWHITE),
            2 => ray.DrawText("DIAGONAL GRADIENT", 540, 10, 20, ray.RAYWHITE),
            3 => ray.DrawText("RADIAL GRADIENT", 580, 10, 20, ray.LIGHTGRAY),
            4 => ray.DrawText("SQUARE GRADIENT", 580, 10, 20, ray.LIGHTGRAY),
            5 => ray.DrawText("CHECKED", 680, 10, 20, ray.RAYWHITE),
            6 => ray.DrawText("WHITE NOISE", 640, 10, 20, ray.RED),
            7 => ray.DrawText("PERLIN NOISE", 640, 10, 20, ray.RED),
            8 => ray.DrawText("CELLULAR", 670, 10, 20, ray.RAYWHITE),
            else => {},
        }

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![程序生成图片][1]

## 总结

通过程序来生成图片并显示。

[1]: images/raylib-texture-generation.png

## 附录
