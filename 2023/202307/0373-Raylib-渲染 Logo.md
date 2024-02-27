# 0373-Raylib-渲染 Logo

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

渲染 Raylib Logo。

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

    while (!ray.WindowShouldClose()) {

        // Update

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawRectangle(screenWidth / 2 - 128, screenHeight / 2 - 128, 256, 256, ray.BLACK);
        ray.DrawRectangle(screenWidth / 2 - 112, screenHeight / 2 - 112, 224, 224, ray.RAYWHITE);
        ray.DrawText("raylib", screenWidth / 2 - 44, screenHeight / 2 + 48, 50, ray.BLACK);

        ray.DrawText("this is NOT a texture!", 350, 370, 10, ray.GRAY);
    }
}
```

## 效果

![Logo][1]

## 总结

渲染 Raylib Logo。

[1]: images/raylib-shapes-logo.png

## 附录
