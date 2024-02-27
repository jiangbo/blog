# 0371-Raylib-碰撞的小球

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

一个碰撞的小球，可以进行暂停。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.SetConfigFlags(ray.FLAG_MSAA_4X_HINT);
    ray.InitWindow(screenWidth, screenHeight, "raylib [shapes] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var ballPosition = ray.Vector2{
        .x = @as(f32, @floatFromInt(screenWidth)) / 2.0,
        .y = @as(f32, @floatFromInt(screenHeight)) / 2.0,
    };
    var ballSpeed = ray.Vector2{ .x = 5.0, .y = 4.0 };
    const ballRadius = 20;

    var pause = false;
    var framesCounter: c_int = 0;

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsKeyPressed(ray.KEY_SPACE)) pause = !pause;

        if (!pause) {
            ballPosition.x += ballSpeed.x;
            ballPosition.y += ballSpeed.y;

            // Check walls collision for bouncing
            const width = @as(f32, @floatFromInt(ray.GetScreenWidth()));
            const height = @as(f32, @floatFromInt(ray.GetScreenHeight()));
            if ((ballPosition.x >= (width - ballRadius)) or (ballPosition.x <= ballRadius))
                ballSpeed.x *= -1.0;
            if ((ballPosition.y >= (height - ballRadius)) or (ballPosition.y <= ballRadius))
                ballSpeed.y *= -1.0;
        } else framesCounter += 1;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawCircleV(ballPosition, ballRadius, ray.MAROON);
        //DrawText("PRESS SPACE to PAUSE BALL MOVEMENT", 10, GetScreenHeight() - 25, 20, LIGHTGRAY);

        // On pause, we draw a blinking message
        if (pause and (@mod(@divTrunc(framesCounter, 30), 2) == 0))
            ray.DrawText("PAUSED", 350, 200, 30, ray.GRAY);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![碰撞的小球][1]

## 总结

实现一个碰撞的小球，可以改变窗口的大小，可以进行暂停。

[1]: images/raylib-shapes-ball.png

## 附录
