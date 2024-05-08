# 0503-WebGPU-初始化窗口

## 环境

- Time 2024-05-08
- Zig 0.12.0-dev.3180+83e578a18
- Windows 11

## 前言

### 说明

参考资料：

1. <https://machengine.org/core/getting-started/>

### 目标

使用 Zig 语言和 mach 框架来初始化一个 WebGPU 窗口。

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
    const app = try mach.CoreApp.init(b, mach_dep.builder, .{
        .name = "demo",
        .src = "src/main.zig",
        .target = target,
        .optimize = optimize,
        .deps = &[_]std.Build.Module.Import{},
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

## main.zig

```zig
const std = @import("std");
const mach = @import("mach");

pub const App = @This();
var gpa = std.heap.GeneralPurposeAllocator(.{}){};

pub fn init(_: *App) !void {
    try mach.core.init(.{
        .size = .{ .width = 800, .height = 600 },
        .title = "学习 WebGPU",
    });
}

pub fn deinit(_: *App) void {
    defer _ = gpa.deinit();
    defer mach.core.deinit();
}

pub fn update(_: *App) !bool {
    var iterator = mach.core.pollEvents();
    while (iterator.next()) |event| if (event == .close) return true;

    const view = mach.core.swap_chain.getCurrentTextureView().?;
    const colorAttachment = mach.gpu.RenderPassColorAttachment{
        .view = view,
        .clear_value = mach.gpu.Color{ .r = 0, .g = 0, .b = 0, .a = 1.0 },
        .load_op = .clear,
        .store_op = .store,
    };

    const encoder = mach.core.device.createCommandEncoder(null);
    const renderPass = mach.gpu.RenderPassDescriptor.init(.{
        .color_attachments = &.{colorAttachment},
    });

    const pass = encoder.beginRenderPass(&renderPass);
    pass.end();
    pass.release();

    var command = encoder.finish(null);
    encoder.release();

    var queue = mach.core.queue;
    queue.submit(&[_]*mach.gpu.CommandBuffer{command});
    command.release();
    mach.core.swap_chain.present();
    view.release();
    return false;
}
```

## 效果

![打开窗口][1]

## 总结

设置了使用 WebGPU 的环境，打开了窗口。

[1]: images/webgpu01.png

## 附录
