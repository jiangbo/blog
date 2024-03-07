# 0375-Raylib-拖拽放大和缩小

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

通过鼠标，拖拽方块来进行放大和缩小。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MOUSE_SCALE_MARK_SIZE = 12;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [shapes] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var rec = ray.Rectangle{ .x = 100, .y = 100, .width = 200, .height = 80 };
    var mousePosition = ray.Vector2{};
    var mouseScaleReady = false;
    var mouseScaleMode = false;

    while (!ray.WindowShouldClose()) {

        // Update
        mousePosition = ray.GetMousePosition();

        if (ray.CheckCollisionPointRec(mousePosition, .{
            .x = rec.x + rec.width - MOUSE_SCALE_MARK_SIZE,
            .y = rec.y + rec.height - MOUSE_SCALE_MARK_SIZE,
            .width = MOUSE_SCALE_MARK_SIZE,
            .height = MOUSE_SCALE_MARK_SIZE,
        })) {
            mouseScaleReady = true;
            if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_LEFT))
                mouseScaleMode = true;
        } else mouseScaleReady = false;

        if (mouseScaleMode) {
            mouseScaleReady = true;

            rec.width = (mousePosition.x - rec.x);
            rec.height = (mousePosition.y - rec.y);

            // Check minimum rec size
            if (rec.width < MOUSE_SCALE_MARK_SIZE) rec.width = MOUSE_SCALE_MARK_SIZE;
            if (rec.height < MOUSE_SCALE_MARK_SIZE) rec.height = MOUSE_SCALE_MARK_SIZE;

            // Check maximum rec size
            const width: f32 = @floatFromInt(ray.GetScreenWidth());
            const height: f32 = @floatFromInt(ray.GetScreenHeight());
            if (rec.width > (width - rec.x)) rec.width = width - rec.x;
            if (rec.height > (height - rec.y)) rec.height = height - rec.y;

            if (ray.IsMouseButtonReleased(ray.MOUSE_BUTTON_LEFT)) mouseScaleMode = false;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("Scale rectangle dragging from bottom-right corner!", 10, 10, 20, ray.GRAY);
        ray.DrawRectangleRec(rec, ray.Fade(ray.GREEN, 0.5));

        if (mouseScaleReady) {
            ray.DrawRectangleLinesEx(rec, 1, ray.RED);
            ray.DrawTriangle(
                .{ .x = rec.x + rec.width - MOUSE_SCALE_MARK_SIZE, .y = rec.y + rec.height },
                .{ .x = rec.x + rec.width, .y = rec.y + rec.height },
                .{ .x = rec.x + rec.width, .y = rec.y + rec.height - MOUSE_SCALE_MARK_SIZE },
                ray.RED,
            );
        }
    }
}
```

## 效果

![拖拽放大][1]

## 总结

渲染 Raylib Logo 动画。

[1]: images/raylib-shapes-scaling.png

## 附录
