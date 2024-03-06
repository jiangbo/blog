# 0409-Raylib-文字自动换行

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

如果一个英语单词显示不全，则自动换到下一行。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

var font: ray.Font = undefined;

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [text] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    const text = "Text cannot escape\tthis container\t...word wrap also works" ++
        "when active so here's a long text for testing.\n\nLorem ipsum " ++
        "dolor sit amet, consectetur adipiscing elit, sed do eiusmod" ++
        "tempor incididunt ut labore et dolore magna aliqua. " ++
        "Nec ullamcorper sit amet risus nullam eget felis eget.";

    var resizing = false;
    var wordWrap = true;

    const screenWidthF: f32 = @floatFromInt(screenWidth);
    const screenHeightF: f32 = @floatFromInt(screenHeight);
    var container = ray.Rectangle{
        .x = 25.0,
        .y = 25.0,
        .width = screenWidthF - 50.0,
        .height = screenHeightF - 250.0,
    };
    var resizer = ray.Rectangle{
        .x = container.x + container.width - 17,
        .y = container.y + container.height - 17,
        .width = 14,
        .height = 14,
    };

    // Minimum width and heigh for the container rectangle
    const minWidth = 60;
    const minHeight = 60;
    const maxWidth: f32 = screenWidthF - 50.0;
    const maxHeight: f32 = screenHeightF - 160.0;

    var lastMouse = ray.Vector2{}; // Stores last mouse coordinates
    var borderColor = ray.MAROON; // Container border color
    font = ray.GetFontDefault(); // Get default system font
    defer ray.UnloadFont(font);

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsKeyPressed(ray.KEY_SPACE)) wordWrap = !wordWrap;

        const mouse = ray.GetMousePosition();

        // Check if the mouse is inside the container and toggle border color
        if (ray.CheckCollisionPointRec(mouse, container))
            borderColor = ray.Fade(ray.MAROON, 0.4)
        else if (!resizing) borderColor = ray.MAROON;

        // Container resizing logic
        if (resizing) {
            if (ray.IsMouseButtonReleased(ray.MOUSE_BUTTON_LEFT)) resizing = false;

            const width = container.width + (mouse.x - lastMouse.x);
            container.width = if (width > minWidth) (if (width < maxWidth) width else maxWidth) else minWidth;

            const height = container.height + (mouse.y - lastMouse.y);
            container.height = if (height > minHeight) (if (height < maxHeight) height else maxHeight) else minHeight;
        } else {
            // Check if we're resizing
            if (ray.IsMouseButtonDown(ray.MOUSE_BUTTON_LEFT) and ray.CheckCollisionPointRec(mouse, resizer)) resizing = true;
        }

        // Move resizer rectangle properly
        resizer.x = container.x + container.width - 17;
        resizer.y = container.y + container.height - 17;

        lastMouse = mouse; // Update mouse

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawRectangleLinesEx(container, 3, borderColor); // Draw container border

        // Draw text in container (add some padding)
        drawTextBoxed(text, .{
            .x = container.x + 4,
            .y = container.y + 4,
            .width = container.width - 4,
            .height = container.height - 4,
        }, 20.0, 2.0, wordWrap, ray.GRAY);

        ray.DrawRectangleRec(resizer, borderColor); // Draw the resize box

        // Draw bottom info
        ray.DrawRectangle(0, screenHeight - 54, screenWidth, 54, ray.GRAY);
        ray.DrawRectangleRec(.{
            .x = 382.0,
            .y = screenHeightF - 34.0,
            .width = 12.0,
            .height = 12.0,
        }, ray.MAROON);

        ray.DrawText("Word Wrap: ", 313, screenHeight - 115, 20, ray.BLACK);
        if (wordWrap) ray.DrawText("ON", 447, screenHeight - 115, 20, ray.RED) else ray.DrawText("OFF", 447, screenHeight - 115, 20, ray.BLACK);

        ray.DrawText("Press [SPACE] to toggle word wrap", 218, screenHeight - 86, 20, ray.GRAY);

        ray.DrawText("Click hold & drag the    to resize the container", 155, screenHeight - 38, 20, ray.RAYWHITE);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}

fn drawTextBoxed(text: [*c]const u8, rec: ray.Rectangle, fontSize: f32, spacing: f32, wordWrap: bool, tint: ray.Color) void {
    drawTextBoxedSelectable(text, rec, fontSize, spacing, wordWrap, tint, 0, 0, ray.WHITE, ray.WHITE);
}

