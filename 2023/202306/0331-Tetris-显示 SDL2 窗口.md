# 0331-Tetris-显示 SDL2 窗口

## 环境

- Time 2023-08-18
- Zig 0.11.0
- WSL-Ubuntu 22.04.3 LTS

## 前言

### 说明

参考资料：

1. <https://www.youtube.com/watch?v=nF_crEtmpBo>
2. <https://github.com/howprice/sdl2-tetris>

### 目标

在上一节，初始化了 Zig 项目环境。这一节，初始化 SDL2 环境，并显示一个窗口。

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

将 SDL2 的头文件封装到 c.zig 文件中，然后定义了一个 panic 方法，用来处理出错。

## main.zig

```zig
const c = @import("c.zig");

pub fn main() !void {
    if (c.SDL_Init(c.SDL_INIT_EVERYTHING) < 0) c.sdlPanic();
    defer c.SDL_Quit();

    const center = c.SDL_WINDOWPOS_CENTERED;
    var window = c.SDL_CreateWindow("俄罗斯方块", center, center, //
        400, 800, c.SDL_WINDOW_SHOWN) orelse c.sdlPanic();
    defer _ = c.SDL_DestroyWindow(window);

    var renderer = c.SDL_CreateRenderer(window, -1, 0) orelse c.sdlPanic();
    defer _ = c.SDL_DestroyRenderer(renderer);

    mainLoop: while (true) {
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event) != 0) {
            if (event.type == c.SDL_QUIT)
                break :mainLoop;
        }
        _ = c.SDL_SetRenderDrawColor(renderer, 0x3B, 0x3B, 0x3B, 0xFF);
        _ = c.SDL_RenderClear(renderer);
        c.SDL_RenderPresent(renderer);
    }
}
```

SDL2 标准的处理流程，和 C 语言类似，初始化窗口和渲染器，设置主循环，监听事件，设置背景色。

## 运行

通过命令 `zig build run`，可以看到弹出了一个窗口。

![显示 SDL2 窗口][1]

## 总结

使用 Zig 和 SDL2，在 WSL 中的 Linux 环境下，显示了一个图形窗口。

[1]: images/sdl2-window.png

## 附录

### 中文显示框框

如果出现中文不能显示的问题，可以安装中文字体 `apt-get install ttf-wqy-microhei`，然后重启 WSL。
