# 0414-Raylib-播放音效

## 环境

- Time 2024-03-06
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

播放音效。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MAX_CIRCLES = 64;

const CircleWave = struct {
    position: ray.Vector2,
    radius: f32,
    alpha: f32 = 0,
    speed: f32,
    color: ray.Color,
};

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [audio] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);
    ray.InitAudioDevice();
    defer ray.CloseAudioDevice();

    const fxWav = ray.LoadSound("res/sound.wav"); // Load WAV audio file
    defer ray.UnloadSound(fxWav);
    const fxOgg = ray.LoadSound("res/target.ogg"); // Load OGG audio file
    defer ray.UnloadSound(fxOgg);

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsKeyPressed(ray.KEY_SPACE)) ray.PlaySound(fxWav); // Play WAV sound
        if (ray.IsKeyPressed(ray.KEY_ENTER)) ray.PlaySound(fxOgg); // Play OGG sound

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("Press SPACE to PLAY the WAV sound!", 200, 180, 20, ray.LIGHTGRAY);
        ray.DrawText("Press ENTER to PLAY the OGG sound!", 200, 220, 20, ray.LIGHTGRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![播放音效][1]

## 总结

播放音效。

[1]: images/raylib-audio-sound.png

## 附录
