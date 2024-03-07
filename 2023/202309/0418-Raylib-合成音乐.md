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

播放音乐时，进行音效混合。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

var exponent: f32 = 1.0; // Audio exponentiation value
var averageVolume: [400]f32 = .{0} ** 400; // Average volume history

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [audio] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    ray.InitAudioDevice(); // Initialize audio device
    defer ray.CloseAudioDevice();

    ray.AttachAudioMixedProcessor(ProcessAudio);
    defer ray.DetachAudioMixedProcessor(ProcessAudio);

    const music = ray.LoadMusicStream("res/country.mp3");
    defer ray.UnloadMusicStream(music);
    const sound = ray.LoadSound("res/coin.wav");
    defer ray.UnloadSound(sound);

    ray.PlayMusicStream(music);

    while (!ray.WindowShouldClose()) {

        // Update
        ray.UpdateMusicStream(music); // Update music buffer with new stream data

        // Modify processing variables
        //----------------------------------------------------------------------------------
        if (ray.IsKeyPressed(ray.KEY_LEFT)) exponent -= 0.05;
        if (ray.IsKeyPressed(ray.KEY_RIGHT)) exponent += 0.05;

        if (exponent <= 0.5) exponent = 0.5;
        if (exponent >= 3.0) exponent = 3.0;

        if (ray.IsKeyPressed(ray.KEY_SPACE)) ray.PlaySound(sound);

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("MUSIC SHOULD BE PLAYING!", 255, 150, 20, ray.LIGHTGRAY);

        ray.DrawText(ray.TextFormat("EXPONENT = %.2f", exponent), 215, 180, 20, ray.LIGHTGRAY);

        ray.DrawRectangle(199, 199, 402, 34, ray.LIGHTGRAY);
        for (0..averageVolume.len) |i| {
            const y: c_int = @intFromFloat(232 - averageVolume[i] * 32);
            ray.DrawLine(@intCast(201 + i), y, @intCast(201 + i), 232, ray.MAROON);
        }
        ray.DrawRectangleLines(199, 199, 402, 34, ray.GRAY);

        ray.DrawText("PRESS SPACE TO PLAY OTHER SOUND", 200, 250, 20, ray.LIGHTGRAY);
        ray.DrawText("USE LEFT AND RIGHT ARROWS TO ALTER DISTORTION", 140, 280, 20, ray.LIGHTGRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}

fn ProcessAudio(buffer: ?*anyopaque, frames: c_uint) callconv(.C) void {
    var samples: [*c]f32 = @ptrCast(@alignCast(buffer.?)); // Samples internally stored as <float>s
    var average: f32 = 0.0; // Temporary average volume

    for (0..frames) |frame| {
        const left = &samples[frame * 2 + 0];
        const right = &samples[frame * 2 + 1];

        left.* = std.math.pow(f32, @abs(left.*), exponent) * @as(f32, if (left.* < 0.0) -1.0 else 1.0);
        right.* = std.math.pow(f32, @abs(right.*), exponent) * @as(f32, if (right.* < 0.0) -1.0 else 1.0);

        average += @abs(left.*) / @as(f32, @floatFromInt(frames)); // accumulating average volume
        average += @abs(right.*) / @as(f32, @floatFromInt(frames));
    }

    // Moving history to the left
    for (0..averageVolume.len - 1) |i| averageVolume[i] = averageVolume[i + 1];

    averageVolume[399] = average; // Adding last average value
}
```

## 效果

![合成音乐][1]

## 总结

播放音乐时，进行音效混合。

[1]: images/raylib-audio-mix.png

## 附录
