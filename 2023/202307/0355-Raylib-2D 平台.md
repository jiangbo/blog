# 0355-Raylib-2D 平台

## 环境

- Time 2024-02-19
- Zig 0.12.0-dev.2790+fc7dd3e28
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

实现了 2D 平台游戏，并且可以切换不同的 2D 相机模式。

## raylib.zig

```zig
pub usingnamespace @cImport({
    @cInclude("raylib.h");
    @cInclude("raymath.h");
    @cInclude("rlgl.h");
});
```

## update

```zig
fn update(camera: *ray.Camera2D) void {
    const deltaTime = ray.GetFrameTime();

    updatePlayer(deltaTime);

    camera.zoom += ray.GetMouseWheelMove() * 0.05;

    if (camera.zoom > 3.0) camera.zoom = 3.0 else if (camera.zoom < 0.25) camera.zoom = 0.25;

    if (ray.IsKeyPressed(ray.KEY_R)) {
        camera.zoom = 1.0;
        player.position = .{ .x = 400, .y = 280 };
    }

    if (ray.IsKeyPressed(ray.KEY_C)) cameraOption = (cameraOption + 1) % updaters.len;

    // Call update camera function by its pointer
    updaters[cameraOption](camera, deltaTime, screenWidth, screenHeight);
}
```

## draw

```zig
fn draw(camera: ray.Camera2D) void {
    ray.ClearBackground(ray.LIGHTGRAY);

    ray.BeginMode2D(camera);

    for (0..envItems.len) |i|
        ray.DrawRectangleRec(envItems[i].rect, envItems[i].color);

    const playerRect = .{ .x = player.position.x - 20, .y = player.position.y - 40, .width = 40, .height = 40 };
    ray.DrawRectangleRec(playerRect, ray.RED);

    const x: c_int = @intFromFloat(player.position.x);
    const y: c_int = @intFromFloat(player.position.y);
    ray.DrawCircle(x, y, 5, ray.GOLD);

    ray.EndMode2D();

    ray.DrawText("Controls:", 20, 20, 10, ray.BLACK);
    ray.DrawText("- Right/Left to move", 40, 40, 10, ray.DARKGRAY);
    ray.DrawText("- Space to jump", 40, 60, 10, ray.DARKGRAY);
    ray.DrawText("- Mouse Wheel to Zoom in-out, R to reset zoom", 40, 80, 10, ray.DARKGRAY);
    ray.DrawText("- C to change camera mode", 40, 100, 10, ray.DARKGRAY);
    ray.DrawText("Current camera mode:", 20, 120, 10, ray.BLACK);
    ray.DrawText(cameraDescriptions[cameraOption], 40, 140, 10, ray.DARKGRAY);
}
```

## main

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const screenWidth = 800;
const screenHeight = 450;

const G = 400;
const PLAYER_JUMP_SPD = 350.0;
const PLAYER_HOR_SPD = 200.0;

const Player = struct {
    position: ray.Vector2,
    speed: f32 = 0,
    canJump: bool = false,
};

const EnvItem = struct {
    rect: ray.Rectangle,
    blocking: bool = true,
    color: ray.Color = ray.GRAY,
};

var player = Player{ .position = .{ .x = 400, .y = 280 } };
const cameraDescriptions = [_][*c]const u8{
    "Follow player center",
    "Follow player center, but clamp to map edges",
    "Follow player center; smoothed",
    "Follow player center horizontally; update player center vertically after landing",
    "Player push camera on getting too close to screen edge",
};

// Store pointers to the multiple update camera functions
const updaters = [_]*const fn (*ray.Camera2D, f32, f32, f32) void{
    UpdateCameraCenter,
    UpdateCameraCenterInsideMap,
    UpdateCameraCenterSmoothFollow,
    UpdateCameraEvenOutOnLanding,
    UpdateCameraPlayerBoundsPush,
};

var cameraOption: usize = 0;

const envItems = [_]EnvItem{
    .{ .rect = .{ .x = 0, .y = 0, .width = 1000, .height = 400 }, .blocking = false, .color = ray.LIGHTGRAY },
    .{ .rect = .{ .x = 0, .y = 400, .width = 1000, .height = 200 } },
    .{ .rect = .{ .x = 300, .y = 200, .width = 400, .height = 10 } },
    .{ .rect = .{ .x = 250, .y = 300, .width = 100, .height = 10 } },
    .{ .rect = .{ .x = 650, .y = 300, .width = 100, .height = 10 } },
};

pub fn main() void {
    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    // 2D 相机
    var camera: ray.Camera2D = .{
        .target = player.position,
        .offset = .{ .x = screenWidth / 2.0, .y = screenHeight / 2.0 },
        .zoom = 1.0,
    };

    while (!ray.WindowShouldClose()) {
        update(&camera);

        ray.BeginDrawing();
        defer ray.EndDrawing();
        draw(camera);
    }
}
```

## 不同视角的切换

```zig
fn updatePlayer(delta: f32) void {
    if (ray.IsKeyDown(ray.KEY_LEFT)) player.position.x -= PLAYER_HOR_SPD * delta;
    if (ray.IsKeyDown(ray.KEY_RIGHT)) player.position.x += PLAYER_HOR_SPD * delta;
    if (ray.IsKeyDown(ray.KEY_SPACE) and player.canJump) {
        player.speed = -PLAYER_JUMP_SPD;
        player.canJump = false;
    }

    var hitObstacle = false;
    for (envItems) |ei| {
        const p = &(player.position);
        if (ei.blocking and
            ei.rect.x <= p.x and
            ei.rect.x + ei.rect.width >= p.x and
            ei.rect.y >= p.y and
            ei.rect.y <= p.y + player.speed * delta)
        {
            hitObstacle = true;
            player.speed = 0.0;
            p.y = ei.rect.y;
            break;
        }
    }

    if (!hitObstacle) {
        player.position.y += player.speed * delta;
        player.speed += G * delta;
        player.canJump = false;
    } else player.canJump = true;
}

