# 0354-Raylib-缩放和拖拽

## 环境

- Time 2024-02-18
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

鼠标滚轮实现画面的缩放，右键可以拖动画面。

## raylib.zig

```zig
pub usingnamespace @cImport({
    @cInclude("raylib.h");
    @cInclude("raymath.h");
    @cInclude("rlgl.h");
});
```

## update

```zig
fn update(camera: *ray.Camera2D) void {

    // 检查鼠标右键是否按下
    if (ray.IsMouseButtonDown(ray.MOUSE_BUTTON_RIGHT)) {
        // 鼠标移动
        const delta = ray.GetMouseDelta();
        const v1 = ray.Vector2Scale(delta, -1.0 / camera.zoom);
        camera.target = ray.Vector2Add(camera.target, v1);
    }

    // 鼠标滚轮移动进行放大和缩小
    const wheel = ray.GetMouseWheelMove();
    if (wheel != 0) {
        // Get the world point that is under the mouse
        const mouseWorldPos = ray.GetScreenToWorld2D(ray.GetMousePosition(), camera.*);

        // Set the offset to where the mouse is
        camera.offset = ray.GetMousePosition();

        // Set the target to match, so that the camera maps the world space point
        // under the cursor to the screen space point under the cursor at any zoom
        camera.target = mouseWorldPos;

        // Zoom increment
        const zoomIncrement = 0.125;

        camera.zoom += (wheel * zoomIncrement);
        if (camera.zoom < zoomIncrement) camera.zoom = zoomIncrement;
    }
}
```

## draw

```zig
fn draw(camera: ray.Camera2D) void {
    ray.ClearBackground(ray.BLACK);

    ray.BeginMode2D(camera);

    // Draw the 3d grid, rotated 90 degrees and centered around 0,0
    // just so we have something in the XY plane
    ray.rlPushMatrix();
    ray.rlTranslatef(0, 25 * 50, 0);
    ray.rlRotatef(90, 1, 0, 0);
    ray.DrawGrid(100, 50);
    ray.rlPopMatrix();

    // Draw a reference circle
    ray.DrawCircle(100, 100, 50, ray.YELLOW);

    ray.EndMode2D();

    ray.DrawText("Mouse right button drag to move, mouse wheel to zoom", 10, 10, 20, ray.WHITE);
}
```

## main

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const screenWidth = 800;
const screenHeight = 450;

pub fn main() void {
    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // 2D 相机
    var camera: ray.Camera2D = .{
        .zoom = 1.0,
    };

    while (!ray.WindowShouldClose()) {
        update(&camera);

        ray.BeginDrawing();
        defer ray.EndDrawing();
        draw(camera);
    }
}
```

## 效果

![2D 相机拖拽][1]

## 总结

实现了使用鼠标进行缩放和右键拖拽功能。

[1]: images/raylib-2d-mouse.png

## 附录
