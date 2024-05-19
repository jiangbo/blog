# 0542-WebGPU-透视投影

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

自带的数学库中没有找到透视投影矩阵，自己跟着例子实现一个透视投影矩阵。

## shader.wgsl

无变化。

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

    const fov = mach.math.degreesToRadians(f32, 100);
    const w: f32 = @as(f32, @floatFromInt(width));
    const aspect: f32 = w / @as(f32, @floatFromInt(height));
    app.projection = perspective(fov, aspect, 1, 2000);

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

fn perspective(fov: f32, aspect: f32, near: f32, far: f32) mach.math.Mat4x4 {
    const f = @tan(std.math.pi * 0.5 - 0.5 * fov);
    const rangeInv = 1 / (near - far);

    return mach.math.Mat4x4.init(
        &mach.math.Mat4x4.RowVec.init(f / aspect, 0, 0, 0),
        &mach.math.Mat4x4.RowVec.init(0, f, 0, 0),
        &mach.math.Mat4x4.RowVec.init(0, 0, far * rangeInv, near * far * rangeInv),
        &mach.math.Mat4x4.RowVec.init(0, 0, -1, 0),
    );
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
    var vec = mach.math.Vec3.init(-65, 0, -120);
    var model = app.projection.mul(&mach.math.Mat4x4.translate(vec));

    model = model.mul(&mach.math.Mat4x4.rotateX(angle));
    model = model.mul(&mach.math.Mat4x4.rotateY(angle));
    model = model.mul(&mach.math.Mat4x4.rotateZ(angle));

    vec = mach.math.Vec3.init(1, 1, 1);
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

![透视投影][1]

## 总结

实现透视投影。

[1]: images/webgpu38.webp

## 附录
