# 0529-WebGPU-屏幕坐标系

## 环境

- Time 2024-05-18
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

将之前的标准设备坐标（NDC）转化到屏幕坐标系下，可以使用屏幕坐标来画出三角形。

## shader.wgsl

```wgsl
@binding(0) @group(0) var<uniform> model: mat4x4f;

struct VertexInput {
    @location(0) position: vec4f,
    @location(1) color: vec4f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let pos = (model * vec4f(in.position.xyz, 1)).xy;
    out.position = vec4f(pos, 1, 1);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}
```

## render.zig

顶点数据变成了屏幕坐标。

```zig
const std = @import("std");
const mach = @import("mach");

pub const RenderContext = struct {
    vertexBuffer: *mach.gpu.Buffer,
    pipeline: *mach.gpu.RenderPipeline,
};

pub fn createRenderPipeline() RenderContext {
    const device = mach.core.device;

    const vertexData = [_]f32{
        300, 120, 1.0, 0.0, 0.0, //
        450, 360, 0.0, 1.0, 0.0,
        150, 360, 0.0, 0.0, 1.0,
    };

    // 编译 shader
    const source = @embedFile("shader.wgsl");
    const module = device.createShaderModuleWGSL("shader.wgsl", source);
    defer module.release();

    // 创建顶点缓冲区
    const vertexBuffer = device.createBuffer(&.{
        .label = "vertex",
        .usage = .{ .copy_dst = true, .vertex = true },
        .size = @sizeOf(f32) * vertexData.len,
    });

    // 将 CPU 内存中的数据复制到 GPU 内存中
    mach.core.queue.writeBuffer(vertexBuffer, 0, &vertexData);

    const vertexLayout = mach.gpu.VertexBufferLayout.init(.{
        .array_stride = @sizeOf(f32) * 5,
        .attributes = &.{
            .{ .format = .float32x2, .offset = 0, .shader_location = 0 },
            .{ .format = .float32x3, .offset = @sizeOf(f32) * 2, .shader_location = 1 },
        },
    });

    const vertex = mach.gpu.VertexState.init(.{
        .module = module,
        .entry_point = "vs_main",
        .buffers = &.{vertexLayout},
    });

    // 片段着色器状态
    const fragment = mach.gpu.FragmentState.init(.{
        .module = module,
        .entry_point = "fs_main",
        .targets = &.{.{ .format = mach.core.descriptor.format }},
    });

    // 创建渲染管线
    const descriptor = mach.gpu.RenderPipeline.Descriptor{
        .fragment = &fragment,
        .vertex = vertex,
    };
    const pipeline = device.createRenderPipeline(&descriptor);
    return .{ .vertexBuffer = vertexBuffer, .pipeline = pipeline };
}
```

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");

pub const App = @This();
const width = 600;
const height = 480;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderPipeline: *mach.gpu.RenderPipeline = undefined,
vertexBuffer: *mach.gpu.Buffer = undefined,
bindGroup: *mach.gpu.BindGroup = undefined,

pub fn init(app: *App) !void {
    try mach.core.init(.{
        .title = "学习 WebGPU",
        .size = .{ .width = width, .height = height },
    });
    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);
    const device = mach.core.device;

    var vec3 = mach.math.Vec3.init(1, -1, 1);
    var model = mach.math.Mat4x4.scale(vec3);

    vec3 = mach.math.Vec3.init(-1, -1, 0);
    model = model.mul(&mach.math.Mat4x4.translate(vec3));

    vec3 = mach.math.Vec3.init(2, 2, 1);
    model = model.mul(&mach.math.Mat4x4.scale(vec3));

    const x = 1.0 / @as(f32, @floatCast(width));
    const y = 1.0 / @as(f32, @floatCast(height));
    vec3 = mach.math.Vec3.init(x, y, 1);
    model = model.mul(&mach.math.Mat4x4.scale(vec3));

    vec3 = mach.math.Vec3.init(0, 0, 0);
    const angle = mach.math.degreesToRadians(f32, 0);
    model = model.mul(&mach.math.Mat4x4.translate(vec3));
    model = model.mul(&mach.math.Mat4x4.rotateZ(angle));
    vec3 = mach.math.Vec3.init(1, 1, 1);
    model = model.mul(&mach.math.Mat4x4.scale(vec3));

    const byteSize = @sizeOf(@TypeOf(model));
    const modelBuffer = device.createBuffer(&.{
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = byteSize,
    });
    device.getQueue().writeBuffer(modelBuffer, 0, (&model)[0..1]);

    const renderContext = render.createRenderPipeline();
    app.renderPipeline = renderContext.pipeline;
    app.vertexBuffer = renderContext.vertexBuffer;

    const Entry = mach.gpu.BindGroup.Entry;
    app.bindGroup = device.createBindGroup(
        &mach.gpu.BindGroup.Descriptor.init(.{
            .layout = app.renderPipeline.getBindGroupLayout(0),
            .entries = &.{
                Entry.buffer(0, modelBuffer, 0, byteSize),
            },
        }),
    );
}

pub fn deinit(app: *App) void {
    app.vertexBuffer.release();
    app.bindGroup.release();
    app.renderPipeline.release();
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
    pass.setPipeline(app.renderPipeline);
    pass.setVertexBuffer(0, app.vertexBuffer, 0, app.vertexBuffer.getSize());
    pass.setBindGroup(0, app.bindGroup, &.{});

    pass.draw(3, 1, 0, 0);
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

![屏幕坐标系][1]

## 总结

将 NDC 坐标扩展成了屏幕坐标。

[1]: images/webgpu25.png

## 附录
