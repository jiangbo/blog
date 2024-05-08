# 0505-WebGPU-多重采样抗锯齿

## 环境

- Time 2024-05-08
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>

### 目标

之前显示的三角形，边缘有小锯齿。使用多重采样抗锯齿（MSAA）来进行平滑。

## 新增变量

```zig
texture: *mach.gpu.Texture,
textureView: *mach.gpu.TextureView,
const sampleCount: u32 = 4;
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
        .multisample = .{ .count = sampleCount },
    });

    // 创建了多重采样的纹理
    app.texture = device.createTexture(&mach.gpu.Texture.Descriptor{
        .size = mach.gpu.Extent3D{
            .width = mach.core.descriptor.width,
            .height = mach.core.descriptor.height,
        },
        .sample_count = sampleCount,
        .format = mach.core.descriptor.format,
        .usage = .{ .render_attachment = true },
    });
    app.textureView = app.texture.createView(null);

    // 初始化计时器，用于计算帧率
    app.timer = try mach.core.Timer.start();
}
```

## update

```zig
pub fn update(app: *App) !bool {

    // 检查窗口是否需要关闭
    var iterator = mach.core.pollEvents();
    while (iterator.next()) |event| switch (event) {
        // 窗口尺寸变化，更新纹理
        .framebuffer_resize => |size| {
            app.texture.release();
            app.texture = mach.core.device.createTexture(&.{
                .size = mach.gpu.Extent3D{
                    .width = size.width,
                    .height = size.height,
                },
                .sample_count = sampleCount,
                .format = mach.core.descriptor.format,
                .usage = .{ .render_attachment = true },
            });

            app.textureView.release();
            app.textureView = app.texture.createView(null);
        },
        .close => return true,
        else => continue,
    };

    // 清屏使用
    const view = mach.core.swap_chain.getCurrentTextureView().?;
    const colorAttachment = mach.gpu.RenderPassColorAttachment{
        .view = app.textureView,
        .resolve_target = view,
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

![MSAA][1]

## 总结

对显示的三角形进行 MSAA。

[1]: images/webgpu03.png

## 附录

### 源码

```zig
const std = @import("std");
const mach = @import("mach");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderPipeline: *mach.gpu.RenderPipeline,
timer: mach.core.Timer,
texture: *mach.gpu.Texture,
textureView: *mach.gpu.TextureView,
const sampleCount: u32 = 4;

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
        .multisample = .{ .count = sampleCount },
    });

    // 创建了多重采样的纹理
    app.texture = device.createTexture(&mach.gpu.Texture.Descriptor{
        .size = mach.gpu.Extent3D{
            .width = mach.core.descriptor.width,
            .height = mach.core.descriptor.height,
        },
        .sample_count = sampleCount,
        .format = mach.core.descriptor.format,
        .usage = .{ .render_attachment = true },
    });
    app.textureView = app.texture.createView(null);

    // 初始化计时器，用于计算帧率
    app.timer = try mach.core.Timer.start();
}

pub fn deinit(app: *App) void {
    defer _ = gpa.deinit();
    defer mach.core.deinit();
    defer app.renderPipeline.release();
}

pub fn update(app: *App) !bool {

    // 检查窗口是否需要关闭
    var iterator = mach.core.pollEvents();
    while (iterator.next()) |event| switch (event) {
        // 窗口尺寸变化，更新纹理
        .framebuffer_resize => |size| {
            app.texture.release();
            app.texture = mach.core.device.createTexture(&.{
                .size = mach.gpu.Extent3D{
                    .width = size.width,
                    .height = size.height,
                },
                .sample_count = sampleCount,
                .format = mach.core.descriptor.format,
                .usage = .{ .render_attachment = true },
            });

            app.textureView.release();
            app.textureView = app.texture.createView(null);
        },
        .close => return true,
        else => continue,
    };

    // 清屏使用
    const view = mach.core.swap_chain.getCurrentTextureView().?;
    const colorAttachment = mach.gpu.RenderPassColorAttachment{
        .view = app.textureView,
        .resolve_target = view,
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
