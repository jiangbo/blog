# 0344-Tetris-游戏结束的条件

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

实现游戏结束的判断，如果新生成的方块，直接就发生了碰撞，则证明游戏结束。
游戏结束了，不能再进行控制。

## app.zig

```zig
...
pub const Game = struct {
    current: Tetrimino,
    prng: std.rand.DefaultPrng,
    over: bool = false,

    pub fn moveDown(self: *Game, screen: *Screen) void {
        if (self.move(screen, 0, 1)) {
            self.current.solid = true;
            draw(&self.current, screen);
            self.current = Tetrimino.random(&self.prng);
            if (self.isFit(screen)) self.over = true;
        }
    }
}
...
```

给游戏增加了一个游戏是否结束的字段，如果新生成的方块发生碰撞，则游戏结束。

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

            if (game.over) break;
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

在 handleInput 前，增加游戏是否结束判断。如果结束，则不处理任何操作。

## 效果

![游戏结束][1]

## 总结

实现了游戏结束的条件判断。

[1]: images/game-over.gif

## 附录

### app.zig 源码

```zig
const std = @import("std");
const Screen = @import("display.zig").Screen;
const Tetrimino = @import("block.zig").Tetrimino;

pub const Game = struct {
    current: Tetrimino,
    prng: std.rand.DefaultPrng,
    over: bool = false,

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
        if (self.move(screen, 0, 1)) {
            self.current.solid = true;
            draw(&self.current, screen);
            self.current = Tetrimino.random(&self.prng);
            if (self.isFit(screen)) self.over = true;
        }
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
        var temp = self.current;
        self.current.rotate();
        self.current.locateIn();
        if (self.isFit(screen)) {
            self.current = temp;
        }
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
        if (block.solid) {
            _ = screen.drawSolid(row, col, block.color);
        } else {
            screen.draw(row, col, block.color);
        }
    }
}
```
