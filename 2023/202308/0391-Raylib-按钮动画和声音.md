# 0391-Raylib-按钮动画和声音

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

渲染一个按钮的不同状态，当点击按钮时，会有音效。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const NUM_FRAMES = 3;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [texture] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    ray.InitAudioDevice(); // Initialize audio device
    defer ray.CloseAudioDevice();

    const fxButton = ray.LoadSound("res/buttonfx.wav"); // Load button sound
    defer ray.UnloadSound(fxButton);
    const button = ray.LoadTexture("res/button.png"); // Load button texture
    defer ray.UnloadTexture(button);

    // Define frame rectangle for drawing
    const frameHeight = @divTrunc(button.height, NUM_FRAMES);
    var sourceRec = ray.Rectangle{
        .width = @floatFromInt(button.width),
        .height = @floatFromInt(frameHeight),
    };

    // Define button bounds on screen
    const screenWidthf = @as(f32, @floatFromInt(screenWidth));
    const screenHeightf = @as(f32, @floatFromInt(screenHeight));
    const y: f32 = @floatFromInt(@divTrunc(button.height, NUM_FRAMES));
    const btnBounds = ray.Rectangle{
        .x = screenWidthf / 2.0 - @as(f32, @floatFromInt(button.width)) / 2.0,
        .y = screenHeightf / 2.0 - y / 2.0,
        .width = @floatFromInt(button.width),
        .height = @floatFromInt(frameHeight),
    };

    var btnState: c_int = 0; // Button state: 0-NORMAL, 1-MOUSE_HOVER, 2-PRESSED
    var btnAction = false; // Button action should be activated

    var mousePoint = ray.Vector2{};

    while (!ray.WindowShouldClose()) {

        // Update
        mousePoint = ray.GetMousePosition();
        btnAction = false;

        // Check button state
        if (ray.CheckCollisionPointRec(mousePoint, btnBounds)) {
            if (ray.IsMouseButtonDown(ray.MOUSE_BUTTON_LEFT)) btnState = 2 else btnState = 1;

            if (ray.IsMouseButtonReleased(ray.MOUSE_BUTTON_LEFT)) btnAction = true;
        } else btnState = 0;

        if (btnAction) {
            ray.PlaySound(fxButton);

            // TODO: Any desired action
        }

        // Calculate button frame rectangle to draw depending on button state
        sourceRec.y = @floatFromInt(btnState * frameHeight);

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawTextureRec(button, sourceRec, .{ .x = btnBounds.x, .y = btnBounds.y }, ray.WHITE); // Draw button frame

        ray.DrawFPS(10, 10);
    }
}
```

## 效果

![按钮动画][1]

## 总结

渲染一个按钮的不同状态，当点击按钮时，会有音效。

[1]: images/raylib-texture-button.png

## 附录
