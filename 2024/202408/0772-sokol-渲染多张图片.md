# 0772-sokol-渲染多张图片

## 目标

之前实现的批量渲染，不清楚怎么渲染多张图片，新增了一个渲染单个图片的类。

## 环境

- Time 2025-02-21
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>
2. <https://www.bilibili.com/video/BV1vM411f7nJ/>

## 想法

将窗口管理的部分单独提出来了，不清楚批量渲染怎么渲染多个图片，现在不清楚，之后了解到再说。

## window.zig

```zig
const std = @import("std");
const sk = @import("sokol");

const context = @import("context.zig");

pub const Event = sk.app.Event;
pub const RunInfo = struct {
    init: *const fn () void,
    frame: *const fn () void,
    event: *const fn (?*const Event) void,
    deinit: *const fn () void,
};

var runInfo: RunInfo = undefined;
pub fn run(info: RunInfo) void {
    runInfo = info;
    sk.app.run(.{
        .width = @as(i32, @intFromFloat(context.width)),
        .height = @as(i32, @intFromFloat(context.height)),
        .window_title = context.title,
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

## graphics.zig

这个文件改动比较大，主要是新增了 TextureSingle 结构体。

```zig
const std = @import("std");
const zm = @import("zmath");
const sk = @import("sokol");

const context = @import("context.zig");
const batch = @import("shader/batch.glsl.zig");

pub const Camera = struct {
    proj: zm.Mat,

    pub fn init(width: f32, height: f32) Camera {
        const proj = zm.orthographicOffCenterLh(0, width, 0, height, 0, 1);
        return .{ .proj = proj };
    }

    pub fn vp(self: Camera) zm.Mat {
        return self.proj;
    }
};

pub const BatchInstance = batch.Batchinstance;
pub const UniformParams = batch.VsParams;
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
    uniform: batch.VsParams = undefined,

    pub fn bindIndexBuffer(self: *BindGroup, buffer: Buffer) void {
        self.value.index_buffer = buffer;
    }

    pub fn bindVertexBuffer(self: *BindGroup, index: u32, buffer: Buffer) void {
        self.value.vertex_buffers[index] = buffer;
    }

    pub fn bindTexture(self: *BindGroup, index: u32, texture: Texture) void {
        self.value.images[index] = texture.value;
    }

    pub fn bindSampler(self: *BindGroup, index: u32, sampler: Sampler) void {
        self.value.samplers[index] = sampler.value;
    }

    pub fn bindStorageBuffer(self: *BindGroup, index: u32, buffer: Buffer) void {
        self.value.storage_buffers[index] = buffer;
    }

    pub fn updateStorageBuffer(self: *BindGroup, index: u32, buffer: anytype) void {
        const range = sk.gfx.asRange(buffer);
        sk.gfx.updateBuffer(self.value.storage_buffers[index], range);
    }

    pub fn bindUniformBuffer(self: *BindGroup, uniform: UniformParams) void {
        self.uniform = uniform;
    }
};

pub const CommandEncoder = struct {
    pub fn beginRenderPass(color: Color) RenderPass {
        return RenderPass.begin(color);
    }
};

pub const RenderPass = struct {
    pub fn begin(color: Color) RenderPass {
        var action = sk.gfx.PassAction{};
        action.colors[0] = .{ .load_action = .CLEAR, .clear_value = color };
        sk.gfx.beginPass(.{ .action = action, .swapchain = sk.glue.swapchain() });
        return RenderPass{};
    }

    pub fn setPipeline(self: *RenderPass, pipeline: RenderPipeline) void {
        _ = self;
        sk.gfx.applyPipeline(pipeline.value);
    }

    pub fn setBindGroup(self: *RenderPass, group: BindGroup) void {
        _ = self;
        sk.gfx.applyUniforms(batch.UB_vs_params, sk.gfx.asRange(&group.uniform));
        sk.gfx.applyBindings(group.value);
    }

    pub fn draw(self: *RenderPass, number: u32) void {
        _ = self;
        sk.gfx.draw(0, number, 1);
    }

    pub fn submit(self: *RenderPass) void {
        _ = self;
        sk.gfx.endPass();
        sk.gfx.commit();
    }
};

