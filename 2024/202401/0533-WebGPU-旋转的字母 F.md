# 0533-WebGPU-旋转的字母 F

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

将显示的字母 F 同时绕着 x，y 和 z 轴旋转。

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
const depth = 400;

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

    const x = 2.0 / @as(f32, width);
    const y = -2.0 / @as(f32, height);
    const z = 0.5 / @as(f32, depth);
    app.projection = mach.math.Mat4x4.init(
        &mach.math.Mat4x4.RowVec.init(x, 0, 0, -1),
        &mach.math.Mat4x4.RowVec.init(0, y, 0, 1),
        &mach.math.Mat4x4.RowVec.init(0, 0, z, 0.5),
        &mach.math.Mat4x4.RowVec.init(0, 0, 0, 1),
    );

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
    });

    // 命令编码器
    const encoder = mach.core.device.createCommandEncoder(null);
    defer encoder.release();
    const pass = encoder.beginRenderPass(&renderPass);

    const angle: f32 = mach.math.degreesToRadians(f32, app.timer.read() * 20);
    var vec = mach.math.Vec3.init(300, 200, 0);
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

    const size = @sizeOf(@TypeOf(render.indexData));
    pass.setIndexBuffer(app.renderContext.indexBuffer, .uint32, 0, size);
    pass.setBindGroup(0, app.bindGroup, &.{});

    pass.drawIndexed(render.indexData.len, 1, 0, 0, 0);
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

![旋转的字母 F][1]

## 总结

旋转的字母 F。

[1]: images/webgpu29.webp

## 附录
