# 0340-Tetris-检测移动碰撞

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

方块下落时，到最底部时应该停止，而不是继续下落到屏幕外。如果下落时碰到其它方块，也应该停止。

## display.zig

```zig
...
    pub fn hasSolid(self: *const Screen, x: usize, y: usize) bool {
        if (x >= WIDTH) return false;
        return y >= HEIGHT or self.buffer[x][y] != 0;
    }
...
```

省略了该文件中其它不相关的部分，hasSolid 方法用来检查是否碰到最底部，或者和其它的方块发生碰撞。现在还没有保存其它方块的状态，后面会使用。

## app.zig

```zig
    pub fn moveLeft(self: *Game, screen: *Screen) void {
        _ = self.move(screen, -1, 0);
    }

    pub fn moveRight(self: *Game, screen: *Screen) void {
        _ = self.move(screen, 1, 0);
    }

    pub fn moveDown(self: *Game, screen: *Screen) void {
        _ = self.move(screen, 0, 1);
    }

    fn move(self: *Game, screen: *const Screen, x: i8, y: i8) bool {
        self.current.x = self.current.x + x;
        self.current.y = self.current.y + y;
        self.current.locateIn();

        return if (self.isFit(screen)) {
            _ = self.move(screen, -x, -y);
            return true;
        } else false;
    }

    fn isFit(self: *const Game, screen: *const Screen) bool {
        const value = self.current.position();
        var index: usize = 0;
        while (index < value.len) : (index += 2) {
            const col = self.current.y + value[index + 1];
            if (col < 0) return true;
            const row: usize = @intCast(self.current.x + value[index]);
            if (screen.hasSolid(row, @intCast(col))) return true;
        }
        return false;
    }
```

isFit 来检查是否发生了碰撞，如果发生了碰撞，需要将当前次的移动还原。

## 效果

![方块碰撞检测][1]

## 总结

实现了移动的碰撞检测，如果移动后发现出现了碰撞，需要将当次碰撞还原。

[1]: images/is-fit.gif

## 附录

### display.zig 源码

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
        c.SDL_Quit();
    }
};
```

### app.zig 源码

```zig
const std = @import("std");
const Screen = @import("display.zig").Screen;
const Tetrimino = @import("block.zig").Tetrimino;

pub const Game = struct {
    current: Tetrimino,
    prng: std.rand.DefaultPrng,

    pub fn new() Game {
        const seed = @as(u64, @intCast(std.time.timestamp()));
        var rand = std.rand.DefaultPrng.init(seed);
        return Game{
            .current = Tetrimino.random(&rand),
            .prng = rand,
        };
    }

    pub fn drawCurrent(self: *Game, screen: *Screen) void {
        draw(&self.current, screen);
    }

    pub fn moveLeft(self: *Game, screen: *Screen) void {
        _ = self.move(screen, -1, 0);
    }

    pub fn moveRight(self: *Game, screen: *Screen) void {
        _ = self.move(screen, 1, 0);
    }

    pub fn moveDown(self: *Game, screen: *Screen) void {
        _ = self.move(screen, 0, 1);
    }

    fn move(self: *Game, screen: *const Screen, x: i8, y: i8) bool {
        self.current.x = self.current.x + x;
        self.current.y = self.current.y + y;
        self.current.locateIn();

        return if (self.isFit(screen)) {
            _ = self.move(screen, -x, -y);
            return true;
        } else false;
    }

    pub fn rotate(self: *Game, screen: *Screen) void {
        _ = screen;
        self.current.rotate();
        self.current.locateIn();
    }

    fn isFit(self: *const Game, screen: *const Screen) bool {
        const value = self.current.position();
        var index: usize = 0;
        while (index < value.len) : (index += 2) {
            const col = self.current.y + value[index + 1];
            if (col < 0) return true;
            const row: usize = @intCast(self.current.x + value[index]);
            if (screen.hasSolid(row, @intCast(col))) return true;
        }
        return false;
    }
};

fn draw(block: *const Tetrimino, screen: *Screen) void {
    const value = block.position();
    var index: usize = 0;
    while (index < value.len) : (index += 2) {
        const row: usize = @intCast(block.x + value[index]);
        const col: usize = @intCast(block.y + value[index + 1]);
        screen.draw(row, col, block.color);
    }
}
```
