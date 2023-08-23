# 0349-Tetris-显示下一个方块

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

方块应该有预告功能，控制当前方块时，可以看到接下来一个方块。

## app.zig

```zig
const std = @import("std");
const Screen = @import("display.zig").Screen;
const Tetrimino = @import("block.zig").Tetrimino;

pub const Game = struct {
    current: Tetrimino,
    next: Tetrimino,
    prng: std.rand.DefaultPrng,
    over: bool = false,
    score: usize = 0,

    pub fn new() Game {
        const seed = @as(u64, @intCast(std.time.timestamp()));
        var rand = std.rand.DefaultPrng.init(seed);
        return Game{
            .current = Tetrimino.random(&rand),
            .next = Tetrimino.random(&rand),
            .prng = rand,
        };
    }

    pub fn drawCurrent(self: *Game, screen: *Screen) void {
        _ = draw(&self.current, screen, self.current.x, self.current.y);
        _ = draw(&self.next, screen, 12, 10);
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
            const cur = &self.current;
            const lines = draw(cur, screen, cur.x, cur.y);
            self.score += computeScore(lines);

            self.current = self.next;
            self.next = Tetrimino.random(&self.prng);
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

fn draw(block: *const Tetrimino, screen: *Screen, x: i32, y: i32) u8 {
    const value = block.position();
    var index: usize = 0;
    var completed: u8 = 0;
    while (index < value.len) : (index += 2) {
        const row: usize = @intCast(x + value[index]);
        const col: usize = @intCast(y + value[index + 1]);
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

1. 修改了 `draw` 方法，坐标直接传递，而不是从方块中取。
2. 增加了 `next` 字段，可以预告下一个方块。
3. 向下移动的时候，如果固定了，则将下一个方块给当前，再随机生成一个给 `next`。

## 效果

![下一个方块][1]

## 总结

实现了方块的预告功能，将下一个方块显示到用户界面上。

[1]: images/next-block.gif

## 附录
