# 0359-0360-Raylib-2D 移动方块

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

展示一个 2D 的的圆球，通过键盘控制它移动。

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

    var ballPosition = ray.Vector2{
        .x = @divExact(screenWidth, 2),
        .y = @divExact(screenHeight, 2),
    };

    while (!ray.WindowShouldClose()) {
        // Update
        if (ray.IsKeyDown(ray.KEY_RIGHT)) ballPosition.x += 2.0;
        if (ray.IsKeyDown(ray.KEY_LEFT)) ballPosition.x -= 2.0;
        if (ray.IsKeyDown(ray.KEY_UP)) ballPosition.y -= 2.0;
        if (ray.IsKeyDown(ray.KEY_DOWN)) ballPosition.y += 2.0;
        //----------------------------------------------------------------------------------
        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();

        ray.ClearBackground(ray.RAYWHITE);
        ray.DrawText("move the ball with arrow keys", 10, 10, 20, ray.DARKGRAY);
        ray.DrawCircleV(ballPosition, 50, ray.MAROON);
    }
}
```

## 效果

![2D 方块移动][1]

## 总结

显示一个 2D 的圆球，通过键盘控制它移动。

[1]: images/raylib-2d-move.png

## 附录
