# 0333-Tetris-显示游戏窗格

## 环境

- Time 2023-08-22
- Zig 0.11.0
- WSL-Ubuntu 22.04.3 LTS

## 前言

### 说明

参考资料：

1. <https://www.youtube.com/watch?v=nF_crEtmpBo>
2. <https://github.com/howprice/sdl2-tetris>

### 目标

在游戏画面上，将游戏的窗格画出来。

## c.zig

```zig
pub usingnamespace @cImport({
    @cInclude("SDL.h");
    @cInclude("SDL_ttf.h");
});

const self = @This();
const std = @import("std");

pub fn sdlPanic() noreturn {
    const str = @as(?[*:0]const u8, self.SDL_GetError());
    @panic(std.mem.sliceTo(str orelse "unknown error", 0));
}
```

## display.zig

增加了两个方法，update 方法和 draw 方法。

```zig
const c = @import("c.zig");

pub const WIDTH = 10;
pub const HEIGHT = 20;

const FPS = 60;
const SCALE = 40; // 放大倍数
const BORDER = 2; // 边框

pub const Screen = struct {
    buffer: [WIDTH][HEIGHT]u32 = undefined,
    window: *c.SDL_Window = undefined,
    renderer: *c.SDL_Renderer = undefined,

    pub fn init(self: *Screen) void {
        if (c.SDL_Init(c.SDL_INIT_EVERYTHING) < 0) c.sdlPanic();

        const center = c.SDL_WINDOWPOS_CENTERED;
        self.window = c.SDL_CreateWindow("俄罗斯方块", center, center, //
            400, 800, c.SDL_WINDOW_SHOWN) orelse c.sdlPanic();

        self.renderer = c.SDL_CreateRenderer(self.window, -1, 0) //
        orelse c.sdlPanic();
    }

    pub fn update(self: *Screen) void {
        _ = c.SDL_SetRenderDrawColor(self.renderer, 0x3b, 0x3b, 0x3b, 0xff);
        _ = c.SDL_RenderClear(self.renderer);
        for (0..WIDTH) |row| {
            for (0..HEIGHT) |col| {
                var color = self.buffer[row][col];
                if (color == 0) color = 0x404040ff;
                self.draw(row, col, color);
            }
        }
    }

    pub fn draw(self: *Screen, x: usize, y: usize, rgba: u32) void {
        const r: u8 = @truncate((rgba >> 24) & 0xff);
        const g: u8 = @truncate((rgba >> 16) & 0xff);
        const b: u8 = @truncate((rgba >> 8) & 0xff);
        const a: u8 = @truncate((rgba >> 0) & 0xff);
       
        _ = c.SDL_SetRenderDrawColor(self.renderer, r, g, b, a);
        const rect = c.SDL_Rect{
            .x = @intCast(x * SCALE + BORDER),
            .y = @intCast(y * SCALE + BORDER),
            .w = @intCast(SCALE - BORDER),
            .h = @intCast(SCALE - BORDER),
        };
        _ = c.SDL_RenderFillRect(self.renderer, &rect);
    }

    pub fn present(self: *Screen) void {
        c.SDL_RenderPresent(self.renderer);
        c.SDL_Delay(1000 / FPS);
    }

    pub fn deinit(self: *Screen) void {
        c.SDL_DestroyRenderer(self.renderer);
        c.SDL_DestroyWindow(self.window);
        c.SDL_Quit();
    }
};
```

## main.zig

```zig
const c = @import("c.zig");
const display = @import("display.zig");

pub fn main() !void {

    var screen = display.Screen{};
    screen.init();
    defer screen.deinit();

    mainLoop: while (true) {
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event) != 0) {
            if (event.type == c.SDL_QUIT)
                break :mainLoop;
        }
        screen.update();
        screen.present();
    }
}
```

## 效果

![显示游戏窗格][1]

## 总结

显示了游戏背景，并且将画面分成一格一格的。

[1]: images/window_grid.png

## 附录
