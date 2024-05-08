# 0504-WebGPU-显示三角形

## 环境

- Time 2024-05-08
- Zig 0.12.0-dev.3180+83e578a18
- Windows 11

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>

### 目标

基于之前设置好的环境，使用 zig 的 mach 框架来显示一个三角形。

## shader.wgsl

对于 wgsl 语法，先根据例子抄过来。如果有需要，后面再学，看起来和 Rust 有点像。

```wgsl
@vertex
fn vs_main(@builtin(vertex_index) VertexIndex : u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5, -0.5)
    );
    return vec4<f32>(pos[VertexIndex], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 1, 1);
}
```

## init

```zig
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

    // 初始化计时器，用于计算帧率
    app.timer = try mach.core.Timer.start();
}
```

## update

WebGPU 和 OpenGL 有点像，不过还是有点差别。

```zig
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
    pass.draw(3, 1, 0, 0);
    pass.end();
    pass.release();

    var command = encoder.finish(null);
    encoder.release();

    // 提交命令
    mach.core.queue.submit(&.{command});
    command.release();
    mach.core.swap_chain.present();
    view.release();

    // 在窗口的标题栏显示帧率
    if (app.timer.read() >= 1.0) {
        app.timer.reset();
        try mach.core.printTitle("[ {d}fps ] [ Input {d}hz ]", .{
            mach.core.frameRate(),
            mach.core.inputRate(),
        });
    }

    // 不退出渲染循环
    return false;
}
```

## 效果

![显示三角形][1]

## 总结

使用 WebGPU 显示了一个三角形。

[1]: images/webgpu02.png

## 附录
