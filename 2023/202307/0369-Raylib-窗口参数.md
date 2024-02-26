# 0369-Raylib-窗口参数

## 环境

- Time 2024-02-26
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

试用一些控制窗口显示的参数，通过不同的按键来控制。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    // Set configuration flags for window creation
    // ray.SetConfigFlags(ray.FLAG_VSYNC_HINT | ray.FLAG_MSAA_4X_HINT | ray.FLAG_WINDOW_HIGHDPI);
    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var ballPosition = ray.Vector2{
        .x = @as(f32, @floatFromInt(ray.GetScreenWidth())) / 2.0,
        .y = @as(f32, @floatFromInt(ray.GetScreenHeight())) / 2.0,
    };
    var ballSpeed = ray.Vector2{ .x = 5.0, .y = 4.0 };
    const ballRadius: f32 = 20;

    var framesCounter: c_int = 0;

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsKeyPressed(ray.KEY_F)) ray.ToggleFullscreen(); // modifies window size when scaling!

        if (ray.IsKeyPressed(ray.KEY_R)) {
            if (ray.IsWindowState(ray.FLAG_WINDOW_RESIZABLE))
                ray.ClearWindowState(ray.FLAG_WINDOW_RESIZABLE)
            else
                ray.SetWindowState(ray.FLAG_WINDOW_RESIZABLE);
        }

        if (ray.IsKeyPressed(ray.KEY_D)) {
            if (ray.IsWindowState(ray.FLAG_WINDOW_UNDECORATED))
                ray.ClearWindowState(ray.FLAG_WINDOW_UNDECORATED)
            else
                ray.SetWindowState(ray.FLAG_WINDOW_UNDECORATED);
        }

        if (ray.IsKeyPressed(ray.KEY_H)) {
            if (!ray.IsWindowState(ray.FLAG_WINDOW_HIDDEN))
                ray.SetWindowState(ray.FLAG_WINDOW_HIDDEN);
            framesCounter = 0;
        }

        if (ray.IsWindowState(ray.FLAG_WINDOW_HIDDEN)) {
            framesCounter += 1;
            if (framesCounter >= 240)
                ray.ClearWindowState(ray.FLAG_WINDOW_HIDDEN); // Show window after 3 seconds
        }

        if (ray.IsKeyPressed(ray.KEY_N)) {
            if (!ray.IsWindowState(ray.FLAG_WINDOW_MINIMIZED)) ray.MinimizeWindow();

            framesCounter = 0;
        }

        if (ray.IsWindowState(ray.FLAG_WINDOW_MINIMIZED)) {
            framesCounter += 1;
            if (framesCounter >= 240) ray.RestoreWindow(); // Restore window after 3 seconds
        }

        if (ray.IsKeyPressed(ray.KEY_M)) {
            // NOTE: Requires FLAG_WINDOW_RESIZABLE enabled!
            if (ray.IsWindowState(ray.FLAG_WINDOW_MAXIMIZED))
                ray.RestoreWindow()
            else
                ray.MaximizeWindow();
        }

        if (ray.IsKeyPressed(ray.KEY_U)) {
            if (ray.IsWindowState(ray.FLAG_WINDOW_UNFOCUSED))
                ray.ClearWindowState(ray.FLAG_WINDOW_UNFOCUSED)
            else
                ray.SetWindowState(ray.FLAG_WINDOW_UNFOCUSED);
        }

        if (ray.IsKeyPressed(ray.KEY_T)) {
            if (ray.IsWindowState(ray.FLAG_WINDOW_TOPMOST))
                ray.ClearWindowState(ray.FLAG_WINDOW_TOPMOST)
            else
                ray.SetWindowState(ray.FLAG_WINDOW_TOPMOST);
        }

        if (ray.IsKeyPressed(ray.KEY_A)) {
            if (ray.IsWindowState(ray.FLAG_WINDOW_ALWAYS_RUN))
                ray.ClearWindowState(ray.FLAG_WINDOW_ALWAYS_RUN)
            else
                ray.SetWindowState(ray.FLAG_WINDOW_ALWAYS_RUN);
        }

        if (ray.IsKeyPressed(ray.KEY_V)) {
            if (ray.IsWindowState(ray.FLAG_VSYNC_HINT))
                ray.ClearWindowState(ray.FLAG_VSYNC_HINT)
            else
                ray.SetWindowState(ray.FLAG_VSYNC_HINT);
        }

        // Bouncing ball logic
        ballPosition.x += ballSpeed.x;
        ballPosition.y += ballSpeed.y;
        const width: f32 = @floatFromInt(ray.GetScreenWidth());
        const height: f32 = @floatFromInt(ray.GetScreenHeight());
        if ((ballPosition.x >= (width - ballRadius)) or (ballPosition.x <= ballRadius)) ballSpeed.x *= -1.0;
        if ((ballPosition.y >= (height - ballRadius)) or (ballPosition.y <= ballRadius)) ballSpeed.y *= -1.0;

        // Draw

        ray.BeginDrawing();
        defer ray.EndDrawing();

        if (ray.IsWindowState(ray.FLAG_WINDOW_TRANSPARENT))
            ray.ClearBackground(ray.BLANK)
        else
            ray.ClearBackground(ray.RAYWHITE);

        ray.DrawCircleV(ballPosition, ballRadius, ray.MAROON);
        ray.DrawRectangleLinesEx(.{ .width = width, .height = height }, 4, ray.RAYWHITE);

        ray.DrawCircleV(ray.GetMousePosition(), 10, ray.DARKBLUE);

        ray.DrawFPS(10, 10);

        ray.DrawText(ray.TextFormat("Screen Size: [%i, %i]", ray.GetScreenWidth(), ray.GetScreenHeight()), 10, 40, 10, ray.GREEN);

        // Draw window state info
        ray.DrawText("Following flags can be set after window creation:", 10, 60, 10, ray.GRAY);
        if (ray.IsWindowState(ray.FLAG_FULLSCREEN_MODE))
            ray.DrawText("[F] FLAG_FULLSCREEN_MODE: on", 10, 80, 10, ray.LIME)
        else
            ray.DrawText("[F] FLAG_FULLSCREEN_MODE: off", 10, 80, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_RESIZABLE))
            ray.DrawText("[R] FLAG_WINDOW_RESIZABLE: on", 10, 100, 10, ray.LIME)
        else
            ray.DrawText("[R] FLAG_WINDOW_RESIZABLE: off", 10, 100, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_UNDECORATED))
            ray.DrawText("[D] FLAG_WINDOW_UNDECORATED: on", 10, 120, 10, ray.LIME)
        else
            ray.DrawText("[D] FLAG_WINDOW_UNDECORATED: off", 10, 120, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_HIDDEN))
            ray.DrawText("[H] FLAG_WINDOW_HIDDEN: on", 10, 140, 10, ray.LIME)
        else
            ray.DrawText("[H] FLAG_WINDOW_HIDDEN: off", 10, 140, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_MINIMIZED))
            ray.DrawText("[N] FLAG_WINDOW_MINIMIZED: on", 10, 160, 10, ray.LIME)
        else
            ray.DrawText("[N] FLAG_WINDOW_MINIMIZED: off", 10, 160, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_MAXIMIZED))
            ray.DrawText("[M] FLAG_WINDOW_MAXIMIZED: on", 10, 180, 10, ray.LIME)
        else
            ray.DrawText("[M] FLAG_WINDOW_MAXIMIZED: off", 10, 180, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_UNFOCUSED))
            ray.DrawText("[G] FLAG_WINDOW_UNFOCUSED: on", 10, 200, 10, ray.LIME)
        else
            ray.DrawText("[U] FLAG_WINDOW_UNFOCUSED: off", 10, 200, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_TOPMOST))
            ray.DrawText("[T] FLAG_WINDOW_TOPMOST: on", 10, 220, 10, ray.LIME)
        else
            ray.DrawText("[T] FLAG_WINDOW_TOPMOST: off", 10, 220, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_ALWAYS_RUN))
            ray.DrawText("[A] FLAG_WINDOW_ALWAYS_RUN: on", 10, 240, 10, ray.LIME)
        else
            ray.DrawText("[A] FLAG_WINDOW_ALWAYS_RUN: off", 10, 240, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_VSYNC_HINT))
            ray.DrawText("[V] FLAG_VSYNC_HINT: on", 10, 260, 10, ray.LIME)
        else
            ray.DrawText("[V] FLAG_VSYNC_HINT: off", 10, 260, 10, ray.MAROON);

        ray.DrawText("Following flags can only be set before window creation:", 10, 300, 10, ray.GRAY);
        if (ray.IsWindowState(ray.FLAG_WINDOW_HIGHDPI))
            ray.DrawText("FLAG_WINDOW_HIGHDPI: on", 10, 320, 10, ray.LIME)
        else
            ray.DrawText("FLAG_WINDOW_HIGHDPI: off", 10, 320, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_WINDOW_TRANSPARENT))
            ray.DrawText("FLAG_WINDOW_TRANSPARENT: on", 10, 340, 10, ray.LIME)
        else
            ray.DrawText("FLAG_WINDOW_TRANSPARENT: off", 10, 340, 10, ray.MAROON);
        if (ray.IsWindowState(ray.FLAG_MSAA_4X_HINT))
            ray.DrawText("FLAG_MSAA_4X_HINT: on", 10, 360, 10, ray.LIME)
        else
            ray.DrawText("FLAG_MSAA_4X_HINT: off", 10, 360, 10, ray.MAROON);
    }
}
```

## 效果

![2D 窗口参数][1]

## 总结

通过按键来控制窗口的参数。

[1]: images/raylib-2d-flags.png

## 附录
