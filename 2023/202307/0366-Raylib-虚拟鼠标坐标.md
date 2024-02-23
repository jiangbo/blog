# 0366-Raylib-虚拟鼠标坐标

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

除了实际的坐标，可以新建虚拟的坐标。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() void {
    const screenWidth = 800;
    const screenHeight = 450;

    ray.SetConfigFlags(ray.FLAG_WINDOW_RESIZABLE | ray.FLAG_VSYNC_HINT);
    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();

    ray.SetWindowMinSize(320, 240);
    ray.SetTargetFPS(60);

    const gameScreenWidth: f32 = 640;
    const gameScreenHeight: f32 = 480;

    // Render texture initialization, used to hold the rendering result so we can easily resize it
    const target = ray.LoadRenderTexture(gameScreenWidth, gameScreenHeight);
    defer ray.UnloadRenderTexture(target);
    ray.SetTextureFilter(target.texture, ray.TEXTURE_FILTER_BILINEAR); // Texture scale filter to use

    var colors: [10]ray.Color = undefined;
    for (0..colors.len) |index| {
        colors[index] = .{
            .r = @intCast(ray.GetRandomValue(100, 250)),
            .g = @intCast(ray.GetRandomValue(50, 150)),
            .b = @intCast(ray.GetRandomValue(10, 100)),
            .a = 255,
        };
    }

    while (!ray.WindowShouldClose()) {

        // Update
        // Compute required framebuffer scaling
        const width: f32 = @floatFromInt(ray.GetScreenWidth());
        const height: f32 = @floatFromInt(ray.GetScreenHeight());
        const scale: f32 = @min(width / gameScreenWidth, height / gameScreenHeight);

        if (ray.IsKeyPressed(ray.KEY_SPACE)) {
            // Recalculate random colors for the bars
            for (0..colors.len) |index| {
                colors[index] = .{
                    .r = @intCast(ray.GetRandomValue(100, 250)),
                    .g = @intCast(ray.GetRandomValue(50, 150)),
                    .b = @intCast(ray.GetRandomValue(10, 100)),
                    .a = 255,
                };
            }
        }

        // Update virtual mouse (clamped mouse value behind game screen)
        const mouse = ray.GetMousePosition();
        var virtualMouse = ray.Vector2{};
        virtualMouse.x = (mouse.x - (width - (gameScreenWidth * scale)) * 0.5) / scale;
        virtualMouse.y = (mouse.y - (height - (gameScreenHeight * scale)) * 0.5) / scale;
        virtualMouse = ray.Vector2Clamp(virtualMouse, .{}, .{ .x = gameScreenWidth, .y = gameScreenHeight });

        // Apply the same transformation as the virtual mouse to the real mouse (i.e. to work with raygui)
        //SetMouseOffset(-(GetScreenWidth() - (gameScreenWidth*scale))*0.5f, -(GetScreenHeight() - (gameScreenHeight*scale))*0.5f);
        //SetMouseScale(1/scale, 1/scale);
        // Draw

        // Draw everything in the render texture, note this will not be rendered on screen, yet
        ray.BeginTextureMode(target);
        ray.ClearBackground(ray.RAYWHITE); // Clear render texture background color

        for (colors, 0..colors.len) |color, i| {
            const y = @as(c_int, @intFromFloat(gameScreenHeight / 10)) * @as(c_int, @intCast(i));
            ray.DrawRectangle(0, y, gameScreenWidth, gameScreenHeight / 10, color);
        }

        ray.DrawText("If executed inside a window,\nyou can resize the window,\nand see the screen scaling!", 10, 25, 20, ray.WHITE);
        const mouseX: c_int = @intFromFloat(mouse.x);
        const mouseY: c_int = @intFromFloat(mouse.y);
        ray.DrawText(ray.TextFormat("Default Mouse: [%i , %i]", mouseX, mouseY), 350, 25, 20, ray.GREEN);
        const virtualMouseX: c_int = @intFromFloat(virtualMouse.x);
        const virtualMouseY: c_int = @intFromFloat(virtualMouse.y);
        ray.DrawText(ray.TextFormat("Virtual Mouse: [%i , %i]", virtualMouseX, virtualMouseY), 350, 55, 20, ray.YELLOW);
        ray.EndTextureMode();

        ray.BeginDrawing();
        defer ray.EndDrawing();

        ray.ClearBackground(ray.BLACK);
        // Draw render texture to screen, properly scaled
        ray.DrawTexturePro(target.texture, .{
            .width = @floatFromInt(target.texture.width),
            .height = @floatFromInt(-target.texture.height),
        }, .{
            .x = (width - (gameScreenWidth * scale)) * 0.5,
            .y = (height - (gameScreenHeight * scale)) * 0.5,
            .width = gameScreenWidth * scale,
            .height = gameScreenHeight * scale,
        }, .{}, 0.0, ray.WHITE);
    }
}
```

## 效果

![2D 虚拟坐标][1]

## 总结

实际坐标和虚拟坐标，两套坐标。

[1]: images/raylib-2d-virtual.png

## 附录
