# 0765-sokol-封装 sokol

## 目标

将 sokol 的使用进行封装，按照 WebGPU 的封装格式来进行的。

## 环境

- Time 2025-02-19
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

WebGPU 应该算一个现代图形 API 的抽象层，按照 WebGPU 的使用方式封装了一下。
当前应该还不算封装，只是将 sokol 隐藏到了 graphics 的里面，后面边写边修改。

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

pub const Color = sk.gfx.Color;
pub const Buffer = sk.gfx.Buffer;

pub const BindGroup = struct {
    value: sk.gfx.Bindings = .{},
    uniform: shd.VsParams = undefined,

    pub fn bindImage(self: *BindGroup, width: u32, height: u32, data: []u8) void {
        self.value.images[shd.IMG_tex] = sk.gfx.allocImage();

        sk.gfx.initImage(self.value.images[shd.IMG_tex], .{
            .width = @as(i32, @intCast(width)),
            .height = @as(i32, @intCast(height)),
            .pixel_format = .RGBA8,
            .data = init: {
                var image = sk.gfx.ImageData{};
                image.subimage[0][0] = sk.gfx.asRange(data);
                break :init image;
            },
        });

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

pub const CommandEncoder = struct {
    pub fn beginRenderPass(self: *CommandEncoder, color: Color) RenderPass {
        _ = self;
        var action = sk.gfx.PassAction{};
        action.colors[0] = .{ .clear_value = color };
        sk.gfx.beginPass(.{ .action = action, .swapchain = sk.glue.swapchain() });
        return RenderPass{};
    }

    pub fn finish(self: *CommandEncoder) void {
        _ = self;
        sk.gfx.commit();
    }
};

pub const RenderPass = struct {
    pub fn setPipeline(self: *RenderPass, pipeline: RenderPipeline) void {
        _ = self;
        sk.gfx.applyPipeline(pipeline.value);
    }

    pub fn setBindGroup(self: *RenderPass, index: u32, group: BindGroup) void {
        _ = self;
        _ = index;
        sk.gfx.applyBindings(group.value);
        sk.gfx.applyUniforms(shd.UB_vs_params, sk.gfx.asRange(&group.uniform));
    }

    pub fn draw(self: *RenderPass, number: u32) void {
        _ = self;
        sk.gfx.draw(0, number, 1);
    }

    pub fn end(self: *RenderPass) void {
        _ = self;
        sk.gfx.endPass();
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
const stbi = @import("stbi");
const gfx = @import("graphics.zig");

var bind: gfx.BindGroup = .{};

var imageWidth: f32 = 0;
var imageHeight: f32 = 0;
const NUMBER = 10000;

fn init() void {
    var image = stbi.Image.loadFromFile("assets/player.bmp", 4) catch unreachable;
    defer image.deinit();
    imageWidth = @floatFromInt(image.width);
    imageHeight = @floatFromInt(image.height);

    bind.bindImage(image.width, image.height, image.data);

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
    var encoder = gfx.CommandEncoder{};
    defer encoder.finish();

    var renderPass = encoder.beginRenderPass(.{ .r = 1, .b = 1, .a = 1 });
    defer renderPass.end();

    for (0..NUMBER) |i| {
        const x = rand.float(f32) * width;
        const y = rand.float(f32) * height;
        fillVertex(i, x, y, imageWidth, imageHeight);
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

## 效果

![封装 sokol][1]

[1]: images/sokol029.webp

## 附录
