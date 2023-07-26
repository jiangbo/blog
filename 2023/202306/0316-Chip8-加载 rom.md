# 0316-Chip8-加载 rom

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

当前使用的测试 rom 来自参考资料的第四个链接。在程序启动的时候，将 rom 加载到内存中。
程序的启动入口在 0x200 处，所以需要将 rom 加载到内存该处。

## screen

代码和之前的无变化，可以参考之前的。

## keypad

代码和之前的无变化，可以参考之前的。

## cpu

代码和之前的无变化，可以参考之前的。

## main

直接将 rom 编译到二进制中。

```zig
const std = @import("std");
const chip8 = @import("chip8.zig");

pub fn main() !void {
    const rom = @embedFile("roms/1-chip8-logo.ch8");
    var emulator = chip8.Emulator.new(rom);
    emulator.run();
}
```

## chip8.zig

1. 定义了程序的入口。
2. 修改了内存的初始化方式，将 rom 传递给内存进行初始化。

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
            .cpu = cpu.CPU{},
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

## memory.zig

1. 将 rom 加载到 0x200 处。

```zig
pub const Memory = struct {
    ram: [4096]u8 = undefined,

    pub fn new(rom: []const u8, entry: u16) Memory {
        var memory = Memory{};
        @memcpy(memory.ram[0..fonts.len], &fonts);
        @memcpy(memory.ram[entry .. entry + rom.len], rom);
        return memory;
    }

    pub fn get(self: *Memory, index: usize) u8 {
        return self.ram[index];
    }

    pub fn set(self: *Memory, index: usize, value: u8) void {
        self.ram[index] = value;
    }
};

const fonts = [_]u8{
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xe0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
};
```

## 启动

`zig build run`

## 效果

![窗口][1]

## 总结

实现了模拟器加载 rom 的逻辑，将其加载到内存的 0x200处。

[1]: ../202305/images/screen.png

## 附录
