# 0353-Raylib-2D 相机

## 环境

- Time 2024-02-18
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

2D 相机，可以实现人物始终居中，并且进行旋转和缩放。

## update

```zig
fn update(camera: *ray.Camera2D) void {

    // 左右移动
    if (ray.IsKeyDown(ray.KEY_RIGHT)) player.x += 2 else if (ray.IsKeyDown(ray.KEY_LEFT)) player.x -= 2;

    // 相机跟随，人物始终居于正中间
    camera.target = .{ .x = player.x + 20, .y = player.y + 20 };

    // 旋转控制
    if (ray.IsKeyDown(ray.KEY_A)) camera.rotation -= 1 else if (ray.IsKeyDown(ray.KEY_S)) camera.rotation += 1;

    // Limit camera rotation to 80 degrees (-40 to 40)
    if (camera.rotation > 40) camera.rotation = 40 else if (camera.rotation < -40) camera.rotation = -40;

    // 缩放控制
    camera.zoom += (ray.GetMouseWheelMove() * 0.05);

    if (camera.zoom > 3.0) camera.zoom = 3.0 else if (camera.zoom < 0.1) camera.zoom = 0.1;

    // 重置
    if (ray.IsKeyPressed(ray.KEY_R)) {
        camera.zoom = 1.0;
        camera.rotation = 0.0;
    }
}
```

## draw

```zig
fn draw(camera: *ray.Camera2D) void {
    ray.ClearBackground(ray.RAYWHITE);

    // 开启2D
    ray.BeginMode2D(camera.*);

    // 背景
    ray.DrawRectangle(-6000, 320, 13000, 8000, ray.DARKGRAY);
    for (0..MAX_BUILDINGS) |index|
        ray.DrawRectangleRec(buildings[index], buildColors[index]);

    // 角色
    ray.DrawRectangleRec(player, ray.RED);
    // 水平和垂直线
    const targetX: c_int = @intFromFloat(camera.target.x);
    const targetY: c_int = @intFromFloat(camera.target.y);
    ray.DrawLine(targetX, -screenHeight * 10, targetX, screenHeight * 10, ray.GREEN);
    ray.DrawLine(-screenWidth * 10, targetY, screenWidth * 10, targetY, ray.GREEN);
    // 结束2D
    ray.EndMode2D();

    // 提示信息
    ray.DrawText("SCREEN AREA", 640, 10, 20, ray.RED);

    ray.DrawRectangle(0, 0, screenWidth, 5, ray.RED);
    ray.DrawRectangle(0, 5, 5, screenHeight - 10, ray.RED);
    ray.DrawRectangle(screenWidth - 5, 5, 5, screenHeight - 10, ray.RED);
    ray.DrawRectangle(0, screenHeight - 5, screenWidth, 5, ray.RED);

    // 增加透明度
    ray.DrawRectangle(10, 10, 250, 113, ray.Fade(ray.SKYBLUE, 0.5));
    ray.DrawRectangleLines(10, 10, 250, 113, ray.BLUE);

    ray.DrawText("Free 2d camera controls:", 20, 20, 10, ray.BLACK);
    ray.DrawText("- Right/Left to move Offset", 40, 40, 10, ray.DARKGRAY);
    ray.DrawText("- Mouse Wheel to Zoom in-out", 40, 60, 10, ray.DARKGRAY);
    ray.DrawText("- A / S to Rotate", 40, 80, 10, ray.DARKGRAY);
    ray.DrawText("- R to reset Zoom and Rotation", 40, 100, 10, ray.DARKGRAY);
}
```

## main

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const screenWidth = 800;
const screenHeight = 450;

const MAX_BUILDINGS = 100;

// 定义角色和背景
var player: ray.Rectangle = .{ .x = 400, .y = 280, .width = 40, .height = 40 };
var buildings: [MAX_BUILDINGS]ray.Rectangle = undefined;
var buildColors: [MAX_BUILDINGS]ray.Color = undefined;

pub fn main() void {
    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    //  初始化背景
    var spacing: f32 = 0;
    for (0..MAX_BUILDINGS) |index| {
        //  获得随机数
        buildings[index].width = @floatFromInt(ray.GetRandomValue(50, 200));
        buildings[index].height = @floatFromInt(ray.GetRandomValue(100, 800));
        buildings[index].y = screenHeight - 130.0 - buildings[index].height;
        buildings[index].x = -6000.0 + spacing;

        spacing += buildings[index].width;

        buildColors[index] = ray.Color{
            .r = @intCast(ray.GetRandomValue(200, 240)),
            .g = @intCast(ray.GetRandomValue(200, 240)),
            .b = @intCast(ray.GetRandomValue(200, 250)),
            .a = 255,
        };
    }

    // 2D 相机
    var camera: ray.Camera2D = .{
        .target = .{ .x = player.x + 20.0, .y = player.y + 20.0 },
        .offset = .{ .x = screenWidth / 2.0, .y = screenHeight / 2.0 },
        .rotation = 0.0,
        .zoom = 1.0,
    };

    while (!ray.WindowShouldClose()) {
        update(&camera);

        ray.BeginDrawing();
        defer ray.EndDrawing();
        draw(&camera);
    }
}
```

## 效果

![2D 相机][1]

## 总结

使用了 raylib 中的 2D 相机功能，实现了居中，旋转，缩放功能。

[1]: images/raylib-2d.png

## 附录
