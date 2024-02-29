# 0384-Raylib-混合模式

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

实现不同的混合模式。

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

    const background = ray.LoadTexture("res/cyberpunk_street_background.png");
    defer ray.UnloadTexture(background);
    const foreground = ray.LoadTexture("res/cyberpunk_street_foreground.png");
    defer ray.UnloadTexture(foreground);

    const blendCountMax = 4;
    var blendMode: ray.BlendMode = 0;

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsKeyPressed(ray.KEY_SPACE)) {
            if (blendMode >= (blendCountMax - 1)) blendMode = 0 else blendMode += 1;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        var x = @divTrunc(screenWidth, 2) - @divTrunc(background.width, 2);
        var y = @divTrunc(screenHeight, 2) - @divTrunc(background.height, 2);
        ray.DrawTexture(background, x, y, ray.WHITE);

        // Apply the blend mode and then draw the foreground texture
        ray.BeginBlendMode(@intCast(blendMode));
        x = @divTrunc(screenWidth, 2) - @divTrunc(foreground.width, 2);
        y = @divTrunc(screenHeight, 2) - @divTrunc(foreground.height, 2);
        ray.DrawTexture(foreground, x, y, ray.WHITE);
        ray.EndBlendMode();

        // Draw the texts
        ray.DrawText("Press SPACE to change blend modes.", 310, 350, 10, ray.GRAY);

        switch (blendMode) {
            ray.BLEND_ALPHA => ray.DrawText("Current: BLEND_ALPHA", (screenWidth / 2) - 60, 370, 10, ray.GRAY),
            ray.BLEND_ADDITIVE => ray.DrawText("Current: BLEND_ADDITIVE", (screenWidth / 2) - 60, 370, 10, ray.GRAY),
            ray.BLEND_MULTIPLIED => ray.DrawText("Current: BLEND_MULTIPLIED", (screenWidth / 2) - 60, 370, 10, ray.GRAY),
            ray.BLEND_ADD_COLORS => ray.DrawText("Current: BLEND_ADD_COLORS", (screenWidth / 2) - 60, 370, 10, ray.GRAY),
            else => {},
        }
        const text = "(c) Cyberpunk Street Environment by Luis Zuno (@ansimuz)";
        ray.DrawText(text, screenWidth - 330, screenHeight - 20, 10, ray.GRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

图片：<https://github.com/raysan5/raylib/blob/master/examples/textures/resources/>

## 效果

![混合模式][1]

## 总结

实现不同的混合模式。

[1]: images/raylib-texture-blend.png

## 附录
