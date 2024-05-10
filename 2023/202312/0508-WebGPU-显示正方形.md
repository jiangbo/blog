# 0508-WebGPU-显示正方形

## 环境

- Time 2024-05-10
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>

### 目标

通过画两个三角形的形式，画出一个正方形。

## shader.wgsl

```wgsl
struct PosAndColor {
    @builtin(position) pos : vec4f,
    @location(0) color : vec4f
};

@vertex
fn vs_main(@builtin(vertex_index) VertexIndex : u32) -> PosAndColor {
    let pos = array(
        vec2f( 0.5,  0.5),
        vec2f( 0.5, -0.5),
        vec2f(-0.5, -0.5),
        vec2f(-0.5, -0.5),
        vec2f(-0.5,  0.5),
        vec2f( 0.5,  0.5)
    );

    let pos4f = vec4f(pos[VertexIndex], 0.0, 1.0);

    return PosAndColor(pos4f, vec4f(0.9, 0.5, 0.7, 1.0));
}

@fragment
fn fs_main(in: PosAndColor) -> @location(0) vec4f {
    return in.color;
}
```

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderPipeline: *mach.gpu.RenderPipeline,

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

    // 编译 shader
    const source = @embedFile("shader/shader.wgsl");
    const shader = device.createShaderModuleWGSL("shader.wgsl", source);
    defer shader.release();

    // 片段着色器
    const fragment = mach.gpu.FragmentState.init(.{
        .module = shader,
        .entry_point = "fs_main",
        .targets = &.{.{ .format = mach.core.descriptor.format }},
    });

    // 创建渲染管线
    app.renderPipeline = device.createRenderPipeline(&.{
        .vertex = .{ .module = shader, .entry_point = "vs_main" },
        .fragment = &fragment,
    });
}

pub fn deinit(app: *App) void {
    defer _ = gpa.deinit();
    defer mach.core.deinit();
    defer app.renderPipeline.release();
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
    // 六个点，画两个三角形
    pass.draw(6, 2, 0, 0);
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

![正方形][1]

## 总结

通过画两个三角形来显示正方形。

[1]: images/webgpu06.png

## 附录
