# 0750-sokol-存储缓冲区

## 目标

之前使用的顶点缓冲区来实现的旋转立方体，这里替换成了存储缓冲区。

## 环境

- Time 2025-02-09
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

存储缓冲区由自己定义，顶点缓冲区还需要显式指定格式，存储缓冲区感觉简单一些。

## main.zig

将顶点缓冲区替换成了存储缓冲区。

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
    bind.storage_buffers[0] = sk.gfx.makeBuffer(.{
        .type = .STORAGEBUFFER,
        .data = sk.gfx.asRange(&[_]shd.SbVertex{
            .{ .pos = .{ -1.0, -1.0, -1.0 }, .color = .{ 1.0, 0.0, 0.0, 1.0 } },
            .{ .pos = .{ 1.0, -1.0, -1.0 }, .color = .{ 1.0, 0.0, 0.0, 1.0 } },
            .{ .pos = .{ 1.0, 1.0, -1.0 }, .color = .{ 1.0, 0.0, 0.0, 1.0 } },
            .{ .pos = .{ -1.0, 1.0, -1.0 }, .color = .{ 1.0, 0.0, 0.0, 1.0 } },
            .{ .pos = .{ -1.0, -1.0, 1.0 }, .color = .{ 0.0, 1.0, 0.0, 1.0 } },
            .{ .pos = .{ 1.0, -1.0, 1.0 }, .color = .{ 0.0, 1.0, 0.0, 1.0 } },
            .{ .pos = .{ 1.0, 1.0, 1.0 }, .color = .{ 0.0, 1.0, 0.0, 1.0 } },
            .{ .pos = .{ -1.0, 1.0, 1.0 }, .color = .{ 0.0, 1.0, 0.0, 1.0 } },
            .{ .pos = .{ -1.0, -1.0, -1.0 }, .color = .{ 0.0, 0.0, 1.0, 1.0 } },
            .{ .pos = .{ -1.0, 1.0, -1.0 }, .color = .{ 0.0, 0.0, 1.0, 1.0 } },
            .{ .pos = .{ -1.0, 1.0, 1.0 }, .color = .{ 0.0, 0.0, 1.0, 1.0 } },
            .{ .pos = .{ -1.0, -1.0, 1.0 }, .color = .{ 0.0, 0.0, 1.0, 1.0 } },
            .{ .pos = .{ 1.0, -1.0, -1.0 }, .color = .{ 1.0, 0.5, 0.0, 1.0 } },
            .{ .pos = .{ 1.0, 1.0, -1.0 }, .color = .{ 1.0, 0.5, 0.0, 1.0 } },
            .{ .pos = .{ 1.0, 1.0, 1.0 }, .color = .{ 1.0, 0.5, 0.0, 1.0 } },
            .{ .pos = .{ 1.0, -1.0, 1.0 }, .color = .{ 1.0, 0.5, 0.0, 1.0 } },
            .{ .pos = .{ -1.0, -1.0, -1.0 }, .color = .{ 0.0, 0.5, 1.0, 1.0 } },
            .{ .pos = .{ -1.0, -1.0, 1.0 }, .color = .{ 0.0, 0.5, 1.0, 1.0 } },
            .{ .pos = .{ 1.0, -1.0, 1.0 }, .color = .{ 0.0, 0.5, 1.0, 1.0 } },
            .{ .pos = .{ 1.0, -1.0, -1.0 }, .color = .{ 0.0, 0.5, 1.0, 1.0 } },
            .{ .pos = .{ -1.0, 1.0, -1.0 }, .color = .{ 1.0, 0.0, 0.5, 1.0 } },
            .{ .pos = .{ -1.0, 1.0, 1.0 }, .color = .{ 1.0, 0.0, 0.5, 1.0 } },
            .{ .pos = .{ 1.0, 1.0, 1.0 }, .color = .{ 1.0, 0.0, 0.5, 1.0 } },
            .{ .pos = .{ 1.0, 1.0, -1.0 }, .color = .{ 1.0, 0.0, 0.5, 1.0 } },
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
        .index_type = .UINT16,
        .depth = .{
            .compare = .LESS_EQUAL,
            .write_enabled = true,
        },
        .cull_mode = .BACK,
    });
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

struct sb_vertex {
    vec3 pos;
    vec4 color;
};

layout(binding=0) readonly buffer ssbo {
    sb_vertex vtx[];
};

out vec4 color;

void main() {
    gl_Position = vp * vec4(vtx[gl_VertexIndex].pos, 1.0);
    color = vtx[gl_VertexIndex].color;
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

![存储缓冲区][1]

[1]: images/sokol016.webp

## 附录
