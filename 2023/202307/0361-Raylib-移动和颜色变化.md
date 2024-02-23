# 0361-Raylib-移动和颜色变化

## 环境

- Time 2024-02-23
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

展示一个 2D 的的圆球，通过鼠标进行移动，点击鼠标变化颜色。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() void {
    const screenWidth = 800;
    const screenHeight = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var ballPosition = ray.Vector2{ .x = -100, .y = -100 };
    var ballColor = ray.DARKBLUE;

    while (!ray.WindowShouldClose()) {
        // Update
        ballPosition = ray.GetMousePosition();

        if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_LEFT)) ballColor = ray.MAROON //
        else if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_MIDDLE)) ballColor = ray.LIME //
        else if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_RIGHT)) ballColor = ray.DARKBLUE //
        else if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_SIDE)) ballColor = ray.PURPLE //
        else if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_EXTRA)) ballColor = ray.YELLOW //
        else if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_FORWARD)) ballColor = ray.ORANGE //
        else if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_BACK)) ballColor = ray.BEIGE;
        //----------------------------------------------------------------------------------
        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();

        ray.ClearBackground(ray.RAYWHITE);
        ray.DrawCircleV(ballPosition, 40, ballColor);
        ray.DrawText("move ball with mouse and click mouse button to change color", 10, 10, 20, ray.DARKGRAY);
    }
}
```

## 效果

![2D 移动和颜色][1]

## 总结

显示一个 2D 的圆球，通过鼠标控制它移动，点击鼠标改变颜色。

[1]: images/raylib-2d-color.png

## 附录
