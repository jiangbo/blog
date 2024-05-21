# 0543-WebGPU-使用 zlm 矩阵库

## 环境

- Time 2024-05-21
- Zig 0.12.0-dev.3180+83e578a18
- mach 26b2351d4b04122d51c140b2d35325c02ccb0a5a

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach/tree/main/src/core/examples>
2. <https://webgpufundamentals.org/>

### 目标

mach.math 下没有透视投影的矩阵方法，切换到 zlm 矩阵库，这个里面有。

## build.zig.zon

```zig
.{
    .name = "demo",
    .version = "0.0.0",
    .dependencies = .{
        .mach = .{
            .url = "https://github.com/hexops/mach/archive/26b2351d4b04122d51c140b2d35325c02ccb0a5a.tar.gz",
            .hash = "12200a56e95eb1de56f744ec86e9ab961f85f2eb27ad8d56456b2a620862ffc9035a",
        },
        .zlm = .{
            .url = "https://github.com/ziglibs/zlm/archive/833031bfdbfd24526e4115a59459f9c360d7b824.tar.gz",
            .hash = "12205dbec9f917a3ab61ca65900dce7a04c9ec4348d0e2241a9c246b9c2d131d061b",
        },
    },
    .paths = .{""},
}
```

## build.zig

```zig
const std = @import("std");
const mach = @import("mach");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const mach_dep = b.dependency("mach", .{
        .target = target,
        .optimize = optimize,
        .core = true,
    });

    const zlm_dep = b.dependency("zlm", .{});

    const app = try mach.CoreApp.init(b, mach_dep.builder, .{
        .name = "demo",
        .src = "src/main.zig",
        .target = target,
        .optimize = optimize,
        .deps = &.{
            .{ .name = "zlm", .module = zlm_dep.module("zlm") },
        },
    });

    if (b.args) |args| app.run.addArgs(args);

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&app.run.step);

    const unit_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
}
```

## shader.wgsl

无变化。

## render.zig

无变化。

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");
const render = @import("render.zig");
const zlm = @import("zlm");

pub const App = @This();
const width = 640;
const height = 480;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
renderContext: render.RenderContext = undefined,
bindGroup: *mach.gpu.BindGroup = undefined,
projection: zlm.Mat4 = undefined,
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

    const fov: f32 = zlm.toRadians(100.0);
    const w = @as(f32, @floatFromInt(width));
    const aspect = w / @as(f32, @floatFromInt(height));
    app.projection = zlm.Mat4.createPerspective(fov, aspect, 1, 2000);

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
        .depth_stencil_attachment = &.{
            .view = app.renderContext.depthView,
            .depth_clear_value = 1.0,
            .depth_load_op = .clear,
            .depth_store_op = .store,
            .stencil_read_only = .true,
        },
    });

    // 命令编码器
    const encoder = mach.core.device.createCommandEncoder(null);
    defer encoder.release();
    const pass = encoder.beginRenderPass(&renderPass);

    var model = zlm.Mat4.createUniformScale(1);

    const angle: f32 = zlm.toRadians(app.timer.read() * 20);
    model = model.mul(zlm.Mat4.createAngleAxis(zlm.Vec3.unitX, angle));
    model = model.mul(zlm.Mat4.createAngleAxis(zlm.Vec3.unitY, angle));
    model = model.mul(zlm.Mat4.createAngleAxis(zlm.Vec3.unitZ, angle));

    model = model.mul(zlm.Mat4.createTranslationXYZ(-65, 0, -120));
    model = model.mul(app.projection);

    mach.core.queue.writeBuffer(app.modelBuffer, 0, &model.fields);

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

![zlm 矩阵库][1]

## 总结

使用 zlm 矩阵库来实现透视投影。

[1]: images/webgpu39.webp

## 附录
