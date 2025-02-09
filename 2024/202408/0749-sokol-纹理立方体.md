# 0749-sokol-纹理立方体

## 目标

在之前旋转立方体的基础上，增加纹理。

## 环境

- Time 2025-02-09
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

之前处理过图片纹理，参考一下之前的代码，拷贝过来直接用。

## main.zig

增加了纹理相关数据，定义了顶点的结构体。

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

const Vertex = struct { x: f32, y: f32, z: f32, color: u32, u: u16, v: u16 };

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
        .data = sk.gfx.asRange(&[_]Vertex{
            .{ .x = -1.0, .y = -1.0, .z = -1.0, .color = 0xFF0000FF, .u = 0, .v = 0 },
            .{ .x = 1.0, .y = -1.0, .z = -1.0, .color = 0xFF0000FF, .u = 32767, .v = 0 },
            .{ .x = 1.0, .y = 1.0, .z = -1.0, .color = 0xFF0000FF, .u = 32767, .v = 32767 },
            .{ .x = -1.0, .y = 1.0, .z = -1.0, .color = 0xFF0000FF, .u = 0, .v = 32767 },
            .{ .x = -1.0, .y = -1.0, .z = 1.0, .color = 0xFF00FF00, .u = 0, .v = 0 },
            .{ .x = 1.0, .y = -1.0, .z = 1.0, .color = 0xFF00FF00, .u = 32767, .v = 0 },
            .{ .x = 1.0, .y = 1.0, .z = 1.0, .color = 0xFF00FF00, .u = 32767, .v = 32767 },
            .{ .x = -1.0, .y = 1.0, .z = 1.0, .color = 0xFF00FF00, .u = 0, .v = 32767 },
            .{ .x = -1.0, .y = -1.0, .z = -1.0, .color = 0xFFFF0000, .u = 0, .v = 0 },
            .{ .x = -1.0, .y = 1.0, .z = -1.0, .color = 0xFFFF0000, .u = 32767, .v = 0 },
            .{ .x = -1.0, .y = 1.0, .z = 1.0, .color = 0xFFFF0000, .u = 32767, .v = 32767 },
            .{ .x = -1.0, .y = -1.0, .z = 1.0, .color = 0xFFFF0000, .u = 0, .v = 32767 },
            .{ .x = 1.0, .y = -1.0, .z = -1.0, .color = 0xFFFF007F, .u = 0, .v = 0 },
            .{ .x = 1.0, .y = 1.0, .z = -1.0, .color = 0xFFFF007F, .u = 32767, .v = 0 },
            .{ .x = 1.0, .y = 1.0, .z = 1.0, .color = 0xFFFF007F, .u = 32767, .v = 32767 },
            .{ .x = 1.0, .y = -1.0, .z = 1.0, .color = 0xFFFF007F, .u = 0, .v = 32767 },
            .{ .x = -1.0, .y = -1.0, .z = -1.0, .color = 0xFFFF7F00, .u = 0, .v = 0 },
            .{ .x = -1.0, .y = -1.0, .z = 1.0, .color = 0xFFFF7F00, .u = 32767, .v = 0 },
            .{ .x = 1.0, .y = -1.0, .z = 1.0, .color = 0xFFFF7F00, .u = 32767, .v = 32767 },
            .{ .x = 1.0, .y = -1.0, .z = -1.0, .color = 0xFFFF7F00, .u = 0, .v = 32767 },
            .{ .x = -1.0, .y = 1.0, .z = -1.0, .color = 0xFF007FFF, .u = 0, .v = 0 },
            .{ .x = -1.0, .y = 1.0, .z = 1.0, .color = 0xFF007FFF, .u = 32767, .v = 0 },
            .{ .x = 1.0, .y = 1.0, .z = 1.0, .color = 0xFF007FFF, .u = 32767, .v = 32767 },
            .{ .x = 1.0, .y = 1.0, .z = -1.0, .color = 0xFF007FFF, .u = 0, .v = 32767 },
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

    // create a checkerboard texture
    const pixels: [4 * 4]u32 = .{
        0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF, 0xFF000000,
        0xFF000000, 0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF, 0xFF000000,
        0xFF000000, 0xFFFFFFFF, 0xFF000000, 0xFFFFFFFF,
    };

    // NOTE: SLOT_tex is provided by shader code generation
    bind.images[shd.IMG_tex] = sk.gfx.makeImage(.{
        .width = 4,
        .height = 4,
        .data = init: {
            var data = sk.gfx.ImageData{};
            data.subimage[0][0] = sk.gfx.asRange(&pixels);
            break :init data;
        },
    });

    // create a sampler object with default attributes
    bind.samplers[shd.SMP_smp] = sk.gfx.makeSampler(.{});

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.attrs[shd.ATTR_test_position].format = .FLOAT3;
            l.attrs[shd.ATTR_test_color0].format = .UBYTE4N;
            l.attrs[shd.ATTR_test_texcoord0].format = .SHORT2N;
            break :init l;
        },
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

in vec4 position;
in vec4 color0;
in vec2 texcoord0;

out vec4 color;
out vec2 uv;

void main() {
    gl_Position = vp * position;
    color = color0;
    uv = texcoord0 * 5.0;
}
@end

@fs fs
layout(binding=0) uniform texture2D tex;
layout(binding=0) uniform sampler smp;

in vec4 color;
in vec2 uv;
out vec4 frag_color;

void main() {
    frag_color = texture(sampler2D(tex,smp), uv) * color;
}
@end

@program test vs fs
```

## 效果

![纹理立方体][1]

[1]: images/sokol015.webp

## 附录
