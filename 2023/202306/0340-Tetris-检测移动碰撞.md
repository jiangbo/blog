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

        return if (self.isFit(screen))
            self.move(screen, -x, -y)
        else
            false;
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
