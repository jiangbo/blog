# 0378-Raylib-视线跟随

## 环境

- Time 2024-02-28
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

模拟眼球跟随鼠标移动。

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

    const scleraLeftPosition = ray.Vector2{ .x = 400 - 100.0, .y = 225 };
    const scleraRightPosition = ray.Vector2{ .x = 400 + 100.0, .y = 225 };
    const scleraRadius = 80;

    var irisLeftPosition = scleraLeftPosition;
    var irisRightPosition = scleraRightPosition;
    const irisRadius = 24;

    var angle: f32 = 0.0;
    var dx: f32 = 0.0;
    var dy: f32 = 0.0;
    var dxx: f32 = 0.0;
    var dyy: f32 = 0.0;

    while (!ray.WindowShouldClose()) {

        // Update
        irisLeftPosition = ray.GetMousePosition();
        irisRightPosition = ray.GetMousePosition();

        // Check not inside the left eye sclera
        if (!ray.CheckCollisionPointCircle(irisLeftPosition, scleraLeftPosition, scleraRadius - irisRadius)) {
            dx = irisLeftPosition.x - scleraLeftPosition.x;
            dy = irisLeftPosition.y - scleraLeftPosition.y;

            angle = std.math.atan2(f32, dy, dx);

            dxx = (scleraRadius - irisRadius) * @cos(angle);
            dyy = (scleraRadius - irisRadius) * @sin(angle);

            irisLeftPosition.x = scleraLeftPosition.x + dxx;
            irisLeftPosition.y = scleraLeftPosition.y + dyy;
        }

        // Check not inside the right eye sclera
        if (!ray.CheckCollisionPointCircle(irisRightPosition, scleraRightPosition, scleraRadius - irisRadius)) {
            dx = irisRightPosition.x - scleraRightPosition.x;
            dy = irisRightPosition.y - scleraRightPosition.y;

            angle = std.math.atan2(f32, dy, dx);

            dxx = (scleraRadius - irisRadius) * @cos(angle);
            dyy = (scleraRadius - irisRadius) * @sin(angle);

            irisRightPosition.x = scleraRightPosition.x + dxx;
            irisRightPosition.y = scleraRightPosition.y + dyy;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawCircleV(scleraLeftPosition, scleraRadius, ray.LIGHTGRAY);
        ray.DrawCircleV(irisLeftPosition, irisRadius, ray.BROWN);
        ray.DrawCircleV(irisLeftPosition, 10, ray.BLACK);

        ray.DrawCircleV(scleraRightPosition, scleraRadius, ray.LIGHTGRAY);
        ray.DrawCircleV(irisRightPosition, irisRadius, ray.DARKGREEN);
        ray.DrawCircleV(irisRightPosition, 10, ray.BLACK);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![视线跟随][1]

## 总结

眼球跟随鼠标进行转动。

[1]: images/raylib-shapes-follow.png

## 附录
