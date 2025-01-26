# 0736-sokol-显示三角形

## 目标

使用 sokol 显示一个三角形。

## 环境

- Time 2025-01-26
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

从代码量上来说，比其它的图形 API 看起来简单，简单的对立面就是不灵活。

## test.glsl

关于不同平台的着色器转换，可以看上一篇。

```glsl
@vs vs
in vec4 position;

void main() {
    gl_Position = position;
}
@end

@fs fs
out vec4 frag_color;

void main() {
    frag_color = vec4(1.0, 0.0, 0.0, 1.0);
}
@end

@program test vs fs
```

## main.zig

增加了 pipeline 和 bindings。

```zig
const std = @import("std");
const sk = @import("sokol");

const shd = @import("shader/test.glsl.zig");

const clearColor: sk.gfx.Color = .{ .a = 1 };
var info: sk.gfx.PassAction = undefined;
var pipeline: sk.gfx.Pipeline = undefined;
var bind: sk.gfx.Bindings = undefined;

export fn init() void {
    sk.gfx.setup(.{ .environment = sk.glue.environment() });
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    bind.vertex_buffers[0] = sk.gfx.makeBuffer(.{
        .data = sk.gfx.asRange(&[_]f32{ 0.0, 0.5, 0.5, -0.5, -0.5, -0.5 }),
    });

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.attrs[shd.ATTR_test_position].format = .FLOAT2;
            break :init l;
        },
    });
}

export fn frame() void {
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

    sk.gfx.applyBindings(bind);
    sk.gfx.applyPipeline(pipeline);
    sk.gfx.draw(0, 3, 1);

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

![三角形][1]

[1]: images/sokol002.png

## 附录
