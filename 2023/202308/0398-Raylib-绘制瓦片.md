# 0398-Raylib-绘制瓦片

## 环境

- Time 2024-03-05
- Zig 0.12.0-dev.3076+6e078883e
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

根据图片中的内容，绘制瓦片。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const OPT_WIDTH = 220; // Max width for the options container
const MARGIN_SIZE = 8; // Size for the margins
const COLOR_SIZE = 16; // Size of the color select buttons

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.SetConfigFlags(ray.FLAG_WINDOW_RESIZABLE); // Make the window resizable
    ray.InitWindow(screenWidth, screenHeight, "raylib [texture] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // NOTE: Textures MUST be loaded after Window initialization (OpenGL context is required)
    const texPattern = ray.LoadTexture("res/patterns.png");
    defer ray.UnloadTexture(texPattern);
    ray.SetTextureFilter(texPattern, ray.TEXTURE_FILTER_TRILINEAR); // Makes the texture smoother when upscaled

    // Coordinates for all patterns inside the texture
    const recPattern = [_]ray.Rectangle{
        .{ .x = 3, .y = 3, .width = 66, .height = 66 },
        .{ .x = 75, .y = 3, .width = 100, .height = 100 },
        .{ .x = 3, .y = 75, .width = 66, .height = 66 },
        .{ .x = 7, .y = 156, .width = 50, .height = 50 },
        .{ .x = 85, .y = 106, .width = 90, .height = 45 },
        .{ .x = 75, .y = 154, .width = 100, .height = 60 },
    };

    // Setup colors
    const colors = [_]ray.Color{
        ray.BLACK, ray.MAROON, ray.ORANGE, ray.BLUE,     ray.PURPLE, //
        ray.BEIGE, ray.LIME,   ray.RED,    ray.DARKGRAY, ray.SKYBLUE,
    };
    var colorRec: [colors.len]ray.Rectangle = undefined;

    // Calculate rectangle for each color
    var x: f32 = 0;
    var y: f32 = 0;
    for (0..colors.len) |i| {
        colorRec[i].x = 2.0 + MARGIN_SIZE + x;
        colorRec[i].y = 22.0 + 256.0 + MARGIN_SIZE + y;
        colorRec[i].width = COLOR_SIZE * 2.0;
        colorRec[i].height = COLOR_SIZE;

        if (i == (colors.len / 2 - 1)) {
            x = 0;
            y += COLOR_SIZE + MARGIN_SIZE;
        } else x += (COLOR_SIZE * 2 + MARGIN_SIZE);
    }

    var activePattern: usize = 0;
    var activeCol: usize = 0;
    var scale: f32 = 1.0;
    var rotation: f32 = 0.0;

    while (!ray.WindowShouldClose()) {

        // Update
        // Handle mouse
        if (ray.IsMouseButtonPressed(ray.MOUSE_BUTTON_LEFT)) {
            const mouse = ray.GetMousePosition();

            // Check which pattern was clicked and set it as the active pattern
            for (0..recPattern.len) |i| {
                if (ray.CheckCollisionPointRec(mouse, .{
                    .x = 2 + MARGIN_SIZE + recPattern[i].x,
                    .y = 40 + MARGIN_SIZE + recPattern[i].y,
                    .width = recPattern[i].width,
                    .height = recPattern[i].height,
                })) {
                    activePattern = i;
                    break;
                }
            }

            // Check to see which color was clicked and set it as the active color
            for (0..colors.len) |i| {
                if (ray.CheckCollisionPointRec(mouse, colorRec[i])) {
                    activeCol = i;
                    break;
                }
            }
        }

        // Handle keys

        // Change scale
        if (ray.IsKeyPressed(ray.KEY_UP)) scale += 0.25;
        if (ray.IsKeyPressed(ray.KEY_DOWN)) scale -= 0.25;
        if (scale > 10.0) scale = 10.0 else if (scale <= 0.0) scale = 0.25;

        // Change rotation
        if (ray.IsKeyPressed(ray.KEY_LEFT)) rotation -= 25.0;
        if (ray.IsKeyPressed(ray.KEY_RIGHT)) rotation += 25.0;

        // Reset
        if (ray.IsKeyPressed(ray.KEY_SPACE)) {
            rotation = 0.0;
            scale = 1.0;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        // Draw the tiled area
        drawTextureTiled(texPattern, recPattern[activePattern], .{
            .x = OPT_WIDTH + MARGIN_SIZE,
            .y = MARGIN_SIZE,
            .width = @as(f32, @floatFromInt(ray.GetScreenWidth())) - OPT_WIDTH - 2.0 * MARGIN_SIZE,
            .height = @as(f32, @floatFromInt(ray.GetScreenHeight())) - 2.0 * MARGIN_SIZE,
        }, .{}, rotation, scale, colors[activeCol]);

        // Draw options
        ray.DrawRectangle(MARGIN_SIZE, MARGIN_SIZE, OPT_WIDTH - MARGIN_SIZE, ray.GetScreenHeight() - 2 * MARGIN_SIZE, ray.ColorAlpha(ray.LIGHTGRAY, 0.5));

        ray.DrawText("Select Pattern", 2 + MARGIN_SIZE, 30 + MARGIN_SIZE, 10, ray.BLACK);
        ray.DrawTexture(texPattern, 2 + MARGIN_SIZE, 40 + MARGIN_SIZE, ray.BLACK);
        const x1: c_int = @intFromFloat(2 + MARGIN_SIZE + recPattern[activePattern].x);
        const y1: c_int = @intFromFloat(40 + MARGIN_SIZE + recPattern[activePattern].y);
        const width: c_int = @intFromFloat(recPattern[activePattern].width);
        const height: c_int = @intFromFloat(recPattern[activePattern].height);
        ray.DrawRectangle(x1, y1, width, height, ray.ColorAlpha(ray.DARKBLUE, 0.3));

        ray.DrawText("Select Color", 2 + MARGIN_SIZE, 10 + 256 + MARGIN_SIZE, 10, ray.BLACK);
        for (0..colors.len) |i| {
            ray.DrawRectangleRec(colorRec[i], colors[i]);
            if (activeCol == i) ray.DrawRectangleLinesEx(colorRec[i], 3, ray.ColorAlpha(ray.WHITE, 0.5));
        }

        ray.DrawText("Scale (UP/DOWN to change)", 2 + MARGIN_SIZE, 80 + 256 + MARGIN_SIZE, 10, ray.BLACK);
        ray.DrawText(ray.TextFormat("%.2fx", scale), 2 + MARGIN_SIZE, 92 + 256 + MARGIN_SIZE, 20, ray.BLACK);

        ray.DrawText("Rotation (LEFT/RIGHT to change)", 2 + MARGIN_SIZE, 122 + 256 + MARGIN_SIZE, 10, ray.BLACK);
        ray.DrawText(ray.TextFormat("%.0f degrees", rotation), 2 + MARGIN_SIZE, 134 + 256 + MARGIN_SIZE, 20, ray.BLACK);

        ray.DrawText("Press [SPACE] to reset", 2 + MARGIN_SIZE, 164 + 256 + MARGIN_SIZE, 10, ray.DARKBLUE);

        // Draw FPS
        ray.DrawText(ray.TextFormat("%i FPS", ray.GetFPS()), 2 + MARGIN_SIZE, 2 + MARGIN_SIZE, 20, ray.BLACK);

        // ray.DrawFPS(screenWidth - 100, 10);
    }
}

// Draw part of a texture (defined by a rectangle) with rotation and scale tiled into dest.
fn drawTextureTiled(texture: ray.Texture2D, source: ray.Rectangle, dest: ray.Rectangle, origin: ray.Vector2, rotation: f32, scale: f32, tint: ray.Color) void {
    if ((texture.id <= 0) or (scale <= 0.0)) return; // Wanna see a infinite loop?!...just delete this line!
    if ((source.width == 0) or (source.height == 0)) return;

    const tileWidth = (source.width * scale);
    const tileHeight = (source.height * scale);
    if ((dest.width < tileWidth) and (dest.height < tileHeight)) {
        // Can fit only one tile
        ray.DrawTexturePro(texture, .{
            .x = source.x,
            .y = source.y,
            .width = (dest.width / tileWidth) * source.width,
            .height = (dest.height / tileHeight) * source.height,
        }, .{
            .x = dest.x,
            .y = dest.y,
            .width = dest.width,
            .height = dest.height,
        }, origin, rotation, tint);
    } else if (dest.width <= tileWidth) {
        // Tiled vertically (one column)
        var dy: f32 = 0;
        while (dy + tileHeight < dest.height) : (dy += tileHeight) {
            ray.DrawTexturePro(texture, .{
                .x = source.x,
                .y = source.y,
                .width = (dest.width / tileWidth) * source.width,
                .height = source.height,
            }, .{
                .x = dest.x,
                .y = dest.y + dy,
                .width = dest.width,
                .height = tileHeight,
            }, origin, rotation, tint);
        }

        // Fit last tile
        if (dy < dest.height) {
            ray.DrawTexturePro(texture, .{
                .x = source.x,
                .y = source.y,
                .width = (dest.width / tileWidth) * source.width,
                .height = ((dest.height - dy) / tileHeight) * source.height,
            }, .{
                .x = dest.x,
                .y = dest.y + dy,
                .width = dest.width,
                .height = dest.height - dy,
            }, origin, rotation, tint);
        }
    } else if (dest.height <= tileHeight) {
        // Tiled horizontally (one row)
        var dx: f32 = 0;
        while (dx + tileWidth < dest.width) : (dx += tileWidth) {
            ray.DrawTexturePro(texture, .{
                .x = source.x,
                .y = source.y,
                .width = source.width,
                .height = (dest.height / tileHeight) * source.height,
            }, .{
                .x = dest.x + dx,
                .y = dest.y,
                .width = tileWidth,
                .height = dest.height,
            }, origin, rotation, tint);
        }

        // Fit last tile
        if (dx < dest.width) {
            ray.DrawTexturePro(texture, .{
                .x = source.x,
                .y = source.y,
                .width = ((dest.width - dx) / tileWidth) * source.width,
                .height = (dest.height / tileHeight) * source.height,
            }, .{
                .x = dest.x + dx,
                .y = dest.y,
                .width = dest.width - dx,
                .height = dest.height,
            }, origin, rotation, tint);
        }
    } else {
        // Tiled both horizontally and vertically (rows and columns)
        var dx: f32 = 0;
        while (dx + tileWidth < dest.width) : (dx += tileWidth) {
            var dy: f32 = 0;
            while (dy + tileHeight < dest.height) : (dy += tileHeight) {
                ray.DrawTexturePro(texture, source, .{
                    .x = dest.x + dx,
                    .y = dest.y + dy,
                    .width = tileWidth,
                    .height = tileHeight,
                }, origin, rotation, tint);
            }

            if (dy < dest.height) {
                ray.DrawTexturePro(texture, .{
                    .x = source.x,
                    .y = source.y,
                    .width = source.width,
                    .height = ((dest.height - dy) / tileHeight) * source.height,
                }, .{
                    .x = dest.x + dx,
                    .y = dest.y + dy,
                    .width = tileWidth,
                    .height = dest.height - dy,
                }, origin, rotation, tint);
            }
        }

        // Fit last column of tiles
        if (dx < dest.width) {
            var dy: f32 = 0;
            while (dy + tileHeight < dest.height) : (dy += tileHeight) {
                ray.DrawTexturePro(texture, .{
                    .x = source.x,
                    .y = source.y,
                    .width = ((dest.width - dx) / tileWidth) * source.width,
                    .height = source.height,
                }, .{
                    .x = dest.x + dx,
                    .y = dest.y + dy,
                    .width = dest.width - dx,
                    .height = tileHeight,
                }, origin, rotation, tint);
            }

            // Draw final tile in the bottom right corner
            if (dy < dest.height) {
                ray.DrawTexturePro(texture, .{
                    .x = source.x,
                    .y = source.y,
                    .width = ((dest.width - dx) / tileWidth) * source.width,
                    .height = ((dest.height - dy) / tileHeight) * source.height,
                }, .{
                    .x = dest.x + dx,
                    .y = dest.y + dy,
                    .width = dest.width - dx,
                    .height = dest.height - dy,
                }, origin, rotation, tint);
            }
        }
    }
}
```

## 效果

![绘制瓦片][1]

## 总结

根据图片中的内容，绘制瓦片。

[1]: images/raylib-texture-tiled.png

## 附录
