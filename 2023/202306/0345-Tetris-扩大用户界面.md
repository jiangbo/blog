# 0345-Tetris-扩大用户界面

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

扩大游戏的用户界面，增加其它的元素。

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

    pub fn init(self: *Screen) void {
        if (c.SDL_Init(c.SDL_INIT_EVERYTHING) < 0) c.sdlPanic();

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

调整了游戏的窗口大小，调整了游戏界面的边框和边距。

## 效果

![扩大用户界面][1]

## 总结

扩大用户界面，以便于增加其它的元素。

[1]: images/user-interface.png

## 附录
