# 0530-WebGPU-3x4 矩阵

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

使用 3x4 矩阵来解决内存对齐的问题。

## shader.wgsl

无变化。

## render.zig

无变化

## mat.zig

无变化。

## main.zig

去掉内存对齐的处理。

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");
const mat = @import("mat.zig");

pub const App = @This();
const width = 640;
const height = 480;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderContext: render.RenderContext = undefined,
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

    const byteSize = 48;
    const modelBuffer = device.createBuffer(&.{
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = byteSize,
    });

    const projection = [_]f32{
        2.0 / @as(f32, width), 0,                       0, 0,
        0,                     -2.0 / @as(f32, height), 0, 0,
        -1,                    1,                       1, 0,
    };

    const angle: f32 = 0 * std.math.pi / 180.0;
    const offset = mat.offset(200, 100);
    const rotate = mat.rotate(angle);
    const scale = mat.scale(2, 2);

    var model = mat.mul(projection, offset);
    model = mat.mul(model, rotate);
    model = mat.mul(model, scale);

    device.getQueue().writeBuffer(modelBuffer, 0, &model);

    app.renderContext = render.createRenderPipeline();

    const Entry = mach.gpu.BindGroup.Entry;
    app.bindGroup = device.createBindGroup(
        &mach.gpu.BindGroup.Descriptor.init(.{
            .layout = app.renderContext.pipeline.getBindGroupLayout(0),
            .entries = &.{
                Entry.buffer(0, modelBuffer, 0, byteSize),
            },
        }),
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
    });

    // 命令编码器
    const encoder = mach.core.device.createCommandEncoder(null);
    defer encoder.release();
    const pass = encoder.beginRenderPass(&renderPass);
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

![3x4 矩阵][1]

## 总结

将之前的 3x3 矩阵改成 3x4 矩阵，解决内存对齐的问题。

[1]: images/webgpu26.png

## 附录
