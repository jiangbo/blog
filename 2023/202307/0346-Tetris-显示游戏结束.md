# 0346-Tetris-显示游戏结束

## 环境

- Time 2023-08-23
- Zig 0.12.0-dev.161+6a5463951
- WSL-Ubuntu 22.04.3 LTS

## 前言

### 说明

参考资料：

1. <https://www.youtube.com/watch?v=nF_crEtmpBo>
2. <https://github.com/howprice/sdl2-tetris>

### 目标

增加字体的支持，同时游戏结束时，显示 `GAME OVER` 提示。  
字体来源：<https://github.com/howprice/sdl2-tetris/blob/master/data/clacon.ttf>

## display.zig

```zig
const c = @import("c.zig");

pub const WIDTH = 10;
pub const HEIGHT = 20;

const FPS = 60;
const SCALE = 40; // 放大倍数
const BORDER = 2; // 边框

pub const Screen = struct {
    line: usize = HEIGHT,
    buffer: [WIDTH][HEIGHT]u32 = undefined,
    window: *c.SDL_Window = undefined,
    renderer: *c.SDL_Renderer = undefined,
    font: *c.TTF_Font = undefined,

    pub fn init(self: *Screen) void {
        if (c.SDL_Init(c.SDL_INIT_EVERYTHING) < 0) c.sdlPanic();
        if (c.TTF_Init() < 0) c.sdlPanic();

        self.font = c.TTF_OpenFont("clacon.ttf", 60) orelse c.sdlPanic();
        const center = c.SDL_WINDOWPOS_CENTERED;
        self.window = c.SDL_CreateWindow("俄罗斯方块", center, center, //
            700, 850, c.SDL_WINDOW_SHOWN) orelse c.sdlPanic();

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
            .x = @intCast(x * SCALE + BORDER + 20),
            .y = @intCast(y * SCALE + BORDER + 20),
            .w = @intCast(SCALE - BORDER * 2),
            .h = @intCast(SCALE - BORDER * 2),
        };
        _ = c.SDL_RenderFillRect(self.renderer, &rect);
    }

    pub fn drawSolid(self: *Screen, x: usize, y: usize, rgba: u32) bool {
        self.draw(x, y, rgba);
        self.buffer[x][y] = rgba;
        self.line = @min(self.line, y);
        for (0..WIDTH) |row| {
            if (self.buffer[row][y] == 0) return false;
        }
        return self.clearRow(y);
    }

    fn clearRow(self: *Screen, col: usize) bool {
        var y = col;
        while (y >= self.line) : (y -= 1) {
            for (0..WIDTH) |x| {
                self.buffer[x][y] = self.buffer[x][y - 1];
            }
        }
        self.line += 1;
        return true;
    }

    pub fn drawText(self: *Screen, text: [*c]const u8, x: i32, y: i32) void {
        var surface = c.TTF_RenderUTF8_Solid(self.font, text, //
            .{ .r = 0xff, .g = 0xff, .b = 0xff, .a = 255 });
        var texture = c.SDL_CreateTextureFromSurface(self.renderer, //
            surface) orelse c.sdlPanic();
        var r = c.SDL_Rect{ .x = x, .y = y, .w = 0, .h = 0 };
        _ = c.SDL_QueryTexture(texture, null, null, &r.w, &r.h);
        _ = c.SDL_RenderCopy(self.renderer, texture, null, &r);
        c.SDL_FreeSurface(surface);
        c.SDL_DestroyTexture(texture);
    }

    pub fn hasSolid(self: *const Screen, x: usize, y: usize) bool {
        if (x >= WIDTH) return false;
        return y >= HEIGHT or self.buffer[x][y] != 0;
    }

    pub fn present(self: *Screen) void {
        c.SDL_RenderPresent(self.renderer);
        c.SDL_Delay(1000 / FPS);
    }

    pub fn deinit(self: *Screen) void {
        c.SDL_DestroyRenderer(self.renderer);
        c.SDL_DestroyWindow(self.window);
        c.TTF_Quit();
        c.SDL_Quit();
    }
};
```

增加了字体的支持，增加了一个 `drawText` 方法来进行字体的显示。

## main.zig

```zig
...
if (game.over) screen.drawText("GAME OVER", 460, 650);
...
```

增加了一行游戏结束的判断，如果结束则显示游戏结束标志。

## 效果

![GAME OVER][1]

## 总结

增加游戏结束的判断，游戏结束时，在界面上显示 `GAME OVER` 标志。

[1]: images/game-over.gif

## 附录

### main.zig 源码

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

            if (game.over) break;
            handleInput(&game, &screen, &event);
        }

        screen.update();
        game.drawCurrent(&screen);
        if (game.over) screen.drawText("GAME OVER", 460, 650);
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
