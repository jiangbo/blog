# 0347-Tetris-游戏分数计算

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

在消除一行或者多行时，应该记录消除的行数的对应得分，并一直累计。

## app.zig

```zig
const std = @import("std");
const Screen = @import("display.zig").Screen;
const Tetrimino = @import("block.zig").Tetrimino;

pub const Game = struct {
    current: Tetrimino,
    prng: std.rand.DefaultPrng,
    over: bool = false,
    score: usize = 0,

    pub fn new() Game {
        const seed = @as(u64, @intCast(std.time.timestamp()));
        var rand = std.rand.DefaultPrng.init(seed);
        return Game{
            .current = Tetrimino.random(&rand),
            .prng = rand,
        };
    }

    pub fn drawCurrent(self: *Game, screen: *Screen) void {
        _ = draw(&self.current, screen);
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
            const lines = draw(&self.current, screen);
            self.score += computeScore(lines);
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

fn draw(block: *const Tetrimino, screen: *Screen) u8 {
    const value = block.position();
    var index: usize = 0;
    var completed: u8 = 0;
    while (index < value.len) : (index += 2) {
        const row: usize = @intCast(block.x + value[index]);
        const col: usize = @intCast(block.y + value[index + 1]);
        if (block.solid) {
            if (screen.drawSolid(row, col, block.color))
                completed += 1;
        } else {
            screen.draw(row, col, block.color);
        }
    }
    return completed;
}

fn computeScore(lines: u8) usize {
    return switch (lines) {
        1 => 100,
        2 => 300,
        3 => 600,
        4 => 1000,
        else => 0,
    };
}
```

1. 增加了一个 `score` 字段来记录游戏的分数。
2. `draw` 方法返回了当前方块固定后消除的行数。
3. 增加了 `computeScore` 来根据行数计算分数。

## 总结

实现了游戏的得分逻辑，和同一场游戏得分的累计。

## 附录
