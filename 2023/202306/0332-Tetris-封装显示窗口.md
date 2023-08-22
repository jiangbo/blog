# 0332-Tetris-封装显示窗口

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

将游戏显示窗口的逻辑，封装到一起。

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

游戏的大小为宽 10，高 20。

```zig
const c = @import("c.zig");

pub const WIDTH = 10;
pub const HEIGHT = 20;

pub const Screen = struct {

    buffer: [WIDTH][HEIGHT]u32 = undefined,
    window: *c.SDL_Window = undefined,
    renderer: *c.SDL_Renderer = undefined,

    pub fn init(self: *Screen) void {

        if (c.SDL_Init(c.SDL_INIT_EVERYTHING) < 0) c.sdlPanic();

        const center = c.SDL_WINDOWPOS_CENTERED;
        self.window = c.SDL_CreateWindow("俄罗斯方块", center, center, //
           400, 800,  c.SDL_WINDOW_SHOWN) orelse c.sdlPanic();

        self.renderer = c.SDL_CreateRenderer(self.window, -1, 0) //
        orelse c.sdlPanic();
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
        _ = c.SDL_SetRenderDrawColor(screen.renderer, 0x3B, 0x3B, 0x3B, 0xFF);
        _ = c.SDL_RenderClear(screen.renderer);
        c.SDL_RenderPresent(screen.renderer);
    }
}
```

将游戏的显示逻辑封装到了 display.zig 文件中，不改变原有的显示逻辑。

## 效果

![显示 SDL2 窗口][1]

## 总结

将其中的显示逻辑进行了封装，没有改变原有的显示逻辑。

[1]: images/sdl2-window.png

## 附录
