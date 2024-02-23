# 0363-Raylib-生成随机数

## 环境

- Time 2024-02-23
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

生成一个随机整数并且显示到界面上。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() void {
    const screenWidth = 800;
    const screenHeight = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var randValue: c_int = ray.GetRandomValue(-8, 5); // Get a random integer number between -8 and 5 (both included)

    var framesCounter: usize = 0; // Variable used to count frames

    while (!ray.WindowShouldClose()) {
        // Update
        framesCounter += 1;

        // Every two seconds (120 frames) a new random value is generated
        if (((framesCounter / 120) % 2) == 1) {
            randValue = ray.GetRandomValue(-8, 5);
            framesCounter = 0;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();

        ray.ClearBackground(ray.RAYWHITE);
        ray.DrawText("Every 2 seconds a new random value is generated:", 130, 100, 20, ray.MAROON);
        ray.DrawText(ray.TextFormat("%i", randValue), 360, 180, 80, ray.LIGHTGRAY);
    }
}
```

## 效果

![2D 随机数][1]

## 总结

生成随机数。

[1]: images/raylib-2d-random.png

## 附录
