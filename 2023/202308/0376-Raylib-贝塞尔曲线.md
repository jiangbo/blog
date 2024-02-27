# 0376-Raylib-贝塞尔曲线

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

画出贝塞尔曲线，可以通过鼠标来改变起点和终点。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MOUSE_SCALE_MARK_SIZE = 12;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.SetConfigFlags(ray.FLAG_MSAA_4X_HINT);
    ray.InitWindow(screenWidth, screenHeight, "raylib [shapes] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var startPoint = ray.Vector2{ .x = 30, .y = 30 };
    var endPoint = ray.Vector2{ .x = screenWidth - 30, .y = screenHeight - 30 };
    var moveStartPoint = false;
    var moveEndPoint = false;

    while (!ray.WindowShouldClose()) {

        // Update
        const mouse = ray.GetMousePosition();

        const collisionStart = ray.CheckCollisionPointCircle(mouse, startPoint, 10.0);
        const collisionEnd = ray.CheckCollisionPointCircle(mouse, endPoint, 10.0);
        if (collisionStart and ray.IsMouseButtonDown(ray.MOUSE_BUTTON_LEFT))
            moveStartPoint = true
        else if (collisionEnd and ray.IsMouseButtonDown(ray.MOUSE_BUTTON_LEFT))
            moveEndPoint = true;

        if (moveStartPoint) {
            startPoint = mouse;
            if (ray.IsMouseButtonReleased(ray.MOUSE_BUTTON_LEFT)) moveStartPoint = false;
        }

        if (moveEndPoint) {
            endPoint = mouse;
            if (ray.IsMouseButtonReleased(ray.MOUSE_BUTTON_LEFT)) moveEndPoint = false;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("MOVE START-END POINTS WITH MOUSE", 15, 20, 20, ray.GRAY);

        // Draw line Cubic Bezier, in-out interpolation (easing), no control points
        ray.DrawLineBezier(startPoint, endPoint, 4.0, ray.BLUE);

        // Draw start-end spline circles with some details
        var color: ray.Color = if (moveStartPoint) ray.RED else ray.BLUE;
        ray.DrawCircleV(startPoint, if (collisionStart) 14 else 8, color);
        color = if (moveEndPoint) ray.RED else ray.BLUE;
        ray.DrawCircleV(endPoint, if (collisionEnd) 14 else 8, color);
    }
}
```

## 效果

![贝塞尔曲线][1]

## 总结

画出曲线，通过鼠标来修改起点和终点。

[1]: images/raylib-shapes-bezier.png

## 附录
