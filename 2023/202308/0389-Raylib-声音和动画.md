# 0389-Raylib-声音和动画

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

播放声音和动画。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const NUM_FRAMES_PER_LINE = 5;
const NUM_LINES = 5;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [texture] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    ray.InitAudioDevice();
    defer ray.CloseAudioDevice();

    // Load explosion sound
    const fxBoom = ray.LoadSound("res/boom.wav");
    defer ray.UnloadSound(fxBoom);

    // Load explosion texture
    const explosion = ray.LoadTexture("res/explosion.png");
    defer ray.UnloadTexture(explosion);

    // Init variables for animation
    // Sprite one frame rectangle width
    const frameWidth: f32 = @floatFromInt(@divTrunc(explosion.width, NUM_FRAMES_PER_LINE));
    // Sprite one frame rectangle height
    const frameHeight: f32 = @floatFromInt(@divTrunc(explosion.height, NUM_LINES));
    var currentFrame: f32 = 0;
    var currentLine: f32 = 0;

    var frameRec = ray.Rectangle{ .width = frameWidth, .height = frameHeight };
    var position = ray.Vector2{};

    var active = false;
    var framesCounter: usize = 0;
    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_LEFT) and !active) {
            position = ray.GetMousePosition();
            active = true;

            position.x -= frameWidth / 2.0;
            position.y -= frameHeight / 2.0;

            ray.PlaySound(fxBoom);
        }

        // Compute explosion animation frames
        if (active) {
            framesCounter += 1;

            if (framesCounter > 2) {
                currentFrame += 1;

                if (currentFrame >= NUM_FRAMES_PER_LINE) {
                    currentFrame = 0;
                    currentLine += 1;

                    if (currentLine >= NUM_LINES) {
                        currentLine = 0;
                        active = false;
                    }
                }

                framesCounter = 0;
            }
        }

        frameRec.x = frameWidth * currentFrame;
        frameRec.y = frameHeight * currentLine;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        // Draw explosion required frame rectangle
        if (active) ray.DrawTextureRec(explosion, frameRec, position, ray.WHITE);

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![声音和动画][1]

## 总结

播放声音和动画。

[1]: images/raylib-texture-explosion.png

## 附录
