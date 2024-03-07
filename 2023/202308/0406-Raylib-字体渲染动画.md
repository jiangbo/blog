# 0406-Raylib-字体渲染动画

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

渲染字体时，一个字一个字地显示，类似动画。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [text] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    const message: [:0]const u8 =
        \\ This sample illustrates a text writing
        \\ animation effect! Check it out! ;)
    ;
    var framesCounter: usize = 0;

    while (!ray.WindowShouldClose()) {

        // Update
        framesCounter += if (ray.IsKeyDown(ray.KEY_SPACE)) 8 else 1;
        if (ray.IsKeyPressed(ray.KEY_ENTER)) framesCounter = 0;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        const text = ray.TextSubtext(message, 0, @as(c_int, @intCast(framesCounter / 10)));
        ray.DrawText(text, 210, 160, 20, ray.MAROON);
        ray.DrawText("PRESS [ENTER] to RESTART!", 240, 260, 20, ray.LIGHTGRAY);
        ray.DrawText("PRESS [SPACE] to SPEED UP!", 239, 300, 20, ray.LIGHTGRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![字体动画][1]

## 总结

渲染字体时，一个字一个字地显示，类似动画。

[1]: images/raylib-text-animation.png

## 附录
