# 0535-WebGPU-旋转的彩色字母

## 环境

- Time 2024-05-19
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

给字母 F 每个面添加不同的颜色，可以更好地观察 3D 效果。

## shader.wgsl

```wgsl
@binding(0) @group(0) var<uniform> model: mat4x4f;

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

    out.position = model * in.position;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}
```

## render.zig

```zig
const std = @import("std");
const mach = @import("mach");

pub const RenderContext = struct {
    vertexBuffer: *mach.gpu.Buffer,
    vertexCount: u32,
    pipeline: *mach.gpu.RenderPipeline,

    pub fn release(self: *RenderContext) void {
        self.vertexBuffer.release();
        self.pipeline.release();
    }
};

pub const positions = [_]f32{
    // left column
    0,   0,   0,
    30,  0,   0,
    0,   150, 0,
    30,  150, 0,

    // top rung
    30,  0,   0,
    100, 0,   0,
    30,  30,  0,
    100, 30,  0,

    // middle rung
    30,  60,  0,
    70,  60,  0,
    30,  90,  0,
    70,  90,  0,

    // left column back
    0,   0,   30,
    30,  0,   30,
    0,   150, 30,
    30,  150, 30,

    // top rung back
    30,  0,   30,
    100, 0,   30,
    30,  30,  30,
    100, 30,  30,

    // middle rung back
    30,  60,  30,
    70,  60,  30,
    30,  90,  30,
    70,  90,  30,
};

pub const indices = [_]u32{
    // front
    0, 1, 2, 2, 1, 3, // left column
    4, 5, 6, 6, 5, 7, // top run
    8, 9, 10, 10, 9, 11, // middle run

    // back
    12, 13, 14, 14, 13, 15, // left column back
    16, 17, 18, 18, 17, 19, // top run back
    20, 21, 22, 22, 21, 23, // middle run back
    0, 5, 12, 12, 5, 17, // top
    5, 7, 17, 17, 7, 19, // top rung right
    6, 7, 18, 18, 7, 19, // top rung bottom
    6, 8, 18, 18, 8, 20, // between top and middle rung
    8, 9, 20, 20, 9, 21, // middle rung top
    9, 11, 21, 21, 11, 23, // middle rung right
    10, 11, 22, 22, 11, 23, // middle rung bottom
    10, 3, 22, 22, 3, 15, // stem right
    2, 3, 14, 14, 3, 15, // bottom
    0, 2, 12, 12, 2, 14, // left
};

const quadColors = [_]u8{
    200, 70, 120, // left column front
    200, 70, 120, // top rung front
    200, 70, 120, // middle rung front
    80, 70, 200, // left column back
    80, 70, 200, // top rung back
    80, 70, 200, // middle rung back
    70, 200, 210, // top
    160, 160, 220, // top rung right
    90, 130, 110, // top rung bottom
    200, 200, 70, // between top and middle rung
    210, 100, 70, // middle rung top
    210, 160, 70, // middle rung right
    70, 180, 210, // middle rung bottom
    100, 70, 210, // stem right
    76, 210, 100, // bottom
    140, 210, 80, // left
};

var vertexData: [indices.len * 4]f32 = undefined;
var colorData: [*]u8 = @as([*]u8, @ptrCast(&vertexData));

pub fn createRenderPipeline() RenderContext {
    const device = mach.core.device;

    for (0..indices.len) |i| {
        const positionNdx = indices[i] * 3;
        const position = positions[positionNdx .. positionNdx + 3];
        @memcpy(vertexData[i * 4 ..][0..3], position);

        const quadNdx = (i / 6 | 0) * 3;
        const color = quadColors[quadNdx .. quadNdx + 3];
        @memcpy(colorData[i * 16 + 12 ..][0..3], color);
        colorData[i * 16 + 15] = 255; // set A
    }

    // 编译 shader
    const source = @embedFile("shader.wgsl");
    const module = device.createShaderModuleWGSL("shader.wgsl", source);
    defer module.release();

    // 顶点缓冲区
    const vertexBuffer = device.createBuffer(&.{
        .label = "vertex",
        .usage = .{ .copy_dst = true, .vertex = true },
        .size = @sizeOf(@TypeOf(vertexData)),
    });

    // 将 CPU 内存中的数据复制到 GPU 内存中
    mach.core.queue.writeBuffer(vertexBuffer, 0, &vertexData);

    const vertexLayout = mach.gpu.VertexBufferLayout.init(.{
        .array_stride = @sizeOf(f32) * 4,
        .attributes = &.{
            .{ .shader_location = 0, .offset = 0, .format = .float32x3 }, // position
            .{ .shader_location = 1, .offset = 12, .format = .unorm8x4 }, // color
        },
    });

    const vertex = mach.gpu.VertexState.init(.{
        .module = module,
        .entry_point = "vs_main",
        .buffers = &.{vertexLayout},
    });

    // 片段着色器状态
    const fragment = mach.gpu.FragmentState.init(.{
        .module = module,
        .entry_point = "fs_main",
        .targets = &.{.{ .format = mach.core.descriptor.format }},
    });

    // 创建渲染管线
    const descriptor = mach.gpu.RenderPipeline.Descriptor{
        .fragment = &fragment,
        .vertex = vertex,
    };
    const pipeline = device.createRenderPipeline(&descriptor);
    return .{
        .vertexBuffer = vertexBuffer,
        .vertexCount = indices.len,
        .pipeline = pipeline,
    };
}
```

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");

