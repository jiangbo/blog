# 0756-sokol-实例化渲染

## 目标

实例化渲染，适合渲染很多相同的物体。

## 环境

- Time 2025-02-17
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

之前学习 OpenGL 的时候，没有学习这个，在这里了解一下。

## main.zig

```zig
const std = @import("std");
const sk = @import("sokol");
const stbi = @import("stbi");
const zm = @import("zmath");

const shd = @import("shader/test.glsl.zig");

const max_particles: usize = 512 * 1024;
const num_particles_emitted_per_frame: usize = 10;

pub const Vec3 = extern struct {
    x: f32,
    y: f32,
    z: f32,

    pub fn zero() Vec3 {
        return Vec3{ .x = 0.0, .y = 0.0, .z = 0.0 };
    }

    pub fn new(x: f32, y: f32, z: f32) Vec3 {
        return Vec3{ .x = x, .y = y, .z = z };
    }

    pub fn up() Vec3 {
        return Vec3{ .x = 0.0, .y = 1.0, .z = 0.0 };
    }

    pub fn len(v: Vec3) f32 {
        return std.math.sqrt(Vec3.dot(v, v));
    }

    pub fn add(left: Vec3, right: Vec3) Vec3 {
        return Vec3{ .x = left.x + right.x, .y = left.y + right.y, .z = left.z + right.z };
    }

    pub fn sub(left: Vec3, right: Vec3) Vec3 {
        return Vec3{ .x = left.x - right.x, .y = left.y - right.y, .z = left.z - right.z };
    }

    pub fn mul(v: Vec3, s: f32) Vec3 {
        return Vec3{ .x = v.x * s, .y = v.y * s, .z = v.z * s };
    }

    pub fn norm(v: Vec3) Vec3 {
        const l = Vec3.len(v);
        if (l != 0.0) {
            return Vec3{ .x = v.x / l, .y = v.y / l, .z = v.z / l };
        } else {
            return Vec3.zero();
        }
    }

    pub fn cross(v0: Vec3, v1: Vec3) Vec3 {
        return Vec3{ .x = (v0.y * v1.z) - (v0.z * v1.y), .y = (v0.z * v1.x) - (v0.x * v1.z), .z = (v0.x * v1.y) - (v0.y * v1.x) };
    }

    pub fn dot(v0: Vec3, v1: Vec3) f32 {
        return v0.x * v1.x + v0.y * v1.y + v0.z * v1.z;
    }
};

const clearColor: sk.gfx.Color = .{ .r = 0, .g = 0, .b = 0, .a = 1 };
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
    bind.vertex_buffers[0] = sk.gfx.makeBuffer(.{
        .data = sk.gfx.asRange(&[_]f32{
            0.0, -r,  0.0, 1.0, 0.0, 0.0, 1.0,
            r,   0.0, r,   0.0, 1.0, 0.0, 1.0,
            r,   0.0, -r,  0.0, 0.0, 1.0, 1.0,
            -r,  0.0, -r,  1.0, 1.0, 0.0, 1.0,
            -r,  0.0, r,   0.0, 1.0, 1.0, 1.0,
            0.0, r,   0.0, 1.0, 0.0, 1.0, 1.0,
        }),
    });

    bind.index_buffer = sk.gfx.makeBuffer(.{
        .type = .INDEXBUFFER,
        .data = sk.gfx.asRange(&[_]u16{
            2, 1, 0, 3, 2, 0,
            4, 3, 0, 1, 4, 0,
            5, 1, 2, 5, 2, 3,
            5, 3, 4, 5, 4, 1,
        }),
    });

    bind.vertex_buffers[1] = sk.gfx.makeBuffer(.{
        .usage = .STREAM,
        .size = max_particles * @sizeOf(Vec3),
    });

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.buffers[1].step_func = .PER_INSTANCE;
            l.attrs[shd.ATTR_test_pos] = .{ .format = .FLOAT3, .buffer_index = 0 };
            l.attrs[shd.ATTR_test_color0] = .{ .format = .FLOAT4, .buffer_index = 0 };
            l.attrs[shd.ATTR_test_inst_pos] = .{ .format = .FLOAT3, .buffer_index = 1 };
            break :init l;
        },
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
var pos: [max_particles]Vec3 = undefined;
var vel: [max_particles]Vec3 = undefined;
const view = zm.lookAtRh(
    zm.f32x4(0, 1.5, 12, 1.0),
    zm.f32x4(0, 0, 0, 1),
    zm.f32x4(0, 1, 0, 0),
);

export fn frame() void {
    const frame_time: f32 = @floatCast(sk.app.frameDuration());

    for (0..num_particles_emitted_per_frame) |_| {
        if (cur_num_particles < max_particles) {
            pos[cur_num_particles] = .{ .x = 0, .y = 0, .z = 0 };
            vel[cur_num_particles] = .{
                .x = rand(-0.5, 0.5),
                .y = rand(2.0, 2.5),
                .z = rand(-0.5, 0.5),
            };
            cur_num_particles += 1;
        } else {
            break;
        }
    }

    // update particle positions
    for (0..max_particles) |i| {
        const v = &vel[i];
        const p = &pos[i];
        v.y -= 1.0 * frame_time;
        p.* = Vec3.add(p.*, Vec3.mul(v.*, frame_time));
        if (p.y < -2.0) {
            p.y = -1.8;
            v.y = -v.y;
            v.* = Vec3.mul(v.*, 0.8);
        }
    }

    // update instance data
    sk.gfx.updateBuffer(bind.vertex_buffers[1], sk.gfx.asRange(pos[0..cur_num_particles]));

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

@vs vs
layout(binding=0) uniform vs_params {
    mat4 mvp;
};

in vec3 pos;
in vec4 color0;
in vec3 inst_pos;

out vec4 color;

void main() {
    vec4 pos = vec4(pos + inst_pos, 1.0);
    gl_Position = mvp * pos;
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

![实例化渲染][1]

[1]: images/sokol022.webp

## 附录