pub const Sampler = struct {
    value: sk.gfx.Sampler,

    pub fn liner() Sampler {
        const sampler = sk.gfx.makeSampler(.{
            .min_filter = .LINEAR,
            .mag_filter = .LINEAR,
        });
        return .{ .value = sampler };
    }

    pub fn nearest() Sampler {
        const sampler = sk.gfx.makeSampler(.{
            .min_filter = .NEAREST,
            .mag_filter = .NEAREST,
        });
        return .{ .value = sampler };
    }
};

const Allocator = std.mem.Allocator;

pub const BatchBuffer = struct {
    const size: usize = 100;

    cpu: std.ArrayListUnmanaged(BatchInstance),
    gpu: Buffer,

    pub fn init(alloc: Allocator) Allocator.Error!BatchBuffer {
        return .{
            .cpu = try std.ArrayListUnmanaged(BatchInstance).initCapacity(alloc, size),
            .gpu = sk.gfx.makeBuffer(.{
                .type = .STORAGEBUFFER,
                .usage = .DYNAMIC,
                .size = size * @sizeOf(BatchInstance),
            }),
        };
    }

    pub fn deinit(self: *BatchBuffer, alloc: Allocator) void {
        self.cpu.deinit(alloc);
    }
};

pub const TextureBatch = struct {
    bind: BindGroup = .{},
    texture: Texture,
    renderPass: RenderPass,
    buffer: BatchBuffer,

    var pipeline: ?RenderPipeline = null;

    pub fn begin(renderPass: RenderPass, texture: Texture) TextureBatch {
        var textureBatch = TextureBatch{
            .texture = texture,
            .renderPass = renderPass,
            .buffer = context.batchBuffer,
        };

        textureBatch.bind.bindUniformBuffer(UniformParams{ .vp = context.camera.vp() });
        textureBatch.bind.bindStorageBuffer(0, textureBatch.buffer.gpu);
        textureBatch.bind.bindTexture(batch.IMG_tex, texture);
        textureBatch.bind.bindSampler(batch.SMP_smp, context.textureSampler);

        pipeline = pipeline orelse RenderPipeline{ .value = sk.gfx.makePipeline(.{
            .shader = sk.gfx.makeShader(batch.batchShaderDesc(sk.gfx.queryBackend())),
            .depth = .{
                .compare = .LESS_EQUAL,
                .write_enabled = true,
            },
            .cull_mode = .BACK,
        }) };

        return textureBatch;
    }

    pub fn draw(self: *TextureBatch, x: f32, y: f32) void {
        self.buffer.cpu.appendAssumeCapacity(.{
            .position = .{ x, y, 0.5, 1.0 },
            .rotation = 0.0,
            .width = self.texture.width,
            .height = self.texture.height,
            .padding = 0.0,
            .texcoord = .{ 0.0, 0.0, 1.0, 1.0 },
            .color = .{ 1.0, 1.0, 1.0, 1.0 },
        });
    }

    pub fn end(self: *TextureBatch) void {
        self.renderPass.setPipeline(pipeline.?);
        self.bind.updateStorageBuffer(0, self.buffer.cpu.items);
        self.renderPass.setBindGroup(self.bind);
        self.renderPass.draw(6 * @as(u32, @intCast(self.buffer.cpu.items.len)));
    }
};

