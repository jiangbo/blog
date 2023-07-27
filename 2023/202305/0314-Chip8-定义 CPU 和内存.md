# 0314-给 chip 8 加上 CPU 和内存

## 环境

- Time 2023-07-26
- Zig 0.11.0-dev.4191+1bf16b172
- SLD2 2.28.1

## 前言

### 说明

参考资料：

1. <https://en.wikipedia.org/wiki/CHIP-8>
2. <https://austinmorlan.com/posts/chip8_emulator/>
3. <https://rsj217.github.io/chip8-py/>
4. <https://github.com/Timendus/chip8-test-suite>

其中最后一个提供了测试的套件，实现的过程中，可以检测哪些指令有问题，帮助很大。

### 目标

给前一节的实现的模拟器，加上 CPU 和内存，同时定义模拟器的 CPU 频率和屏幕帧率。

## screen

代码和之前的无变化，可以参考之前的，或者最后的附录。

## keypad

代码和之前的无变化，可以参考之前的，或者最后的附录。

## cpu.zig

1. CPU 包含一个指令周期 cycle，期间需要进行三个操作。
2. 三个操作分别为：取指 fetch，翻译 decode，执行 execute。

```zig
const std = @import("std");
const Memory = @import("memory.zig").Memory;

pub const CPU = struct {
    pub fn cycle(self: *CPU, memory: *Memory) void {
        self.fetch(memory);
        self.decode();
        self.execute(memory);
    }

    fn fetch(self: *CPU, memory: *Memory) void {
        _ = memory;
        _ = self;
    }

    fn decode(self: *CPU) void {
        _ = self;
    }

    fn execute(self: *CPU, memory: *Memory) void {
        _ = memory;
        _ = self;
    }
};
```

## chip8.zig

1. 定义了 CPU 的频率为 500。
2. 屏幕帧率 FPS 为 60。

```zig
const cpu = @import("cpu.zig");
const memory = @import("memory.zig");
const screen = @import("screen.zig");
const keypad = @import("keypad.zig");

const HZ = 500;
const FPS = 60;

pub const Emulator = struct {
    cpu: cpu.CPU,
    memory: memory.Memory,
    screen: screen.Screen,
    keypad: keypad.Keypad,

    pub fn new() Emulator {
        return Emulator{
            .cpu = cpu.CPU{},
            .memory = memory.Memory{},
            .screen = screen.Screen{},
            .keypad = keypad.Keypad{},
        };
    }

    pub fn run(self: *Emulator) void {
        self.screen.init();
        defer self.screen.deinit();

        var index: usize = 0;
        while (self.keypad.poll()) : (index += 1) {
            for (0..(HZ / FPS)) |_|
                self.cpu.cycle(&self.memory);

            if (index % 44 == 0) self.screen.clear();
            _ = self.screen.setIndex(index);
            self.screen.update(FPS);
        }
    }
};
```

## main.zig

```zig
const std = @import("std");
const chip8 = @import("chip8.zig");

pub fn main() !void {
    var emulator = chip8.Emulator.new();
    emulator.run();
}
```

## 启动

`zig build run`

## 效果

![窗口][1]

## 总结

给新建的 chip8 模拟器新增了 CPU 和内存，定义了频率和帧率。

[1]: images/screen.png

## 附录
