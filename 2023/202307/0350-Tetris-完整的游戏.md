# 0350-Tetris-完整的游戏

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

方块的预告功能，还差一个提示的文字和背景。

## display.zig

```zig
...
    pub fn update(self: *Screen, score: usize, over: bool) void {
        _ = c.SDL_SetRenderDrawColor(self.renderer, 0x3b, 0x3b, 0x3b, 0xff);
        _ = c.SDL_RenderClear(self.renderer);
        for (0..WIDTH) |row| {
            for (0..HEIGHT) |col| {
                var color = self.buffer[row][col];
                if (color == 0) color = 0x404040ff;
                self.draw(row, col, color);
            }
        }

        self.drawScore(score);
        self.drawText("Next", 510, 280);
        var r = c.SDL_Rect{ .x = 440, .y = 360, .w = 240, .h = 200 };
        _ = c.SDL_RenderFillRect(self.renderer, &r);
        if (over) self.drawText("GAME OVER", 460, 650);
    }
...
```

增加了一个方块的提示文字和背景。

## 效果

![完整的游戏][1]

## 总结

实现了游戏的大部分逻辑，已经可以正常游戏。

[1]: images/tetris.gif

## 附录

### 完整源码

<https://github.com/jiangbo/game/tree/main/zig/tetris>
