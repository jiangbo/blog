# 0763-sokol-顶点批量渲染

## 目标

前一节，批量渲染性能降低了，有 5000 draw call，使用批量渲染优化。

## 环境

- Time 2025-02-18
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

使用批量渲染绘制了 5000 个精灵，FPS 没有明显的波动。

## graphics.zig

```zig
const std = @import("std");
const zm = @import("zmath");

pub const Camera = struct {
    view: zm.Mat,
    proj: zm.Mat,

    pub fn init(width: f32, height: f32) Camera {
        return .{
            .view = zm.lookAtLh(
                zm.f32x4(0, 0, 0, 0), // 眼睛所在位置
                zm.f32x4(0, 0, 1, 0), // 眼睛看向的位置
                zm.f32x4(0, 1, 0, 0), // 头顶方向
            ),
            .proj = zm.orthographicOffCenterLh(0, width, 0, height, 0, 1),
        };
    }

    pub fn vp(self: Camera) zm.Mat {
        return zm.mul(self.view, self.proj);
    }
};
```

## main.zig

```zig
const std = @import("std");
const sk = @import("sokol");
const stbi = @import("stbi");
const zm = @import("zmath");
const gfx = @import("graphics.zig");

const shd = @import("shader/test.glsl.zig");

const clearColor: sk.gfx.Color = .{ .r = 1, .b = 1, .a = 1 };
var info: sk.gfx.PassAction = undefined;
var pipeline: sk.gfx.Pipeline = undefined;
var bind: sk.gfx.Bindings = undefined;

var imageWidth: f32 = 0;
var imageHeight: f32 = 0;
const NUMBER = 5000;

export fn init() void {
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });
    info.colors[0] = .{ .load_action = .CLEAR, .clear_value = clearColor };

    var image = stbi.Image.loadFromFile("assets/player.bmp", 4) catch unreachable;
    defer image.deinit();
    imageWidth = @floatFromInt(image.width);
    imageHeight = @floatFromInt(image.height);

    vertex = allocator.alloc(f32, 4 * 8 * NUMBER) catch unreachable;
    bind.vertex_buffers[0] = sk.gfx.makeBuffer(.{
        .data = sk.gfx.asRange(vertex),
    });

    index = allocator.alloc(u16, 6 * NUMBER) catch unreachable;
    bind.index_buffer = sk.gfx.makeBuffer(.{
        .type = .INDEXBUFFER,
        .data = sk.gfx.asRange(index),
    });

    bind.images[shd.IMG_tex] = sk.gfx.allocImage();
    sk.gfx.initImage(bind.images[shd.IMG_tex], .{
        .width = @intCast(image.width),
        .height = @intCast(image.height),
        .pixel_format = .RGBA8,
        .data = init: {
            var data = sk.gfx.ImageData{};
            data.subimage[0][0] = sk.gfx.asRange(image.data);
            break :init data;
        },
    });

    bind.samplers[shd.SMP_smp] = sk.gfx.makeSampler(.{
        .min_filter = .LINEAR,
        .mag_filter = .LINEAR,
    });

    pipeline = sk.gfx.makePipeline(.{
        .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
        .layout = init: {
            var l = sk.gfx.VertexLayoutState{};
            l.attrs[shd.ATTR_test_position].format = .FLOAT3;
            l.attrs[shd.ATTR_test_color0].format = .FLOAT3;
            l.attrs[shd.ATTR_test_texcoord0].format = .FLOAT2;
            break :init l;
        },
        .index_type = .UINT16,
        .depth = .{
            .compare = .LESS_EQUAL,
            .write_enabled = true,
        },
    });

    const camera = gfx.Camera.init(width, height);
    params = shd.VsParams{ .vp = camera.vp() };
}

var vertex: []f32 = undefined;
var index: []u16 = undefined;

fn fillVertex(idx: usize, x: f32, y: f32, w: f32, h: f32) void {
    const temp: [32]f32 = .{
        // 顶点和颜色
        x,     y + h, 0.5, 1.0, 1.0, 1.0, 0, 1,
        x + w, y + h, 0.5, 1.0, 1.0, 1.0, 1, 1,
        x + w, y,     0.5, 1.0, 1.0, 1.0, 1, 0,
        x,     y,     0.5, 1.0, 1.0, 1.0, 0, 0,
    };
    @memcpy(vertex[32 * idx ..][0..32], &temp);
}

fn fillIndex(idx: usize) void {
    const i: u16 = @intCast(idx * 4);
    const temp: [6]u16 = .{
        0 + i, 1 + i, 2 + i, 0 + i, 2 + i, 3 + i,
    };
    @memcpy(index[6 * idx ..][0..6], &temp);
}

var params: shd.VsParams = undefined;

export fn frame() void {
    sk.gfx.beginPass(.{ .action = info, .swapchain = sk.glue.swapchain() });

    sk.gfx.applyPipeline(pipeline);
    sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&params));

    for (0..NUMBER) |i| {
        const x = rand.float(f32) * width;
        const y = rand.float(f32) * height;
        fillVertex(i, x, y, imageWidth, imageHeight);
        fillIndex(i);
    }

    sk.gfx.destroyBuffer(bind.vertex_buffers[0]);
    sk.gfx.destroyBuffer(bind.index_buffer);
    bind.vertex_buffers[0] = sk.gfx.makeBuffer(.{
        .data = sk.gfx.asRange(vertex),
    });
    bind.index_buffer = sk.gfx.makeBuffer(.{
        .type = .INDEXBUFFER,
        .data = sk.gfx.asRange(index),
    });
    sk.gfx.applyBindings(bind);
    sk.gfx.draw(0, 6 * NUMBER, 1);

    sk.gfx.endPass();
    sk.gfx.commit();
}

export fn cleanup() void {
    sk.gfx.shutdown();
    allocator.free(vertex);
    allocator.free(index);
}

const width = 640;
const height = 480;
var rand: std.Random = undefined;
var allocator: std.mem.Allocator = undefined;

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    stbi.init(gpa.allocator());
    defer stbi.deinit();

    var prng = std.Random.DefaultPrng.init(@intCast(std.time.timestamp()));
    rand = prng.random();
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
    uv = texcoord0;
}
@end

@fs fs

layout(binding=0) uniform texture2D tex;
layout(binding=0) uniform sampler smp;

in vec4 color;
in vec2 uv;
out vec4 frag_color;

void main() {
     frag_color = texture(sampler2D(tex, smp), uv) * color;
}
@end

@program test vs fs
```

## 统计

```text
Draw calls: 1
Dispatch calls: 0
API calls: 22
 Index/vertex bind calls: 3
 Constant bind calls: 2
 Sampler bind calls: 2
 Resource bind calls: 2
 Shader set calls: 2
 Blend set calls: 1
 Depth/stencil set calls: 1
 Rasterization set calls: 3
 Resource update calls: 1
 Output set calls: 1
API:Draw/Dispatch call ratio: 22
```

## 效果

![批量渲染][1]

[1]: images/sokol027.webp

## 附录
