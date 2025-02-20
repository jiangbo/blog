# 0766-sokol-封装纹理缓存

## 目标

将图片加载为纹理进行封装，新增一个缓存，如果加载过就不加载了。

## 环境

- Time 2025-02-20
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

新增了一个 cache 缓存模块，用来缓存一些加载消耗时间的资源。

## cache.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");

var allocator: std.mem.Allocator = undefined;

pub fn init(alloc: std.mem.Allocator) void {
    allocator = alloc;
    TextureCache.init();
}

pub fn deinit() void {
    TextureCache.deinit();
}

pub const TextureCache = struct {
    const stbi = @import("stbi");
    const Cache = std.StringHashMap(gfx.Texture);

    var cache: Cache = undefined;

    pub fn init() void {
        cache = Cache.init(allocator);
        stbi.init(allocator);
    }

    pub fn get(path: [:0]const u8) ?gfx.Texture {
        const entry = cache.getOrPut(path) catch |e| {
            std.log.err("texture cache allocate error: {}", .{e});
            return null;
        };
        if (entry.found_existing) return entry.value_ptr.*;

        std.log.info("loading texture from: {s}", .{path});
        var image = stbi.Image.loadFromFile(path, 4) catch |e| {
            std.log.err("loading image error: {}", .{e});
            return null;
        };

        defer image.deinit();

        const texture = gfx.Texture.init(image.width, image.height, image.data);
        entry.value_ptr.* = texture;
        return texture;
    }

    pub fn deinit() void {
        stbi.deinit();
        cache.deinit();
    }
};
```

## graphics.zig

```zig
const std = @import("std");
const zm = @import("zmath");
const sk = @import("sokol");

const shd = @import("shader/test.glsl.zig");

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

pub const BatchInstance = shd.Batchinstance;
pub const UniformParams = shd.VsParams;
pub const Image = sk.gfx.Image;
pub const Texture = struct {
    x: f32 = 0,
    y: f32 = 0,
    width: f32,
    height: f32,
    value: sk.gfx.Image,

    pub fn init(width: u32, height: u32, data: []u8) Texture {
        const image = sk.gfx.allocImage();

        sk.gfx.initImage(image, .{
            .width = @as(i32, @intCast(width)),
            .height = @as(i32, @intCast(height)),
            .pixel_format = .RGBA8,
            .data = init: {
                var imageData = sk.gfx.ImageData{};
                imageData.subimage[0][0] = sk.gfx.asRange(data);
                break :init imageData;
            },
        });

        return .{
            .width = @floatFromInt(width),
            .height = @floatFromInt(height),
            .value = image,
        };
    }
};

pub const Color = sk.gfx.Color;
pub const Buffer = sk.gfx.Buffer;

pub const BindGroup = struct {
    value: sk.gfx.Bindings = .{},
    uniform: shd.VsParams = undefined,

    pub fn bindTexture(self: *BindGroup, texture: Texture) void {
        self.value.images[shd.IMG_tex] = texture.value;
        self.value.samplers[shd.SMP_smp] = sk.gfx.makeSampler(.{
            .min_filter = .LINEAR,
            .mag_filter = .LINEAR,
        });
    }

    pub fn bindStorageBuffer(self: *BindGroup, index: u32, storageBuffer: anytype) void {
        self.value.storage_buffers[index] = sk.gfx.makeBuffer(.{
            .type = .STORAGEBUFFER,
            .data = sk.gfx.asRange(storageBuffer),
        });
    }

    pub fn updateStorageBuffer(self: *BindGroup, index: u32, storageBuffer: anytype) void {
        sk.gfx.destroyBuffer(self.value.storage_buffers[index]);
        self.value.storage_buffers[index] = sk.gfx.makeBuffer(.{
            .type = .STORAGEBUFFER,
            .data = sk.gfx.asRange(storageBuffer),
        });
    }

    pub fn bindUniformBuffer(self: *BindGroup, uniform: UniformParams) void {
        self.uniform = uniform;
    }
};

pub const CommandEncoder = struct {};

