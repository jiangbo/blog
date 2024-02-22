# 0357-Raylib-3D 相机

## 环境

- Time 2024-02-22
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

展示一个 3D 的方块，并显示当前的 FPS。

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

    const camera: ray.Camera = .{
        .position = .{ .x = 10.0, .y = 10.0, .z = 10.0 },
        .up = .{ .x = 0.0, .y = 1.0, .z = 0.0 },
        .fovy = 45.0,
        .projection = ray.CAMERA_PERSPECTIVE,
    };

    while (!ray.WindowShouldClose()) {
        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.BeginMode3D(camera);

        ray.DrawCube(.{}, 2.0, 2.0, 2.0, ray.RED);
        ray.DrawCubeWires(.{}, 2.0, 2.0, 2.0, ray.MAROON);
        ray.DrawGrid(10, 1.0);

        ray.EndMode3D();

        ray.DrawText("Welcome to the third dimension!", 10, 40, 20, ray.DARKGRAY);
        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![3D 相机][1]

## 总结

使用 3D 相机展示了一个 3D 的方块，同时显示了当前的 FPS。

[1]: images/raylib-3d-camera.png

## 附录
