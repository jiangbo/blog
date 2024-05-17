# 0525-WebGPU-2D 缩放

## 环境

- Time 2024-05-17
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

将三角形在 2D 平面内进行缩放。

## shader.wgsl

```wgsl
struct Model {
    offset: vec2f, // 平移
    rotate: vec2f, // 旋转
    scale: vec2f, // 缩放
};

@binding(0) @group(0) var<uniform> model: Model;

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
    // 缩放
    var x = in.position.x * model.scale.x;
    var y = in.position.y * model.scale.y;
    // 旋转
    x = x * model.rotate.x - y * model.rotate.y;
    y = x * model.rotate.y + y * model.rotate.x;
    // 平移
    x = x + model.offset.x;
    y = y + model.offset.y;

    // 翻转 Y 轴，来适合屏幕坐标系
    out.position = vec4f(x, -y, in.position.z, in.position.w);
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
const Model = extern struct {
    offset: [2]f32,
    rotate: [2]f32,
    scale: [2]f32,
};

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderPipeline: *mach.gpu.RenderPipeline = undefined,
vertexBuffer: *mach.gpu.Buffer = undefined,
bindGroup: *mach.gpu.BindGroup = undefined,

pub fn init(app: *App) !void {
    try mach.core.init(.{
        .title = "学习 WebGPU",
        .size = .{ .width = 600, .height = 480 },
    });
    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);
    const device = mach.core.device;

    const modelBuffer = device.createBuffer(&.{
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = @sizeOf(Model),
    });

    // 旋转角度
    const rotate: f32 = 0;
    const angle = rotate * std.math.pi / 180;
    var modelData = Model{
        .offset = .{ 0.2, 0.2 },
        .rotate = .{ @cos(angle), @sin(angle) },
        .scale = .{ 2, 2 },
    };
    const data: [*]f32 = @ptrCast(&modelData);
    device.getQueue().writeBuffer(modelBuffer, 0, data[0..6]);

    const renderContext = render.createRenderPipeline();
    app.renderPipeline = renderContext.pipeline;
    app.vertexBuffer = renderContext.vertexBuffer;

    const Entry = mach.gpu.BindGroup.Entry;
    app.bindGroup = device.createBindGroup(
        &mach.gpu.BindGroup.Descriptor.init(.{
            .layout = app.renderPipeline.getBindGroupLayout(0),
            .entries = &.{
                Entry.buffer(0, modelBuffer, 0, @sizeOf(Model)),
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

![2D 缩放][1]

## 总结

将 2D 三角形扩大了两倍。

[1]: images/webgpu21.png

## 附录
