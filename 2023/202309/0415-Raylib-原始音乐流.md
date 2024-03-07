# 0415-Raylib-原始音乐流

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

播放原始音乐流。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MAX_SAMPLES = 512;
const MAX_SAMPLES_PER_UPDATE = 4096;

// Cycles per second (hz)
var frequency: f32 = 440.0;

// Audio frequency, for smoothing
var audioFrequency: f32 = 440.0;

// Previous value, used to test if sine needs to be rewritten, and to smoothly modulate frequency
var oldFrequency: f32 = 1.0;

// Index for audio rendering
var sineIdx: f32 = 0.0;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [audio] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    ray.InitAudioDevice(); // Initialize audio device
    defer ray.CloseAudioDevice();

    ray.SetAudioStreamBufferSizeDefault(MAX_SAMPLES_PER_UPDATE);

    // Init raw audio stream (sample rate: 44100, sample size: 16bit-short, channels: 1-mono)
    const stream = ray.LoadAudioStream(44100, 16, 1);
    defer ray.UnloadAudioStream(stream);

    ray.SetAudioStreamCallback(stream, audioInputCallback);

    // Buffer for the single cycle waveform we are synthesizing
    const allocator = std.heap.c_allocator;
    var data = try allocator.alloc(c_short, MAX_SAMPLES);
    defer allocator.free(data);

    // Frame buffer, describing the waveform when repeated over the course of a frame
    // var writeBuf = allocator.alloc(c_short, MAX_SAMPLES_PER_UPDATE);

    ray.PlayAudioStream(stream); // Start processing stream buffer (no data loaded currently)

    // Position read in to determine next frequency
    var mousePosition = ray.Vector2{ .x = -100.0, .y = -100.0 };

    // Computed size in samples of the sine wave
    var waveLength: c_int = 1;

    var position = ray.Vector2{};
    while (!ray.WindowShouldClose()) {

        // Update
        // Sample mouse input.
        mousePosition = ray.GetMousePosition();

        if (ray.IsMouseButtonDown(ray.MOUSE_BUTTON_LEFT)) {
            const fp = (mousePosition.y);
            frequency = 40.0 + (fp);

            const pan = (mousePosition.x) / screenWidth;
            ray.SetAudioStreamPan(stream, pan);
        }

        // Rewrite the sine wave
        // Compute two cycles to allow the buffer padding, simplifying any modulation, resampling, etc.
        if (frequency != oldFrequency) {
            // Compute wavelength. Limit size in both directions.
            //int oldWavelength = waveLength;
            waveLength = @intFromFloat(22050 / frequency);
            if (waveLength > MAX_SAMPLES / 2) waveLength = MAX_SAMPLES / 2;
            if (waveLength < 1) waveLength = 1;

            // Write sine wave
            for (0..@intCast(waveLength * 2)) |i| {
                const index: f32 = @floatFromInt(i);
                const value = 2 * std.math.pi * index / @as(f32, @floatFromInt(waveLength));
                data[i] = @intFromFloat(@sin(value) * 32000);
            }
            // Make sure the rest of the line is flat
            for (@intCast(waveLength * 2)..MAX_SAMPLES) |j| {
                data[j] = 0;
            }

            // Scale read cursor's position to minimize transition artifacts
            //readCursor = (int)(readCursor * ((float)waveLength / (float)oldWavelength));
            oldFrequency = frequency;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText(ray.TextFormat("sine frequency: %i", frequency), ray.GetScreenWidth() - 220, 10, 20, ray.RED);
        ray.DrawText("click mouse button to change frequency or pan", 10, 10, 20, ray.DARKGRAY);

        // Draw the current buffer state proportionate to the screen
        for (0..screenWidth) |i| {
            position.x = @floatFromInt(i);
            position.y = 250 + @as(f32, @floatFromInt(50 * data[i * @divTrunc(MAX_SAMPLES, screenWidth)])) / 32000.0;

            ray.DrawPixelV(position, ray.RED);
        }

        ray.DrawFPS(screenWidth - 100, 10);
    }
}

// Audio input processing callback
fn audioInputCallback(buffer: ?*anyopaque, frames: c_uint) callconv(.C) void {
    audioFrequency = frequency + (audioFrequency - frequency) * 0.95;

    const incr = audioFrequency / 44100.0;
    // short *d = (short *)buffer ;
    var d: [*c]c_short = @ptrCast(@alignCast(buffer.?));

    for (0..frames) |i| {
        d[i] = @intFromFloat((32000.0 * @sin(2 * std.math.pi * sineIdx)));
        sineIdx += incr;
        if (sineIdx > 1.0) sineIdx -= 1.0;
    }
}
```

## 效果

![原始音乐][1]

## 总结

播放原始音乐流。

[1]: images/raylib-audio-raw.png

## 附录
