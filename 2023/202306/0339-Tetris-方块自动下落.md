# 0339-Tetris-方块自动下落

## 环境

- Time 2023-08-22
- Zig 0.12.0-dev.161+6a5463951
- WSL-Ubuntu 22.04.3 LTS

## 前言

### 说明

参考资料：

1. <https://www.youtube.com/watch?v=nF_crEtmpBo>
2. <https://github.com/howprice/sdl2-tetris>

### 目标

使用 SDL2 自带的定时器来实现方块的自动下落。

## main.zig

```zig
const c = @import("c.zig");
const std = @import("std");
const display = @import("display.zig");
const app = @import("app.zig");

pub fn main() !void {
    var screen = display.Screen{};
    screen.init();
    defer screen.deinit();
    var game = app.Game.new();
    _ = c.SDL_AddTimer(500, tick, null);

    mainLoop: while (true) {
        var event: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&event) != 0) {
            if (event.type == c.SDL_QUIT)
                break :mainLoop;

            handleInput(&game, &screen, &event);
        }

        screen.update();
        game.drawCurrent(&screen);
        screen.present();
    }
}

fn handleInput(game: *app.Game, screen: *display.Screen, event: *c.SDL_Event) void {
    if (event.type != c.SDL_KEYDOWN) return;

    const code = event.key.keysym.sym;
    switch (code) {
        c.SDLK_LEFT => game.moveLeft(screen),
        c.SDLK_RIGHT => game.moveRight(screen),
        c.SDLK_UP => game.rotate(screen),
        c.SDLK_DOWN => game.moveDown(screen),
        c.SDLK_SPACE => game.rotate(screen),
        else => return,
    }
}

fn tick(interval: u32, _: ?*anyopaque) callconv(.C) u32 {
    var event: c.SDL_Event = std.mem.zeroes(c.SDL_Event);
    event.type = c.SDL_KEYDOWN;
    event.key.keysym.sym = c.SDLK_DOWN;
    _ = c.SDL_PushEvent(&event);
    return interval;
}
```

只需要修改 main.zig 文件，增加一个 tick 方法，并将其加入到定时器中。

## 效果

![方块自动下落][1]

## 总结

通过定时器来实现方块的自动下落。

[1]: images/auto-down.gif

## 附录
