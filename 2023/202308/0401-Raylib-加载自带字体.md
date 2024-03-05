# 0401-Raylib-加载自带字体

## 环境

- Time 2024-03-05
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

加载 Raylib 自带的字体。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MAX_FRAME_DELAY = 20;
const MIN_FRAME_DELAY = 1;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [textures] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var animFrames: c_int = 0;

    // Load all GIF animation frames into a single Image
    // NOTE: GIF data is always loaded as RGBA (32bit) by default
    // NOTE: Frames are just appended one after another in image.data memory
    const imScarfyAnim = ray.LoadImageAnim("res/scarfy_run.gif", &animFrames);
    defer ray.UnloadImage(imScarfyAnim);
    // Load texture from image
    // NOTE: We will update this texture when required with next frame data
    // WARNING: It's not recommended to use this technique for sprites animation,
    // use spritesheets instead, like illustrated in textures_sprite_anim example
    const texScarfyAnim = ray.LoadTextureFromImage(imScarfyAnim);
    defer ray.UnloadTexture(texScarfyAnim);

    var nextFrameDataOffset: c_int = 0; // Current byte offset to next frame in image.data

    var currentAnimFrame: c_int = 0; // Current animation frame to load and draw
    var frameDelay: c_int = 8; // Frame delay to switch between animation frames
    var frameCounter: c_int = 0; // General frames counter

    while (!ray.WindowShouldClose()) {

        // Update
        frameCounter += 1;
        if (frameCounter >= frameDelay) {
            // Move to next frame
            // NOTE: If final frame is reached we return to first frame
            currentAnimFrame += 1;
            if (currentAnimFrame >= animFrames) currentAnimFrame = 0;

            // Get memory offset position for next frame data in image.data
            nextFrameDataOffset = imScarfyAnim.width * imScarfyAnim.height * 4 * currentAnimFrame;

            // Update GPU texture data with next frame image data
            // WARNING: Data size (frame size) and pixel format must match already created texture
            const offset: usize = @intCast(nextFrameDataOffset);
            const a: *anyopaque = @ptrFromInt(@intFromPtr(imScarfyAnim.data.?) + offset);
            ray.UpdateTexture(texScarfyAnim, a);

            frameCounter = 0;
        }

        // Control frames delay
        if (ray.IsKeyPressed(ray.KEY_RIGHT))
            frameDelay += 1
        else if (ray.IsKeyPressed(ray.KEY_LEFT))
            frameDelay -= 1;

        if (frameDelay > MAX_FRAME_DELAY) frameDelay = MAX_FRAME_DELAY else if (frameDelay < MIN_FRAME_DELAY) frameDelay = MIN_FRAME_DELAY;
        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText(ray.TextFormat("TOTAL GIF FRAMES:  %02i", animFrames), 50, 30, 20, ray.LIGHTGRAY);
        ray.DrawText(ray.TextFormat("CURRENT FRAME: %02i", currentAnimFrame), 50, 60, 20, ray.GRAY);
        ray.DrawText(ray.TextFormat("CURRENT FRAME IMAGE.DATA OFFSET: %02i", nextFrameDataOffset), 50, 90, 20, ray.GRAY);

        ray.DrawText("FRAMES DELAY: ", 100, 305, 10, ray.DARKGRAY);
        ray.DrawText(ray.TextFormat("%02i frames", frameDelay), 620, 305, 10, ray.DARKGRAY);
        ray.DrawText("PRESS RIGHT/LEFT KEYS to CHANGE SPEED!", 290, 350, 10, ray.DARKGRAY);

        for (0..MAX_FRAME_DELAY) |i| {
            const index: c_int = @intCast(i);
            if (i < frameDelay) ray.DrawRectangle(190 + 21 * index, 300, 20, 20, ray.RED);
            ray.DrawRectangleLines(190 + 21 * index, 300, 20, 20, ray.MAROON);
        }

        const y = @divTrunc(ray.GetScreenWidth(), 2) - @divTrunc(texScarfyAnim.width, 2);
        ray.DrawTexture(texScarfyAnim, y, 140, ray.WHITE);

        ray.DrawText("(c) Scarfy sprite by Eiden Marsal", screenWidth - 200, screenHeight - 20, 10, ray.GRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
const std = @import("std");
const ray = @import("raylib.zig");

const MAX_FRAME_DELAY = 20;
const MIN_FRAME_DELAY = 1;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [textures] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var animFrames: c_int = 0;

    // Load all GIF animation frames into a single Image
    // NOTE: GIF data is always loaded as RGBA (32bit) by default
    // NOTE: Frames are just appended one after another in image.data memory
    const imScarfyAnim = ray.LoadImageAnim("res/scarfy_run.gif", &animFrames);
    defer ray.UnloadImage(imScarfyAnim);
    // Load texture from image
    // NOTE: We will update this texture when required with next frame data
    // WARNING: It's not recommended to use this technique for sprites animation,
    // use spritesheets instead, like illustrated in textures_sprite_anim example
    const texScarfyAnim = ray.LoadTextureFromImage(imScarfyAnim);
    defer ray.UnloadTexture(texScarfyAnim);

    var nextFrameDataOffset: c_int = 0; // Current byte offset to next frame in image.data

    var currentAnimFrame: c_int = 0; // Current animation frame to load and draw
    var frameDelay: c_int = 8; // Frame delay to switch between animation frames
    var frameCounter: c_int = 0; // General frames counter

    while (!ray.WindowShouldClose()) {

        // Update
        frameCounter += 1;
        if (frameCounter >= frameDelay) {
            // Move to next frame
            // NOTE: If final frame is reached we return to first frame
            currentAnimFrame += 1;
            if (currentAnimFrame >= animFrames) currentAnimFrame = 0;

            // Get memory offset position for next frame data in image.data
            nextFrameDataOffset = imScarfyAnim.width * imScarfyAnim.height * 4 * currentAnimFrame;

            // Update GPU texture data with next frame image data
            // WARNING: Data size (frame size) and pixel format must match already created texture
            const offset: usize = @intCast(nextFrameDataOffset);
            const a: *anyopaque = @ptrFromInt(@intFromPtr(imScarfyAnim.data.?) + offset);
            ray.UpdateTexture(texScarfyAnim, a);

            frameCounter = 0;
        }

        // Control frames delay
        if (ray.IsKeyPressed(ray.KEY_RIGHT))
            frameDelay += 1
        else if (ray.IsKeyPressed(ray.KEY_LEFT))
            frameDelay -= 1;

        if (frameDelay > MAX_FRAME_DELAY) frameDelay = MAX_FRAME_DELAY else if (frameDelay < MIN_FRAME_DELAY) frameDelay = MIN_FRAME_DELAY;
        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText(ray.TextFormat("TOTAL GIF FRAMES:  %02i", animFrames), 50, 30, 20, ray.LIGHTGRAY);
        ray.DrawText(ray.TextFormat("CURRENT FRAME: %02i", currentAnimFrame), 50, 60, 20, ray.GRAY);
        ray.DrawText(ray.TextFormat("CURRENT FRAME IMAGE.DATA OFFSET: %02i", nextFrameDataOffset), 50, 90, 20, ray.GRAY);

        ray.DrawText("FRAMES DELAY: ", 100, 305, 10, ray.DARKGRAY);
        ray.DrawText(ray.TextFormat("%02i frames", frameDelay), 620, 305, 10, ray.DARKGRAY);
        ray.DrawText("PRESS RIGHT/LEFT KEYS to CHANGE SPEED!", 290, 350, 10, ray.DARKGRAY);

        for (0..MAX_FRAME_DELAY) |i| {
            const index: c_int = @intCast(i);
            if (i < frameDelay) ray.DrawRectangle(190 + 21 * index, 300, 20, 20, ray.RED);
            ray.DrawRectangleLines(190 + 21 * index, 300, 20, 20, ray.MAROON);
        }

        const y = @divTrunc(ray.GetScreenWidth(), 2) - @divTrunc(texScarfyAnim.width, 2);
        ray.DrawTexture(texScarfyAnim, y, 140, ray.WHITE);

        ray.DrawText("(c) Scarfy sprite by Eiden Marsal", screenWidth - 200, screenHeight - 20, 10, ray.GRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![自带字体][1]

## 总结

加载 Raylib 自带的字体。

[1]: images/raylib-texture-font.png

## 附录
