# 0734-sokol-实现清屏

## 目标

实现清屏。

## 环境

- Time 2025-01-26
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

之前写 DirectX 12 写了很多才能实现清屏功能，还是使用库来得简单。

## main.zig

1. `beginPass` 类似 WebGPU 中的 `commandEncoder.beginRenderPass`。
2. `endPass` 类似 `passEncoder.end`。
3. `commit` 类似 `submit`。

```zig
const std = @import("std");
const sk = @import("sokol");

var info: sk.gfx.PassAction = .{};
const clearColor: sk.gfx.Color = .{ .r = 1, .b = 1, .a = 1 };

export fn init() void {
    sk.gfx.setup(.{ .environment = sk.glue.environment() });
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };
}

export fn frame() void {
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

    sk.gfx.endPass();
    sk.gfx.commit();
}

export fn cleanup() void {
    sk.gfx.shutdown();
}

pub fn main() void {
    sk.app.run(.{
        .width = 800,
        .height = 600,
        .window_title = "学习 sokol",
        .init_cb = init,
        .frame_cb = frame,
        .cleanup_cb = cleanup,
    });
}
```

## 效果

![清屏][1]

[1]: images/sokol001.png

## 附录
