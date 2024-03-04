# 0396-Raylib-批量渲染

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

如果出现了很多的精灵，可以使用批量渲染。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MAX_BUNNIES = 50000; // 50K bunnies limit
// This is the maximum amount of elements (quads) per batch
// NOTE: This value is defined in [rlgl] module and can be changed there
const MAX_BATCH_ELEMENTS = 8192;

const Bunny = struct {
    position: ray.Vector2,
    speed: ray.Vector2,
    color: ray.Color,
};

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [texture] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // Load bunny texture
    const texBunny = ray.LoadTexture("res/wabbit_alpha.png");
    defer ray.UnloadTexture(texBunny);

    const allocator = std.heap.c_allocator;
    var bunnies = try allocator.alloc(Bunny, MAX_BUNNIES);
    defer allocator.free(bunnies);
    var bunniesCount: usize = 0; // Bunnies counter

    while (!ray.WindowShouldClose()) {

        // Update
        if (ray.IsMouseButtonDown(ray.MOUSE_BUTTON_LEFT)) {
            // Create more bunnies
            for (0..100) |_| {
                if (bunniesCount < MAX_BUNNIES) {
                    bunnies[bunniesCount].position = ray.GetMousePosition();
                    bunnies[bunniesCount].speed.x = @as(f32, @floatFromInt(ray.GetRandomValue(-250, 250))) / 60.0;
                    bunnies[bunniesCount].speed.y = @as(f32, @floatFromInt(ray.GetRandomValue(-250, 250))) / 60.0;
                    bunnies[bunniesCount].color = .{
                        .r = @intCast(ray.GetRandomValue(50, 240)),
                        .g = @intCast(ray.GetRandomValue(80, 240)),
                        .b = @intCast(ray.GetRandomValue(100, 240)),
                        .a = 255,
                    };
                    bunniesCount += 1;
                }
            }
        }

        // Update bunnies
        for (0..bunniesCount) |i| {
            bunnies[i].position.x += bunnies[i].speed.x;
            bunnies[i].position.y += bunnies[i].speed.y;

            const x: c_int = @intFromFloat(bunnies[i].position.x);
            const y: c_int = @intFromFloat(bunnies[i].position.y);
            const width = x + @divTrunc(texBunny.width, 2);
            const height = y + @divTrunc(texBunny.height, 2);
            if (width > ray.GetScreenWidth() or width < 0)
                bunnies[i].speed.x *= -1;
            if (height > ray.GetScreenHeight() or (height - 60) < 0)
                bunnies[i].speed.y *= -1;
        }

        // Draw
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        for (0..bunniesCount) |i| {
            // NOTE: When internal batch buffer limit is reached (MAX_BATCH_ELEMENTS),
            // a draw call is launched and buffer starts being filled again;
            // before issuing a draw call, updated vertex data from internal CPU buffer is send to GPU...
            // Process of sending data is costly and it could happen that GPU data has not been completely
            // processed for drawing while new data is tried to be sent (updating current in-use buffers)
            // it could generates a stall and consequently a frame drop, limiting the number of drawn bunnies
            const x: c_int = @intFromFloat(bunnies[i].position.x);
            const y: c_int = @intFromFloat(bunnies[i].position.y);
            ray.DrawTexture(texBunny, x, y, bunnies[i].color);
        }

        ray.DrawRectangle(0, 0, screenWidth, 40, ray.BLACK);
        ray.DrawText(ray.TextFormat("bunnies: %i", bunniesCount), 120, 10, 20, ray.GREEN);
        const text = ray.TextFormat("batched draw calls: %i", 1 + bunniesCount / MAX_BATCH_ELEMENTS);
        ray.DrawText(text, 320, 10, 20, ray.MAROON);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![批量渲染][1]

## 总结

批量渲染精灵。

[1]: images/raylib-texture-batch.png

## 附录
