# 0512-WebGPU-分离颜色数据

## 环境

- Time 2024-05-11
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://eliemichel.github.io/LearnWebGPU/index.html>

### 目标

之前是把颜色和顶点信息放在一个缓冲区中，下面分离颜色和顶点数据。

## shader.wgsl

shader.wgsl 无变化。

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const vertexData = [_]f32{
    0.5,  0.5, //
    0.5,  -0.5,
    -0.5, -0.5,
    -0.5, -0.5,
    -0.5, 0.5,
    0.5,  0.5,
};

const colorData = [_]f32{
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0, 1.0, 0.0,
    1.0, 0.0, 0.0,
};

renderPipeline: *mach.gpu.RenderPipeline,
vertexBuffer: *mach.gpu.Buffer,
colorBuffer: *mach.gpu.Buffer,

pub fn init(app: *App) !void {

    // 定义了窗口的宽和高，以及窗口的标题
    try mach.core.init(.{
        .size = .{ .width = 800, .height = 600 },
        .title = "学习 WebGPU",
    });

    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);
    const device = mach.core.device;

    // 创建顶点缓冲区
    app.vertexBuffer = device.createBuffer(&.{
        .label = "vertex",
        .usage = .{ .copy_dst = true, .vertex = true },
        .size = @sizeOf(@TypeOf(vertexData)),
    });

    // 创建颜色缓冲区
    app.colorBuffer = device.createBuffer(&.{
        .label = "color",
        .usage = .{ .copy_dst = true, .vertex = true },
        .size = @sizeOf(@TypeOf(colorData)),
    });

    // 将 CPU 内存中的数据复制到 GPU 内存中
    mach.core.queue.writeBuffer(app.vertexBuffer, 0, &vertexData);
    mach.core.queue.writeBuffer(app.colorBuffer, 0, &colorData);

    // 编译 shader
    const source = @embedFile("shader/shader.wgsl");
    const shader = device.createShaderModuleWGSL("shader.wgsl", source);
    defer shader.release();

    // 顶点的布局
    const vertexLayout = mach.gpu.VertexBufferLayout.init(.{
        .array_stride = @sizeOf(f32) * 2,
        .attributes = &.{
            .{ .format = .float32x2, .offset = 0, .shader_location = 0 },
        },
    });

    // 颜色布局
    const colorLayout = mach.gpu.VertexBufferLayout.init(.{
        .array_stride = @sizeOf(f32) * 3,
        .attributes = &.{
            .{ .format = .float32x3, .offset = 0, .shader_location = 1 },
        },
    });

    // 顶点着色器状态
    const vertex = mach.gpu.VertexState.init(.{
        .module = shader,
        .entry_point = "vs_main",
        // 分离颜色和顶点数据，传递给渲染管线
        .buffers = &.{ vertexLayout, colorLayout },
    });

    // 片段着色器状态
    const fragment = mach.gpu.FragmentState.init(.{
        .module = shader,
        .entry_point = "fs_main",
        .targets = &.{.{ .format = mach.core.descriptor.format }},
    });

    // 创建渲染管线
    app.renderPipeline = device.createRenderPipeline(&.{
        .vertex = vertex,
        .fragment = &fragment,
    });
}

pub fn deinit(app: *App) void {
    app.vertexBuffer.release();
    app.renderPipeline.release();
    mach.core.deinit();
    _ = gpa.deinit();
}

pub fn update(app: *App) !bool {

    // 检查窗口是否需要关闭
    var iterator = mach.core.pollEvents();
    while (iterator.next()) |event| if (event == .close) return true;

    // 清屏使用
    const view = mach.core.swap_chain.getCurrentTextureView().?;
    const colorAttachment = mach.gpu.RenderPassColorAttachment{
        .view = view,
        .clear_value = std.mem.zeroes(mach.gpu.Color),
        .load_op = .clear,
        .store_op = .store,
    };

    const renderPass = mach.gpu.RenderPassDescriptor.init(.{
        .color_attachments = &.{colorAttachment},
    });

    // 命令编码器
    const encoder = mach.core.device.createCommandEncoder(null);
    const pass = encoder.beginRenderPass(&renderPass);
    // 绘制
    pass.setPipeline(app.renderPipeline);

    //  设置顶点缓冲的位置
    pass.setVertexBuffer(0, app.vertexBuffer, 0, @sizeOf(@TypeOf(vertexData)));
    pass.setVertexBuffer(1, app.colorBuffer, 0, @sizeOf(@TypeOf(colorData)));
    // 六个点，画两个三角形
    pass.draw(vertexData.len / 2, 2, 0, 0);
    pass.end();
    pass.release();

    var command = encoder.finish(null);
    encoder.release();

    // 提交命令
    mach.core.queue.submit(&.{command});
    command.release();
    mach.core.swap_chain.present();
    view.release();

    // 不退出渲染循环
    return false;
}
```

## 效果

![分离顶点和颜色][1]

## 总结

分离颜色和顶点数据，单独把颜色信息传递给了着色器。

[1]: images/webgpu09.png

## 附录
