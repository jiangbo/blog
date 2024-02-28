# 0377-Raylib-碰撞区域

## 环境

- Time 2024-02-28
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

检查两个矩形区域是否发生了碰撞，计算碰撞的区域。

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

    var boxA = ray.Rectangle{ .x = 10, .y = 225 - 50, .width = 200, .height = 100 };
    var boxASpeedX: f32 = 4;

    // Box B: Mouse moved box
    var boxB = ray.Rectangle{ .x = 400 - 30, .y = 450 - 30, .width = 60, .height = 60 };

    var boxCollision = ray.Rectangle{}; // Collision rectangle

    const screenUpperLimit = 40; // Top menu limits

    var pause = false; // Movement pause
    var collision = false; // Collision detection

    while (!ray.WindowShouldClose()) {

        // Update
        if (!pause) boxA.x += boxASpeedX;

        // Bounce box on x screen limits
        const width: f32 = @floatFromInt(ray.GetScreenWidth());
        const height: f32 = @floatFromInt(ray.GetScreenHeight());
        if (((boxA.x + boxA.width) >= width) or boxA.x <= 0) boxASpeedX *= -1;

        // Update player-controlled-box (box02)
        boxB.x = @as(f32, @floatFromInt(ray.GetMouseX())) - boxB.width / 2;
        boxB.y = @as(f32, @floatFromInt(ray.GetMouseY())) - boxB.height / 2;

        // Make sure Box B does not go out of move area limits
        if ((boxB.x + boxB.width) >= width)
            boxB.x = width - boxB.width
        else if (boxB.x <= 0) boxB.x = 0;

        if ((boxB.y + boxB.height) >= height)
            boxB.y = height - boxB.height
        else if (boxB.y <= screenUpperLimit) boxB.y = screenUpperLimit;

        // Check boxes collision
        collision = ray.CheckCollisionRecs(boxA, boxB);

        // Get collision rectangle (only on collision)
        if (collision)
            boxCollision = ray.GetCollisionRec(boxA, boxB);

        // Pause Box A movement
        if (ray.IsKeyPressed(ray.KEY_SPACE)) pause = !pause;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawRectangle(0, 0, screenWidth, screenUpperLimit, if (collision) ray.RED else ray.BLACK);

        ray.DrawRectangleRec(boxA, ray.GOLD);
        ray.DrawRectangleRec(boxB, ray.BLUE);

        if (collision) {
            // Draw collision area
            ray.DrawRectangleRec(boxCollision, ray.LIME);

            // Draw collision message
            const x = @divTrunc(ray.GetScreenWidth(), 2) - @divTrunc(ray.MeasureText("COLLISION!", 20), 2);
            ray.DrawText("COLLISION!", x, screenUpperLimit / 2 - 10, 20, ray.BLACK);

            // Draw collision area
            const area: c_int = @intFromFloat(boxCollision.width * boxCollision.height);
            const text = ray.TextFormat("Collision Area: %i", area);
            ray.DrawText(text, @divTrunc(ray.GetScreenWidth(), 2) - 100, screenUpperLimit + 10, 20, ray.BLACK);
        }

        // Draw help instructions
        ray.DrawText("Press SPACE to PAUSE/RESUME", 20, screenHeight - 35, 20, ray.LIGHTGRAY);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![碰撞区域][1]

## 总结

检查是否发生了碰撞，计算碰撞的区域大小。

[1]: images/raylib-shapes-collision.png

## 附录
