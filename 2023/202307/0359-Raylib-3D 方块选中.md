# 0359-Raylib-3D 方块选中

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

展示一个 3D 的方块，使用鼠标选中时，高亮显示方块。

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

    var camera: ray.Camera = .{
        .position = .{ .x = 10.0, .y = 10.0, .z = 10.0 },
        .up = .{ .x = 0.0, .y = 1.0, .z = 0.0 },
        .fovy = 45.0,
        .projection = ray.CAMERA_PERSPECTIVE,
    };

    const cubePosition: ray.Vector3 = .{ .y = 1.0 };
    const cubeSize: ray.Vector3 = .{ .x = 2.0, .y = 2.0, .z = 2.0 };

    var line = ray.Ray{}; // Picking line ray
    var collision = ray.RayCollision{}; // Ray collision hit info

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsCursorHidden()) ray.UpdateCamera(&camera, ray.CAMERA_FIRST_PERSON);

        // Toggle camera controls
        if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_RIGHT)) {
            if (ray.IsCursorHidden()) ray.EnableCursor() else ray.DisableCursor();
        }

        if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_LEFT)) {
            if (!collision.hit) {
                line = ray.GetMouseRay(ray.GetMousePosition(), camera);

                // Check collision between ray and box
                collision = ray.GetRayCollisionBox(line, .{
                    .min = .{
                        .x = cubePosition.x - cubeSize.x / 2,
                        .y = cubePosition.y - cubeSize.y / 2,
                        .z = cubePosition.z - cubeSize.z / 2,
                    },
                    .max = .{
                        .x = cubePosition.x + cubeSize.x / 2,
                        .y = cubePosition.y + cubeSize.y / 2,
                        .z = cubePosition.z + cubeSize.z / 2,
                    },
                });
            } else collision.hit = false;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.BeginMode3D(camera);

        if (collision.hit) {
            ray.DrawCube(cubePosition, cubeSize.x, cubeSize.y, cubeSize.z, ray.RED);
            ray.DrawCubeWires(cubePosition, cubeSize.x, cubeSize.y, cubeSize.z, ray.MAROON);

            ray.DrawCubeWires(cubePosition, cubeSize.x + 0.2, cubeSize.y + 0.2, cubeSize.z + 0.2, ray.GREEN);
        } else {
            ray.DrawCube(cubePosition, cubeSize.x, cubeSize.y, cubeSize.z, ray.GRAY);
            ray.DrawCubeWires(cubePosition, cubeSize.x, cubeSize.y, cubeSize.z, ray.DARKGRAY);
        }

        ray.DrawRay(line, ray.MAROON);
        ray.DrawGrid(10, 1.0);

        ray.EndMode3D();

        ray.DrawText("Try clicking on the box with your mouse!", 240, 10, 20, ray.DARKGRAY);

        const x = @divTrunc(screenWidth - ray.MeasureText("BOX SELECTED", 30), 2);
        if (collision.hit) ray.DrawText("BOX SELECTED", x, screenHeight * 0.1, 30, ray.GREEN);

        ray.DrawText("Right click mouse to toggle camera controls", 10, 430, 10, ray.GRAY);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![3D 方块选中][1]

## 总结

在选中 3D 的方块时，进行高亮显示。

[1]: images/raylib-3d-select.png

## 附录
