# 0334-Tetris-定义方块的类型

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

将方块的七种类型表示出来，定义四个方向，方块可以变形。  
表示方式为一个二维数组，一维来表示方向，另一维来表示方块所占的方格。  
一个方块包含四个小方格，一个方格需要横纵坐标，所以使用长度为 8 的数组来表示。  

## 七种方格的定义

```zig
const std = @import("std");
const screen = @import("screen.zig");

const tetriminoes: [7]Tetrimino = label: {
    var arr: [7]Tetrimino = undefined;
    // I
    arr[0] = .{ .y = -1, .value = .{
        .{ 0, 1, 1, 1, 2, 1, 3, 1 },
        .{ 2, 0, 2, 1, 2, 2, 2, 3 },
        .{ 0, 2, 1, 2, 2, 2, 3, 2 },
        .{ 1, 0, 1, 1, 1, 2, 1, 3 },
    }, .color = 0x00ffffff };
    // J
    arr[1] = .{ .value = .{
        .{ 0, 0, 0, 1, 1, 1, 2, 1 },
        .{ 1, 0, 2, 0, 1, 1, 1, 2 },
        .{ 0, 1, 1, 1, 2, 1, 2, 2 },
        .{ 1, 0, 1, 1, 0, 2, 1, 2 },
    }, .color = 0x0000ffff };
    // L
    arr[2] = .{ .value = .{
        .{ 2, 0, 0, 1, 1, 1, 2, 1 },
        .{ 1, 0, 1, 1, 1, 2, 2, 2 },
        .{ 0, 1, 1, 1, 2, 1, 0, 2 },
        .{ 0, 0, 1, 0, 1, 1, 1, 2 },
    }, .color = 0xffaa00ff };
    // O
    arr[3] = .{ .value = .{
        .{ 1, 0, 2, 0, 1, 1, 2, 1 },
        .{ 1, 0, 2, 0, 1, 1, 2, 1 },
        .{ 1, 0, 2, 0, 1, 1, 2, 1 },
        .{ 1, 0, 2, 0, 1, 1, 2, 1 },
    }, .color = 0xffff00ff };
    // S
    arr[4] = .{ .value = .{
        .{ 1, 0, 2, 0, 0, 1, 1, 1 },
        .{ 1, 0, 1, 1, 2, 1, 2, 2 },
        .{ 1, 1, 2, 1, 0, 2, 1, 2 },
        .{ 0, 0, 0, 1, 1, 1, 1, 2 },
    }, .color = 0x00ff00ff };
    // T
    arr[5] = .{ .value = .{
        .{ 1, 0, 0, 1, 1, 1, 2, 1 },
        .{ 1, 0, 1, 1, 2, 1, 1, 2 },
        .{ 0, 1, 1, 1, 2, 1, 1, 2 },
        .{ 1, 0, 0, 1, 1, 1, 1, 2 },
    }, .color = 0x9900ffff };
    // Z
    arr[6] = .{ .value = .{
        .{ 0, 0, 1, 0, 1, 1, 2, 1 },
        .{ 2, 0, 1, 1, 2, 1, 1, 2 },
        .{ 0, 1, 1, 1, 1, 2, 2, 2 },
        .{ 1, 0, 0, 1, 1, 1, 0, 2 },
    }, .color = 0xff0000ff };
    break :label arr;
};
```

## 结构

```zig
pub const Facing = enum { North, East, South, West };
pub const Tetrimino = struct {
    x: i32 = 3,
    y: i32 = 0,
    facing: Facing = .North,
    value: [4][8]u8 = undefined,
    color: u32,
    solid: bool = false,

    pub fn position(self: *Tetrimino) [8]u8 {
        return self.value[@intFromEnum(self.facing)];
    }

    pub fn random(rand: *std.rand.DefaultPrng) Tetrimino {
        const len = tetriminoes.len;
        return tetriminoes[rand.random().uintLessThan(usize, len)];
    }

    pub fn rotate(self: *Tetrimino) void {
        const int: u8 = @intFromEnum(self.facing);
        const len = std.enums.values(Facing).len;
        self.facing = @enumFromInt(int + 1 % len);
    }

    pub fn locateIn(self: *Tetrimino) void {
        const pos = self.position();

        const minx = @min(@min(@min(pos[0], pos[2]), pos[4]), pos[6]);
        if (self.x + minx < 0) self.x -= self.x + minx;

        const maxx = @max(@max(@max(pos[0], pos[2]), pos[4]), pos[6]);
        const x = self.x + maxx - screen.WIDTH;
        if (x >= 0) self.x -= x + 1;
    }
};
```

