# 0352-Raylib-屏幕切换

## 环境

- Time 2024-02-07
- Zig 0.12.0-dev.2543+9eda6ccef
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

在不同的屏幕之间进行切换。

## update

```zig
fn update() void {
    switch (currentScreen) {
        .LOGO => {
            // TODO: Update LOGO screen variables here!
            framesCounter += 1; // Count frames
            // Wait for 2 seconds (120 frames) before jumping to TITLE screen
            if (framesCounter > 120) currentScreen = .TITLE;
        },
        .TITLE => {
            // TODO: Update TITLE screen variables here!

            // Press enter to change to GAMEPLAY screen
            if (ray.IsKeyPressed(ray.KEY_ENTER) or ray.IsGestureDetected(ray.GESTURE_TAP)) {
                currentScreen = .GAMEPLAY;
            }
        },
        .GAMEPLAY => {
            // TODO: Update GAMEPLAY screen variables here!

            // Press enter to change to ENDING screen
            if (ray.IsKeyPressed(ray.KEY_ENTER) or ray.IsGestureDetected(ray.GESTURE_TAP)) {
                currentScreen = .ENDING;
            }
        },
        .ENDING => {
            // TODO: Update ENDING screen variables here!

            // Press enter to return to TITLE screen
            if (ray.IsKeyPressed(ray.KEY_ENTER) or ray.IsGestureDetected(ray.GESTURE_TAP)) {
                currentScreen = .TITLE;
            }
        },
    }
}
```

## draw

```zig
fn draw() void {
    ray.BeginDrawing();
    defer ray.EndDrawing();

    ray.ClearBackground(ray.RAYWHITE);
    switch (currentScreen) {
        .LOGO => {
            // TODO: Draw LOGO screen here!
            ray.DrawText("LOGO SCREEN", 20, 20, 40, ray.LIGHTGRAY);
            ray.DrawText("WAIT for 2 SECONDS...", 290, 220, 20, ray.GRAY);
        },
        .TITLE => {
            // TODO: Draw TITLE screen here!
            ray.DrawRectangle(0, 0, screenWidth, screenHeight, ray.GREEN);
            ray.DrawText("TITLE SCREEN", 20, 20, 40, ray.DARKGREEN);
            ray.DrawText("PRESS ENTER or TAP to JUMP to GAMEPLAY SCREEN", 120, 220, 20, ray.DARKGREEN);
        },
        .GAMEPLAY => {
            // TODO: Draw GAMEPLAY screen here!
            ray.DrawRectangle(0, 0, screenWidth, screenHeight, ray.PURPLE);
            ray.DrawText("GAMEPLAY SCREEN", 20, 20, 40, ray.MAROON);
            ray.DrawText("PRESS ENTER or TAP to JUMP to ENDING SCREEN", 130, 220, 20, ray.MAROON);
        },
        .ENDING => {
            // TODO: Draw ENDING screen here!
            ray.DrawRectangle(0, 0, screenWidth, screenHeight, ray.BLUE);
            ray.DrawText("ENDING SCREEN", 20, 20, 40, ray.DARKBLUE);
            ray.DrawText("PRESS ENTER or TAP to RETURN to TITLE SCREEN", 120, 220, 20, ray.DARKBLUE);
        },
    }
}
```

## main

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const GameScreen = enum { LOGO, TITLE, GAMEPLAY, ENDING };

const screenWidth = 800;
const screenHeight = 450;

var currentScreen = GameScreen.LOGO;
var framesCounter: usize = 0;

pub fn main() void {
    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    while (!ray.WindowShouldClose()) {
        update();
        draw();
    }
}
```

## 效果

![屏幕切换][1]

## 总结

使用 Zig 和 Raylib 库，实现了多个屏幕之间的切换。

[1]: images/raylib-screen.png

## 附录
