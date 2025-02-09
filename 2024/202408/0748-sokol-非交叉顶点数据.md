# 0748-sokol-非交叉顶点数据

## 目标

之前渲染的立方体，顶点数据是位置和颜色放一起，每个进行了交叉，现在将所有的位置放一起，所有颜色放一起。

## 环境

- Time 2025-02-09
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

在之前的基础上修改的话，改动不大，不清楚这种有什么用，先了解着。

## main.zig

位置和颜色没有进行交叉。

```zig
const std = @import("std");
const sk = @import("sokol");
const stbi = @import("stbi");
const zm = @import("zmath");

const vec3 = @import("math.zig").Vec3;
const mat4 = @import("math.zig").Mat4;

const shd = @import("shader/test.glsl.zig");

const clearColor: sk.gfx.Color = .{ .r = 1, .b = 1, .a = 1 };
var info: sk.gfx.PassAction = undefined;
var pipeline: sk.gfx.Pipeline = undefined;
var bind: sk.gfx.Bindings = undefined;

export fn init() void {

    // 设置初始化环境
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });

    // 背景清除颜色
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    // 顶点数据
    bind.vertex_buffers[0] = sk.gfx.makeBuffer(.{
        .data = sk.gfx.asRange(&[_]f32{
            // 位置
            -1.0, -1.0, -1.0, 1.0,  -1.0, -1.0, 1.0,  1.0,  -1.0, -1.0, 1.0,  -1.0,
            -1.0, -1.0, 1.0,  1.0,  -1.0, 1.0,  1.0,  1.0,  1.0,  -1.0, 1.0,  1.0,
            -1.0, -1.0, -1.0, -1.0, 1.0,  -1.0, -1.0, 1.0,  1.0,  -1.0, -1.0, 1.0,
            1.0,  -1.0, -1.0, 1.0,  1.0,  -1.0, 1.0,  1.0,  1.0,  1.0,  -1.0, 1.0,
            -1.0, -1.0, -1.0, -1.0, -1.0, 1.0,  1.0,  -1.0, 1.0,  1.0,  -1.0, -1.0,
            -1.0, 1.0,  -1.0, -1.0, 1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  -1.0,
            // 颜色
            1.0,  0.5,  0.0,  1.0,  1.0,  0.5,  0.0,  1.0,  1.0,  0.5,  0.0,  1.0,
            1.0,  0.5,  0.0,  1.0,  0.5,  1.0,  0.0,  1.0,  0.5,  1.0,  0.0,  1.0,
            0.5,  1.0,  0.0,  1.0,  0.5,  1.0,  0.0,  1.0,  0.5,  0.0,  1.0,  1.0,
            0.5,  0.0,  1.0,  1.0,  0.5,  0.0,  1.0,  1.0,  0.5,  0.0,  1.0,  1.0,
            1.0,  0.5,  1.0,  1.0,  1.0,  0.5,  1.0,  1.0,  1.0,  0.5,  1.0,  1.0,
            1.0,  0.5,  1.0,  1.0,  0.5,  1.0,  1.0,  1.0,  0.5,  1.0,  1.0,  1.0,
            0.5,  1.0,  1.0,  1.0,  0.5,  1.0,  1.0,  1.0,  1.0,  1.0,  0.5,  1.0,
            1.0,  1.0,  0.5,  1.0,  1.0,  1.0,  0.5,  1.0,  1.0,  1.0,  0.5,  1.0,
        }),
    });

    // 索引数据
    bind.index_buffer = sk.gfx.makeBuffer(.{
        .type = .INDEXBUFFER,
        .data = sk.gfx.asRange(&[_]u16{
            0,  1,  2,  0,  2,  3,
            6,  5,  4,  7,  6,  4,
            8,  9,  10, 8,  10, 11,
            14, 13, 12, 15, 14, 12,
            16, 17, 18, 16, 18, 19,
            22, 21, 20, 23, 22, 20,
        }),
    });

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.attrs[shd.ATTR_test_position] = .{ .format = .FLOAT3, .buffer_index = 0 };
            l.attrs[shd.ATTR_test_color0] = .{ .format = .FLOAT4, .buffer_index = 1 };
            break :init l;
        },
        .index_type = .UINT16,
        .depth = .{
            .compare = .LESS_EQUAL,
            .write_enabled = true,
        },
        .cull_mode = .BACK,
    });

    bind.vertex_buffers[1] = bind.vertex_buffers[0];
    // position vertex components are at the start of the buffer
    bind.vertex_buffer_offsets[0] = 0;
    // color vertex components follow after the positions
    bind.vertex_buffer_offsets[1] = 24 * 3 * @sizeOf(f32);
}

const width = 800;
const height = 600;

var rx: f32 = 0;
var ry: f32 = 0;
const view: zm.Mat = zm.lookAtRh(
    zm.f32x4(0, 1.5, 6, 1.0), // eye position
    zm.f32x4(0.0, 0.0, 0.0, 1.0), // focus point
    zm.f32x4(0.0, 1.0, 0.0, 0.0), // up direction
);

export fn frame() void {
    const dt: f32 = @floatCast(sk.app.frameDuration() * 60);
    rx += 1.0 * dt;
    ry += 2.0 * dt;
    const params = computeParams();

    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

    sk.gfx.applyPipeline(pipeline);
    sk.gfx.applyBindings(bind);

    sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&params));
    sk.gfx.draw(0, 36, 1);

    sk.gfx.endPass();
    sk.gfx.commit();
}

fn computeParams() shd.VsParams {
    const rxm = zm.rotationX(std.math.degreesToRadians(rx));
    const rym = zm.rotationY(std.math.degreesToRadians(ry));
    const model = zm.mul(rym, rxm);

    const aspect = sk.app.widthf() / sk.app.heightf();
    const radians = std.math.degreesToRadians(60.0);
    const proj = zm.perspectiveFovRh(radians, aspect, 0.01, 10.0);

    return shd.VsParams{ .vp = zm.mul(zm.mul(model, view), proj) };
}

export fn cleanup() void {
    sk.gfx.shutdown();
}

pub fn main() void {
    sk.app.run(.{
        .width = width,
        .height = height,
        .window_title = "学习 sokol",
        .logger = .{ .func = sk.log.func },
        .win32_console_attach = true,
        .init_cb = init,
        .frame_cb = frame,
        .cleanup_cb = cleanup,
    });
}
```

## test.glsl

```glsl
#pragma sokol @header const zm = @import("zmath")
#pragma sokol @ctype mat4 zm.Mat

@vs vs
layout(binding=0) uniform vs_params {
    mat4 vp;
};

in vec4 position;
in vec4 color0;

out vec4 color;

void main() {
    gl_Position = vp * position;
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

## 效果

![非交叉顶点][1]

[1]: images/sokol014.webp

## 附录
