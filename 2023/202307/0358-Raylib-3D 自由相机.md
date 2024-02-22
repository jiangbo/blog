# 0358-Raylib-3D 自由相机

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

展示一个 3D 的方块，可以通过滚轮放大和缩小，移动视角。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() void {
    // Initialization
    //--------------------------------------------------------------------------------------
    const screenWidth = 800;
    const screenHeight = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // Define the camera to look into our 3d world (position, target, up vector)
    var camera: ray.Camera = .{
        .position = .{ .x = 10.0, .y = 10.0, .z = 10.0 },
        .target = .{ .x = 0.0, .y = 0.0, .z = 0.0 },
        .up = .{ .x = 0.0, .y = 1.0, .z = 0.0 },
        .fovy = 45.0,
        .projection = ray.CAMERA_PERSPECTIVE,
    };

    const cubePosition: ray.Vector3 = .{};

    ray.DisableCursor(); // Limit cursor to relative movement inside the window
    //--------------------------------------------------------------------------------------
    // Main game loop
    while (!ray.WindowShouldClose()) // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        ray.UpdateCamera(&camera, ray.CAMERA_FREE);

        if (ray.IsKeyPressed('Z')) camera.target = .{};
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.BeginMode3D(camera);

        ray.DrawCube(cubePosition, 2.0, 2.0, 2.0, ray.RED);
        ray.DrawCubeWires(cubePosition, 2.0, 2.0, 2.0, ray.MAROON);

        ray.DrawGrid(10, 1.0);

        ray.EndMode3D();

        ray.DrawRectangle(10, 10, 320, 93, ray.Fade(ray.SKYBLUE, 0.5));
        ray.DrawRectangleLines(10, 10, 320, 93, ray.BLUE);

        ray.DrawText("Free camera default controls:", 20, 20, 10, ray.BLACK);
        ray.DrawText("- Mouse Wheel to Zoom in-out", 40, 40, 10, ray.DARKGRAY);
        ray.DrawText("- Mouse Wheel Pressed to Pan", 40, 60, 10, ray.DARKGRAY);
        ray.DrawText("- Z to zoom to (0, 0, 0)", 40, 80, 10, ray.DARKGRAY);
    }
}
```

## 效果

![3D 自由相机][1]

## 总结

实现了一个 3D 的方块，不过在 WSL 下好像有点问题，不过当前了解 Raylib 还是以 2D 为主。

[1]: images/raylib-3d-free.png

## 附录
