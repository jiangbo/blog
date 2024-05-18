# 0528-WebGPU-增加投影矩阵

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

将着色器中的变换移除，使用投影矩阵实现相同的功能。

## shader.wgsl

```wgsl
@binding(0) @group(0) var<uniform> model: mat3x3f;

struct VertexInput {
    @location(0) position: vec4f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let xy = (model * vec3f(in.position.xy, 1)).xy;

    out.position = vec4f(xy, 0.0, 1.0);
    out.color = vec4f(0, 1, 0, 1);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}
```

## render.zig

无变化。

## mat.zig

无变化。

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");
const mat = @import("mat.zig");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderContext: render.RenderContext = undefined,
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

    const byteSize = 48;
    const modelBuffer = device.createBuffer(&.{
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = byteSize,
    });

    const scaleBy1OverResolutionMatrix = mat.scale(1.0 / 640.0, 1.0 / 480.0);
    const scaleBy2Matrix = mat.scale(2, 2);
    const translateByMinus1 = mat.offset(-1, -1);
    const scaleBy1Minus1 = mat.scale(1, -1);

    const angle: f32 = 0 * std.math.pi / 180.0;
    const offset = mat.offset(200, 100);
    const rotate = mat.rotate(angle);
    const scale = mat.scale(2, 2);

    var matrix = mat.mul(scaleBy1Minus1, translateByMinus1);
    matrix = mat.mul(matrix, scaleBy2Matrix);
    matrix = mat.mul(matrix, scaleBy1OverResolutionMatrix);
    matrix = mat.mul(matrix, offset);
    matrix = mat.mul(matrix, rotate);
    matrix = mat.mul(matrix, scale);

    // mat3x3 矩阵应该按照 48 字节对齐
    // 参考：https://www.w3.org/TR/WGSL/#alignment-and-size
    var model: [12]f32 = undefined;
    for (matrix, 0..) |value, index| {
        const mod, const div = .{ index % 3, index / 3 };
        if (mod == 1) model[index] = 0;
        model[index + div] = value;
    }
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

![投影矩阵][1]

## 总结

增加投影矩阵。

[1]: images/webgpu24.png

## 附录
