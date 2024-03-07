# 0416-Raylib-同时播放音效

## 环境

- Time 2024-03-07
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

同时播放几个音效。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MAX_SOUNDS = 10;
var soundArray: [MAX_SOUNDS]ray.Sound = undefined;
var currentSound: usize = 0;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [audio] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    ray.InitAudioDevice(); // Initialize audio device
    defer ray.CloseAudioDevice();

    // load the sound list
    // Load WAV audio file into the first slot as the 'source' sound
    soundArray[0] = ray.LoadSound("res/sound.wav");
    defer ray.UnloadSound(soundArray[0]);

    // this sound owns the sample data
    for (1..MAX_SOUNDS) |i| {
        // Load an alias of the sound into slots 1-9. These do not own the sound data, but can be played
        soundArray[i] = ray.LoadSoundAlias(soundArray[0]);
    }
    defer for (soundArray[1..]) |sound| ray.UnloadSoundAlias(sound);

    currentSound = 0;

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsKeyPressed(ray.KEY_SPACE)) {
            ray.PlaySound(soundArray[currentSound]); // play the next open sound slot
            currentSound += 1; // increment the sound slot
            if (currentSound >= MAX_SOUNDS) currentSound = 0;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("Press SPACE to PLAY a WAV sound!", 200, 180, 20, ray.LIGHTGRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![同时播放][1]

## 总结

同时播放几个音效。

[1]: images/raylib-audio-multi.png

## 附录
