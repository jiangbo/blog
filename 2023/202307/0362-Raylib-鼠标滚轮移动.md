# 0362-Raylib-鼠标滚轮移动

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

展示一个 2D 的方块，通过鼠标滚轮的滚动进行上下移动。

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

    var boxPositionY: c_int = @divTrunc(screenHeight, 2) - 40;
    const scrollSpeed: c_int = 10; // Scrolling speed in pixels

    while (!ray.WindowShouldClose()) {
        // Update
        boxPositionY -= (@as(c_int, @intFromFloat(ray.GetMouseWheelMove())) * scrollSpeed);
        //----------------------------------------------------------------------------------
        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();

        ray.ClearBackground(ray.RAYWHITE);
        ray.DrawRectangle(screenWidth / 2 - 40, boxPositionY, 80, 80, ray.MAROON);
        ray.DrawText("Use mouse wheel to move the cube up and down!", 10, 10, 20, ray.GRAY);
        ray.DrawText(ray.TextFormat("Box position Y: %03i", boxPositionY), 10, 40, 20, ray.LIGHTGRAY);
    }
}
```

## 效果

![2D 滚轮移动][1]

## 总结

显示一个 2D 的方块，通过鼠标的滚轮来进行方块的移动。

[1]: images/raylib-2d-wheel.png

## 附录
