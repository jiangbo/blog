# 0364-Raylib-可见窗口

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

只能看到窗口中鼠标周围一部分的内容。

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

    var scissorArea = ray.Rectangle{ .width = 300, .height = 300 };
    var scissorMode = true;

    while (!ray.WindowShouldClose()) {
        // Update
        if (ray.IsKeyPressed(ray.KEY_S)) scissorMode = !scissorMode;

        // Centre the scissor area around the mouse position
        scissorArea.x = @as(f32, @floatFromInt(ray.GetMouseX())) - scissorArea.width / 2;
        scissorArea.y = @as(f32, @floatFromInt(ray.GetMouseY())) - scissorArea.height / 2;
        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();

        ray.ClearBackground(ray.RAYWHITE);
        if (scissorMode) {
            const x: c_int = @intFromFloat(scissorArea.x);
            const y: c_int = @intFromFloat(scissorArea.y);
            const width: c_int = @intFromFloat(scissorArea.width);
            const height: c_int = @intFromFloat(scissorArea.height);
            ray.BeginScissorMode(x, y, width, height);
        }
        // Draw full screen rectangle and some text
        // NOTE: Only part defined by scissor area will be rendered
        ray.DrawRectangle(0, 0, ray.GetScreenWidth(), ray.GetScreenHeight(), ray.RED);
        ray.DrawText("Move the mouse around to reveal this text!", 190, 200, 20, ray.LIGHTGRAY);

        if (scissorMode) ray.EndScissorMode();

        ray.DrawRectangleLinesEx(scissorArea, 1, ray.BLACK);
        ray.DrawText("Press S to toggle scissor test", 10, 10, 20, ray.BLACK);
    }
}
```

## 效果

![2D 可见窗口][1]

## 总结

只能看到窗口中一部分内容。

[1]: images/raylib-2d-scissor.png

## 附录
