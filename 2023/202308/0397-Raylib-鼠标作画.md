# 0397-Raylib-鼠标作画

## 环境

- Time 2024-03-04
- Zig 0.12.0-dev.3076+6e078883e
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

渲染一个画板，可以使用鼠标进行作画，并且将画好的图片保存下来。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [textures] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // Colors to choose from
    const colors = [_]ray.Color{
        ray.RAYWHITE,  ray.YELLOW,    ray.GOLD,   ray.ORANGE,     ray.PINK,    ray.RED,
        ray.MAROON,    ray.GREEN,     ray.LIME,   ray.DARKGREEN,  ray.SKYBLUE, ray.BLUE,
        ray.DARKBLUE,  ray.PURPLE,    ray.VIOLET, ray.DARKPURPLE, ray.BEIGE,   ray.BROWN,
        ray.DARKBROWN, ray.LIGHTGRAY, ray.GRAY,   ray.DARKGRAY,   ray.BLACK,
    };

    // Define colorsRecs data (for every rectangle)
    const colorsRecs = label: {
        var recs: [colors.len]ray.Rectangle = undefined;
        for (0..colors.len) |i| {
            const index: f32 = @floatFromInt(i);
            recs[i].x = 10 + 30.0 * index + 2 * index;
            recs[i].y = 10;
            recs[i].width = 30;
            recs[i].height = 30;
        }
        break :label recs;
    };

    var colorSelected: usize = 0;
    var colorSelectedPrev = colorSelected;
    var colorMouseHover: c_int = 0;
    var brushSize: f32 = 20.0;
    var mouseWasPressed = false;

    const btnSaveRec = ray.Rectangle{ .x = 750, .y = 10, .width = 40, .height = 30 };
    var btnSaveMouseHover = false;
    var showSaveMessage = false;
    var saveMessageCounter: usize = 0;

    // Create a RenderTexture2D to use as a canvas
    const target = ray.LoadRenderTexture(screenWidth, screenHeight);
    defer ray.UnloadRenderTexture(target);

    // Clear render texture before entering the game loop
    ray.BeginTextureMode(target);
    ray.ClearBackground(colors[0]);
    ray.EndTextureMode();
    while (!ray.WindowShouldClose()) {

        // Update
        const mousePos = ray.GetMousePosition();

        // Move between colors with keys
        if (ray.IsKeyPressed(ray.KEY_RIGHT))
            colorSelected += 1
        else if (ray.IsKeyPressed(ray.KEY_LEFT)) colorSelected -= 1;

        if (colorSelected >= colors.len)
            colorSelected = colors.len - 1
        else if (colorSelected < 0) colorSelected = 0;

        // Choose color with mouse
        for (0..colors.len) |i| {
            if (ray.CheckCollisionPointRec(mousePos, colorsRecs[i])) {
                colorMouseHover = @intCast(i);
                break;
            } else colorMouseHover = -1;
        }

        if ((colorMouseHover >= 0) and ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_LEFT)) {
            colorSelected = @intCast(colorMouseHover);
            colorSelectedPrev = colorSelected;
        }

        // Change brush size
        brushSize += ray.GetMouseWheelMove() * 5;
        if (brushSize < 2) brushSize = 2;
        if (brushSize > 50) brushSize = 50;

        if (ray.IsKeyPressed(ray.KEY_C)) {
            // Clear render texture to clear color
            ray.BeginTextureMode(target);
            ray.ClearBackground(colors[0]);
            ray.EndTextureMode();
        }

        const x: c_int = @intFromFloat(mousePos.x);
        const y: c_int = @intFromFloat(mousePos.y);
        if (ray.IsMouseButtonDown(ray.MOUSE_BUTTON_LEFT) or (ray.GetGestureDetected() == ray.GESTURE_DRAG)) {
            // Paint circle into render texture
            // NOTE: To avoid discontinuous circles, we could store
            // previous-next mouse points and just draw a line using brush size
            ray.BeginTextureMode(target);
            if (mousePos.y > 50) ray.DrawCircle(x, y, brushSize, colors[colorSelected]);
            ray.EndTextureMode();
        }

        if (ray.IsMouseButtonDown(ray.MOUSE_BUTTON_RIGHT)) {
            if (!mouseWasPressed) {
                colorSelectedPrev = colorSelected;
                colorSelected = 0;
            }

            mouseWasPressed = true;

            // Erase circle from render texture
            ray.BeginTextureMode(target);
            if (mousePos.y > 50) ray.DrawCircle(x, y, brushSize, colors[0]);
            ray.EndTextureMode();
        } else if (ray.IsMouseButtonReleased(ray.MOUSE_BUTTON_RIGHT) and mouseWasPressed) {
            colorSelected = colorSelectedPrev;
            mouseWasPressed = false;
        }

        // Check mouse hover save button
        btnSaveMouseHover = ray.CheckCollisionPointRec(mousePos, btnSaveRec);

        // Image saving logic
        // NOTE: Saving painted texture to a default named image
        if ((btnSaveMouseHover and ray.IsMouseButtonReleased(ray.MOUSE_BUTTON_LEFT)) or ray.IsKeyPressed(ray.KEY_S)) {
            var image = ray.LoadImageFromTexture(target.texture);
            ray.ImageFlipVertical(&image);
            _ = ray.ExportImage(image, "my_amazing_texture_painting.png");
            ray.UnloadImage(image);
            showSaveMessage = true;
        }

        if (showSaveMessage) {
            // On saving, show a full screen message for 2 seconds
            saveMessageCounter += 1;
            if (saveMessageCounter > 240) {
                showSaveMessage = false;
                saveMessageCounter = 0;
            }
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
        ray.DrawTextureRec(target.texture, .{
            .width = @floatFromInt(target.texture.width),
            .height = @floatFromInt(-target.texture.height),
        }, .{}, ray.WHITE);

        // Draw drawing circle for reference
        if (mousePos.y > 50) {
            if (ray.IsMouseButtonDown(ray.MOUSE_BUTTON_RIGHT))
                ray.DrawCircleLines(x, y, brushSize, ray.GRAY)
            else
                ray.DrawCircle(ray.GetMouseX(), ray.GetMouseY(), brushSize, colors[colorSelected]);
        }

        // Draw top panel
        ray.DrawRectangle(0, 0, ray.GetScreenWidth(), 50, ray.RAYWHITE);
        ray.DrawLine(0, 50, ray.GetScreenWidth(), 50, ray.LIGHTGRAY);

        // Draw color selection rectangles
        for (0..colors.len) |i| ray.DrawRectangleRec(colorsRecs[i], colors[i]);
        ray.DrawRectangleLines(10, 10, 30, 30, ray.LIGHTGRAY);

        if (colorMouseHover >= 0)
            ray.DrawRectangleRec(colorsRecs[@intCast(colorMouseHover)], ray.Fade(ray.WHITE, 0.6));

        ray.DrawRectangleLinesEx(.{
            .x = colorsRecs[colorSelected].x - 2,
            .y = colorsRecs[colorSelected].y - 2,
            .width = colorsRecs[colorSelected].width + 4,
            .height = colorsRecs[colorSelected].height + 4,
        }, 2, ray.BLACK);

        // Draw save image button
        ray.DrawRectangleLinesEx(btnSaveRec, 2, if (btnSaveMouseHover) ray.RED else ray.BLACK);
        ray.DrawText("SAVE!", 755, 20, 10, if (btnSaveMouseHover) ray.RED else ray.BLACK);

        // Draw save image message
        if (showSaveMessage) {
            ray.DrawRectangle(0, 0, ray.GetScreenWidth(), ray.GetScreenHeight(), ray.Fade(ray.RAYWHITE, 0.8));
            ray.DrawRectangle(0, 150, ray.GetScreenWidth(), 80, ray.BLACK);
            ray.DrawText("IMAGE SAVED:  my_amazing_texture_painting.png", 150, 180, 20, ray.RAYWHITE);
        }

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![鼠标作画][1]

## 总结

渲染一个画板，可以使用鼠标进行作画，并且将画好的图片保存下来。

[1]: images/raylib-texture-painting.png

## 附录
