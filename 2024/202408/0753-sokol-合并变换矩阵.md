# 0753-sokol-合并变换矩阵

## 目标

之前针对每个形状都需要一次 draw 调用，现在将所有形状的变换合并到一起。

## 环境

- Time 2025-02-12
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

好像有点批量渲染的意思。

## main.zig

```zig
const std = @import("std");
const sk = @import("sokol");
const stbi = @import("stbi");
const zm = @import("zmath");

const shd = @import("shader/test.glsl.zig");

const clearColor: sk.gfx.Color = .{ .r = 1, .b = 1, .a = 1 };
var info: sk.gfx.PassAction = undefined;
var pipeline: sk.gfx.Pipeline = undefined;
var bind: sk.gfx.Bindings = undefined;

const Vec3 = struct { x: f32 = 0, y: f32 = 0, z: f32 = 0 };

const Shape = struct {
    pos: Vec3 = .{},
    draw: sk.shape.ElementRange = .{},
};

var shapes: [5]Shape = .{
    .{ .pos = .{ .x = -1, .y = 1, .z = 0 } },
    .{ .pos = .{ .x = 1, .y = 1, .z = 0 } },
    .{ .pos = .{ .x = -2, .y = -1, .z = 0 } },
    .{ .pos = .{ .x = 2, .y = -1, .z = 0 } },
    .{ .pos = .{ .x = 0, .y = -1, .z = 0 } },
};

export fn init() void {

    // 设置初始化环境
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });

    // 背景清除颜色
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.buffers[0] = sk.shape.vertexBufferLayoutState();
            l.attrs[shd.ATTR_test_position] = sk.shape.positionVertexAttrState();
            l.attrs[shd.ATTR_test_normal] = sk.shape.normalVertexAttrState();
            l.attrs[shd.ATTR_test_texcoord] = sk.shape.texcoordVertexAttrState();
            l.attrs[shd.ATTR_test_color0] = sk.shape.colorVertexAttrState();
            break :init l;
        },
        .index_type = .UINT16,
        .depth = .{
            .compare = .LESS_EQUAL,
            .write_enabled = true,
        },
        .cull_mode = .NONE,
    });

    // 生成形状
    var vertices: [6 * 1024]sk.shape.Vertex = undefined;
    var indices: [16 * 1024]u16 = undefined;
    var buf: sk.shape.Buffer = .{
        .vertices = .{ .buffer = sk.shape.asRange(&vertices) },
        .indices = .{ .buffer = sk.shape.asRange(&indices) },
    };

    // 变换矩阵

    const box_transform = zm.translation(-1.0, 0.0, 1.0);
    const sphere_transform = zm.translation(1.0, 0.0, 1.0);
    const cylinder_transform = zm.translation(-1.0, 0.0, -1.0);
    const torus_transform = zm.translation(1.0, 0.0, -1.0);

    buf = sk.shape.buildBox(buf, .{
        .width = 1.0,
        .height = 1.0,
        .depth = 1.0,
        .tiles = 10,
        .random_colors = true,
        .transform = toMat4(box_transform),
    });

    buf = sk.shape.buildSphere(buf, .{
        .merge = true,
        .radius = 0.75,
        .slices = 36,
        .stacks = 20,
        .random_colors = true,
        .transform = toMat4(sphere_transform),
    });

    buf = sk.shape.buildCylinder(buf, .{
        .merge = true,
        .radius = 0.5,
        .height = 1.5,
        .slices = 36,
        .stacks = 10,
        .random_colors = true,
        .transform = toMat4(cylinder_transform),
    });

    buf = sk.shape.buildTorus(buf, .{
        .merge = true,
        .radius = 0.5,
        .ring_radius = 0.3,
        .rings = 36,
        .sides = 18,
        .random_colors = true,
        .transform = toMat4(torus_transform),
    });
    std.debug.assert(buf.valid);
    elementRange = sk.shape.elementRange(buf);

    // one vertex- and index-buffer for all shapes
    bind.vertex_buffers[0] = sk.gfx.makeBuffer(sk.shape.vertexBufferDesc(buf));
    bind.index_buffer = sk.gfx.makeBuffer(sk.shape.indexBufferDesc(buf));
}

fn toMat4(m: zm.Mat) sk.shape.Mat4 {
    const matrix: [4][4]f32 = .{
        .{ m[0][0], m[0][1], m[0][2], m[0][3] },
        .{ m[1][0], m[1][1], m[1][2], m[1][3] },
        .{ m[2][0], m[2][1], m[2][2], m[2][3] },
        .{ m[3][0], m[3][1], m[3][2], m[3][3] },
    };

    return .{ .m = matrix };
}

var elementRange: sk.shape.ElementRange = undefined;

const width = 640;
const height = 480;

var rx: f32 = 0;
var ry: f32 = 0;
const view: zm.Mat = zm.lookAtRh(
    zm.f32x4(0, 1.5, 6, 1.0), // eye position
    zm.f32x4(0.0, 0.0, 0.0, 1.0), // focus point
    zm.f32x4(0.0, 1.0, 0.0, 0.0), // up direction
);
var params: shd.VsParams = undefined;

export fn frame() void {
    const aspect = sk.app.widthf() / sk.app.heightf();
    const radians = std.math.degreesToRadians(60.0);
    const proj = zm.perspectiveFovRh(radians, aspect, 0.01, 10.0);

    const dt: f32 = @floatCast(sk.app.frameDuration() * 60);
    rx += 1.0 * dt;
    ry += 2.0 * dt;
    const rxm = zm.rotationX(std.math.degreesToRadians(rx));
    const rym = zm.rotationY(std.math.degreesToRadians(ry));
    const model = zm.mul(rym, rxm);
    params.vp = zm.mul(zm.mul(model, view), proj);

    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

    sk.gfx.applyPipeline(pipeline);
    sk.gfx.applyBindings(bind);

    sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&params));
    sk.gfx.draw(elementRange.base_element, elementRange.num_elements, 1);

    sk.gfx.endPass();
    sk.gfx.commit();
}

export fn input(event: ?*const sk.app.Event) void {
    const ev = event.?;
    if (ev.type == .KEY_DOWN) {
        params.draw_mode = switch (ev.key_code) {
            ._1 => 0.0,
            ._2 => 1.0,
            ._3 => 2.0,
            else => params.draw_mode,
        };
    }
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
        .event_cb = input,
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
    float draw_mode;
    mat4 vp;
};

in vec4 position;
in vec3 normal;
in vec2 texcoord;
in vec4 color0;

out vec4 color;

void main() {
    gl_Position = vp * position;
    if (draw_mode == 0.0) {
        color = vec4((normal + 1.0) * 0.5, 1.0);
    }
    else if (draw_mode == 1.0) {
        color = vec4(texcoord, 0.0, 1.0);
    }
    else {
        color = color0;
    }
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

![合并变换][1]

[1]: images/sokol019.webp

## 附录
