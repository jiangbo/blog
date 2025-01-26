# 0739-sokol-无缓冲三角形

## 目标

不使用缓冲区，直接在着色器中定义顶点和颜色数据。

## 环境

- Time 2025-01-26
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

之前就想先不使用缓冲区的，结果没有成功，刚好看到有一个例子是这个，现在增加进来。

## test.glsl

```glsl
@vs vs
const vec4 colors[3] = {
    vec4(1, 1, 0, 1),
    vec4(0, 1, 1, 1),
    vec4(1, 0, 1, 1),
};
const vec3 positions[3] = {
    vec3(0.0, 0.5, 0.5),
    vec3(0.5, -0.5, 0.5),
    vec3(-0.5, -0.5, 0.5),
};

out vec4 color;

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 1.0);
    color = colors[gl_VertexIndex];
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

export fn init() void {
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
    });
}

export fn frame() void {
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

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
        .logger = .{ .func = sk.log.func },
        .init_cb = init,
        .frame_cb = frame,
        .cleanup_cb = cleanup,
    });
}
```

## 效果

![无缓冲三角形][1]

[1]: images/sokol005.png

## 附录
