# 0385-Raylib-渲染多边形

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

渲染多边形。

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

    // Define texture coordinates to map our texture to poly
    const texcoords = [_]ray.Vector2{
        .{ .x = 0.75 },
        .{ .x = 0.25 },
        .{ .y = 0.5 },
        .{ .y = 0.75 },
        .{ .x = 0.25, .y = 1.0 },
        .{ .x = 0.375, .y = 0.875 },
        .{ .x = 0.625, .y = 0.875 },
        .{ .x = 0.75, .y = 1.0 },
        .{ .x = 1.0, .y = 0.75 },
        .{ .x = 1.0, .y = 0.5 },
        .{ .x = 0.75 },
    };

    // Define the base poly vertices from the UV's
    // NOTE: They can be specified in any other way
    var points: [texcoords.len]ray.Vector2 = undefined;
    for (texcoords, 0..) |coord, i| {
        points[i].x = (coord.x - 0.5) * 256.0;
        points[i].y = (coord.y - 0.5) * 256.0;
    }

    // Define the vertices drawing position
    // NOTE: Initially same as points but updated every frame
    var positions: [texcoords.len]ray.Vector2 = undefined;
    for (0..texcoords.len) |i| positions[i] = points[i];

    // Load texture to be mapped to poly
    const texture = ray.LoadTexture("res/cat.png");
    var angle: f32 = 0.0; // Rotation angle (in degrees)

    while (!ray.WindowShouldClose()) {

        // Update
        // Update points rotation with an angle transform
        // NOTE: Base points position are not modified
        angle += 1;
        for (0..texcoords.len) |i|
            positions[i] = ray.Vector2Rotate(points[i], angle * ray.DEG2RAD);

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("textured polygon", 20, 20, 20, ray.DARKGRAY);

        drawTexturePoly(texture, .{
            .x = @as(f32, @floatFromInt(ray.GetScreenWidth())) / 2.0,
            .y = @as(f32, @floatFromInt(ray.GetScreenHeight())) / 2.0,
        }, &positions, &texcoords, ray.WHITE);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}

// Draw textured polygon, defined by vertex and texture coordinates
// NOTE: Polygon center must have straight line path to all points
// without crossing perimeter, points must be in anticlockwise order
fn drawTexturePoly(texture: ray.Texture2D, center: ray.Vector2, points: []ray.Vector2, texcoords: []const ray.Vector2, tint: ray.Color) void {
    ray.rlSetTexture(texture.id);

    // Texturing is only supported on RL_QUADS
    ray.rlBegin(ray.RL_QUADS);

    ray.rlColor4ub(tint.r, tint.g, tint.b, tint.a);

    for (0..texcoords.len - 1) |i| {
        ray.rlTexCoord2f(0.5, 0.5);
        ray.rlVertex2f(center.x, center.y);

        ray.rlTexCoord2f(texcoords[i].x, texcoords[i].y);
        ray.rlVertex2f(points[i].x + center.x, points[i].y + center.y);

        ray.rlTexCoord2f(texcoords[i + 1].x, texcoords[i + 1].y);
        ray.rlVertex2f(points[i + 1].x + center.x, points[i + 1].y + center.y);

        ray.rlTexCoord2f(texcoords[i + 1].x, texcoords[i + 1].y);
        ray.rlVertex2f(points[i + 1].x + center.x, points[i + 1].y + center.y);
    }
    ray.rlEnd();

    ray.rlSetTexture(0);
}
```

图片：<https://github.com/raysan5/raylib/blob/master/examples/textures/resources/>

## 效果

![渲染多边形][1]

## 总结

渲染多边形。

[1]: images/raylib-texture-polygon.png

## 附录
