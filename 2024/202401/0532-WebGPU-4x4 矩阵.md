# 0532-WebGPU-4x4 矩阵

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

将之前的二维转换到三维空间，使用 4x4 矩阵来表示。

## shader.wgsl

```wgsl
@binding(0) @group(0) var<uniform> model: mat4x4f;

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

    out.position = model * in.position;
    out.color = vec4f(0, 1, 0, 1);
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
    indexBuffer: *mach.gpu.Buffer,
    pipeline: *mach.gpu.RenderPipeline,

    pub fn release(self: *RenderContext) void {
        self.vertexBuffer.release();
        self.indexBuffer.release();
        self.pipeline.release();
    }
};

pub const vertexData = [_]f32{
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
};

pub const indexData = [_]u32{
    0, 1, 2, 2, 1, 3, // left column
    4, 5, 6, 6, 5, 7, // top run
    8, 9, 10, 10, 9, 11, // middle run
};

pub fn createRenderPipeline() RenderContext {
    const device = mach.core.device;

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

    // 索引缓冲区
    const indexBuffer = device.createBuffer(&.{
        .label = "index",
        .size = @sizeOf(@TypeOf(indexData)),
        .usage = .{ .index = true, .copy_dst = true },
    });

    // 将 CPU 内存中的数据复制到 GPU 内存中
    mach.core.queue.writeBuffer(vertexBuffer, 0, &vertexData);
    mach.core.queue.writeBuffer(indexBuffer, 0, &indexData);

    const vertexLayout = mach.gpu.VertexBufferLayout.init(.{
        .array_stride = @sizeOf(f32) * 3,
        .attributes = &.{
            .{ .format = .float32x3, .offset = 0, .shader_location = 0 },
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
        .indexBuffer = indexBuffer,
        .pipeline = pipeline,
    };
}
```

## main.zig

省略了无变化的 update 方法。

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
    const z = 2 / @as(f32, depth);
    const projection = mach.math.Mat4x4.init(
        &mach.math.Mat4x4.RowVec.init(x, 0, 0, -1),
        &mach.math.Mat4x4.RowVec.init(0, y, 0, 1),
        &mach.math.Mat4x4.RowVec.init(0, 0, z, 1),
        &mach.math.Mat4x4.RowVec.init(0, 0, 0, 1),
    );

    const angle: f32 = mach.math.degreesToRadians(f32, 10);
    var vec = mach.math.Vec3.init(200, 100, 0);
    var model = projection.mul(&mach.math.Mat4x4.translate(vec));
    model = model.mul(&mach.math.Mat4x4.rotateZ(angle));
    vec = mach.math.Vec3.init(2, 2, 1);
    model = model.mul(&mach.math.Mat4x4.scale(vec));

    const byteSize = @sizeOf(@TypeOf(model));
    const modelBuffer = device.createBuffer(&.{
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = byteSize,
    });
    device.getQueue().writeBuffer(modelBuffer, 0, (&model)[0..1]);

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
...
}
```

## 效果

![4x4 矩阵][1]

## 总结

将之前的 3x3 矩阵改成 4x4 矩阵。

[1]: images/webgpu28.png

## 附录
