# 0527-WebGPU-显示字母 F

## 环境

- Time 2024-05-17
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

修改顶点和索引数据，显示一个大写的 F 字母，和教程保持一致。

## shader.wgsl

```wgsl
@binding(0) @group(0) var<uniform> model: mat3x3f;

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
    let xy = (model * in.position.xyz).xy;

    let position = (model * vec3f(in.position.xy, 1)).xy;

    let resolution = vec2f(640,480);
    // convert the position from pixels to a 0.0 to 1.0 value
    let zeroToOne = position / resolution;

    // convert from 0 <-> 1 to 0 <-> 2
    let zeroToTwo = zeroToOne * 2.0;

    // covert from 0 <-> 2 to -1 <-> +1 (clip space)
    let flippedClipSpace = zeroToTwo - 1.0;

    // flip Y
    let clipSpace = flippedClipSpace * vec2f(1, -1);

    out.position = vec4f(clipSpace, 0.0, 1.0);

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
    0,   0,
    30,  0,
    0,   150,
    30,  150,

    // top rung
    30,  0,
    100, 0,
    30,  30,
    100, 30,

    // middle rung
    30,  60,
    70,  60,
    30,  90,
    70,  90,
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
        .array_stride = @sizeOf(f32) * 2,
        .attributes = &.{
            .{ .format = .float32x2, .offset = 0, .shader_location = 0 },
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

## mat3.zig

无变化。

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");
const mat = @import("mat.zig");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderContext: render.RenderContext = undefined,
bindGroup: *mach.gpu.BindGroup = undefined,

pub fn init(app: *App) !void {
    try mach.core.init(.{
        .title = "学习 WebGPU",
        .size = .{ .width = 600, .height = 480 },
    });
    // 设置帧率
    mach.core.setFrameRateLimit(30);
    mach.core.setInputFrequency(30);
    const device = mach.core.device;

    const byteSize = 48;
    const modelBuffer = device.createBuffer(&.{
        .usage = .{ .copy_dst = true, .uniform = true },
        .size = byteSize,
    });

    const angle: f32 = 10 * std.math.pi / 180.0;
    const offset = mat.offset(200, 100);
    const rotate = mat.rotate(angle);
    const scale = mat.scale(2, 2);

    var matrix = mat.mul(scale, rotate);
    matrix = mat.mul(matrix, offset);

    // mat3x3 矩阵应该按照 48 字节对齐
    // 参考：https://www.w3.org/TR/WGSL/#alignment-and-size
    var model: [12]f32 = undefined;
    for (matrix, 0..) |value, index| {
        const mod, const div = .{ index % 3, index / 3 };
        if (mod == 1) model[index] = 0;
        model[index + div] = value;
    }
    device.getQueue().writeBuffer(modelBuffer, 0, &model);

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
    // 设置渲染管线
    pass.setPipeline(app.renderContext.pipeline);
    const vertexBuffer = app.renderContext.vertexBuffer;
    pass.setVertexBuffer(0, vertexBuffer, 0, vertexBuffer.getSize());

    const size = @sizeOf(@TypeOf(render.indexData));
    pass.setIndexBuffer(app.renderContext.indexBuffer, .uint32, 0, size);
    pass.setBindGroup(0, app.bindGroup, &.{});

    pass.drawIndexed(render.indexData.len, 1, 0, 0, 0);
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

![显示字母 F][1]

## 总结

修改成和教程一致，显示一个大写的字母 F。

[1]: images/webgpu22.png

## 附录
