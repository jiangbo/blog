# 0399-Raylib-战争迷雾

## 环境

- Time 2024-03-05
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- Raylib 5.0

## 前言

### 说明

参考资料：

1. <https://www.raylib.com/examples.html>

### 目标

模拟战争迷雾的功能。

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");

const MAP_TILE_SIZE = 32; // Tiles size 32x32 pixels
const PLAYER_SIZE = 16; // Player size
const PLAYER_TILE_VISIBILITY = 2; // Player can see 2 tiles around its position

// Map data type
const Map = struct {
    tilesX: usize, // Number of tiles in X axis
    tilesY: usize, // Number of tiles in Y axis
    tileIds: []bool = undefined, // Tile ids (tilesX*tilesY), defines type of tile to draw
    tileFog: []u8 = undefined, // Tile fog state (tilesX*tilesY), defines if a tile has fog or half-fog

    fn x(self: Map) c_int {
        return @intCast(self.tilesX);
    }

    fn y(self: Map) c_int {
        return @intCast(self.tilesY);
    }
};

const Vector2 = struct {
    x: usize,
    y: usize,

    fn toRayVector(self: Vector2) ray.Vector2 {
        return ray.Vector2{ .x = @floatFromInt(self.x), .y = @floatFromInt(self.y) };
    }
};

pub fn main() !void {
    const screenWidth: c_int = 800;
    const screenHeight: c_int = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [texture] example");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);

    var map = Map{ .tilesX = 25, .tilesY = 15 };

    // NOTE: We can have up to 256 values for tile ids and for tile fog state,
    // probably we don't need that many values for fog state, it can be optimized
    // to use only 2 bits per fog state (reducing size by 4) but logic will be a bit more complex

    const allocator = std.heap.c_allocator;
    const length: usize = map.tilesX * map.tilesY;
    map.tileIds = try allocator.alloc(bool, length);
    defer allocator.free(map.tileIds);
    map.tileFog = try allocator.alloc(u8, length);
    defer allocator.free(map.tileFog);
    @memset(map.tileFog, 0);

    // Load map tiles (generating 2 random tile ids for testing)
    // NOTE: Map tile ids should be probably loaded from an external map file
    for (map.tileIds) |*id| id.* = ray.GetRandomValue(0, 1) == 0;

    // Player position on the screen (pixel coordinates, not tile coordinates)
    var playerPosition = Vector2{ .x = 180, .y = 130 };
    var playerTileX: usize = 0;
    var playerTileY: usize = 0;

    // Render texture to render fog of war
    // NOTE: To get an automatic smooth-fog effect we use a render texture to render fog
    // at a smaller size (one pixel per tile) and scale it on drawing with bilinear filtering
    const fogOfWar = ray.LoadRenderTexture(map.x(), map.y());
    defer ray.UnloadRenderTexture(fogOfWar);
    ray.SetTextureFilter(fogOfWar.texture, ray.TEXTURE_FILTER_BILINEAR);

    while (!ray.WindowShouldClose()) {

        // Update
        // Move player around
        if (ray.IsKeyDown(ray.KEY_RIGHT)) playerPosition.x +|= 5;
        if (ray.IsKeyDown(ray.KEY_LEFT)) playerPosition.x -|= 5;
        if (ray.IsKeyDown(ray.KEY_DOWN)) playerPosition.y +|= 5;
        if (ray.IsKeyDown(ray.KEY_UP)) playerPosition.y -|= 5;

        // Check player position to avoid moving outside tilemap limits
        if ((playerPosition.x + PLAYER_SIZE) > (map.tilesX * MAP_TILE_SIZE))
            playerPosition.x = map.tilesX * MAP_TILE_SIZE - PLAYER_SIZE;

        if ((playerPosition.y + PLAYER_SIZE) > (map.tilesY * MAP_TILE_SIZE))
            playerPosition.y = map.tilesY * MAP_TILE_SIZE - PLAYER_SIZE;

        // Previous visited tiles are set to partial fog
        for (map.tileFog) |*fog| {
            if (fog.* == 1) fog.* = 2;
        }

        // Get current tile position from player pixel position
        playerTileX = ((playerPosition.x + MAP_TILE_SIZE / 2) / MAP_TILE_SIZE);
        playerTileY = ((playerPosition.y + MAP_TILE_SIZE / 2) / MAP_TILE_SIZE);

        // Check visibility and update fog
        // NOTE: We check tilemap limits to avoid processing tiles out-of-array-bounds (it could crash program)
        for ((playerTileY -| PLAYER_TILE_VISIBILITY)..(playerTileY + PLAYER_TILE_VISIBILITY)) |y| {
            for ((playerTileX -| PLAYER_TILE_VISIBILITY)..(playerTileX + PLAYER_TILE_VISIBILITY)) |x| {
                if ((x >= 0) and (x < map.tilesX) and (y >= 0) and (y < map.tilesY))
                    map.tileFog[y * map.tilesX + x] = 1;
            }
        }

        // Draw
        // Draw fog of war to a small render texture for automatic smoothing on scaling
        ray.BeginTextureMode(fogOfWar);
        ray.ClearBackground(ray.BLANK);
        for (map.tileFog, 0..) |fog, index| {
            const x: c_int = @intCast(index % map.tilesX);
            const y: c_int = @intCast(index / map.tilesX);
            if (fog == 0)
                ray.DrawRectangle(x, y, 1, 1, ray.BLACK)
            else if (fog == 2)
                ray.DrawRectangle(x, y, 1, 1, ray.Fade(ray.BLACK, 0.8));
        }
        ray.EndTextureMode();

        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.RAYWHITE);

        for (map.tileIds, 0..) |id, index| {
            // Draw tiles from id (and tile borders)
            const x: c_int = @intCast(index % map.tilesX * MAP_TILE_SIZE);
            const y: c_int = @intCast(index / map.tilesX * MAP_TILE_SIZE);
            const color = if (id) ray.BLUE else ray.Fade(ray.BLUE, 0.9);
            ray.DrawRectangle(x, y, MAP_TILE_SIZE, MAP_TILE_SIZE, color);
            ray.DrawRectangleLines(x, y, MAP_TILE_SIZE, MAP_TILE_SIZE, ray.Fade(ray.DARKBLUE, 0.5));
        }

        // Draw player
        ray.DrawRectangleV(playerPosition.toRayVector(), .{ .x = PLAYER_SIZE, .y = PLAYER_SIZE }, ray.RED);

        // Draw fog of war (scaled to full map, bilinear filtering)
        ray.DrawTexturePro(fogOfWar.texture, .{
            .width = @floatFromInt(fogOfWar.texture.width),
            .height = @floatFromInt(-fogOfWar.texture.height),
        }, .{
            .width = @floatFromInt(map.tilesX * MAP_TILE_SIZE),
            .height = @floatFromInt(map.tilesY * MAP_TILE_SIZE),
        }, .{}, 0.0, ray.WHITE);

        // Draw player current tile
        ray.DrawText(ray.TextFormat("Current tile: [%i,%i]", playerTileX, playerTileY), 10, 10, 20, ray.RAYWHITE);
        ray.DrawText("ARROW KEYS to move", 10, screenHeight - 25, 20, ray.RAYWHITE);

        ray.DrawFPS(screenWidth - 100, 10);
    }
}
```

## 效果

![战争迷雾][1]

## 总结

模拟战争迷雾的功能。

[1]: images/raylib-texture-fog.png

## 附录
