# 0365-Raylib-退出确认

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

在退出的时候，弹出一个退出的确认。

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
    // ray.SetExitKey(ray.KEY_NULL);

    var exitWindowRequested = false; // Flag to request window to exit
    var exitWindow = false; // Flag to set window to exit
    while (!exitWindow) {

        // Update
        if (ray.WindowShouldClose()) exitWindowRequested = true;

        if (exitWindowRequested) {
            // A request for close window has been issued, we can save data before closing
            // or just show a message asking for confirmation

            if (ray.IsKeyPressed(ray.KEY_Y)) exitWindow = true //
            else if (ray.IsKeyPressed(ray.KEY_N)) exitWindowRequested = false;
        }
        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();

        ray.ClearBackground(ray.RAYWHITE);
        if (exitWindowRequested) {
            ray.DrawRectangle(0, 100, screenWidth, 200, ray.BLACK);
            ray.DrawText("Are you sure you want to exit program? [Y/N]", 40, 180, 30, ray.WHITE);
        } else ray.DrawText("Try to close the window to get confirmation message!", 120, 200, 20, ray.LIGHTGRAY);
    }
}
```

## 效果

![2D 退出确认][1]

## 总结

在退出窗口的时候，退出一个确认提示。

[1]: images/raylib-2d-confirm.png

## 附录
