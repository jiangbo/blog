# 0355-Box-引入 raylib

## 环境

- Time 2024-02-05
- Zig 0.12.0-dev.2543+9eda6ccef
- WSL-Ubuntu 22.04.3 LTS
- raylib 5.0

## 前言

### 说明

参考资料：

1. 《游戏开发：世嘉新人培训教材》
2. <https://github.com/raysan5/raylib/blob/master/examples/core/core_basic_window.c>

### 目标

在之前控制台推箱子的基础上，引入 raylib 库，可以进行图形界面的显示。

## 引入 raylib 依赖

build.zig.zon 文件中增加 raylib 的依赖。

```zig
.{
    .name = "demo",
    .version = "0.0.0",
    .dependencies = .{
        .raylib = .{
            .url = "https://github.com/raysan5/raylib/archive/e9291fa4c77c85e1fe6808289632e5ce4a93eed6.tar.gz",
            .hash = "1220ee786fc07876bb53e359c0a204e13dfb1a89735c1221363888021322f0b9f131",
        },
    },
    .paths = .{""},
}
```

## build.zig

```zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // 增加了 raylib 的依赖库
    const raylib_dep = b.dependency("raylib", .{
        .target = target,
        .optimize = optimize,
    });
    exe.linkLibrary(raylib_dep.artifact("raylib"));

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const exe_unit_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_exe_unit_tests.step);
}
```

## raylib.zig

```zig
pub usingnamespace @cImport({
    @cInclude("raylib.h");
    @cInclude("raymath.h");
    @cInclude("rlgl.h");
});
```

## main.zig

先将之前的 main 函数注释，添加一个新的 main 函数。

```zig
const std = @import("std");
const ray = @import("raylib.zig");

pub fn main() void {

    // Initialization
    //--------------------------------------------------------------------------------------
    const screenWidth = 800;
    const screenHeight = 450;

    ray.InitWindow(screenWidth, screenHeight, "raylib [core] example - basic window");
    defer ray.CloseWindow();

    ray.SetTargetFPS(60); // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    while (!ray.WindowShouldClose()) // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        // TODO: Update your variables here
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        ray.BeginDrawing();

        ray.ClearBackground(ray.RAYWHITE);

        ray.DrawText("Congrats! You created your first window!", 190, 200, 20, ray.LIGHTGRAY);

        ray.EndDrawing();
        //----------------------------------------------------------------------------------
    }
}
```

## 效果

![raylib 简单窗口][1]

## 总结

使用 Zig 语言，引入 raylib 库，实现了显示一个图形窗口。

[1]: images/raylib-hello.png

## 附录
