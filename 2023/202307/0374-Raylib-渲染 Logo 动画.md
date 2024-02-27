# 0374-Raylib-渲染 Logo 动画

## 环境

- Time 2024-02-27
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

渲染 Raylib Logo 的动画。

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

    const logoPositionX = screenWidth / 2 - 128;
    const logoPositionY = screenHeight / 2 - 128;

    var framesCounter: c_int = 0;
    var lettersCount: c_int = 0;

    var topSideRecWidth: c_int = 16;
    var leftSideRecHeight: c_int = 16;

    var bottomSideRecWidth: c_int = 16;
    var rightSideRecHeight: c_int = 16;

    var state: c_int = 0; // Tracking animation states (State Machine)
    var alpha: f32 = 1.0;

    while (!ray.WindowShouldClose()) {

        // Update
        if (state == 0) // State 0: Small box blinking
        {
            framesCounter += 1;

            if (framesCounter == 120) {
                state = 1;
                framesCounter = 0; // Reset counter... will be used later...
            }
        } else if (state == 1) // State 1: Top and left bars growing
        {
            topSideRecWidth += 4;
            leftSideRecHeight += 4;

            if (topSideRecWidth == 256) state = 2;
        } else if (state == 2) // State 2: Bottom and right bars growing
        {
            bottomSideRecWidth += 4;
            rightSideRecHeight += 4;

            if (bottomSideRecWidth == 256) state = 3;
        } else if (state == 3) // State 3: Letters appearing (one by one)
        {
            framesCounter += 1;

            if (@divTrunc(framesCounter, 12) == 0) // Every 12 frames, one more letter!
            {
                lettersCount += 1;
                framesCounter = 0;
            }

            if (lettersCount >= 10) // When all letters have appeared, just fade out everything
            {
                alpha -= 0.02;

                if (alpha <= 0.0) {
                    alpha = 0.0;
                    state = 4;
                }
            }
        } else if (state == 4) // State 4: Reset and Replay
        {
            if (ray.IsKeyPressed(ray.KEY_R)) {
                framesCounter = 0;
                lettersCount = 0;

                topSideRecWidth = 16;
                leftSideRecHeight = 16;

                bottomSideRecWidth = 16;
                rightSideRecHeight = 16;

                alpha = 1.0;
                state = 0; // Return to State 0
            }
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        if (state == 0) {
            if (@mod(@divTrunc(framesCounter, 15), 2) == 0)
                ray.DrawRectangle(logoPositionX, logoPositionY, 16, 16, ray.BLACK);
        } else if (state == 1) {
            ray.DrawRectangle(logoPositionX, logoPositionY, topSideRecWidth, 16, ray.BLACK);
            ray.DrawRectangle(logoPositionX, logoPositionY, 16, leftSideRecHeight, ray.BLACK);
        } else if (state == 2) {
            ray.DrawRectangle(logoPositionX, logoPositionY, topSideRecWidth, 16, ray.BLACK);
            ray.DrawRectangle(logoPositionX, logoPositionY, 16, leftSideRecHeight, ray.BLACK);

            ray.DrawRectangle(logoPositionX + 240, logoPositionY, 16, rightSideRecHeight, ray.BLACK);
            ray.DrawRectangle(logoPositionX, logoPositionY + 240, bottomSideRecWidth, 16, ray.BLACK);
        } else if (state == 3) {
            ray.DrawRectangle(logoPositionX, logoPositionY, topSideRecWidth, 16, ray.Fade(ray.BLACK, alpha));
            ray.DrawRectangle(logoPositionX, logoPositionY + 16, 16, leftSideRecHeight - 32, ray.Fade(ray.BLACK, alpha));

            ray.DrawRectangle(logoPositionX + 240, logoPositionY + 16, 16, rightSideRecHeight - 32, ray.Fade(ray.BLACK, alpha));
            ray.DrawRectangle(logoPositionX, logoPositionY + 240, bottomSideRecWidth, 16, ray.Fade(ray.BLACK, alpha));

            ray.DrawRectangle(@divTrunc(ray.GetScreenWidth(), 2) - 112, @divTrunc(ray.GetScreenHeight(), 2) - 112, 224, 224, ray.Fade(ray.RAYWHITE, alpha));

            ray.DrawText(ray.TextSubtext("raylib", 0, lettersCount), @divTrunc(ray.GetScreenWidth(), 2) - 44, @divTrunc(ray.GetScreenHeight(), 2) + 48, 50, ray.Fade(ray.BLACK, alpha));
        } else if (state == 4) {
            ray.DrawText("[R] REPLAY", 340, 200, 20, ray.GRAY);
        }
    }
}
```

## 效果

![Logo 动画][1]

## 总结

渲染 Raylib Logo 动画。

[1]: images/raylib-shapes-logo-anim.png

## 附录
