# 0310-文本冒险 Hello

## 环境

- Time 2023-07-13
- Zig 0.10.1

## 前言

### 说明

参考：

- <https://helderman.github.io/htpataic/>

### 目标

根据提供的教程，实现一个文本冒险游戏，先搭建环境。

## main

```zig
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Welcome to Little Cave Adventure.\n", .{});
    try stdout.print("It is very dark in here.\n", .{});
    try stdout.print("\nBye!\n", .{});
}
```

## 总结

初始化文本冒险的开发环境。

## 附录
