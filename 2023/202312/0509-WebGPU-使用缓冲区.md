# 0509-WebGPU-使用缓冲区

## 环境

- Time 2024-05-11
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://eliemichel.github.io/LearnWebGPU/index.html>

### 目标

创建了两个 GPU 缓冲区，将数据从一个缓冲区复制到另一个缓冲区。

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
// renderPipeline: *mach.gpu.RenderPipeline,

const numbers: [16]u8 = .{ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 };

pub fn init(_: *App) !void {

    // 定义了窗口的宽和高，以及窗口的标题
    try mach.core.init(.{
        .size = .{ .width = 800, .height = 600 },
        .title = "学习 WebGPU",
    });

    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);
    const device = mach.core.device;

    // 创建 buffer1
    const buffer1 = device.createBuffer(&.{
        .label = "buffer1",
        .usage = .{ .copy_dst = true, .copy_src = true },
        .size = numbers.len,
    });
    defer buffer1.release();

    // 创建 buffer2
    const buffer2 = device.createBuffer(&.{
        .label = "buffer2",
        .usage = .{ .copy_dst = true, .map_read = true },
        .size = numbers.len,
    });
    defer buffer2.release();

    // 将 CPU 内存中的数据复制到 GPU 内存中
    mach.core.queue.writeBuffer(buffer1, 0, &numbers);

    const encoder = mach.core.device.createCommandEncoder(null);
    // 将 GPU 中 buffer1 的数据复制到 buffer2 中
    encoder.copyBufferToBuffer(buffer1, 0, buffer2, 0, numbers.len);
    const command = encoder.finish(null);
    mach.core.queue.submit(&.{command});
    command.release();
    encoder.release();

    // 从 buffer2 中异步读取数据
    const Buffer = mach.gpu.Buffer;
    buffer2.mapAsync(.{ .read = true }, 0, numbers.len, buffer2, struct {
        inline fn callback(buffer: *Buffer, status: Buffer.MapAsyncStatus) void {
            if (status == .success) {
                const data = buffer.getConstMappedRange(u8, 0, numbers.len).?;
                std.debug.print("data: {any}", .{data});
            }
            buffer.unmap();
        }
    }.callback);
}

pub fn deinit(_: *App) void {
    defer _ = gpa.deinit();
    defer mach.core.deinit();
}

pub fn update(_: *App) !bool {

    // 检查窗口是否需要关闭
    var iterator = mach.core.pollEvents();
    while (iterator.next()) |event| if (event == .close) return true;

    // 不退出渲染循环
    return false;
}
```

## 效果

```text
data: { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16 }
```

## 总结

创建了两个 GPU 缓冲区，然后进行复制，可以从目的缓冲区中得到写入的数据。

## 附录
