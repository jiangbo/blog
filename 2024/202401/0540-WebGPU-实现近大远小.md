# 0540-WebGPU-实现近大远小

## 环境

- Time 2024-05-19
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

在 3D 空间，人眼观察到的效果应该是近大远小。

## shader.wgsl

```zig
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

    let pos = model * in.position;
    let zToDivideBy = 1.0 + pos.z * 10;
    // WebGPU 会自动将 xyzw 除以 w，实现近大远小的效果
    out.position = vec4f(pos.xyz, zToDivideBy);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}
```

## render.zig

无变化。

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");

pub const App = @This();
const width = 640;
const height = 480;
const depth = -1000;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderContext: render.RenderContext = undefined,
bindGroup: *mach.gpu.BindGroup = undefined,
projection: mach.math.Mat4x4 = undefined,
modelBuffer: *mach.gpu.Buffer = undefined,
timer: mach.Timer = undefined,

pub fn init(app: *App) !void {
    try mach.core.init(.{
        .title = "学习 WebGPU",
        .size = .{ .width = width, .height = height },
    });
    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);
    const device = mach.core.device;

    app.projection = mach.math.Mat4x4.projection2D(.{
        .left = 0,
        .right = width,
        .bottom = height,
        .top = 0,
        .near = 1200,
        .far = -1000,
    });

    const byteSize = @sizeOf(@TypeOf(app.projection));
    app.modelBuffer = device.createBuffer(&.{
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = byteSize,
    });

    app.renderContext = render.createRenderPipeline();

    const Entry = mach.gpu.BindGroup.Entry;
    app.bindGroup = device.createBindGroup(
        &mach.gpu.BindGroup.Descriptor.init(.{
            .layout = app.renderContext.pipeline.getBindGroupLayout(0),
            .entries = &.{
                Entry.buffer(0, app.modelBuffer, 0, byteSize),
            },
        }),
    );

    app.timer = try mach.Timer.start();
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
        .depth_stencil_attachment = &.{
            .view = app.renderContext.depthView,
            .depth_clear_value = 1.0,
            .depth_load_op = .clear,
            .depth_store_op = .store,
            .stencil_read_only = .true,
        },
    });

    // 命令编码器
    const encoder = mach.core.device.createCommandEncoder(null);
    defer encoder.release();
    const pass = encoder.beginRenderPass(&renderPass);

    const angle: f32 = mach.math.degreesToRadians(f32, app.timer.read() * 20);
    var vec = mach.math.Vec3.init(300, 200, -800);
    var model = app.projection.mul(&mach.math.Mat4x4.translate(vec));

    model = model.mul(&mach.math.Mat4x4.rotateX(angle));
    model = model.mul(&mach.math.Mat4x4.rotateY(angle));
    model = model.mul(&mach.math.Mat4x4.rotateZ(angle));

    vec = mach.math.Vec3.init(3, 3, 3);
    model = model.mul(&mach.math.Mat4x4.scale(vec));
    mach.core.queue.writeBuffer(app.modelBuffer, 0, (&model)[0..1]);

    // 设置渲染管线
    pass.setPipeline(app.renderContext.pipeline);
    const vertexBuffer = app.renderContext.vertexBuffer;
    pass.setVertexBuffer(0, vertexBuffer, 0, vertexBuffer.getSize());

    pass.setBindGroup(0, app.bindGroup, &.{});

    pass.draw(app.renderContext.vertexCount, 1, 0, 0);
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

![近大远小][1]

## 总结

实现近大远小的效果。

[1]: images/webgpu36.webp

## 附录
