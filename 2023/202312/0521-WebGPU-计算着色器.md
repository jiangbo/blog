# 0521-WebGPU-计算着色器

## 环境

- Time 2024-05-15
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://eliemichel.github.io/LearnWebGPU/index.html>

### 目标

简单使用计算着色器。

## shader.wgsl

```wgsl
// 输入数据和临时存储输出数据
@group(0) @binding(0) var<storage, read_write> data: array<f32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) id : vec3<u32>) {
    // 将每个数乘以 2
    data[id.x] = data[id.x] * 2;
}
```

## main.zig

```zig
const std = @import("std");

const mach = @import("mach");

pub const App = @This();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};

pub fn init(_: *App) !void {
    try mach.core.init(.{});

    const device = mach.core.device;
    const source = @embedFile("shader/shader.wgsl");
    const module = device.createShaderModuleWGSL("compute.wgsl", source);
    defer module.release();

    // 创建计算管线
    const computePipeline = device.createComputePipeline(&.{
        .label = "compute pipeline",
        .compute = mach.gpu.ProgrammableStageDescriptor{
            .module = module,
            .entry_point = "main",
        },
    });
    defer computePipeline.release();

    // 输入 buffer
    const input = [_]f32{ 1, 3, 5 };
    const workBuffer = device.createBuffer(&.{
        .label = "work buffer",
        .size = @sizeOf(@TypeOf(input)),
        .usage = .{ .storage = true, .copy_src = true, .copy_dst = true },
    });
    defer workBuffer.release();
    device.getQueue().writeBuffer(workBuffer, 0, &input);

    // 输出 buffer
    const resultBuffer = device.createBuffer(&.{
        .label = "result buffer",
        .size = @sizeOf(@TypeOf(input)),
        .usage = .{ .map_read = true, .copy_dst = true },
    });
    defer resultBuffer.release();

    // 通过绑定组输入数据
    const size = resultBuffer.getSize();
    const entry = mach.gpu.BindGroup.Entry.buffer(0, workBuffer, 0, size);
    const bindGroup = device.createBindGroup(&.{
        .label = "bind group",
        .layout = computePipeline.getBindGroupLayout(0),
        .entry_count = 1,
        .entries = ([_]mach.gpu.BindGroup.Entry{entry})[0..].ptr,
    });
    defer bindGroup.release();

    // 提交指令
    const encoder = device.createCommandEncoder(null);
    const pass = encoder.beginComputePass(null);

    pass.setPipeline(computePipeline);
    pass.setBindGroup(0, bindGroup, &.{});
    pass.dispatchWorkgroups(input.len, 1, 1);
    pass.end();
    pass.release();

    encoder.copyBufferToBuffer(workBuffer, 0, resultBuffer, 0, size);

    var commandBuffer = encoder.finish(null);
    encoder.release();

    mach.core.queue.submit(&[_]*mach.gpu.CommandBuffer{commandBuffer});
    commandBuffer.release();

    // 异步得到返回结果
    const Status = mach.gpu.Buffer.MapAsyncStatus;
    var response: Status = undefined;
    resultBuffer.mapAsync(.{ .read = true }, 0, size, &response, struct {
        pub inline fn callback(ctx: *Status, status: Status) void {
            ctx.* = status;
        }
    }.callback);

    while (true) {
        if (response == .success) break;
        mach.core.device.tick();
    }

    const result = resultBuffer.getConstMappedRange(f32, 0, input.len);
    for (result.?) |v| {
        std.debug.print("{d} ", .{v});
    }
    std.debug.print("\n", .{});
    resultBuffer.unmap();
}

pub fn deinit(app: *App) void {
    _ = app;
    mach.core.deinit();
    _ = gpa.deinit();
}

pub fn update(_: *App) !bool {
    return true;
}
```

## 效果

```text
2 6 10
```

会将输入的 1 3 5 乘以 2 后输出。

## 总结

简单创建了一个计算管线，将输入的 1 3 5 乘以 2。

## 附录
