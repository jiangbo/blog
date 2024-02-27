# 0372-Raylib-调色板

## 环境

- Time 2024-02-27
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

渲染一个调色板。

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

    const colors = [_]ray.Color{
        ray.DARKGRAY, ray.MAROON,     ray.ORANGE,    ray.DARKGREEN, //
        ray.DARKBLUE, ray.DARKPURPLE, ray.DARKBROWN, ray.GRAY,
        ray.RED,      ray.GOLD,       ray.LIME,      ray.BLUE,
        ray.VIOLET,   ray.BROWN,      ray.LIGHTGRAY, ray.PINK,
        ray.YELLOW,   ray.GREEN,      ray.SKYBLUE,   ray.PURPLE,
        ray.BEIGE,
    };

    const colorNames = [_][*c]const u8{
        "DARKGRAY",   "MAROON",    "ORANGE", "DARKGREEN", "DARKBLUE", //
        "DARKPURPLE", "DARKBROWN", "GRAY",   "RED",       "GOLD",
        "LIME",       "BLUE",      "VIOLET", "BROWN",     "LIGHTGRAY",
        "PINK",       "YELLOW",    "GREEN",  "SKYBLUE",   "PURPLE",
        "BEIGE",
    };

    var colorsRecs: [colors.len]ray.Rectangle = undefined; // Rectangles array

    // Fills colorsRecs data (for every rectangle)
    for (0..colors.len) |i| {
        colorsRecs[i].x = 20.0 + 110.0 * @as(f32, @floatFromInt(@mod(i, 7)));
        colorsRecs[i].y = 80.0 + 110.0 * @as(f32, @floatFromInt(i / 7));
        colorsRecs[i].width = 100.0;
        colorsRecs[i].height = 100.0;
    }

    var colorState: [colors.len]bool = std.mem.zeroes([colors.len]bool);

    var mousePoint = ray.Vector2{};

    while (!ray.WindowShouldClose()) {

        // Update
        mousePoint = ray.GetMousePosition();

        for (&colorState, colorsRecs) |*state, recs| {
            state.* = ray.CheckCollisionPointRec(mousePoint, recs);
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("raylib colors palette", 28, 42, 20, ray.BLACK);
        ray.DrawText("press SPACE to see all colors", ray.GetScreenWidth() - 180, ray.GetScreenHeight() - 40, 10, ray.GRAY);

        for (0..colors.len) |i| // Draw all rectangles
        {
            const hover: f32 = if (colorState[i]) 0.6 else 1.0;
            ray.DrawRectangleRec(colorsRecs[i], ray.Fade(colors[i], hover));

            if (ray.IsKeyDown(ray.KEY_SPACE) or colorState[i]) {
                const x: c_int = @intFromFloat(colorsRecs[i].x);
                const y: c_int = @intFromFloat(colorsRecs[i].y);
                const height: c_int = @intFromFloat(colorsRecs[i].height);
                const width: c_int = @intFromFloat(colorsRecs[i].width);
                ray.DrawRectangle(x, y + height - 26, width, 20, ray.BLACK);
                ray.DrawRectangleLinesEx(colorsRecs[i], 6, ray.Fade(ray.BLACK, 0.3));
                const textX = x + width - ray.MeasureText(colorNames[i], 10) - 12;
                ray.DrawText(colorNames[i], textX, y + height - 20, 10, colors[i]);
            }
        }
    }
}
```

## 效果

![调色板][1]

## 总结

实现一个调色板。

[1]: images/raylib-shapes-palette.png

## 附录
