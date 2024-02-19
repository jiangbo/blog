# 0356-Raylib-2D 屏幕分屏

## 环境

- Time 2024-02-19
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

实现了 2D 屏幕的分屏显示，可以各自控制一半屏幕。

## update

```zig
fn update(camera1: *ray.Camera2D, camera2: *ray.Camera2D) void {
    if (ray.IsKeyDown(ray.KEY_S)) player1.y += 3.0 else if (ray.IsKeyDown(ray.KEY_W)) player1.y -= 3.0;
    if (ray.IsKeyDown(ray.KEY_D)) player1.x += 3.0 else if (ray.IsKeyDown(ray.KEY_A)) player1.x -= 3.0;

    if (ray.IsKeyDown(ray.KEY_UP)) player2.y -= 3.0 else if (ray.IsKeyDown(ray.KEY_DOWN)) player2.y += 3.0;
    if (ray.IsKeyDown(ray.KEY_RIGHT)) player2.x += 3.0 else if (ray.IsKeyDown(ray.KEY_LEFT)) player2.x -= 3.0;

    camera1.target = .{ .x = player1.x, .y = player1.y };
    camera2.target = .{ .x = player2.x, .y = player2.y };
}
```

## draw

```zig
fn draw(camera1: ray.Camera2D, camera2: ray.Camera2D) void {
    ray.BeginTextureMode(screenCamera1);
    ray.ClearBackground(ray.RAYWHITE);

    ray.BeginMode2D(camera1);

    // Draw full scene with first camera
    for (0..screenWidth / PLAYER_SIZE + 1) |i| {
        const x: f32 = @floatFromInt(PLAYER_SIZE * i);
        ray.DrawLineV(.{ .x = x }, .{ .x = x, .y = screenHeight }, ray.LIGHTGRAY);
    }

    for (0..screenHeight / PLAYER_SIZE + 1) |i| {
        const y: f32 = @floatFromInt(PLAYER_SIZE * i);
        ray.DrawLineV(.{ .y = y }, .{ .x = screenWidth, .y = y }, ray.LIGHTGRAY);
    }

    for (0..screenWidth / PLAYER_SIZE) |i| {
        for (0..screenHeight / PLAYER_SIZE) |j| {
            const x: c_int = @intCast(10 + PLAYER_SIZE * i);
            const y: c_int = @intCast(15 + PLAYER_SIZE * j);
            ray.DrawText(ray.TextFormat("[%i,%i]", i, j), x, y, 10, ray.LIGHTGRAY);
        }
    }

    ray.DrawRectangleRec(player1, ray.RED);
    ray.DrawRectangleRec(player2, ray.BLUE);
    ray.EndMode2D();

    ray.DrawRectangle(0, 0, @divTrunc(ray.GetScreenWidth(), 2), 30, ray.Fade(ray.RAYWHITE, 0.6));
    ray.DrawText("PLAYER1: W/S/A/D to move", 10, 10, 10, ray.MAROON);

    ray.EndTextureMode();

    ray.BeginTextureMode(screenCamera2);
    ray.ClearBackground(ray.RAYWHITE);

    ray.BeginMode2D(camera2);

    // Draw full scene with second camera
    for (0..screenWidth / PLAYER_SIZE + 1) |i| {
        const x: f32 = @floatFromInt(PLAYER_SIZE * i);
        ray.DrawLineV(.{ .x = x }, .{ .x = x, .y = screenHeight }, ray.LIGHTGRAY);
    }

    for (0..screenHeight / PLAYER_SIZE + 1) |i| {
        const y: f32 = @floatFromInt(PLAYER_SIZE * i);
        ray.DrawLineV(.{ .y = y }, .{ .x = screenWidth, .y = y }, ray.LIGHTGRAY);
    }

    for (0..screenWidth / PLAYER_SIZE) |i| {
        for (0..screenHeight / PLAYER_SIZE) |j| {
            const x: c_int = @intCast(10 + PLAYER_SIZE * i);
            const y: c_int = @intCast(15 + PLAYER_SIZE * j);
            ray.DrawText(ray.TextFormat("[%i,%i]", i, j), x, y, 10, ray.LIGHTGRAY);
        }
    }

    ray.DrawRectangleRec(player1, ray.RED);
    ray.DrawRectangleRec(player2, ray.BLUE);

    ray.EndMode2D();

    ray.DrawRectangle(0, 0, @divTrunc(ray.GetScreenWidth(), 2), 30, ray.Fade(ray.RAYWHITE, 0.6));
    ray.DrawText("PLAYER2: UP/DOWN/LEFT/RIGHT to move", 10, 10, 10, ray.DARKBLUE);

    ray.EndTextureMode();

    // Draw both views render textures to the screen side by side
    ray.BeginDrawing();
    ray.ClearBackground(ray.BLACK);

    ray.DrawTextureRec(screenCamera1.texture, splitScreenRect, .{}, ray.WHITE);
    ray.DrawTextureRec(screenCamera2.texture, splitScreenRect, .{ .x = screenWidth / 2.0 }, ray.WHITE);

    ray.DrawRectangle(@divTrunc(ray.GetScreenWidth(), 2) - 2, 0, 4, ray.GetScreenHeight(), ray.LIGHTGRAY);
    ray.EndDrawing();
}
```

## main

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const screenWidth = 800;
const screenHeight = 450;
const PLAYER_SIZE = 40;

var player1 = ray.Rectangle{ .x = 200, .y = 200, .width = PLAYER_SIZE, .height = PLAYER_SIZE };
var player2 = ray.Rectangle{ .x = 250, .y = 200, .width = PLAYER_SIZE, .height = PLAYER_SIZE };

var screenCamera1: ray.RenderTexture = undefined;
var screenCamera2: ray.RenderTexture = undefined;

// Build a flipped rectangle the size of the split view to use for drawing later
var splitScreenRect: ray.Rectangle = undefined;

pub fn main() void {
    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    screenCamera1 = ray.LoadRenderTexture(screenWidth / 2, screenHeight);
    screenCamera2 = ray.LoadRenderTexture(screenWidth / 2, screenHeight);

    splitScreenRect = .{
        .x = 0.0,
        .y = 0.0,
        .width = @floatFromInt(screenCamera1.texture.width),
        .height = @floatFromInt(-screenCamera1.texture.height),
    };

    var camera1: ray.Camera2D = .{
        .target = .{ .x = player1.x, .y = player1.y },
        .offset = .{ .x = 200, .y = 200 },
        .zoom = 1.0,
    };

    var camera2: ray.Camera2D = .{
        .target = .{ .x = player2.x, .y = player2.y },
        .offset = .{ .x = 200, .y = 200 },
        .zoom = 1.0,
    };

    while (!ray.WindowShouldClose()) {
        update(&camera1, &camera2);
        draw(camera1, camera2);
    }
}
```

## 效果

![2D 分屏][1]

## 总结

实现了 2D 分屏，可以各自控制屏幕的一边，两边可以进行同步。

[1]: images/raylib-2d-split.png

## 附录
