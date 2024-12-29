# 0685-DirectX9-基本计时器

## 目标

使用 Windows 中的计时器功能。

## 环境

- Time 2024-12-29
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

需要使用高精度计时器。

## timer.zig

```zig
const std = @import("std");
const win32 = @import("win32");

pub const Timer = struct {
    elapsed: f32 = 0.0,
    total: f32 = 0.0,
    start: f32 = 0,
    frequency: f32 = 0.0,

    pub fn init() Timer {
        var timer: Timer = undefined;

        var i: win32.foundation.LARGE_INTEGER = undefined;

        _ = win32.system.performance.QueryPerformanceFrequency(&i);
        timer.frequency = @floatFromInt(i.QuadPart);

        _ = win32.system.performance.QueryPerformanceCounter(&i);
        timer.start = @floatFromInt(i.QuadPart);
        return timer;
    }

    pub fn update(self: *Timer) void {
        var i: win32.foundation.LARGE_INTEGER = undefined;
        _ = win32.system.performance.QueryPerformanceCounter(&i);
        const current: f32 = @floatFromInt(i.QuadPart);
        self.elapsed = (current - self.start) / self.frequency;
        self.total += self.elapsed;
        self.start = current;
    }
};
```

## 附录
