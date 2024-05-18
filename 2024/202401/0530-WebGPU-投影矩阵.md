# 0530-WebGPU-投影矩阵

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

之前通过四个矩阵的变换得到了屏幕坐标，可以合并四个矩阵为一个投影矩阵。

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

    const x = 2.0 / @as(f32, @floatCast(width));
    const y = -2.0 / @as(f32, @floatCast(height));
    const projection = mach.math.Mat4x4.init(
        &mach.math.Mat4x4.RowVec.init(x, 0, 0, -1),
        &mach.math.Mat4x4.RowVec.init(0, y, 0, 1),
        &mach.math.Mat4x4.RowVec.init(0, 0, 1, 0),
        &mach.math.Mat4x4.RowVec.init(0, 0, 0, 1),
    );

    var vec3 = mach.math.Vec3.init(0, 0, 0);
    const angle = mach.math.degreesToRadians(f32, 0);
    var model = projection.mul(&mach.math.Mat4x4.translate(vec3));
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

![投影矩阵][1]

## 总结

合并四个矩阵为一个投影矩阵。

[1]: images/webgpu26.png

## 附录
