# 0313-Zig 实现 chip8 模拟器

## 环境

- Time 2023-07-26
- Zig 0.11.0-dev.4191+1bf16b172
- SLD2 2.28.1

## 前言

### 说明

具体的介绍直接网络上搜索，以下是参考资料：

1. <https://en.wikipedia.org/wiki/CHIP-8>
2. <https://austinmorlan.com/posts/chip8_emulator/>
3. <https://rsj217.github.io/chip8-py/>
4. <https://github.com/Timendus/chip8-test-suite>

其中最后一个提供了测试的套件，实现的过程中，可以检测哪些指令有问题，帮助很大。

### 目标

在前面几节，初始化好了屏幕和按键的封装，接下来初始化一个模拟器。

## screen

代码和之前的无变化，可以参考之前的，或者最后的附录。

## keypad

代码和之前的无变化，可以参考之前的，或者最后的附录。

## chip8.zig

```zig
const screen = @import("screen.zig");
const keypad = @import("keypad.zig");

const FPS = 60;

pub const Emulator = struct {
    screen: screen.Screen,
    keypad: keypad.Keypad,

    pub fn new() Emulator {
        return Emulator{
            .screen = screen.Screen{},
            .keypad = keypad.Keypad{},
        };
    }

    pub fn run(self: *Emulator) void {
        self.screen.init();
        defer self.screen.deinit();

        var index: usize = 0;
        while (self.keypad.poll()) : (index += 1) {
            if (index % 44 == 0) self.screen.clear();
            _ = self.screen.setIndex(index);
            self.screen.update(FPS);
        }
    }
};
```

## main.zig

```zig
const std = @import("std");
const chip8 = @import("chip8.zig");

pub fn main() !void {
    var emulator = chip8.Emulator.new();
    emulator.run();
}
```

## 启动

`zig build run`

## 效果

![窗口][1]

## 总结

新建了一个模拟器的文件，将屏幕和键盘放到了模拟器中，接下来给模拟器插上 CPU 和内存。

[1]: images/screen.png

## 附录

### screen.zig

```zig
const c = @cImport(@cInclude("SDL.h"));

const WIDTH: c_int = 64;
const HEIGHT: c_int = 32;
const BUFFER_SIZE = WIDTH * HEIGHT;

pub const Screen = struct {
    scale: u8 = 10,
    buffer: [BUFFER_SIZE]bool = undefined,
    needRender: bool = true,
    window: *c.SDL_Window = undefined,
    renderer: *c.SDL_Renderer = undefined,
    texture: *c.SDL_Texture = undefined,

    pub fn init(self: *Screen) void {
        if (c.SDL_Init(c.SDL_INIT_EVERYTHING) < 0)
            @panic("sdl init failed");

        const center = c.SDL_WINDOWPOS_CENTERED;
        self.window = c.SDL_CreateWindow("chip8", center, center, //
            WIDTH * self.scale, HEIGHT * self.scale, c.SDL_WINDOW_SHOWN) //
        orelse @panic("create window failed");

        self.renderer = c.SDL_CreateRenderer(self.window, -1, 0) //
        orelse @panic("create renderer failed");

        self.texture = c.SDL_CreateTexture(self.renderer, //
            c.SDL_PIXELFORMAT_RGBA8888, c.SDL_TEXTUREACCESS_STREAMING, //
            WIDTH, HEIGHT) orelse @panic("create texture failed");

        _ = c.SDL_SetRenderTarget(self.renderer, self.texture);
        _ = c.SDL_RenderSetLogicalSize(self.renderer, WIDTH, HEIGHT);
    }

    pub fn update(self: *Screen, fps: u32) void {
        defer c.SDL_Delay(1000 / fps);
        if (!self.needRender) return;

        _ = c.SDL_SetRenderDrawColor(self.renderer, 0, 0, 0, 255);
        _ = c.SDL_RenderClear(self.renderer);
        _ = c.SDL_SetRenderDrawColor(self.renderer, 255, 255, 255, 255);

        for (self.buffer, 0..) |value, index| {
            if (value) {
                const x: c_int = @intCast(index % WIDTH);
                const y: c_int = @intCast(@divTrunc(index, WIDTH));
                _ = c.SDL_RenderDrawPoint(self.renderer, x, y);
            }
        }
        c.SDL_RenderPresent(self.renderer);
        self.needRender = false;
    }

    pub fn setIndex(self: *Screen, i: usize) bool {
        self.needRender = true;
        const index = if (i >= BUFFER_SIZE) i % BUFFER_SIZE else i;
        self.buffer[index] = !self.buffer[index];
        return self.buffer[index];
    }

    pub fn setPixel(self: *Screen, x: usize, y: usize) bool {
        return self.setIndex(x + y * WIDTH);
    }

    pub fn clear(self: *Screen) void {
        @memset(&self.buffer, false);
        self.needRender = true;
    }

    pub fn deinit(self: *Screen) void {
        c.SDL_DestroyTexture(self.texture);
        c.SDL_DestroyRenderer(self.renderer);
        c.SDL_DestroyWindow(self.window);
        c.SDL_Quit();
    }
};
```

### keypad.zig

```zig
const c = @cImport(@cInclude("SDL.h"));

pub const Keypad = struct {
    event: c.SDL_Event = undefined,

    pub fn poll(self: *Keypad) bool {
        while (c.SDL_PollEvent(&self.event) > 0) {
            if (self.event.type == c.SDL_QUIT) return false;
        }
        return true;
    }
};
```

### build.zig

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
