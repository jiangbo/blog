# 0410-Raylib-显示非英文字符

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

显示非英文字符，并且将重复的字符去除。

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

    const text = "いろはにほへと　ちりぬるを\nわかよたれそ　つねならむ\nうゐのおくやま　けふこえて\nあさきゆめみし　ゑひもせす";
    // Get codepoints from text
    var count: c_int = 0;
    const loadedCodepoints = ray.LoadCodepoints(text, &count);
    const codepoints: []c_int = loadedCodepoints[0..@intCast(count)];

    // Removed duplicate codepoints to generate smaller font atlas
    const allocator = std.heap.c_allocator;
    const codes = try removeDuplicates(allocator, codepoints);
    allocator.free(codes);
    ray.UnloadCodepoints(loadedCodepoints);

    // Load font containing all the provided codepoint glyphs
    // A texture font atlas is automatically generated
    const font = ray.LoadFontEx("res/DotGothic16-Regular.ttf", 48, loadedCodepoints, count);

    // Set bilinear scale filter for better font scaling
    ray.SetTextureFilter(font.texture, ray.TEXTURE_FILTER_BILINEAR);

    ray.SetTextLineSpacing(54); // Set line spacing for multiline text (when line breaks are included '\n')

    // Free codepoints, atlas has already been generated
    var showFontAtlas = false;

    var codepointSize: c_int = 0;
    var ptr = @intFromPtr(text);

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsKeyPressed(ray.KEY_SPACE)) showFontAtlas = !showFontAtlas;

        // Testing code: getting next and previous codepoints on provided text
        if (ray.IsKeyPressed(ray.KEY_RIGHT)) {
            // Get next codepoint in string and move pointer
            _ = ray.GetCodepointNext(ptr, &codepointSize);
            ptr += @intCast(codepointSize);
        } else if (ray.IsKeyPressed(ray.KEY_LEFT)) {
            // Get previous codepoint in string and move pointer
            _ = ray.GetCodepointPrevious(ptr, &codepointSize);
            ptr -= @intCast(codepointSize);
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawRectangle(0, 0, ray.GetScreenWidth(), 70, ray.BLACK);
        ray.DrawText(ray.TextFormat("Total codepoints contained in provided text: %i", count), 10, 10, 20, ray.GREEN);
        ray.DrawText(ray.TextFormat("Total codepoints required for font atlas (duplicates excluded): %i", codes.len), 10, 40, 20, ray.GREEN);

        if (showFontAtlas) {
            // Draw generated font texture atlas containing provided codepoints
            ray.DrawTexture(font.texture, 150, 100, ray.BLACK);
            ray.DrawRectangleLines(150, 100, font.texture.width, font.texture.height, ray.BLACK);
        } else {
            // Draw provided text with laoded font, containing all required codepoint glyphs
            ray.DrawTextEx(font, text, .{ .x = 160, .y = 110 }, 48, 5, ray.BLACK);
        }

        ray.DrawText("Press SPACE to toggle font atlas view!", 10, ray.GetScreenHeight() - 30, 20, ray.GRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}

// Remove codepoint duplicates if requested
// WARNING: This process could be a bit slow if there text to process is very long
fn removeDuplicates(allocator: std.mem.Allocator, codepoints: []c_int) ![]c_int {
    var codepointsNoDupsCount = codepoints.len;
    var codepointsNoDups = try allocator.alloc(c_int, codepoints.len);
    @memcpy(codepointsNoDups, codepoints);

    // Remove duplicates
    for (0..codepointsNoDupsCount) |i| {
        var j = i + 1;
        while (j < codepointsNoDupsCount) : (j += 1) {
            if (codepointsNoDups[i] == codepointsNoDups[j]) {
                for (j..codepointsNoDupsCount - 1) |k|
                    codepointsNoDups[k] = codepointsNoDups[k + 1];
                codepointsNoDupsCount -= 1;
                j -= 1;
            }
        }
    }

    // NOTE: The size of codepointsNoDups is the same as original array but
    // only required positions are filled (codepointsNoDupsCount)

    return codepointsNoDups[0..codepointsNoDupsCount];
}
```

## 效果

![非英文字符][1]

## 总结

显示非英文字符，并且将重复的字符去除。

[1]: images/raylib-text-codepoints.png

## 附录
