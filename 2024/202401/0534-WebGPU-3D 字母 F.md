# 0534-WebGPU-3D 字母 F

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

给字母 F 多增加几个面，变成 3D 的字母。

## shader.wgsl

无变化。

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

pub const indexData = [_]u32{
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

无变化。

## 效果

![3D 字母][1]

## 总结

显示一个旋转的 3D 字母 F。

[1]: images/webgpu30.webp

## 附录