1. position 方法会根据当前方向，计算出方块的像素点。
2. rotate 会调整方块的朝向。
3. random 可以随机生成七种方块中的一种。
4. locateIn 保证方块不超出屏幕的两边。

## 总结

定义了全部方块的类型和结构，定义了其中的一些方法。方块模块 block.zig 就这些代码，后续不再修改和列出。

## 附录

### block.zig

```zig
const std = @import("std");
const screen = @import("screen.zig");

const tetriminoes: [7]Tetrimino = label: {
    var arr: [7]Tetrimino = undefined;
    // I
    arr[0] = .{ .y = -1, .value = .{
        .{ 0, 1, 1, 1, 2, 1, 3, 1 },
        .{ 2, 0, 2, 1, 2, 2, 2, 3 },
        .{ 0, 2, 1, 2, 2, 2, 3, 2 },
        .{ 1, 0, 1, 1, 1, 2, 1, 3 },
    }, .color = 0x00ffffff };
    // J
    arr[1] = .{ .value = .{
        .{ 0, 0, 0, 1, 1, 1, 2, 1 },
        .{ 1, 0, 2, 0, 1, 1, 1, 2 },
        .{ 0, 1, 1, 1, 2, 1, 2, 2 },
        .{ 1, 0, 1, 1, 0, 2, 1, 2 },
    }, .color = 0x0000ffff };
    // L
    arr[2] = .{ .value = .{
        .{ 2, 0, 0, 1, 1, 1, 2, 1 },
        .{ 1, 0, 1, 1, 1, 2, 2, 2 },
        .{ 0, 1, 1, 1, 2, 1, 0, 2 },
        .{ 0, 0, 1, 0, 1, 1, 1, 2 },
    }, .color = 0xffaa00ff };
    // O
    arr[3] = .{ .value = .{
        .{ 1, 0, 2, 0, 1, 1, 2, 1 },
        .{ 1, 0, 2, 0, 1, 1, 2, 1 },
        .{ 1, 0, 2, 0, 1, 1, 2, 1 },
        .{ 1, 0, 2, 0, 1, 1, 2, 1 },
    }, .color = 0xffff00ff };
    // S
    arr[4] = .{ .value = .{
        .{ 1, 0, 2, 0, 0, 1, 1, 1 },
        .{ 1, 0, 1, 1, 2, 1, 2, 2 },
        .{ 1, 1, 2, 1, 0, 2, 1, 2 },
        .{ 0, 0, 0, 1, 1, 1, 1, 2 },
    }, .color = 0x00ff00ff };
    // T
    arr[5] = .{ .value = .{
        .{ 1, 0, 0, 1, 1, 1, 2, 1 },
        .{ 1, 0, 1, 1, 2, 1, 1, 2 },
        .{ 0, 1, 1, 1, 2, 1, 1, 2 },
        .{ 1, 0, 0, 1, 1, 1, 1, 2 },
    }, .color = 0x9900ffff };
    // Z
    arr[6] = .{ .value = .{
        .{ 0, 0, 1, 0, 1, 1, 2, 1 },
        .{ 2, 0, 1, 1, 2, 1, 1, 2 },
        .{ 0, 1, 1, 1, 1, 2, 2, 2 },
        .{ 1, 0, 0, 1, 1, 1, 0, 2 },
    }, .color = 0xff0000ff };
    break :label arr;
};

pub const Facing = enum { North, East, South, West };
pub const Tetrimino = struct {
    x: i32 = 3,
    y: i32 = 0,
    facing: Facing = .North,
    value: [4][8]u8 = undefined,
    color: u32,
    solid: bool = false,

    pub fn position(self: *Tetrimino) [8]u8 {
        return self.value[@intFromEnum(self.facing)];
    }

    pub fn random(rand: *std.rand.DefaultPrng) Tetrimino {
        const len = tetriminoes.len;
        return tetriminoes[rand.random().uintLessThan(usize, len)];
    }

    pub fn rotate(self: *Tetrimino) void {
        const int: u8 = @intFromEnum(self.facing);
        const len = std.enums.values(Facing).len;
        self.facing = @enumFromInt(int + 1 % len);
    }

    pub fn locateIn(self: *Tetrimino) void {
        const pos = self.position();

        const minx = @min(@min(@min(pos[0], pos[2]), pos[4]), pos[6]);
        if (self.x + minx < 0) self.x -= self.x + minx;

        const maxx = @max(@max(@max(pos[0], pos[2]), pos[4]), pos[6]);
        const x = self.x + maxx - screen.WIDTH;
        if (x >= 0) self.x -= x + 1;
    }
};
```
