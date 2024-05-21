# 0546-WebGPU-纹理 Texture

## 环境

- Time 2024-05-21
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

截至到目前后面的 Matrix Stacks 和 Scene Graphs 还没有内容，先跳过这部分，学习纹理。

## shader.wgsl

```wgsl
struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec2f,
};

@vertex
fn vs_main(@builtin(vertex_index) index : u32) -> VertexOutput {

    let pos = array(
          // 1st triangle
          vec2f( 0.0,  0.0),  // center
          vec2f( 1.0,  0.0),  // right, center
          vec2f( 0.0,  1.0),  // center, top

          // 2st triangle
          vec2f( 0.0,  1.0),  // center, top
          vec2f( 1.0,  0.0),  // right, center
          vec2f( 1.0,  1.0),  // right, top
        );

    var out: VertexOutput;
    let xy = pos[index];
    out.position = vec4f(xy, 0.0, 1.0);
    out.texcoord = xy;
    return out;
}

@group(0) @binding(0) var ourSampler: sampler;
@group(0) @binding(1) var ourTexture: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return textureSample(ourTexture, ourSampler, in.texcoord);
}
```

## render.zig

```zig
const std = @import("std");
const mach = @import("mach");

const Color = extern struct {
    r: u8 = 0,
    g: u8 = 0,
    b: u8 = 0,
    a: u8 = 255,
};

pub const RenderContext = struct {
    bindGroup: *mach.gpu.BindGroup,
    pipeline: *mach.gpu.RenderPipeline,

    pub fn release(self: *RenderContext) void {
        self.pipeline.release();
    }
};

pub fn createRenderPipeline() RenderContext {
    const device = mach.core.device;

    // 编译 shader
    const source = @embedFile("shader.wgsl");
    const module = device.createShaderModuleWGSL("shader.wgsl", source);
    defer module.release();

    const vertex = mach.gpu.VertexState.init(.{
        .module = module,
        .entry_point = "vs_main",
    });

    // 片段着色器状态
    const fragment = mach.gpu.FragmentState.init(.{
        .module = module,
        .entry_point = "fs_main",
        .targets = &.{.{ .format = mach.core.descriptor.format }},
    });

    const r: Color = Color{ .r = 255 };
    const y: Color = Color{ .r = 255, .g = 255 };
    const b: Color = Color{ .b = 255 };
    const textureData = [7][5]Color{
        .{ b, r, r, r, r },
        .{ r, y, y, y, r },
        .{ r, y, r, r, r },
        .{ r, y, y, r, r },
        .{ r, y, r, r, r },
        .{ r, y, r, r, r },
        .{ r, r, r, r, r },
    };

    const width, const height = .{ textureData[0].len, textureData.len };
    const texture = device.createTexture(&.{
        .label = "F texture",
        .size = .{ .width = width, .height = height },
        .format = .rgba8_unorm,
        .usage = .{ .texture_binding = true, .copy_dst = true },
    });

    const layout = mach.gpu.Texture.DataLayout{
        .bytes_per_row = width * 4,
        .rows_per_image = height,
    };
    const size = mach.gpu.Extent3D{ .width = width, .height = height };
    device.getQueue().writeTexture(&.{ .texture = texture }, &layout, &size, &textureData);

    const sampler = device.createSampler(&.{});

    // 创建渲染管线
    const descriptor = mach.gpu.RenderPipeline.Descriptor{
        .fragment = &fragment,
        .vertex = vertex,
    };

    const pipeline = device.createRenderPipeline(&descriptor);

    const view = texture.createView(&.{
        .format = .rgba8_unorm,
        .dimension = .dimension_2d,
    });
    const bindGroup = device.createBindGroup(
        &mach.gpu.BindGroup.Descriptor.init(.{
            .layout = pipeline.getBindGroupLayout(0),
            .entries = &.{
                mach.gpu.BindGroup.Entry.sampler(0, sampler),
                mach.gpu.BindGroup.Entry.textureView(1, view),
            },
        }),
    );

    return RenderContext{
        .bindGroup = bindGroup,
        .pipeline = pipeline,
    };
}
```

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");

pub const App = @This();
const width = 640;
const height = 480;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderContext: render.RenderContext = undefined,

pub fn init(app: *App) !void {
    try mach.core.init(.{
        .title = "学习 WebGPU",
        .size = .{ .width = width, .height = height },
    });
    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);

    app.renderContext = render.createRenderPipeline();
}

pub fn deinit(app: *App) void {
    app.renderContext.release();
    mach.core.deinit();
    _ = gpa.deinit();
}

pub fn update(app: *App) !bool {
    // 检查窗口是否需要关闭
    var iterator = mach.core.pollEvents();
    while (iterator.next()) |event| if (event == .close) return true;

    const view = mach.core.swap_chain.getCurrentTextureView().?;
    defer view.release();

    const renderPass = mach.gpu.RenderPassDescriptor.init(.{
        .color_attachments = &.{.{
            .view = view,
            .clear_value = std.mem.zeroes(mach.gpu.Color),
            .load_op = .clear,
            .store_op = .store,
        }},
    });

    // 命令编码器
    const encoder = mach.core.device.createCommandEncoder(null);
    defer encoder.release();
    const pass = encoder.beginRenderPass(&renderPass);

    // 设置渲染管线
    pass.setPipeline(app.renderContext.pipeline);
    pass.setBindGroup(0, app.renderContext.bindGroup, &.{});

    pass.draw(6, 1, 0, 0);
    pass.end();
    pass.release();

    var command = encoder.finish(null);
    defer command.release();

    // 提交命令
    mach.core.queue.submit(&.{command});
    mach.core.swap_chain.present();

    // 不退出渲染循环
    return false;
}
```

## 效果

![纹理][1]

## 总结

简单使用 WebGPU 中的纹理。

[1]: images/webgpu42.png

## 附录
