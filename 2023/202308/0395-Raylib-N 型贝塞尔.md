# 0395-Raylib-N 型贝塞尔

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

N-Patches的中文翻译为“N型贝塞尔曲面”或“N型贝塞尔片”。
在图形学领域，N-Patches指的是一种用于曲面建模的技术，它可以用来替代三角形网格，以实现更加真实和逼真的曲面效果。
具体的概念信息不清楚，之后如果涉及到这方面的知识，再补充。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [texture] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    const nPatchTexture = ray.LoadTexture("res/ninepatch_button.png");
    defer ray.UnloadTexture(nPatchTexture);

    var mousePosition = ray.Vector2{};
    const origin = ray.Vector2{};

    // Position and size of the n-patches
    var dstRec1 = ray.Rectangle{ .x = 480.0, .y = 160.0, .width = 32.0, .height = 32.0 };
    var dstRec2 = ray.Rectangle{ .x = 160.0, .y = 160.0, .width = 32.0, .height = 32.0 };
    var dstRecH = ray.Rectangle{ .x = 160.0, .y = 93.0, .width = 32.0, .height = 32.0 };
    var dstRecV = ray.Rectangle{ .x = 92.0, .y = 160.0, .width = 32.0, .height = 32.0 };

    // A 9-patch (NPATCH_NINE_PATCH) changes its sizes in both axis
    const ninePatchInfo1 = ray.NPatchInfo{
        .source = .{ .width = 64.0, .height = 64.0 },
        .left = 12,
        .top = 40,
        .right = 12,
        .bottom = 12,
        .layout = ray.NPATCH_NINE_PATCH,
    };
    const ninePatchInfo2 = ray.NPatchInfo{
        .source = .{ .y = 128.0, .width = 64.0, .height = 64.0 },
        .left = 16,
        .top = 16,
        .right = 16,
        .bottom = 16,
        .layout = ray.NPATCH_NINE_PATCH,
    };

    // A horizontal 3-patch (NPATCH_THREE_PATCH_HORIZONTAL) changes its sizes along the x axis only
    const h3PatchInfo = ray.NPatchInfo{
        .source = .{ .y = 64.0, .width = 64.0, .height = 64.0 },
        .left = 8,
        .top = 8,
        .right = 8,
        .bottom = 8,
        .layout = ray.NPATCH_THREE_PATCH_HORIZONTAL,
    };

    // A vertical 3-patch (NPATCH_THREE_PATCH_VERTICAL) changes its sizes along the y axis only
    const v3PatchInfo = ray.NPatchInfo{
        .source = .{ .y = 192.0, .width = 64.0, .height = 64.0 },
        .left = 6,
        .top = 6,
        .right = 6,
        .bottom = 6,
        .layout = ray.NPATCH_THREE_PATCH_VERTICAL,
    };

    while (!ray.WindowShouldClose()) {

        // Update
        mousePosition = ray.GetMousePosition();

        // Resize the n-patches based on mouse position
        dstRec1.width = mousePosition.x - dstRec1.x;
        dstRec1.height = mousePosition.y - dstRec1.y;
        dstRec2.width = mousePosition.x - dstRec2.x;
        dstRec2.height = mousePosition.y - dstRec2.y;
        dstRecH.width = mousePosition.x - dstRecH.x;
        dstRecV.height = mousePosition.y - dstRecV.y;

        // Set a minimum width and/or height
        if (dstRec1.width < 1.0) dstRec1.width = 1.0;
        if (dstRec1.width > 300.0) dstRec1.width = 300.0;
        if (dstRec1.height < 1.0) dstRec1.height = 1.0;
        if (dstRec2.width < 1.0) dstRec2.width = 1.0;
        if (dstRec2.width > 300.0) dstRec2.width = 300.0;
        if (dstRec2.height < 1.0) dstRec2.height = 1.0;
        if (dstRecH.width < 1.0) dstRecH.width = 1.0;
        if (dstRecV.height < 1.0) dstRecV.height = 1.0;

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        // Draw the n-patches
        ray.DrawTextureNPatch(nPatchTexture, ninePatchInfo2, dstRec2, origin, 0.0, ray.WHITE);
        ray.DrawTextureNPatch(nPatchTexture, ninePatchInfo1, dstRec1, origin, 0.0, ray.WHITE);
        ray.DrawTextureNPatch(nPatchTexture, h3PatchInfo, dstRecH, origin, 0.0, ray.WHITE);
        ray.DrawTextureNPatch(nPatchTexture, v3PatchInfo, dstRecV, origin, 0.0, ray.WHITE);

        // Draw the source texture
        ray.DrawRectangleLines(5, 88, 74, 266, ray.BLUE);
        ray.DrawTexture(nPatchTexture, 10, 93, ray.WHITE);
        ray.DrawText("TEXTURE", 15, 360, 10, ray.DARKGRAY);

        ray.DrawText("Move the mouse to stretch or shrink the n-patches", 10, 20, 20, ray.DARKGRAY);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![N-Patches][1]

## 总结

N型贝塞尔曲面。

[1]: images/raylib-texture-npatches.png

## 附录
