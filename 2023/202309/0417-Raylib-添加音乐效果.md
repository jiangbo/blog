# 0417-Raylib-添加音乐效果

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

播放音乐时，添加音乐效果。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

// Required delay effect variables
var delayBuffer: []f32 = undefined;
var delayBufferSize: c_uint = 0;
var delayReadIndex: c_uint = 2;
var delayWriteIndex: c_uint = 0;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [audio] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    ray.InitAudioDevice(); // Initialize audio device
    defer ray.CloseAudioDevice();

    const music = ray.LoadMusicStream("res/country.mp3");
    defer ray.UnloadMusicStream(music);

    // Allocate buffer for the delay effect
    delayBufferSize = 48000 * 2; // 1 second delay (device sampleRate*channels)
    const allocator = std.heap.c_allocator;
    delayBuffer = try allocator.alloc(f32, delayBufferSize);

    ray.PlayMusicStream(music);

    var timePlayed: f32 = 0.0; // Time played normalized [0.0f..1.0f]
    var pause = false; // Music playing paused

    var enableEffectLPF = false; // Enable effect low-pass-filter
    var enableEffectDelay = false; // Enable effect delay (1 second)

    while (!ray.WindowShouldClose()) {

        // Update
        ray.UpdateMusicStream(music); // Update music buffer with new stream data

        // Restart music playing (stop and play)
        if (ray.IsKeyPressed(ray.KEY_SPACE)) {
            ray.StopMusicStream(music);
            ray.PlayMusicStream(music);
        }

        // Pause/Resume music playing
        if (ray.IsKeyPressed(ray.KEY_P)) {
            pause = !pause;
            if (pause) ray.PauseMusicStream(music) else ray.ResumeMusicStream(music);
        }

        // Add/Remove effect: lowpass filter
        if (ray.IsKeyPressed(ray.KEY_F)) {
            enableEffectLPF = !enableEffectLPF;
            if (enableEffectLPF)
                ray.AttachAudioStreamProcessor(music.stream, AudioProcessEffectLPF)
            else
                ray.DetachAudioStreamProcessor(music.stream, AudioProcessEffectLPF);
        }

        // Add/Remove effect: delay
        if (ray.IsKeyPressed(ray.KEY_D)) {
            enableEffectDelay = !enableEffectDelay;
            if (enableEffectDelay)
                ray.AttachAudioStreamProcessor(music.stream, AudioProcessEffectDelay)
            else
                ray.DetachAudioStreamProcessor(music.stream, AudioProcessEffectDelay);
        }

        // Get normalized time played for current music stream
        timePlayed = ray.GetMusicTimePlayed(music) / ray.GetMusicTimeLength(music);

        if (timePlayed > 1.0) timePlayed = 1.0; // Make sure time played is no longer than music

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("MUSIC SHOULD BE PLAYING!", 245, 150, 20, ray.LIGHTGRAY);

        ray.DrawRectangle(200, 180, 400, 12, ray.LIGHTGRAY);
        ray.DrawRectangle(200, 180, @intFromFloat((timePlayed * 400.0)), 12, ray.MAROON);
        ray.DrawRectangleLines(200, 180, 400, 12, ray.GRAY);

        ray.DrawText("PRESS SPACE TO RESTART MUSIC", 215, 230, 20, ray.LIGHTGRAY);
        ray.DrawText("PRESS P TO PAUSE/RESUME MUSIC", 208, 260, 20, ray.LIGHTGRAY);

        var text: [*c]const u8 = if (enableEffectLPF) "ON" else "OFF";
        ray.DrawText(ray.TextFormat("PRESS F TO TOGGLE LPF EFFECT: %s", text), 200, 320, 20, ray.GRAY);
        text = if (enableEffectDelay) "ON" else "OFF";
        ray.DrawText(ray.TextFormat("PRESS D TO TOGGLE DELAY EFFECT: %s", text), 180, 350, 20, ray.GRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}

var low: [2]f32 = .{ 0, 0 };
const cutoff = 70.0 / 44100.0; // 70 Hz lowpass filter

// Audio effect: lowpass filter
fn AudioProcessEffectLPF(buffer: ?*anyopaque, frames: c_uint) callconv(.C) void {
    const k = cutoff / (cutoff + 0.1591549431); // RC filter formula

    // Converts the buffer data before using it
    var bufferData: [*c]f32 = @ptrCast(@alignCast(buffer.?));
    var i: c_uint = 0;
    while (i < frames * 2) : (i += 2) {
        const l = bufferData[i];
        const r = bufferData[i + 1];

        low[0] += k * (l - low[0]);
        low[1] += k * (r - low[1]);
        bufferData[i] = low[0];
        bufferData[i + 1] = low[1];
    }
}

// Audio effect: delay
fn AudioProcessEffectDelay(b: ?*anyopaque, frames: c_uint) callconv(.C) void {
    var i: c_uint = 0;
    var buffer: [*c]f32 = @ptrCast(@alignCast(b.?));
    while (i < frames * 2) : (i += 2) {
        const leftDelay = delayBuffer[delayReadIndex]; // ERROR: Reading buffer -> WHY??? Maybe thread related???
        delayReadIndex += 1;
        const rightDelay = delayBuffer[delayReadIndex];
        delayReadIndex += 1;
        if (delayReadIndex == delayBufferSize) delayReadIndex = 0;

        (buffer)[i] = 0.5 * (buffer)[i] + 0.5 * leftDelay;
        (buffer)[i + 1] = 0.5 * (buffer)[i + 1] + 0.5 * rightDelay;

        delayBuffer[delayWriteIndex] = (buffer)[i];
        delayWriteIndex += 1;
        delayBuffer[delayWriteIndex] = (buffer)[i + 1];
        delayWriteIndex += 1;
        if (delayWriteIndex == delayBufferSize) delayWriteIndex = 0;
    }
}
```

## 效果

![音乐效果][1]

## 总结

播放音乐时，添加音乐效果。

[1]: images/raylib-audio-effect.png

## 附录
