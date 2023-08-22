# 0335-Tetris-渲染方块

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

手动将各种方块渲染到屏幕上。  
方块定义 block.zig 文件，先手动将 tetriminoes 定义为 pub，手动画后，再修改回私有。

## main.zig

```zig
const c = @import("c.zig");
const display = @import("display.zig");
const block = @import("block.zig");

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

        var tetrimino = block.tetriminoes[0];
        draw(&tetrimino, &screen);

        tetrimino = block.tetriminoes[1];
        tetrimino.y = 3;
        draw(&tetrimino, &screen);

        tetrimino = block.tetriminoes[2];
        tetrimino.y = 6;
        draw(&tetrimino, &screen);

        tetrimino = block.tetriminoes[3];
        tetrimino.y = 9;
        draw(&tetrimino, &screen);

        tetrimino = block.tetriminoes[4];
        tetrimino.y = 12;
        draw(&tetrimino, &screen);

        tetrimino = block.tetriminoes[5];
        tetrimino.y = 15;
        draw(&tetrimino, &screen);

        tetrimino = block.tetriminoes[6];
        tetrimino.y = 18;
        draw(&tetrimino, &screen);
        screen.present();
    }
}

fn draw(tetrimino: *block.Tetrimino, screen: *display.Screen) void {
    const value = tetrimino.position();
    var index: usize = 0;
    while (index < value.len) : (index += 2) {
        const row: usize = @intCast(tetrimino.x + value[index]);
        const col: usize = @intCast(tetrimino.y + value[index + 1]);
        screen.draw(row, col, tetrimino.color);
    }
}
```

## 效果

![渲染各种方块][1]

## 总结

手动渲染了各种方块，将其排列成一竖排。

[1]: images/renderer-block.png

## 附录
