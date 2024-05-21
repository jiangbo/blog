# 0545-WebGPU-摄像机

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

使用 zlm 矩阵库自带的摄像机。

## shader.wgsl

无变化。

## render.zig

无变化。

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");
const zlm = @import("zlm");

pub const App = @This();
const width = 640;
const height = 480;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderContext: render.RenderContext = undefined,
bindGroup: *mach.gpu.BindGroup = undefined,
timer: mach.Timer = undefined,

pub fn init(app: *App) !void {
    try mach.core.init(.{
        .title = "学习 WebGPU",
        .size = .{ .width = width, .height = height },
    });
    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);

    app.renderContext = render.createRenderPipeline();
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

    const texutureView = mach.core.swap_chain.getCurrentTextureView().?;
    defer texutureView.release();

    const renderPass = mach.gpu.RenderPassDescriptor.init(.{
        .color_attachments = &.{.{
            .view = texutureView,
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

    const fov: f32 = zlm.toRadians(100.0);
    const w = @as(f32, @floatFromInt(width));
    const aspect = w / @as(f32, @floatFromInt(height));
    const projection = zlm.Mat4.createPerspective(fov, aspect, 1, 2000);

    const radius: f32 = 200;
    var angle: f32 = zlm.toRadians(app.timer.read() * 20);
    var view = zlm.Mat4.createAngleAxis(zlm.Vec3.unitY, angle);

    var vec = zlm.Vec3.new(0, 0, radius * 1.5).transformDirection(view);
    const center = zlm.Vec3.new(radius, 0, 0);
    view = zlm.Mat4.createLookAt(vec, center, zlm.Vec3.unitY);

    const vp = view.mul(projection);

    // 设置渲染管线
    pass.setPipeline(app.renderContext.pipeline);
    const vertexBuffer = app.renderContext.vertexBuffer;
    pass.setVertexBuffer(0, vertexBuffer, 0, vertexBuffer.getSize());

    const uniforms = app.renderContext.uniforms;
    for (uniforms, 0..) |value, i| {
        const index = @as(f32, @floatFromInt(i));
        angle = index / uniforms.len * std.math.pi * 2;
        const x = @cos(angle) * radius;
        const z = @sin(angle) * radius;

        vec = zlm.Vec3.new(x, 0, z);
        const mvp = zlm.Mat4.createTranslation(vec).mul(vp);

        mach.core.queue.writeBuffer(value.buffer, 0, &mvp.fields);

        pass.setBindGroup(0, value.bindGroup, &.{});
        pass.draw(app.renderContext.vertexCount, 1, 0, 0);
    }

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

![摄像机][1]

## 总结

使用 zlm 库自带的摄像机来处理旋转。

[1]: images/webgpu41.webp

## 附录
