# 0511-WebGPU-顶点和颜色

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

之前只把顶点通过参数传给了着色器，其实颜色也可以通过参数传给着色器。

## shader.wgsl

```wgsl
struct VertexInput {
    @location(0) position: vec2f,
    @location(1) color: vec3f,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4f(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1.0);
}
```

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const vertexData = [_]f32{
    0.5,  0.5,  1.0, 0.0, 0.0, //
    0.5,  -0.5, 0.0, 1.0, 0.0,
    -0.5, -0.5, 0.0, 0.0, 1.0,
    -0.5, -0.5, 0.0, 0.0, 1.0,
    -0.5, 0.5,  0.0, 1.0, 0.0,
    0.5,  0.5,  1.0, 0.0, 0.0,
};

renderPipeline: *mach.gpu.RenderPipeline,
vertexBuffer: *mach.gpu.Buffer,

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

    // 将 CPU 内存中的数据复制到 GPU 内存中
    mach.core.queue.writeBuffer(app.vertexBuffer, 0, &vertexData);

    // 编译 shader
    const source = @embedFile("shader/shader.wgsl");
    const shader = device.createShaderModuleWGSL("shader.wgsl", source);
    defer shader.release();

    const vertextLayout = mach.gpu.VertexBufferLayout.init(.{
        // 前面两个是坐标，后面三个是颜色
        .array_stride = @sizeOf(f32) * 5,
        .attributes = &.{
            // 第一个是顶点坐标，偏移从 0 开始
            .{ .format = .float32x2, .offset = 0, .shader_location = 0 },
            // 第二个是颜色，偏移从 2 开始，shader_location 对应 WGSL 中的 location 位置
            .{ .format = .float32x3, .offset = @sizeOf(f32) * 2, .shader_location = 1 },
        },
    });

    // 顶点着色器状态
    const vertex = mach.gpu.VertexState.init(.{
        .module = shader,
        .entry_point = "vs_main",
        .buffers = &.{vertextLayout},
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
    // 六个点，画两个三角形
    pass.draw(vertexData.len / 5, 2, 0, 0);
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

![彩色正方形][1]

## 总结

通过顶点缓冲区，把颜色信息传递给了着色器。

[1]: images/webgpu08.png

## 附录
