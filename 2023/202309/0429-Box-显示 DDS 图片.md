# 0429-Box-显示 DDS 图片

## 环境

- Time 2024-03-10
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- raylib 5.0

## 前言

### 说明

参考资料：

1. 《游戏开发：世嘉新人培训教材》
2. 图片素材：<https://www.ituring.com.cn/book/1742>

### 目标

解析 DDS 格式的图片并显示出来。

## dds.zig

```zig
const std = @import("std");
const file = @import("file.zig");
const ray = @import("raylib.zig");

pub const Image = struct {
    width: usize = 0,
    height: usize = 0,
    content: []const u8 = undefined,
    data: []const u8 = undefined,
    allocator: std.mem.Allocator = undefined,

    pub fn init(allocator: std.mem.Allocator, path: []const u8) ?Image {
        return doInit(allocator, path) catch |e| {
            std.log.err("init image error: {}", .{e});
            return null;
        };
    }

    fn doInit(allocator: std.mem.Allocator, path: []const u8) !?Image {
        std.log.info("load image: {s}", .{path});
        const content = try file.readAll(allocator, path);

        if (!std.mem.eql(u8, content[0..4], "DDS ")) {
            std.log.info("image format error", .{});
            return null;
        }

        const image = Image{
            .height = @as(usize, getU32(content[12..])),
            .width = @as(usize, getU32(content[16..])),
            .content = content,
            .data = content[128..],
            .allocator = allocator,
        };

        std.log.info("dds image width: {}, height: {}", .{ image.width, image.height });
        if (image.data.len != image.width * image.height * 4) {
            std.log.info("image size error: {}", .{image.data.len});
            return null;
        }

        return image;
    }

    fn getU32(c: []const u8) u32 {
        return std.mem.bytesToValue(u32, c[0..4]);
    }

    pub fn getRgbaU32(self: Image, index: usize) u32 {
        const c = self.data[index * 4 ..];
        return @as(u32, c[2]) << 24 | @as(u32, c[1]) << 16 //
        | @as(u32, c[0]) << 8 | @as(u32, c[3]) << 0;
    }

    pub fn deinit(self: Image) void {
        self.allocator.free(self.content);
    }
};
```

## main.zig

```zig
const std = @import("std");
const ray = @import("raylib.zig");
const map = @import("map.zig");
const dds = @import("dds.zig");

const screenWidth = 320;
const screenHeight = 240;

pub fn main() void {
    ray.InitWindow(screenWidth, screenHeight, "推箱子");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);
    ray.SetExitKey(ray.KEY_NULL);

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const image = dds.Image.init(allocator, "data/image/bar.dds") orelse return;
    defer image.deinit();

    while (!ray.WindowShouldClose()) {
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.WHITE);

        for (0..image.height) |y| {
            for (0..image.width) |x| {
                const color = ray.GetColor(image.getRgbaU32(x + y * image.width));
                ray.DrawPixel(@intCast(x), @intCast(y), color);
            }
        }
        ray.DrawFPS(screenWidth - 80, 10);
    }

    // 游戏胜利
    std.debug.print("Congratulation's! you win.\n", .{});
}
```

## Raylib 显示 DDS 图片

在 raylib 中，默认已经集成了 DDS 文件的解析和显示。

```zig
const std = @import("std");
const ray = @import("raylib.zig");
const map = @import("map.zig");

const screenWidth = 320;
const screenHeight = 240;

pub fn main() void {
    ray.InitWindow(screenWidth, screenHeight, "推箱子");
    defer ray.CloseWindow();
    ray.SetTargetFPS(60);
    ray.SetExitKey(ray.KEY_NULL);

    const texture = ray.LoadTexture("data/image/bar.dds");

    while (!ray.WindowShouldClose()) {
        ray.BeginDrawing();
        defer ray.EndDrawing();
        ray.ClearBackground(ray.WHITE);

        ray.DrawTexture(texture, 0, 0, ray.WHITE);

        ray.DrawFPS(screenWidth - 80, 10);
    }

    // 游戏胜利
    std.debug.print("Congratulation's! you win.\n", .{});
}
```

## 效果

![box5][1]

## 总结

分别使用了自解析和 Raylib 自带的方式显示了 DDS 格式的图片。

[1]: images/box5.png

## 附录
