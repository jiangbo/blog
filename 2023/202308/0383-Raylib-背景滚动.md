# 0383-Raylib-背景滚动

## 环境

- Time 2024-02-29
- Zig 0.12.0-dev.3076+6e078883e
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

实现背景的滚动。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [shapes] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // NOTE: Be careful, background width must be equal or bigger than screen width
    // if not, texture should be draw more than two times for scrolling effect
    const background = ray.LoadTexture("res/cyberpunk_street_background.png");
    defer ray.UnloadTexture(background);
    const midground = ray.LoadTexture("res/cyberpunk_street_midground.png");
    defer ray.UnloadTexture(midground);
    const foreground = ray.LoadTexture("res/cyberpunk_street_foreground.png");
    defer ray.UnloadTexture(foreground);

    var scrollingBack: f32 = 0.0;
    var scrollingMid: f32 = 0.0;
    var scrollingFore: f32 = 0.0;

    while (!ray.WindowShouldClose()) {

        // Update
        scrollingBack -= 0.1;
        scrollingMid -= 0.5;
        scrollingFore -= 1.0;

        // NOTE: Texture is scaled twice its size, so it sould be considered on scrolling
        const bwidth: f32 = @floatFromInt(background.width);
        const mwidth: f32 = @floatFromInt(midground.width);
        const fwidth: f32 = @floatFromInt(foreground.width);
        if (scrollingBack <= -bwidth * 2) scrollingBack = 0;
        if (scrollingMid <= -mwidth * 2) scrollingMid = 0;
        if (scrollingFore <= -fwidth * 2) scrollingFore = 0;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.GetColor(0x052c46ff));

        // Draw background image twice
        // NOTE: Texture is scaled twice its size
        ray.DrawTextureEx(background, .{ .x = scrollingBack, .y = 20 }, 0.0, 2.0, ray.WHITE);
        ray.DrawTextureEx(background, .{ .x = bwidth * 2 + scrollingBack, .y = 20 }, 0.0, 2.0, ray.WHITE);

        // Draw midground image twice
        ray.DrawTextureEx(midground, .{ .x = scrollingMid, .y = 20 }, 0.0, 2.0, ray.WHITE);
        ray.DrawTextureEx(midground, .{ .x = mwidth * 2 + scrollingMid, .y = 20 }, 0.0, 2.0, ray.WHITE);

        // Draw foreground image twice
        ray.DrawTextureEx(foreground, .{ .x = scrollingFore, .y = 70 }, 0.0, 2.0, ray.WHITE);
        ray.DrawTextureEx(foreground, .{ .x = fwidth * 2 + scrollingFore, .y = 70 }, 0.0, 2.0, ray.WHITE);

        ray.DrawText("BACKGROUND SCROLLING & PARALLAX", 10, 10, 20, ray.RED);
        const text = "(c) Cyberpunk Street Environment by Luis Zuno (@ansimuz)";
        ray.DrawText(text, screenWidth - 330, screenHeight - 20, 10, ray.RAYWHITE);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

图片：<https://github.com/raysan5/raylib/blob/master/examples/textures/resources/>

## 效果

![背景滚动][1]

## 总结

实现背景滚动的效果。

[1]: images/raylib-texture-background.png

## 附录
