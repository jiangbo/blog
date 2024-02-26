# 0367-Raylib-进度条

## 环境

- Time 2024-02-26
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

实现一个进度条，显示加载的进度。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const State = enum { STATE_WAITING, STATE_LOADING, STATE_FINISHED };
var dataLoaded = std.atomic.Value(bool).init(false);
var dataProgress = std.atomic.Value(i64).init(0);

pub fn main() !void {
    const screenWidth = 800;
    const screenHeight = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var framesCounter: usize = 0;
    var state = State.STATE_WAITING;
    var thread: std.Thread = undefined;

    while (!ray.WindowShouldClose()) {

        // Update
        switch (state) {
            .STATE_WAITING => {
                if (ray.IsKeyPressed(ray.KEY_ENTER)) {
                    thread = try std.Thread.spawn(.{}, loadData, .{});
                    state = .STATE_LOADING;
                }
            },
            .STATE_LOADING => {
                framesCounter += 1;
                if (dataLoaded.load(.Unordered)) {
                    framesCounter = 0;
                    thread.join();
                    state = .STATE_FINISHED;
                }
            },
            .STATE_FINISHED => {
                if (ray.IsKeyPressed(ray.KEY_ENTER)) {
                    // Reset everything to launch again
                    dataLoaded.store(false, .Unordered);
                    dataProgress.store(0, .Unordered);
                    state = .STATE_WAITING;
                }
            },
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        switch (state) {
            .STATE_WAITING => ray.DrawText("PRESS ENTER to START LOADING DATA", 150, 170, 20, ray.DARKGRAY),
            .STATE_LOADING => {
                const width: c_int = @intCast(dataProgress.load(.Unordered));
                ray.DrawRectangle(150, 200, width, 60, ray.SKYBLUE);
                ray.DrawText("LOADING DATA...", 240, 210, 40, ray.DARKBLUE);
            },
            .STATE_FINISHED => {
                ray.DrawRectangle(150, 200, 500, 60, ray.LIME);
                ray.DrawText("DATA LOADED!", 250, 210, 40, ray.GREEN);
            },
        }

        ray.DrawRectangleLines(150, 200, 500, 60, ray.DARKGRAY);
    }
}

// Loading data thread function definition
fn loadData() void {
    var timeCounter: i64 = 0; // Time counted in ms
    // clock_t prevTime = clock();     // Previous time
    const prevTime = std.time.milliTimestamp();
    // We simulate data loading with a time counter for 5 seconds
    while (timeCounter < 5000) {
        timeCounter = std.time.milliTimestamp() - prevTime;
        // We accumulate time over a global variable to be used in
        // main thread as a progress bar
        dataProgress.store(@divTrunc(timeCounter, 10), .Unordered);
    }

    // When data has finished loading, we set global variable
    dataLoaded.store(true, .Unordered);
}
```

## 效果

![2D 进度条][1]

## 总结

实现进度条，实时显示进度。

[1]: images/raylib-2d-progress.png

## 附录
