# 0310-Chip8-SDL2 开发环境

## 环境

- Time 2023-07-26
- Windows 11
- Zig 0.11.0-dev.4191+1bf16b172
- SLD2 2.28.1

## 前言

### 说明

参考：

- <https://github.com/libsdl-org/SDL>

下载路径：
通过 github 的发布页面下载 SDL2 的开发包：<https://github.com/libsdl-org/SDL/releases/download/release-2.28.1/SDL2-devel-2.28.1-VC.zip>，下载完成后（版本需要一致，或者手动调整路径），将其解压到 Zig 的工程根目录下的libs目录下。

```text
├─libs
│  └─SDL2-2.28.1
│      ├─include
│      └─lib
│          └─x64
├─src
│  └─main.zig
├─zig-cache
├─zig-out
├─zig-cache
├─build-zig
```

### 目标

使用 Zig 语言，依赖 SDL2 来启动一个图形化的界面。

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
    // 这里需要和下载的版本一致
    const sdl_path = "libs\\SDL2-2.28.1\\";
    exe.addIncludePath(sdl_path ++ "include");
    exe.addLibraryPath(sdl_path ++ "lib\\x64");
    b.installBinFile(sdl_path ++ "lib\\x64\\SDL2.dll", "SDL2.dll");
    exe.linkSystemLibrary("SDL2");
    exe.linkLibC();
    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
```

## main.zig

```zig
const std = @import("std");
const c = @cImport(@cInclude("SDL.h"));

const WIDTH: c_int = 640;
const HEIGHT: c_int = 320;

var window: *c.SDL_Window = undefined;
var renderer: *c.SDL_Renderer = undefined;
var texture: *c.SDL_Texture = undefined;

pub fn main() !void {
    init();
    defer deinit();

    mainloop: while (true) {
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event) > 0) {
            if (event.type == c.SDL_QUIT)
                break :mainloop;
        }
        c.SDL_Delay(44);
    }
}

fn init() void {
    if (c.SDL_Init(c.SDL_INIT_EVERYTHING) < 0)
        @panic("sdl init failed");

    const center = c.SDL_WINDOWPOS_CENTERED;
    window = c.SDL_CreateWindow("demo", center, center, //
        WIDTH, HEIGHT, c.SDL_WINDOW_SHOWN) //
    orelse @panic("create window failed");

    renderer = c.SDL_CreateRenderer(window, -1, 0) //
    orelse @panic("create renderer failed");

    texture = c.SDL_CreateTexture(renderer, //
        c.SDL_PIXELFORMAT_RGBA8888, c.SDL_TEXTUREACCESS_STREAMING, //
        WIDTH, HEIGHT) orelse @panic("create texture failed");
}

fn deinit() void {
    c.SDL_DestroyTexture(texture);
    c.SDL_DestroyRenderer(renderer);
    c.SDL_DestroyWindow(window);
    c.SDL_Quit();
}
```

## 启动

`zig build run`

## 效果

![窗口][1]

## 总结

使用 Zig 语言链接到 SDL2 库，实现了渲染一个窗口。

[1]: images/sdl2.png

## 附录
