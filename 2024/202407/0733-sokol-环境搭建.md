# 0733-sokol-环境搭建

## 目标

引入 sokol 依赖，搭建 sokol 的开发环境。

## 环境

- Time 2025-01-26
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

sokol 对窗口和图形 API 进行了一个薄的包装，学习了解一下试试。

## build.zig

```zig
const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);

    const sokol = b.dependency("sokol", .{
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addImport("sokol", sokol.module("sokol"));

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
```

## build.zig.zon

```zig
.{
    .name = "demo",
    .version = "0.0.0",
    .dependencies = .{
        .sokol = .{
            .url = "git+https://github.com/floooh/sokol-zig.git#9a02a991ce8374f1664ca3fbf8483d2d7ded84d2",
            .hash = "12205fe90783eed8f7ab3423f83cd84094c53f93fa6c342021203301061d6df1f412",
        },
    },

    .paths = .{""},
}
```

## main.zig

```zig
const std = @import("std");
const sk = @import("sokol");

pub fn main() void {
    _ = sk;
    std.log.info("hello world", .{});
}
```

## 附录
