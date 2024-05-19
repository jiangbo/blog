# 0537-WebGPU-调整顶点顺序

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

调整顶点的顺序，因为反转了 Y 轴，所以使用修改背面剔除为前向剔除。

## shader.wgsl

无变化。

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
    12, 14, 13, 14, 15, 13, // left column back
    16, 18, 17, 18, 19, 17, // top run back
    20, 22, 21, 22, 23, 21, // middle run back
    0, 12, 5, 12, 17, 5, // top
    5, 17, 7, 17, 19, 7, // top rung right
    6, 7, 18, 18, 7, 19, // top rung bottom
    6, 18, 8, 18, 20, 8, // between top and middle rung
    8, 20, 9, 20, 21, 9, // middle rung top
    9, 21, 11, 21, 23, 11, // middle rung right
    10, 11, 22, 22, 11, 23, // middle rung bottom
    10, 22, 3, 22, 15, 3, // stem right
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
        .primitive = .{ .cull_mode = .front },
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

无变化。

## 效果

![前向剔除][1]

## 总结

由于翻转了 Y 轴，使用 front 剔除。

[1]: images/webgpu33.webp

## 附录
