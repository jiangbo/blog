# 0517-WebGPU-Uniform 缓冲区

## 环境

- Time 2024-05-12
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://eliemichel.github.io/LearnWebGPU/index.html>

### 目标

使用 Uniform 缓冲区来进行 Uniform 数据的传递，并根据时间修改。

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

@group(0) @binding(0) var<uniform> uTime: f32;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // 屏幕比率矫正，缩放
    let ratio = 800.0 / 600.0;
    // 平移
    var offset = vec2f(-0.6875, -0.463);
    offset += 0.3 * vec2f(cos(uTime), sin(uTime));
    let y = (in.position.y + offset.y) * ratio;
    out.position = vec4f(in.position.x + offset.x, y, 0.0, 1.0);
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

const BufferData = struct {
    vertex: []f32,
    // 顶点必须要 4 字节的倍数，为了简单，直接使用 u32
    index: []u32,
};

renderPipeline: *mach.gpu.RenderPipeline,
vertexBuffer: *mach.gpu.Buffer,
indexBuffer: *mach.gpu.Buffer,
indexCount: u32 = 0,
uniformBuffer: *mach.gpu.Buffer,
bindGroup: *mach.gpu.BindGroup,
timer: mach.Timer,

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

    //  创建 uniform 缓冲区
    app.uniformBuffer = device.createBuffer(&.{
        .label = "bind",
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = @sizeOf(f32),
    });

    // 将 CPU 内存中的数据复制到 GPU 内存中
    mach.core.queue.writeBuffer(app.vertexBuffer, 0, bufferData.value.vertex);
    mach.core.queue.writeBuffer(app.indexBuffer, 0, bufferData.value.index);
    mach.core.queue.writeBuffer(app.uniformBuffer, 0, &[1]f32{1});

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

    // 创建绑定组布局
    const Entry = mach.gpu.BindGroupLayout.Entry;
    const entry = Entry.buffer(0, .{ .vertex = true }, .uniform, true, @sizeOf(f32));
    const bindGroupLayout = device.createBindGroupLayout(
        &mach.gpu.BindGroupLayout.Descriptor.init(.{
            .label = "bindgroup layout",
            .entries = &.{entry},
        }),
    );

    // 创建绑定组
    app.bindGroup = mach.core.device.createBindGroup(
        &mach.gpu.BindGroup.Descriptor.init(.{
            .layout = bindGroupLayout,
            .entries = &.{
                mach.gpu.BindGroup.Entry.buffer(0, app.uniformBuffer, 0, @sizeOf(f32)),
            },
        }),
    );

    // 创建渲染管线
    app.renderPipeline = device.createRenderPipeline(&.{
        .vertex = vertex,
        .fragment = &fragment,
        .layout = device.createPipelineLayout(
            &mach.gpu.PipelineLayout.Descriptor.init(.{
                .label = "pipeline layout",
                .bind_group_layouts = &.{bindGroupLayout},
            }),
        ),
    });

    app.timer = try mach.Timer.start();
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

    const time = app.timer.read();
    mach.core.queue.writeBuffer(app.uniformBuffer, 0, &[1]f32{time});

    //  设置顶点缓冲和索引缓冲
    pass.setVertexBuffer(0, app.vertexBuffer, 0, app.vertexBuffer.getSize());
    pass.setIndexBuffer(app.indexBuffer, .uint32, 0, app.indexBuffer.getSize());
    pass.setBindGroup(0, app.bindGroup, &.{0});

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

## 效果

![Uniform 变量][1]

## 总结

使用 Uniform 缓冲区来进行 Uniform 数据的传递。

[1]: images/webgpu14.gif

## 附录
