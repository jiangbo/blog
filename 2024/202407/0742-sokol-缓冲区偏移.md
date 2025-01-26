# 0742-sokol-缓冲区偏移

## 目标

使用索引画出一个三角形和一个矩形。

## 环境

- Time 2025-01-26
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

一共有三个三角形，类型其它图形 API 的 offset 那种，应该是。

## main.zig

```zig
const std = @import("std");
const sk = @import("sokol");

const shd = @import("shader/test.glsl.zig");

const clearColor: sk.gfx.Color = .{ .a = 1 };
var info: sk.gfx.PassAction = undefined;
var pipeline: sk.gfx.Pipeline = undefined;
var bind: sk.gfx.Bindings = undefined;

const Vertex = extern struct { x: f32, y: f32, r: f32, g: f32, b: f32 };

export fn init() void {
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    bind.vertex_buffers[0] = sk.gfx.makeBuffer(.{
        .data = sk.gfx.asRange(&[_]Vertex{
            // 三角形顶点
            .{ .x = 0.0, .y = 0.55, .r = 1.0, .g = 0.0, .b = 0.0 },
            .{ .x = 0.25, .y = 0.05, .r = 0.0, .g = 1.0, .b = 0.0 },
            .{ .x = -0.25, .y = 0.05, .r = 0.0, .g = 0.0, .b = 1.0 },
            // 四边形顶点
            .{ .x = -0.25, .y = -0.05, .r = 0.0, .g = 0.0, .b = 1.0 },
            .{ .x = 0.25, .y = -0.05, .r = 0.0, .g = 1.0, .b = 0.0 },
            .{ .x = 0.25, .y = -0.55, .r = 1.0, .g = 0.0, .b = 0.0 },
            .{ .x = -0.25, .y = -0.55, .r = 1.0, .g = 1.0, .b = 0.0 },
        }),
    });

    bind.index_buffer = sk.gfx.makeBuffer(.{
        .type = .INDEXBUFFER,
        .data = sk.gfx.asRange(&[_]u16{
            // triangle indices
            0, 1, 2,
            // quad indices
            0, 1, 2,
            0, 2, 3,
        }),
    });

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.attrs[shd.ATTR_test_position].format = .FLOAT2;
            l.attrs[shd.ATTR_test_color0].format = .FLOAT3;
            break :init l;
        },
        .index_type = .UINT16,
    });
}

export fn frame() void {
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });
    sk.gfx.applyPipeline(pipeline);

    // 三角形
    bind.vertex_buffer_offsets[0] = 0;
    bind.index_buffer_offset = 0;
    sk.gfx.applyBindings(bind);
    sk.gfx.draw(0, 3, 1);

    // 四边形
    bind.vertex_buffer_offsets[0] = 3 * @sizeOf(Vertex);
    bind.index_buffer_offset = 3 * @sizeOf(u16);
    sk.gfx.applyBindings(bind);
    sk.gfx.draw(0, 6, 1);

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
        .logger = .{ .func = sk.log.func },
        .init_cb = init,
        .frame_cb = frame,
        .cleanup_cb = cleanup,
    });
}
```

## 效果

![缓冲区偏移][1]

[1]: images/sokol008.png

## 附录
