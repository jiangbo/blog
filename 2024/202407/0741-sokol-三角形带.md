# 0741-sokol-三角形带

## 目标

使用三角形带，不使用索引，也可以矩形。

## 环境

- Time 2025-01-26
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

感觉颜色的插值和之前不太一样，不清楚为什么很少人使用三角形带，感觉这个不用传递索引，更高效一点吧。

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
            -0.5, -0.5, 1.0, 1.0, 0.0, 1.0,
            0.5,  -0.5, 0.0, 0.0, 1.0, 1.0,
        }),
    });

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.attrs[shd.ATTR_test_position].format = .FLOAT2;
            l.attrs[shd.ATTR_test_color0].format = .FLOAT4;
            break :init l;
        },
        .primitive_type = .TRIANGLE_STRIP,
    });
}

export fn frame() void {
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

    sk.gfx.applyPipeline(pipeline);
    sk.gfx.applyBindings(bind);
    sk.gfx.draw(0, 4, 1);

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

![三角形带][1]

[1]: images/sokol007.png

## 附录
