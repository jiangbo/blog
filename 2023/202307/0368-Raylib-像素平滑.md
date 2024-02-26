# 0368-Raylib-像素平滑

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

像素平滑相机，好像可以解决像素失真或抖动的问题。不是很了解，之后如果用到，回过头来再看。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    const virtualScreenWidth: c_int = 160;
    const virtualScreenHeight: c_int = 90;
    const virtualRatio = screenWidth / virtualScreenWidth;

    var worldSpaceCamera = ray.Camera2D{ .zoom = 1.0 }; // Game world camera
    var screenSpaceCamera = ray.Camera2D{ .zoom = 1.0 }; // Smoothing camera

    const target = ray.LoadRenderTexture(virtualScreenWidth, virtualScreenHeight); // This is where we'll draw all our objects.

    const rec01 = ray.Rectangle{ .x = 70.0, .y = 35.0, .width = 20.0, .height = 20.0 };
    const rec02 = ray.Rectangle{ .x = 90.0, .y = 55.0, .width = 30.0, .height = 10.0 };
    const rec03 = ray.Rectangle{ .x = 80.0, .y = 65.0, .width = 15.0, .height = 25.0 };

    // The target's height is flipped (in the source Rectangle), due to OpenGL reasons
    const sourceRec = ray.Rectangle{
        .width = @floatFromInt(target.texture.width),
        .height = @floatFromInt(-target.texture.height),
    };
    const destRec = ray.Rectangle{
        .x = -virtualRatio,
        .y = -virtualRatio,
        .width = screenWidth + (virtualRatio * 2),
        .height = screenHeight + (virtualRatio * 2),
    };

    const origin = ray.Vector2{};
    var rotation: f32 = 0.0;
    var cameraX: f32 = 0.0;
    var cameraY: f32 = 0.0;

    while (!ray.WindowShouldClose()) {

        // Update
        rotation += 60.0 * ray.GetFrameTime(); // Rotate the rectangles, 60 degrees per second

        // Make the camera move to demonstrate the effect
        const time: f32 = @floatCast(ray.GetTime());
        cameraX = @sin(time) * 50.0 - 10.0;
        cameraY = @cos(time) * 30.0;

        // Set the camera's target to the values computed above
        screenSpaceCamera.target = .{ .x = cameraX, .y = cameraY };

        // Round worldSpace coordinates, keep decimals into screenSpace coordinates
        worldSpaceCamera.target.x = screenSpaceCamera.target.x;
        screenSpaceCamera.target.x -= worldSpaceCamera.target.x;
        screenSpaceCamera.target.x *= virtualRatio;

        worldSpaceCamera.target.y = screenSpaceCamera.target.y;
        screenSpaceCamera.target.y -= worldSpaceCamera.target.y;
        screenSpaceCamera.target.y *= virtualRatio;

        // Draw

        ray.BeginTextureMode(target);
        ray.ClearBackground(ray.RAYWHITE);

        ray.BeginMode2D(worldSpaceCamera);
        ray.DrawRectanglePro(rec01, origin, rotation, ray.BLACK);
        ray.DrawRectanglePro(rec02, origin, -rotation, ray.RED);
        ray.DrawRectanglePro(rec03, origin, rotation + 45.0, ray.BLUE);
        ray.EndMode2D();
        ray.EndTextureMode();

        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RED);

        ray.BeginMode2D(screenSpaceCamera);
        ray.DrawTexturePro(target.texture, sourceRec, destRec, origin, 0.0, ray.WHITE);
        ray.EndMode2D();

        ray.DrawText(ray.TextFormat("Screen resolution: %ix%i", screenWidth, screenHeight), 10, 10, 20, ray.DARKBLUE);
        ray.DrawText(ray.TextFormat("World resolution: %ix%i", virtualScreenWidth, virtualScreenHeight), 10, 40, 20, ray.DARKGREEN);
        ray.DrawFPS(ray.GetScreenWidth() - 95, 10);
    }
}
```

## 效果

![2D 像素平滑][1]

## 总结

像素平滑相机。

[1]: images/raylib-2d-smooth.png

## 附录
