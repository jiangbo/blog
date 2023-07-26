# 0318-Chip8-CPU 结构定义

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

定义 CPU 的结构和实现部分功能。Chip-8 的 CPU 含有 16 个 8 位的寄存器，一个索引寄存器，一个程序计数器。

## chip8.zig

程序启动时，将 CPU 的程序计数器指向入口地址。

```zig
const cpu = @import("cpu.zig");
const memory = @import("memory.zig");
const screen = @import("screen.zig");
const keypad = @import("keypad.zig");

const ENTRY = 0x200;
const HZ = 500;
const FPS = 60;

pub const Emulator = struct {
    cpu: cpu.CPU,
    memory: memory.Memory,
    screen: screen.Screen,
    keypad: keypad.Keypad,

    pub fn new(rom: []const u8) Emulator {
        return Emulator{
            .cpu = cpu.CPU{ .pc = ENTRY },
            .memory = memory.Memory.new(rom, ENTRY),
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

## cpu

1. 将当前正在运行的指令 instruct 保存到 CPU 中。
2. 16 个 8 位的寄存器 register。
3. 索引寄存器 index。
4. 程序计数器 pc。
5. 实现了取指方法 fetch，因为一次取两个字节，所以 pc 每次加 2。
6. 实现译码 decode 方法。

```zig
const std = @import("std");
const Instruct = @import("instruct.zig").Instruct;
const Memory = @import("memory.zig").Memory;

pub const CPU = struct {
    instruct: Instruct = undefined,
    register: [16]u8 = std.mem.zeroes([16]u8),
    index: u16 = 0,
    pc: u16,

    pub fn cycle(self: *CPU, memory: *Memory) void {
        self.fetch(memory);
        self.decode();
        self.execute(memory);
    }

    fn fetch(self: *CPU, memory: *Memory) void {
        var opcode = memory.load(self.pc);
        self.instruct = Instruct{ .opcode = opcode };
        self.next();
    }

    fn next(self: *CPU) void {
        self.pc += 2;
    }

    fn decode(self: *CPU) void {
        self.instruct.decode();
    }

    fn execute(self: *CPU, memory: *Memory) void {
        _ = memory;
        _ = self;
    }
};
```

## 启动

`zig build run`

## 效果

![窗口][1]

## 总结

定义了 CPU 的结构，并且实现了 CPU 的初始化，取指和译码两个过程。

[1]: ../202305/images/screen.png

## 附录
