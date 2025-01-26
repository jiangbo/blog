# 0740-sokol-索引缓冲区

## 目标

定义四个顶点，然后通过索引来绘制一个矩形。

## 环境

- Time 2025-01-26
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

这种之前也练习过了，应该不难。

## test.glsl

```glsl
@vs vs
in vec4 position;
in vec4 color0;

out vec4 color;

void main() {
    gl_Position = position;
    color = color0;
}
@end

@fs fs
in vec4 color;
out vec4 frag_color;

void main() {
    frag_color = color;
}
@end

@program test vs fs
```

## main.zig

```zig
const std = @import("std");
const sk = @import("sokol");

const shd = @import("shader/test.glsl.zig");

const clearColor: sk.gfx.Color = .{ .a = 1 };
var info: sk.gfx.PassAction = undefined;
var pipeline: sk.gfx.Pipeline = undefined;
var bind: sk.gfx.Bindings = undefined;

export fn init() void {
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    bind.vertex_buffers[0] = sk.gfx.makeBuffer(.{
        .data = sk.gfx.asRange(&[_]f32{
            // 顶点和颜色
            -0.5, 0.5,  1.0, 0.0, 0.0, 1.0,
            0.5,  0.5,  0.0, 1.0, 0.0, 1.0,
            0.5,  -0.5, 0.0, 0.0, 1.0, 1.0,
            -0.5, -0.5, 1.0, 1.0, 0.0, 1.0,
        }),
    });

    bind.index_buffer = sk.gfx.makeBuffer(.{
        .type = .INDEXBUFFER,
        .data = sk.gfx.asRange(&[_]u16{ 0, 1, 2, 0, 2, 3 }),
    });

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.attrs[shd.ATTR_test_position].format = .FLOAT2;
            l.attrs[shd.ATTR_test_color0].format = .FLOAT4;
            break :init l;
        },
        .index_type = .UINT16,
    });
}

export fn frame() void {
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

    sk.gfx.applyPipeline(pipeline);
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

![索引缓冲区][1]

[1]: images/sokol006.png

## 附录
