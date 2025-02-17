# 0757-sokol-实例化渲染 Pull

## 目标

不是通过顶点缓冲区实现，通过存储缓冲区实现。

## 环境

- Time 2025-02-17
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

不清楚这两种方式哪种好，也没有看到哪里有对这个的说明。

## main.zig

```zig
const std = @import("std");
const sk = @import("sokol");
const stbi = @import("stbi");
const zm = @import("zmath");

const shd = @import("shader/test.glsl.zig");

const max_particles: usize = 512 * 1024;
const num_particles_emitted_per_frame: usize = 10;

const clearColor: sk.gfx.Color = .{ .r = 0, .g = 0.1, .b = 0.2, .a = 1 };
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

    const r = 0.05;
    bind.storage_buffers[shd.SBUF_vertices] = sk.gfx.makeBuffer(.{
        .type = .STORAGEBUFFER,
        .data = sk.gfx.asRange(&[_]f32{
            0.0, -r,  0.0, 1, 1.0, 0.0, 0.0, 1.0,
            r,   0.0, r,   1, 0.0, 1.0, 0.0, 1.0,
            r,   0.0, -r,  1, 0.0, 0.0, 1.0, 1.0,
            -r,  0.0, -r,  1, 1.0, 1.0, 0.0, 1.0,
            -r,  0.0, r,   1, 0.0, 1.0, 1.0, 1.0,
            0.0, r,   0.0, 1, 1.0, 0.0, 1.0, 1.0,
        }),
    });

    bind.index_buffer = sk.gfx.makeBuffer(.{
        .type = .INDEXBUFFER,
        .data = sk.gfx.asRange(&[_]u16{
            0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 1,
            5, 1, 2, 5, 2, 3, 5, 3, 4, 5, 4, 1,
        }),
    });

    bind.storage_buffers[shd.SBUF_instances] = sk.gfx.makeBuffer(.{
        .type = .STORAGEBUFFER,
        .usage = .STREAM,
        .size = max_particles * @sizeOf(shd.SbInstance),
    });

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .index_type = .UINT16,
        .cull_mode = .BACK,
        .depth = .{
            .compare = .LESS_EQUAL,
            .write_enabled = true,
        },
    });
}

const width = 640;
const height = 480;

var rx: f32 = 0;
var ry: f32 = 0;
var params: shd.VsParams = undefined;
var cur_num_particles: u32 = 0;
var instance: [max_particles]shd.SbInstance = undefined;
var vel: [max_particles]zm.Vec = undefined;
const view = zm.lookAtRh(
    zm.f32x4(0, 1.5, 12, 1.0),
    zm.f32x4(0, 0, 0, 1),
    zm.f32x4(0, 1, 0, 0),
);

export fn frame() void {
    const frame_time: f32 = @floatCast(sk.app.frameDuration());

    for (0..num_particles_emitted_per_frame) |_| {
        if (cur_num_particles < max_particles) {
            instance[cur_num_particles].pos = zm.f32x4s(0);
            vel[cur_num_particles] = zm.f32x4(
                rand(-0.5, 0.5),
                rand(2.0, 2.5),
                rand(-0.5, 0.5),
                1,
            );
            cur_num_particles += 1;
        } else {
            break;
        }
    }

    // update particle positions
    for (0..max_particles) |i| {
        vel[i][1] -= 1.0 * frame_time;
        instance[i].pos[0] += vel[i][0] * frame_time;
        instance[i].pos[1] += vel[i][1] * frame_time;
        instance[i].pos[2] += vel[i][2] * frame_time;

        if (instance[i].pos[1] < -2.0) {
            instance[i].pos[1] = -1.8;
            vel[i][1] = -vel[i][1];
            vel[i][0] *= 0.8;
            vel[i][1] *= 0.8;
            vel[i][2] *= 0.8;
        }
    }

    // update instance data
    sk.gfx.updateBuffer(
        bind.storage_buffers[shd.SBUF_instances],
        sk.gfx.asRange(instance[0..cur_num_particles]),
    );

    // compute vertex shader parameters (the mvp matrix)
    ry += 1.0;
    const vs_params = computeParams(1.0, ry);

    // and finally draw...
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });
    sk.gfx.applyPipeline(pipeline);
    sk.gfx.applyBindings(bind);
    sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&vs_params));
    sk.gfx.draw(0, 24, cur_num_particles);
    sk.gfx.endPass();
    sk.gfx.commit();
}

fn computeParams(r1: f32, r2: f32) shd.VsParams {
    const rxm = zm.rotationX(std.math.degreesToRadians(r1));
    const rym = zm.rotationY(std.math.degreesToRadians(r2));
    const model = zm.mul(rym, rxm);

    const aspect = sk.app.widthf() / sk.app.heightf();
    const proj = zm.perspectiveFovRh(std.math.degreesToRadians(45), aspect, 0.01, 50);

    return shd.VsParams{ .mvp = zm.mul(zm.mul(model, view), proj) };
}

export fn cleanup() void {
    sk.gfx.shutdown();
}

fn rand(min_val: f32, max_val: f32) f32 {
    return (@as(f32, @floatFromInt(xorshift32() & 0xFFFF)) / 0x10000) * (max_val - min_val) + min_val;
}

fn xorshift32() u32 {
    const static = struct {
        var x: u32 = 0x12345678;
    };
    var x = static.x;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    static.x = x;
    return x;
}

pub fn main() void {
    sk.app.run(.{
        .width = width,
        .height = height,
        .window_title = "学习 sokol",
        .logger = .{ .func = sk.log.func },
        .win32_console_attach = true,
        .sample_count = 4,
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
#pragma sokol @ctype vec4 zm.Vec

@vs vs
layout(binding=0) uniform vs_params {
    mat4 mvp;
};

struct sb_vertex {
    vec4 pos;
    vec4 color;
};

struct sb_instance {
    vec4 pos;
};

layout(binding=0) readonly buffer vertices {
    sb_vertex vtx[];
};

layout(binding=1) readonly buffer instances {
    sb_instance inst[];
};

out vec4 color;

void main() {
    const vec4 pos = vtx[gl_VertexIndex].pos + inst[gl_InstanceIndex].pos;
    gl_Position = mvp * pos;
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

![实例化渲染-pull][1]

[1]: images/sokol023.webp

## 附录
