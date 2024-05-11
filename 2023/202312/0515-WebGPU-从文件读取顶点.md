# 0515-WebGPU-从文件读取顶点

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

从文件读取顶点数据，然后进行渲染。

## shader.wgsl

无变化。

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};

const BufferData = struct {
    vertex: []f32,
    // 顶点必须要 4 字节的倍数，为了简单，直接使用 u32
    index: []u32,
};

renderPipeline: *mach.gpu.RenderPipeline,
vertexBuffer: *mach.gpu.Buffer,
indexBuffer: *mach.gpu.Buffer,
indexCount: u32 = 0,

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

    const bufferDataString = @embedFile("shader/bufferData.json");
    var bufferData = try parseJson(BufferData, bufferDataString);
    defer bufferData.deinit();

    // 创建顶点缓冲区
    app.vertexBuffer = device.createBuffer(&.{
        .label = "vertex",
        .usage = .{ .copy_dst = true, .vertex = true },
        .size = bufferData.value.vertex.len * @sizeOf(f32),
    });

    // 创建索引缓冲区
    app.indexCount = @intCast(bufferData.value.index.len);
    app.indexBuffer = device.createBuffer(&.{
        .label = "index",
        .usage = .{ .copy_dst = true, .index = true },
        .size = app.indexCount * @sizeOf(u32),
    });

    // 将 CPU 内存中的数据复制到 GPU 内存中
    mach.core.queue.writeBuffer(app.vertexBuffer, 0, bufferData.value.vertex);
    mach.core.queue.writeBuffer(app.indexBuffer, 0, bufferData.value.index);

    // 编译 shader
    const source = @embedFile("shader/shader.wgsl");
    const shader = device.createShaderModuleWGSL("shader.wgsl", source);
    defer shader.release();

    // 顶点的布局
    const vertexLayout = mach.gpu.VertexBufferLayout.init(.{
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
        .buffers = &.{vertexLayout},
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

    //  设置顶点缓冲和索引缓冲
    pass.setVertexBuffer(0, app.vertexBuffer, 0, app.vertexBuffer.getSize());
    pass.setIndexBuffer(app.indexBuffer, .uint32, 0, app.indexBuffer.getSize());

    // 修改了 draw 方法为 drawIndexed
    pass.drawIndexed(app.indexCount, 1, 0, 0, 0);
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

fn parseJson(comptime T: type, json: []const u8) !std.json.Parsed(T) {
    return try std.json.parseFromSlice(T, gpa.allocator(), json, .{});
}
```

## bufferData.json

```json
{ "vertex": [ 0.5, 0.0, 0.0, 0.353, 0.612, 1.0, 0.866, 0.0, 0.353, 0.612, 0.0,
0.866, 0.0, 0.353, 0.612, 0.75, 0.433, 0.0, 0.4, 0.7, 1.25, 0.433, 0.0, 0.4,
0.7, 1.0, 0.866, 0.0, 0.4, 0.7, 1.0, 0.0, 0.0, 0.463, 0.8, 1.25, 0.433, 0.0,
0.463, 0.8, 0.75, 0.433, 0.0, 0.463, 0.8, 1.25, 0.433, 0.0, 0.525, 0.91, 1.375,
 0.65, 0.0, 0.525, 0.91, 1.125, 0.65, 0.0, 0.525, 0.91, 1.125, 0.65, 0.0,
 0.576, 1.0, 1.375, 0.65, 0.0, 0.576, 1.0, 1.25, 0.866, 0.0, 0.576, 1.0 ],
 "index": [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 ] }
```

## 效果

![文件读取顶点][1]

## 总结

从文件中读取顶点数据和索引，然后进行渲染。

[1]: images/webgpu12.png

## 附录
