# 0754-sokol-离屏渲染

## 目标

进行离屏渲染。

## 环境

- Time 2025-02-12
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

不太清除这个干什么用的，而且一直觉得 MVP 的矩阵变换有点问题，现在看起来方向就是反的。

## main.zig

```zig
const std = @import("std");
const sk = @import("sokol");
const stbi = @import("stbi");
const zm = @import("zmath");

const shd = @import("shader/test.glsl.zig");

const clearColor: sk.gfx.Color = .{ .r = 0.25, .g = 0.45, .b = 0.65, .a = 1.0 };
var info: sk.gfx.PassAction = undefined;
var pipeline: sk.gfx.Pipeline = undefined;
var bind: sk.gfx.Bindings = undefined;

const offscreen_sample_count = 1;

const offscreen = struct {
    var pass_action: sk.gfx.PassAction = .{};
    var attachments: sk.gfx.Attachments = .{};
    var pip: sk.gfx.Pipeline = .{};
    var bind: sk.gfx.Bindings = .{};
};

var donut: sk.shape.ElementRange = .{};
var sphere: sk.shape.ElementRange = .{};

export fn init() void {

    // 设置初始化环境
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });

    // 背景清除颜色
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    // offscreen pass action: clear to black
    offscreen.pass_action.colors[0] = .{
        .load_action = .CLEAR,
        .clear_value = .{ .r = 0.25, .g = 0.25, .b = 0.25, .a = 1.0 },
    };

    // a render pass with one color- and one depth-attachment image
    var img_desc = sk.gfx.ImageDesc{
        .render_target = true,
        .width = 256,
        .height = 256,
        .pixel_format = .RGBA8,
        .sample_count = offscreen_sample_count,
    };
    const color_img = sk.gfx.makeImage(img_desc);
    img_desc.pixel_format = .DEPTH;
    const depth_img = sk.gfx.makeImage(img_desc);

    var atts_desc = sk.gfx.AttachmentsDesc{};
    atts_desc.colors[0].image = color_img;
    atts_desc.depth_stencil.image = depth_img;
    offscreen.attachments = sk.gfx.makeAttachments(atts_desc);

    // a donut shape which is rendered into the offscreen render target, and
    // a sphere shape which is rendered into the default framebuffer
    var vertices: [4000]sk.shape.Vertex = undefined;
    var indices: [24000]u16 = undefined;
    var buf: sk.shape.Buffer = .{
        .vertices = .{ .buffer = sk.shape.asRange(&vertices) },
        .indices = .{ .buffer = sk.shape.asRange(&indices) },
    };
    buf = sk.shape.buildTorus(buf, .{ .radius = 0.5, .ring_radius = 0.3, .sides = 20, .rings = 36 });
    donut = sk.shape.elementRange(buf);
    buf = sk.shape.buildSphere(buf, .{
        .radius = 0.5,
        .slices = 72,
        .stacks = 40,
    });
    sphere = sk.shape.elementRange(buf);

    const vbuf = sk.gfx.makeBuffer(sk.shape.vertexBufferDesc(buf));
    const ibuf = sk.gfx.makeBuffer(sk.shape.indexBufferDesc(buf));

    // shader and pipeline object for offscreen rendering
    offscreen.pip = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.buffers[0] = sk.shape.vertexBufferLayoutState();
            l.attrs[shd.ATTR_test_position] = sk.shape.positionVertexAttrState();
            l.attrs[shd.ATTR_test_normal] = sk.shape.normalVertexAttrState();
            break :init l;
        },
        .index_type = .UINT16,
        .cull_mode = .BACK,
        .sample_count = offscreen_sample_count,
        .depth = .{
            .pixel_format = .DEPTH,
            .compare = .LESS_EQUAL,
            .write_enabled = true,
        },
        .colors = init: {
            var c: [4]sk.gfx.ColorTargetState = @splat(.{});
            c[0].pixel_format = .RGBA8;
            break :init c;
        },
    });

    // shader and pipeline object for the default render pass
    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.defaultShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.buffers[0] = sk.shape.vertexBufferLayoutState();
            l.attrs[shd.ATTR_default_position] = sk.shape.positionVertexAttrState();
            l.attrs[shd.ATTR_default_normal] = sk.shape.normalVertexAttrState();
            l.attrs[shd.ATTR_default_texcoord0] = sk.shape.texcoordVertexAttrState();
            break :init l;
        },
        .index_type = .UINT16,
        .cull_mode = .BACK,
        .depth = .{
            .compare = .LESS_EQUAL,
            .write_enabled = true,
        },
    });

    // a sampler object for sampling the render target texture
    const smp = sk.gfx.makeSampler(.{
        .min_filter = .LINEAR,
        .mag_filter = .LINEAR,
        .wrap_u = .REPEAT,
        .wrap_v = .REPEAT,
    });

    // resource bindings to render a non-textured cube (into the offscreen render target)
    offscreen.bind.vertex_buffers[0] = vbuf;
    offscreen.bind.index_buffer = ibuf;

    // resource bindings to render a textured cube, using the offscreen render target as texture
    bind.vertex_buffers[0] = vbuf;
    bind.index_buffer = ibuf;
    bind.images[shd.IMG_tex] = color_img;
    bind.samplers[shd.SMP_smp] = smp;
}

const width = 640;
const height = 480;

var rx: f32 = 0;
var ry: f32 = 0;
var params: shd.VsParams = undefined;

export fn frame() void {
    const dt: f32 = @floatCast(sk.app.frameDuration() * 60);
    rx += 1.0 * dt;
    ry += 2.0 * dt;
    const aspect = sk.app.widthf() / sk.app.heightf();

    // the offscreen pass, rendering a rotating untextured donut into a render target image
    sk.gfx.beginPass(.{ .action = offscreen.pass_action, .attachments = offscreen.attachments });
    sk.gfx.applyPipeline(offscreen.pip);
    sk.gfx.applyBindings(offscreen.bind);
    sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&computeParams(rx, ry, 1.0, 2.5)));
    sk.gfx.draw(donut.base_element, donut.num_elements, 1);
    sk.gfx.endPass();

    // and the display pass, rendering a rotating textured sphere, using the previously
    // rendered offscreen render target as texture
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });
    sk.gfx.applyPipeline(pipeline);
    sk.gfx.applyBindings(bind);
    sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&computeParams(-rx * 0.25, ry * 0.25, aspect, 2)));
    sk.gfx.draw(sphere.base_element, sphere.num_elements, 1);
    sk.gfx.endPass();

    sk.gfx.commit();
}

fn computeParams(r1: f32, r2: f32, aspect: f32, eye_dist: f32) shd.VsParams {
    const proj = zm.perspectiveFovRh(std.math.degreesToRadians(45), aspect, 0.01, 10);
    const view = zm.lookAtRh(
        zm.f32x4(0, 1.5, eye_dist, 1.0),
        zm.f32x4(0, 0, 0, 1),
        zm.f32x4(0, 1, 0, 0),
    );

    const rxm = zm.rotationX(std.math.degreesToRadians(r1));
    const rym = zm.rotationY(std.math.degreesToRadians(r2));
    const model = zm.mul(rym, rxm);

    return shd.VsParams{ .mvp = zm.mul(zm.mul(model, view), proj) };
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

// shared code for all shaders
@block uniforms
layout(binding=0) uniform vs_params {
    mat4 mvp;
};
@end

// offscreen rendering shaders
@vs vs
@include_block uniforms

in vec4 position;
in vec4 normal;
out vec4 nrm;

void main() {
    gl_Position = mvp * position;
    nrm = normal;
}
@end

@fs fs
in vec4 nrm;
out vec4 frag_color;

void main() {
    frag_color = vec4(nrm.xyz * 0.5 + 0.5, 1.0);
}
@end

@program test vs fs

@vs vs_default
@include_block uniforms

in vec4 position;
in vec4 normal;
in vec2 texcoord0;
out vec4 nrm;
out vec2 uv;

void main() {
    gl_Position = mvp * position;
    uv = texcoord0;
    nrm = mvp * normal;
}
@end

@fs fs_default
layout(binding=0) uniform texture2D tex;
layout(binding=0) uniform sampler smp;

in vec4 nrm;
in vec2 uv;

out vec4 frag_color;

void main() {
    vec4 c = texture(sampler2D(tex, smp), uv * vec2(20.0, 10.0));
    float l = clamp(dot(nrm.xyz, normalize(vec3(1.0, 1.0, -1.0))), 0.0, 1.0) * 2.0;
    frag_color = vec4(c.xyz * (l + 0.25), 1.0);
}
@end

@program default vs_default fs_default
```

## 效果

![离屏渲染][1]

[1]: images/sokol020.webp

## 附录
