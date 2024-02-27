# 0370-Raylib-基础图形

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

画一些基础的图形。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    // Set configuration flags for window creation
    // ray.SetConfigFlags(ray.FLAG_VSYNC_HINT | ray.FLAG_MSAA_4X_HINT | ray.FLAG_WINDOW_HIGHDPI);
    ray.InitWindow(screenWidth, screenHeight, "raylib [shapes] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var rotation: f32 = 0.0;
    while (!ray.WindowShouldClose()) {

        // Update
        rotation += 0.2;

        // Draw

        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("some basic shapes available on raylib", 20, 20, 20, ray.DARKGRAY);

        // Circle shapes and lines
        ray.DrawCircle(screenWidth / 5, 120, 35, ray.DARKBLUE);
        ray.DrawCircleGradient(screenWidth / 5, 220, 60, ray.GREEN, ray.SKYBLUE);
        ray.DrawCircleLines(screenWidth / 5, 340, 80, ray.DARKBLUE);

        // Rectangle shapes and lines
        ray.DrawRectangle(screenWidth / 4 * 2 - 60, 100, 120, 60, ray.RED);
        ray.DrawRectangleGradientH(screenWidth / 4 * 2 - 90, 170, 180, 130, ray.MAROON, ray.GOLD);
        ray.DrawRectangleLines(screenWidth / 4 * 2 - 40, 320, 80, 60, ray.ORANGE); // NOTE: Uses QUADS internally, not lines

        // Triangle shapes and lines
        const width: f32 = @floatFromInt(screenWidth);
        // const height: f32 = @floatFromInt(screenHeight);
        ray.DrawTriangle(.{ .x = width / 4.0 * 3.0, .y = 80.0 }, //
            .{ .x = width / 4.0 * 3.0 - 60.0, .y = 150.0 }, //
            .{ .x = width / 4.0 * 3.0 + 60.0, .y = 150.0 }, ray.VIOLET);
        ray.DrawTriangleLines(.{ .x = width / 4.0 * 3.0, .y = 160.0 }, //
            .{ .x = width / 4.0 * 3.0 - 20.0, .y = 230.0 }, //
            .{ .x = width / 4.0 * 3.0 + 20.0, .y = 230.0 }, ray.DARKBLUE);

        // Polygon shapes and lines
        ray.DrawPoly(.{ .x = width / 4.0 * 3, .y = 330 }, 6, 80, rotation, ray.BROWN);
        ray.DrawPolyLines(.{ .x = width / 4.0 * 3, .y = 330 }, 6, 90, rotation, ray.BROWN);
        ray.DrawPolyLinesEx(.{ .x = width / 4.0 * 3, .y = 330 }, 6, 85, rotation, 6, ray.BEIGE);

        // NOTE: We draw all LINES based shapes together to optimize internal drawing,
        // this way, all LINES are rendered in a single draw pass
        ray.DrawLine(18, 42, screenWidth - 18, 42, ray.BLACK);
    }
}
```

## 效果

![2D 基础图形][1]

## 总结

画一些基础的几何图形。

[1]: images/raylib-shapes-basic.png

## 附录
