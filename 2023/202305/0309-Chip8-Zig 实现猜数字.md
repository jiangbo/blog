# 0309-Chip8-Zig 实现猜数字

## 环境

- Time 2023-07-13
- Zig 0.10.1

## 前言

### 说明

参考：

- <https://opensource.com/article/23/1/learn-zig-programming>

### 目标

使用 Zig 语言，编写一个猜数字游戏，了解其基本的概念和用法。

## main

```zig
const std = @import("std");

pub fn main() !void {

    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();
    var buf: [10]u8 = undefined;

    var prng = std.rand.DefaultPrng.init(@intCast(u64, std.time.timestamp()));
    const value = prng.random().intRangeAtMost(i64, 1, 100);

    while (true) {
        try stdout.print("Guess a number between 1 and 100: ", .{});

        if (try stdin.readUntilDelimiterOrEof(buf[0..], '\n')) |input| {
            const number = std.mem.trim(u8, input, "\r");
            const guess = try std.fmt.parseInt(i64, number, 10);

            if (guess == value) {
                break;
            }
            const message = if (guess < value) "low" else "high";
            try stdout.print("Too {s}\n", .{message});
        }
    }
    try stdout.print("That's right\n", .{});
}
```

## 总结

使用 Zig 实现了一个猜数字游戏。

## 附录