pub const App = @This();
const width = 640;
const height = 480;
const depth = 400;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderContext: render.RenderContext = undefined,
bindGroup: *mach.gpu.BindGroup = undefined,
projection: mach.math.Mat4x4 = undefined,
modelBuffer: *mach.gpu.Buffer = undefined,
timer: mach.Timer = undefined,

pub fn init(app: *App) !void {
    try mach.core.init(.{
        .title = "学习 WebGPU",
        .size = .{ .width = width, .height = height },
    });
    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);
    const device = mach.core.device;

    const x = 2.0 / @as(f32, width);
    const y = -2.0 / @as(f32, height);
    const z = 0.5 / @as(f32, depth);
    app.projection = mach.math.Mat4x4.init(
        &mach.math.Mat4x4.RowVec.init(x, 0, 0, -1),
        &mach.math.Mat4x4.RowVec.init(0, y, 0, 1),
        &mach.math.Mat4x4.RowVec.init(0, 0, z, 0.5),
        &mach.math.Mat4x4.RowVec.init(0, 0, 0, 1),
    );

    const byteSize = @sizeOf(@TypeOf(app.projection));
    app.modelBuffer = device.createBuffer(&.{
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = byteSize,
    });

    app.renderContext = render.createRenderPipeline();

    const Entry = mach.gpu.BindGroup.Entry;
    app.bindGroup = device.createBindGroup(
        &mach.gpu.BindGroup.Descriptor.init(.{
            .layout = app.renderContext.pipeline.getBindGroupLayout(0),
            .entries = &.{
                Entry.buffer(0, app.modelBuffer, 0, byteSize),
            },
        }),
    );

    app.timer = try mach.Timer.start();
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

    const angle: f32 = mach.math.degreesToRadians(f32, app.timer.read() * 20);
    var vec = mach.math.Vec3.init(300, 200, 0);
    var model = app.projection.mul(&mach.math.Mat4x4.translate(vec));

    model = model.mul(&mach.math.Mat4x4.rotateX(angle));
    model = model.mul(&mach.math.Mat4x4.rotateY(angle));
    model = model.mul(&mach.math.Mat4x4.rotateZ(angle));

    vec = mach.math.Vec3.init(1, 1, 1);
    model = model.mul(&mach.math.Mat4x4.scale(vec));
    mach.core.queue.writeBuffer(app.modelBuffer, 0, (&model)[0..1]);

    // 设置渲染管线
    pass.setPipeline(app.renderContext.pipeline);
    const vertexBuffer = app.renderContext.vertexBuffer;
    pass.setVertexBuffer(0, vertexBuffer, 0, vertexBuffer.getSize());

    pass.setBindGroup(0, app.bindGroup, &.{});

    pass.draw(app.renderContext.vertexCount, 1, 0, 0);
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

![彩色字母][1]

## 总结

给字母 F 的每个面添加一个颜色。

[1]: images/webgpu31.webp

## 附录