fn UpdateCameraCenter(camera: *ray.Camera2D, _: f32, width: f32, height: f32) void {
    camera.offset = .{ .x = width / 2.0, .y = height / 2.0 };
    camera.target = player.position;
}

fn UpdateCameraCenterInsideMap(camera: *ray.Camera2D, _: f32, width: f32, height: f32) void {
    camera.target = player.position;
    camera.offset = .{ .x = width / 2.0, .y = height / 2.0 };
    var minX: f32 = 1000;
    var minY: f32 = 1000;
    var maxX: f32 = -1000;
    var maxY: f32 = -1000;

    for (envItems) |ei| {
        minX = ray.fminf(ei.rect.x, minX);
        maxX = ray.fmaxf(ei.rect.x + ei.rect.width, maxX);
        minY = ray.fminf(ei.rect.y, minY);
        maxY = ray.fmaxf(ei.rect.y + ei.rect.height, maxY);
    }

    const max = ray.GetWorldToScreen2D(.{ .x = maxX, .y = maxY }, camera.*);
    const min = ray.GetWorldToScreen2D(.{ .x = minX, .y = minY }, camera.*);

    if (max.x < width) camera.offset.x = width - (max.x - width / 2);
    if (max.y < height) camera.offset.y = height - (max.y - height / 2);
    if (min.x > 0) camera.offset.x = width / 2 - min.x;
    if (min.y > 0) camera.offset.y = height / 2 - min.y;
}

var minSpeed: f32 = 30;
var minEffectLength: f32 = 10;
var fractionSpeed: f32 = 0.8;

fn UpdateCameraCenterSmoothFollow(camera: *ray.Camera2D, delta: f32, width: f32, height: f32) void {
    camera.offset = .{ .x = width / 2.0, .y = height / 2.0 };
    const diff = ray.Vector2Subtract(player.position, camera.target);
    const length = ray.Vector2Length(diff);

    if (length > minEffectLength) {
        const speed = ray.fmaxf(fractionSpeed * length, minSpeed);
        camera.target = ray.Vector2Add(camera.target, ray.Vector2Scale(diff, speed * delta / length));
    }
}

var evenOutSpeed: f32 = 700;
var eveningOut: bool = false;
var evenOutTarget: f32 = 0.0;

fn UpdateCameraEvenOutOnLanding(camera: *ray.Camera2D, delta: f32, width: f32, height: f32) void {
    camera.offset = .{ .x = width / 2.0, .y = height / 2.0 };
    camera.target.x = player.position.x;

    if (eveningOut) {
        if (evenOutTarget > camera.target.y) {
            camera.target.y += evenOutSpeed * delta;

            if (camera.target.y > evenOutTarget) {
                camera.target.y = evenOutTarget;
                eveningOut = false;
            }
        } else {
            camera.target.y -= evenOutSpeed * delta;

            if (camera.target.y < evenOutTarget) {
                camera.target.y = evenOutTarget;
                eveningOut = false;
            }
        }
    } else {
        if (player.canJump and (player.speed == 0) and (player.position.y != camera.target.y)) {
            eveningOut = true;
            evenOutTarget = player.position.y;
        }
    }
}

var bbox = .{ .x = 0.2, .y = 0.2 };
fn UpdateCameraPlayerBoundsPush(camera: *ray.Camera2D, _: f32, width: f32, height: f32) void {
    const bboxWorldMin = ray.GetScreenToWorld2D(.{ .x = (1 - bbox.x) * 0.5 * width, .y = (1 - bbox.y) * 0.5 * height }, camera.*);
    const bboxWorldMax = ray.GetScreenToWorld2D(.{ .x = (1 + bbox.x) * 0.5 * width, .y = (1 + bbox.y) * 0.5 * height }, camera.*);
    camera.offset = .{ .x = (1 - bbox.x) * 0.5 * width, .y = (1 - bbox.y) * 0.5 * height };

    if (player.position.x < bboxWorldMin.x) camera.target.x = player.position.x;
    if (player.position.y < bboxWorldMin.y) camera.target.y = player.position.y;
    if (player.position.x > bboxWorldMax.x) camera.target.x = bboxWorldMin.x + (player.position.x - bboxWorldMax.x);
    if (player.position.y > bboxWorldMax.y) camera.target.y = bboxWorldMin.y + (player.position.y - bboxWorldMax.y);
}
```

## 效果

![2D 平台][1]

## 总结

实现了一个 2D 平台游戏，并且可以切换不同的 2D 相机模型。

[1]: images/raylib-2d-platform.png

## 附录
