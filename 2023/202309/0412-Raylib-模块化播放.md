# 0412-Raylib-模块化播放

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

音乐播放。

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

    ray.InitAudioDevice(); // Initialize audio device
    defer ray.CloseAudioDevice();

    const colors = [_]ray.Color{
        ray.ORANGE, ray.RED,     ray.GOLD,      ray.LIME,  ray.BLUE, //
        ray.VIOLET, ray.BROWN,   ray.LIGHTGRAY, ray.PINK,  ray.YELLOW,
        ray.GREEN,  ray.SKYBLUE, ray.PURPLE,    ray.BEIGE,
    };

    // Creates some circles for visual effect
    var circles: [MAX_CIRCLES]CircleWave = undefined;

    for (&circles) |*circle| {
        const radius = ray.GetRandomValue(10, 40);
        circle.* = CircleWave{
            .radius = @floatFromInt(radius),
            .position = .{
                .x = randomValueF(radius, screenWidth - radius),
                .y = randomValueF(radius, screenHeight - radius),
            },
            .speed = randomValueF(1, 100) / 2000.0,
            .color = colors[@intCast(ray.GetRandomValue(0, colors.len - 1))],
        };
    }

    var music = ray.LoadMusicStream("res/mini1111.xm");
    defer ray.UnloadMusicStream(music);
    music.looping = false;
    var pitch: f32 = 1.0;

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

        if (ray.IsKeyDown(ray.KEY_DOWN))
            pitch -= 0.01
        else if (ray.IsKeyDown(ray.KEY_UP)) pitch += 0.01;

        ray.SetMusicPitch(music, pitch);

        // Get timePlayed scaled to bar dimensions
        timePlayed = ray.GetMusicTimePlayed(music) / ray.GetMusicTimeLength(music) * (screenWidth - 40);

        // Color circles animation
        var it = std.mem.reverseIterator(&circles);
        while (it.nextPtr()) |circle| {
            circle.alpha += circle.speed;
            circle.radius += circle.speed * 10.0;

            if (circle.alpha > 1.0) circle.speed *= -1;

            if (circle.alpha <= 0.0) {
                const radius = ray.GetRandomValue(10, 40);
                circle.* = CircleWave{
                    .radius = @floatFromInt(radius),
                    .position = .{
                        .x = randomValueF(radius, (screenWidth - radius)),
                        .y = randomValueF(radius, (screenHeight - radius)),
                    },
                    .color = colors[@intCast(ray.GetRandomValue(0, colors.len - 1))],
                    .speed = randomValueF(1, 100) / 2000.0,
                };
            }
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        it = std.mem.reverseIterator(&circles);
        while (it.nextPtr()) |circle| {
            ray.DrawCircleV(circle.position, circle.radius, ray.Fade(circle.color, circle.alpha));
        }

        // Draw time bar
        ray.DrawRectangle(20, screenHeight - 20 - 12, screenWidth - 40, 12, ray.LIGHTGRAY);
        std.log.info("time played: {}", .{timePlayed});
        const width = @as(c_int, @intFromFloat(timePlayed));
        ray.DrawRectangle(20, screenHeight - 20 - 12, width, 12, ray.MAROON);
        ray.DrawRectangleLines(20, screenHeight - 20 - 12, screenWidth - 40, 12, ray.GRAY);

        // Draw help instructions
        ray.DrawRectangle(20, 20, 425, 145, ray.WHITE);
        ray.DrawRectangleLines(20, 20, 425, 145, ray.GRAY);
        ray.DrawText("PRESS SPACE TO RESTART MUSIC", 40, 40, 20, ray.BLACK);
        ray.DrawText("PRESS P TO PAUSE/RESUME", 40, 70, 20, ray.BLACK);
        ray.DrawText("PRESS UP/DOWN TO CHANGE SPEED", 40, 100, 20, ray.BLACK);
        ray.DrawText(ray.TextFormat("SPEED: %f", pitch), 40, 130, 20, ray.MAROON);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}

fn randomValueF(min: c_int, max: c_int) f32 {
    return @floatFromInt(ray.GetRandomValue(min, max));
}
```

## 效果

![音乐播放。][1]

## 总结

音乐播放。

[1]: images/raylib-audio-module.png

## 附录
