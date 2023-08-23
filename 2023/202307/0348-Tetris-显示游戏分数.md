# 0348-Tetris-显示游戏分数

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

在上一节，实现了游戏的得分计算，这一节将显示游戏的得分。

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
        if (over) self.drawText("GAME OVER", 460, 650);
    }

    fn drawScore(self: *Screen, score: usize) void {
        self.drawText("Score", 500, 50);

        _ = c.SDL_SetRenderDrawColor(self.renderer, 40, 40, 40, 0xff);
        var r = c.SDL_Rect{ .x = 440, .y = 120, .w = 240, .h = 100 };
        _ = c.SDL_RenderFillRect(self.renderer, &r);
        var buf: [9]u8 = undefined;
        var text = fmt.bufPrintZ(&buf, "{:0>7}", .{score});
        self.drawText(text catch unreachable, 480, 145);
    }
...
```

增加了一个 `drawScore` 的方法，并且将游戏结束的方法移动到了 `update` 方法中。

## 效果

![游戏分数][1]

## 总结

能够实时显示游戏的分数。

[1]: images/renderer-score.gif

## 附录