pub const RenderPass = struct {
    pub fn begin(color: Color) RenderPass {
        var action = sk.gfx.PassAction{};
        action.colors[0] = .{ .clear_value = color };
        sk.gfx.beginPass(.{ .action = action, .swapchain = sk.glue.swapchain() });
        return RenderPass{};
    }

    pub fn setPipeline(self: *RenderPass, pipeline: RenderPipeline) void {
        _ = self;
        sk.gfx.applyPipeline(pipeline.value);
    }

    pub fn setBindGroup(self: *RenderPass, index: u32, group: BindGroup) void {
        _ = self;
        _ = index;
        sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&group.uniform));
        sk.gfx.applyBindings(group.value);
    }

    pub fn draw(self: *RenderPass, number: u32) void {
        _ = self;
        sk.gfx.draw(0, number, 1);
    }

    pub fn end(self: *RenderPass) void {
        _ = self;
        sk.gfx.endPass();
        sk.gfx.commit();
    }
};

pub const RenderPipeline = struct {
    value: sk.gfx.Pipeline,
    pub var texturePipeline: ?RenderPipeline = null;

    pub fn getTexturePipeline() RenderPipeline {
        if (texturePipeline) |p| return p;

        const pip = sk.gfx.makePipeline(.{
            .shader = sk.gfx.makeShader(shd.testShaderDesc(sk.gfx.queryBackend())),
            .depth = .{
                .compare = .LESS_EQUAL,
                .write_enabled = true,
            },
        });
        texturePipeline = RenderPipeline{ .value = pip };
        return texturePipeline.?;
    }
};

pub const Event = sk.app.Event;
pub const RunInfo = struct {
    width: u16,
    height: u16,
    title: [:0]const u8,
    init: *const fn () void,
    frame: *const fn () void,
    event: *const fn (?*const Event) void,
    deinit: *const fn () void,
};

var runInfo: RunInfo = undefined;
pub fn run(info: RunInfo) void {
    runInfo = info;
    sk.app.run(.{
        .width = info.width,
        .height = info.height,
        .window_title = info.title,
        .logger = .{ .func = sk.log.func },
        .win32_console_attach = true,
        .high_dpi = true,
        .init_cb = init,
        .event_cb = event,
        .frame_cb = frame,
        .cleanup_cb = cleanup,
    });
}

export fn init() void {
    sk.gfx.setup(.{
        .environment = sk.glue.environment(),
        .logger = .{ .func = sk.log.func },
    });
    runInfo.init();
}

export fn event(evt: ?*const Event) void {
    runInfo.event(evt);
}

export fn frame() void {
    runInfo.frame();
}

export fn cleanup() void {
    sk.gfx.shutdown();
    runInfo.deinit();
}
```

## main.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const cache = @import("cache.zig");

var bind: gfx.BindGroup = .{};

const NUMBER = 1;

fn init() void {
    cache.init(allocator);

    const texture = cache.TextureCache.get("assets/player.bmp").?;
    bind.bindTexture(texture);

    storageBuffer = allocator.alloc(gfx.BatchInstance, NUMBER) catch unreachable;
    bind.bindStorageBuffer(0, storageBuffer);

    const camera = gfx.Camera.init(width, height);
    bind.bindUniformBuffer(gfx.UniformParams{ .vp = camera.vp() });
}

var storageBuffer: []gfx.BatchInstance = undefined;

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

fn frame() void {
    var renderPass = gfx.RenderPass.begin(.{ .r = 1, .b = 1, .a = 1 });
    defer renderPass.end();

    const texture = cache.TextureCache.get("assets/player.bmp").?;
    for (0..NUMBER) |i| {
        const x = rand.float(f32) * width * 0;
        const y = rand.float(f32) * height * 0;
        fillVertex(i, x, y, texture.width, texture.height);
    }

    bind.updateStorageBuffer(0, storageBuffer);
    renderPass.setPipeline(gfx.RenderPipeline.getTexturePipeline());
    renderPass.setBindGroup(0, bind);

    renderPass.draw(6 * NUMBER);
}

fn event(evt: ?*const gfx.Event) void {
    _ = evt;
}

fn deinit() void {
    allocator.free(storageBuffer);
    cache.deinit();
}

const width = 640;
const height = 480;
var rand: std.Random = undefined;
var allocator: std.mem.Allocator = undefined;

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();

    var prng = std.Random.DefaultPrng.init(@intCast(std.time.timestamp()));
    rand = prng.random();
    gfx.run(.{
        .width = width,
        .height = height,
        .title = "学习 sokol",
        .init = init,
        .event = event,
        .frame = frame,
        .deinit = deinit,
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

![封装纹理缓存][1]

[1]: images/sokol030.png

## 附录
