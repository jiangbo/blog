# 0413-Raylib-播放音乐

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

播放音乐，实现了音乐的暂停和从头开始播放。

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

    const music = ray.LoadMusicStream("res/country.mp3");
    defer ray.UnloadMusicStream(music);
    ray.PlayMusicStream(music);

    var timePlayed: f32 = 0.0;
    var pause = false;

    while (!ray.WindowShouldClose()) {

        // Update
        ray.UpdateMusicStream(music); // Update music buffer with new stream data

        // Restart music playing (stop and play)
        if (ray.IsKeyPressed(ray.KEY_SPACE)) {
            ray.StopMusicStream(music);
            ray.PlayMusicStream(music);
            pause = false;
        }

        // Pause/Resume music playing
        if (ray.IsKeyPressed(ray.KEY_P)) {
            pause = !pause;
            if (pause) ray.PauseMusicStream(music) else ray.ResumeMusicStream(music);
        }

        // Get timePlayed scaled to bar dimensions
        timePlayed = ray.GetMusicTimePlayed(music) / ray.GetMusicTimeLength(music);
        if (timePlayed > 1.0) timePlayed = 1.0; // Make sure time played is no longer than music

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("MUSIC SHOULD BE PLAYING!", 255, 150, 20, ray.LIGHTGRAY);

        const width: c_int = @intFromFloat(timePlayed * 400.0);
        ray.DrawRectangle(200, 200, 400, 12, ray.LIGHTGRAY);
        ray.DrawRectangle(200, 200, width, 12, ray.MAROON);
        ray.DrawRectangleLines(200, 200, 400, 12, ray.GRAY);

        ray.DrawText("PRESS SPACE TO RESTART MUSIC", 215, 250, 20, ray.LIGHTGRAY);
        ray.DrawText("PRESS P TO PAUSE/RESUME MUSIC", 208, 280, 20, ray.LIGHTGRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![音乐播放][1]

## 总结

播放音乐，实现了音乐的暂停和从头开始播放。

[1]: images/raylib-audio-music.png

## 附录