// Draw text using font inside rectangle limits with support for text selection
fn drawTextBoxedSelectable(text: [*c]const u8, rec: ray.Rectangle, fontSize: f32, spacing: f32, wordWrap: bool, tint: ray.Color, selectStart: c_int, selectLength: c_int, selectTint: ray.Color, selectBackTint: ray.Color) void {
    const length = ray.TextLength(text); // Total length in bytes of the text, scanned by codepoints in loop

    var start: c_int = selectStart;
    var textOffsetY: f32 = 0; // Offset between lines (on line break '\n')
    var textOffsetX: f32 = 0.0; // Offset X to next character to draw

    const baseSize: f32 = @floatFromInt(font.baseSize);
    const scaleFactor: f32 = fontSize / baseSize; // Character rectangle scaling factor
    // Word/character wrapping mechanism variables
    var state: bool = !wordWrap;

    var startLine: c_int = -1; // Index where to begin drawing (where a line begins)
    var endLine: c_int = -1; // Index where to stop drawing (where a line ends)
    var lastk: c_int = -1; // Holds last value of the character position

    var i: c_int = 0;
    var k: c_int = 0;
    while (i < length) : (k += 1) {
        // for (0..length, 0..length) |i, k| {
        // Get next codepoint from byte string and glyph index in font
        var codepointByteCount: c_int = 0;
        const codepoint = ray.GetCodepoint(&text[@intCast(i)], &codepointByteCount);
        const index: usize = @intCast(ray.GetGlyphIndex(font, codepoint));

        // NOTE: Normally we exit the decoding sequence as soon as a bad byte is found (and return 0x3f)
        // but we need to draw all of the bad bytes using the '?' symbol moving one byte
        if (codepoint == 0x3f) codepointByteCount = 1;
        i += codepointByteCount - 1;

        var glyphWidth: f32 = 0;
        if (codepoint != '\n') {
            glyphWidth = if (font.glyphs[index].advanceX == 0)
                font.recs[index].width * scaleFactor
            else
                @as(f32, @floatFromInt(font.glyphs[index].advanceX)) * scaleFactor;

            if (i + 1 < length) glyphWidth = glyphWidth + spacing;
        }

        // NOTE: When wordWrap is ON we first measure how much of the text we can draw before going outside of the rec container
        // We store this info in startLine and endLine, then we change states, draw the text between those two variables
        // and change states again and again recursively until the end of the text (or until we get outside of the container).
        // When wordWrap is OFF we don't need the measure state so we go to the drawing state immediately
        // and begin drawing on the next line before we can get outside the container.
        if (!state) {
            // TODO: There are multiple types of spaces in UNICODE, maybe it's a good idea to add support for more
            // Ref: http://jkorpela.fi/chars/spaces.html
            if ((codepoint == ' ') or (codepoint == '\t') or (codepoint == '\n'))
                endLine = i;

            if ((textOffsetX + glyphWidth) > rec.width) {
                endLine = if (endLine < 1) i else endLine;
                if (i == endLine) endLine -= codepointByteCount;
                if ((startLine + codepointByteCount) == endLine)
                    endLine = i - codepointByteCount;

                state = !state;
            } else if ((i + 1) == length) {
                endLine = i;
                state = !state;
            } else if (codepoint == '\n') state = !state;

            if (state) {
                textOffsetX = 0;
                i = startLine;
                glyphWidth = 0;

                // Save character position when we switch states
                const tmp = lastk;
                lastk = k - 1;
                k = tmp;
            }
        } else {
            if (codepoint == '\n') {
                if (!wordWrap) {
                    textOffsetY += (baseSize + baseSize / 2) * scaleFactor;
                    textOffsetX = 0;
                }
            } else {
                if (!wordWrap and ((textOffsetX + glyphWidth) > rec.width)) {
                    textOffsetY += (baseSize + baseSize / 2) * scaleFactor;
                    textOffsetX = 0;
                }

                // When text overflows rectangle height limit, just stop drawing
                if ((textOffsetY + baseSize * scaleFactor) > rec.height) break;

                // Draw selection background
                var isGlyphSelected = false;
                if ((selectStart >= 0) and (k >= selectStart) and (k < (selectStart + selectLength))) {
                    ray.DrawRectangleRec(.{
                        .x = rec.x + textOffsetX - 1,
                        .y = rec.y + textOffsetY,
                        .width = glyphWidth,
                        .height = baseSize * scaleFactor,
                    }, selectBackTint);
                    isGlyphSelected = true;
                }

                // Draw current character glyph
                if ((codepoint != ' ') and (codepoint != '\t')) {
                    ray.DrawTextCodepoint(font, codepoint, .{
                        .x = rec.x + textOffsetX,
                        .y = rec.y + textOffsetY,
                    }, fontSize, if (isGlyphSelected) selectTint else tint);
                }
            }

            if (wordWrap and (i == endLine)) {
                textOffsetY += (baseSize + baseSize / 2) * scaleFactor;
                textOffsetX = 0;
                startLine = endLine;
                endLine = -1;
                glyphWidth = 0;
                start += lastk - k;
                k = lastk;

                state = !state;
            }
        }

        if ((textOffsetX != 0) or (codepoint != ' ')) textOffsetX += glyphWidth; // avoid leading spaces

        i += 1;
    }
}
```

## 效果

![自动换行][1]

## 总结

如果一个英语单词显示不全，则自动换到下一行。

[1]: images/raylib-text-wrap.png

## 附录
