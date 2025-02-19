# 0764-sokol-存储批量渲染

## 目标

除了使用顶点缓冲的批量渲染，也可以使用存储缓冲区的批量渲染，加到了 10000 个精灵，帧率也稳定。

## 环境

- Time 2025-02-19
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://moonside.games/posts/sdl-gpu-sprite-batcher/>

## 想法

使用存储缓冲区批量渲染感觉比顶点缓冲区简单，不需要指定输入的格式。

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
const NUMBER = 10000;

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
        .depth = .{
            .compare = .LESS_EQUAL,
            .write_enabled = true,
        },
    });

    storageBuffer = allocator.alloc(shd.Batchinstance, NUMBER) catch unreachable;
    bind.storage_buffers[0] = sk.gfx.makeBuffer(.{
        .type = .STORAGEBUFFER,
        .data = sk.gfx.asRange(storageBuffer),
    });

    const camera = gfx.Camera.init(width, height);
    params = shd.VsParams{ .vp = camera.vp() };
}

var storageBuffer: []shd.Batchinstance = undefined;

fn fillVertex(idx: usize, x: f32, y: f32, w: f32, h: f32) void {
    storageBuffer[idx] = .{
        .position = .{ x, y, 0.5, 1.0 },
        .rotation = 0.0,
        .width = w,
        .height = h,
        .padding = 0.0,
        .texcoord = .{ 0.0, 0.0, 1.0, 1.0 },
        .color = .{ 1.0, 1.0, 1.0, 1.0 },
    };
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
    }

    sk.gfx.destroyBuffer(bind.storage_buffers[0]);
    bind.storage_buffers[0] = sk.gfx.makeBuffer(.{
        .type = .STORAGEBUFFER,
        .data = sk.gfx.asRange(storageBuffer),
    });

    sk.gfx.applyBindings(bind);
    sk.gfx.draw(0, 6 * NUMBER, 1);

    sk.gfx.endPass();
    sk.gfx.commit();
}

export fn cleanup() void {
    sk.gfx.shutdown();
    allocator.free(storageBuffer);
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

使用工具生成的着色器代码有警告，先不管这个，继续往下。

```glsl
#pragma sokol @header const zm = @import("zmath")
#pragma sokol @ctype mat4 zm.Mat
#pragma sokol @ctype vec4 zm.Vec

@vs vs
layout(binding=0) uniform vs_params {
    mat4 vp;
};

struct BatchInstance
{
    vec4 position;
    float width;
    float height;
    float rotation;
    float padding;
    vec4 texcoord;
    vec4 color;
};

layout(binding=0) readonly buffer SSBO {
    BatchInstance dataBuffer[];
};

out vec4 color;
out vec2 uv;

const uint triangleIndices[6] = {0, 1, 2, 3, 2, 1};
const vec2 vertexPos[4] = {
    {0.0f, 0.0f},
    {1.0f, 0.0f},
    {0.0f, 1.0f},
    {1.0f, 1.0f}
};

void main() {

    uint spriteIndex = gl_VertexIndex / 6;
    uint vertexIndex = gl_VertexIndex % 6;
    uint vert = triangleIndices[vertexIndex];
    BatchInstance sprite = dataBuffer[spriteIndex];

    vec4 uvwh = sprite.texcoord;
    vec2 texcoord[4] = {
        {uvwh.x,          uvwh.y         },
        {uvwh.x + uvwh.z, uvwh.y         },
        {uvwh.x,          uvwh.y + uvwh.w},
        {uvwh.x + uvwh.z, uvwh.y + uvwh.w}
    };

    float c = cos(sprite.rotation);
    float s = sin(sprite.rotation);

    vec2 coord = vertexPos[vert];
    coord *= vec2(sprite.width, sprite.height);
    mat2 rotation = mat2(c, s, -s, c);
    coord = coord * rotation;

    vec3 coordWithDepth = vec3(coord + sprite.position.xy, sprite.position.z);

    gl_Position = vp * vec4(coordWithDepth, 1.0);
    color = sprite.color;
    uv = texcoord[vert];
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

## 效果

![存储批量渲染][1]

[1]: images/sokol028.webp

## 附录
