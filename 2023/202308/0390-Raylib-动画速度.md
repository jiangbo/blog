# 0390-Raylib-动画速度

## 环境

- Time 2024-03-01
- Zig 0.12.0-dev.3076+6e078883e
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

实现精灵动画，并且可以控制显示的速度。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MAX_FRAME_SPEED = 15;
const MIN_FRAME_SPEED = 1;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [texture] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    const scarfy = ray.LoadTexture("res/scarfy.png"); // Texture loading
    defer ray.UnloadTexture(scarfy);

    const position = ray.Vector2{ .x = 350.0, .y = 280.0 };
    var frameRec = ray.Rectangle{
        .width = @floatFromInt(@divTrunc(scarfy.width, 6)),
        .height = @floatFromInt(scarfy.height),
    };

    var currentFrame: c_int = 0;
    var framesCounter: usize = 0;
    var framesSpeed: usize = 8; // Number of spritesheet frames shown by second

    while (!ray.WindowShouldClose()) {

        // Update
        framesCounter += 1;

        if (framesCounter >= (60 / framesSpeed)) {
            framesCounter = 0;
            currentFrame += 1;

            if (currentFrame > 5) currentFrame = 0;

            frameRec.x = @floatFromInt(@divTrunc(currentFrame * scarfy.width, 6));
        }

        // Control frames speed
        if (ray.IsKeyPressed(ray.KEY_RIGHT))
            framesSpeed += 1
        else if (ray.IsKeyPressed(ray.KEY_LEFT)) framesSpeed -= 1;

        if (framesSpeed > MAX_FRAME_SPEED)
            framesSpeed = MAX_FRAME_SPEED
        else if (framesSpeed < MIN_FRAME_SPEED) framesSpeed = MIN_FRAME_SPEED;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawTexture(scarfy, 15, 40, ray.WHITE);
        ray.DrawRectangleLines(15, 40, scarfy.width, scarfy.height, ray.LIME);
        const x = @as(c_int, @intFromFloat(frameRec.x));
        const y = @as(c_int, @intFromFloat(frameRec.y));
        const width = @as(c_int, @intFromFloat(frameRec.width));
        const height = @as(c_int, @intFromFloat(frameRec.height));
        ray.DrawRectangleLines(15 + x, 40 + y, width, height, ray.RED);

        ray.DrawText("FRAME SPEED: ", 165, 210, 10, ray.DARKGRAY);
        ray.DrawText(ray.TextFormat("%02i FPS", framesSpeed), 575, 210, 10, ray.DARKGRAY);
        ray.DrawText("PRESS RIGHT/LEFT KEYS to CHANGE SPEED!", 290, 240, 10, ray.DARKGRAY);

        for (0..MAX_FRAME_SPEED) |i| {
            const index = @as(c_int, @intCast(i));
            if (i < framesSpeed) ray.DrawRectangle(250 + 21 * index, 205, 20, 20, ray.RED);
            ray.DrawRectangleLines(250 + 21 * index, 205, 20, 20, ray.MAROON);
        }

        ray.DrawTextureRec(scarfy, frameRec, position, ray.WHITE); // Draw part of the texture

        ray.DrawText("(c) Scarfy sprite by Eiden Marsal", screenWidth - 200, screenHeight - 20, 10, ray.GRAY);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![声音和动画][1]

## 总结

实现精灵动画，并且可以控制显示的速度。

[1]: images/raylib-texture-sprite.png

## 附录