pub const TextureSingle = struct {
    bind: BindGroup,
    renderPass: RenderPass,

    const single = @import("shader/single.glsl.zig");
    var vertexBuffer: ?Buffer = null;
    var indexBuffer: ?Buffer = null;
    var pipeline: ?RenderPipeline = null;

    pub fn begin(renderPass: RenderPass) TextureSingle {
        var self = TextureSingle{ .bind = .{}, .renderPass = renderPass };

        vertexBuffer = vertexBuffer orelse sk.gfx.makeBuffer(.{
            .data = sk.gfx.asRange(&[_]f32{
                // 顶点和颜色
                0, 1, 0.5, 1.0, 1.0, 1.0, 0, 1,
                1, 1, 0.5, 1.0, 1.0, 1.0, 1, 1,
                1, 0, 0.5, 1.0, 1.0, 1.0, 1, 0,
                0, 0, 0.5, 1.0, 1.0, 1.0, 0, 0,
            }),
        });
        self.bind.bindVertexBuffer(0, vertexBuffer.?);

        indexBuffer = indexBuffer orelse sk.gfx.makeBuffer(.{
            .type = .INDEXBUFFER,
            .data = sk.gfx.asRange(&[_]u16{ 0, 1, 2, 0, 2, 3 }),
        });
        self.bind.bindIndexBuffer(indexBuffer.?);

        self.bind.bindSampler(single.SMP_smp, context.textureSampler);

        pipeline = pipeline orelse RenderPipeline{ .value = sk.gfx.makePipeline(.{
            .shader = sk.gfx.makeShader(single.singleShaderDesc(sk.gfx.queryBackend())),
            .layout = init: {
                var l = sk.gfx.VertexLayoutState{};
                l.attrs[single.ATTR_single_position].format = .FLOAT3;
                l.attrs[single.ATTR_single_color0].format = .FLOAT3;
                l.attrs[single.ATTR_single_texcoord0].format = .FLOAT2;
                break :init l;
            },
            .index_type = .UINT16,
            .depth = .{
                .compare = .LESS_EQUAL,
                .write_enabled = true,
            },
        }) };

        return self;
    }

    pub fn draw(self: *TextureSingle, x: f32, y: f32, tex: Texture) void {
        const w = tex.width;
        const h = tex.height;
        const model = zm.mul(zm.scaling(w, h, 1), zm.translation(x, y, 0));
        const params = UniformParams{ .vp = zm.mul(model, context.camera.vp()) };
        self.bind.bindUniformBuffer(params);

        self.renderPass.setPipeline(pipeline.?);
        self.bind.bindTexture(single.IMG_tex, tex);
        self.renderPass.setBindGroup(self.bind);
        sk.gfx.draw(0, 6, 1);
    }

    pub fn end(self: *TextureSingle) void {
        _ = self;
        sk.gfx.endPass();
    }
};

pub const RenderPipeline = struct {
    value: sk.gfx.Pipeline,
};
```

## main.zig

```zig
const std = @import("std");
const gfx = @import("graphics.zig");
const cache = @import("cache.zig");
const context = @import("context.zig");
const window = @import("window.zig");

const playerAnimationNumber = 6;

var background: gfx.Texture = undefined;
var playerLeft: [playerAnimationNumber]gfx.Texture = undefined;
var playerRight: [playerAnimationNumber]gfx.Texture = undefined;

fn init() void {
    const allocator = context.allocator;
    cache.init(allocator);

    context.camera = gfx.Camera.init(context.width, context.height);
    context.textureSampler = gfx.Sampler.liner();

    context.batchBuffer = gfx.BatchBuffer.init(allocator) catch unreachable;

    // 加载背景
    background = cache.TextureCache.load("assets/img/background.png").?;

    // 加载角色
    var nameBuffer: [64]u8 = undefined;
    for (0..playerAnimationNumber) |index| {
        playerLeft[index] = loadTexture(&nameBuffer, "left", index).?;
    }
    for (0..playerAnimationNumber) |index| {
        playerRight[index] = loadTexture(&nameBuffer, "right", index).?;
    }
}

const pathFmt = "assets/img/player_{s}_{}.png";
fn loadTexture(buffer: []u8, direction: []const u8, index: usize) ?gfx.Texture {
    const path = std.fmt.bufPrintZ(buffer, pathFmt, .{ direction, index });
    return cache.TextureCache.load(path catch unreachable).?;
}

var frameCounter: usize = 0;
var playerAnimationIndex: usize = 0;

fn frame() void {
    frameCounter += 1;
    if (frameCounter % 5 == 0) playerAnimationIndex += 1;

    playerAnimationIndex %= playerAnimationNumber;

    var renderPass = gfx.CommandEncoder.beginRenderPass(context.clearColor);
    defer renderPass.submit();

    var single = gfx.TextureSingle.begin(renderPass);

    single.draw(0, 0, background);
    single.draw(100, 100, playerLeft[playerAnimationIndex]);
}

fn event(evt: ?*const window.Event) void {
    _ = evt;
}

fn deinit() void {
    context.batchBuffer.deinit(context.allocator);
    cache.deinit();
}

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    context.allocator = gpa.allocator();

    context.width = 1280;
    context.height = 720;

    var prng = std.Random.DefaultPrng.init(@intCast(std.time.timestamp()));
    context.rand = prng.random();
    window.run(.{ .init = init, .event = event, .frame = frame, .deinit = deinit });
}
```

## 效果

![渲染多张图片][1]

[1]: images/sokol035.webp

## 附录
