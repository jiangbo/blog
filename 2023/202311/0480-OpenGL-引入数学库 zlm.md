# 0480-OpenGL-引入数学库 zlm

## 环境

- Time 2024-04-27
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach-glfw-opengl-example/tree/main>
2. <https://learnopengl-cn.github.io/01%20Getting%20started/03%20Hello%20Window/>
3. <https://www.bilibili.com/video/BV1Ni4y1o7Au>

### 目标

把 <https://github.com/ziglibs/zlm> 引入到项目中。

## build.zig.zon

```zig
.{
    .name = "demo",
    .version = "0.0.0",
    .dependencies = .{
        .@"mach-glfw" = .{
            .url = "https://pkg.machengine.org/mach-glfw/1a9a03399058fd83f7fbb597f3f8304007ff6a3c.tar.gz",
            .hash = "1220b4d58ec6cf53abfd8d7547d39afb9bffa41822d4d58f52625230466e51cc93bb",
        },
        .zigglgen = .{
            .url = "https://github.com/castholm/zigglgen/releases/download/v0.2.1/zigglgen.tar.gz",
            .hash = "122059d1ff6787eedb40771eabb0f04d04fe299bd1bae7b216c4a9d894719c2a148a",
        },
        .zstbi = .{
            .path = "lib/zstbi",
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

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const glfw_dep = b.dependency("mach-glfw", .{ .target = target, .optimize = optimize });
    exe.root_module.addImport("mach-glfw", glfw_dep.module("mach-glfw"));

    const zlm = b.dependency("zlm", .{});
    exe.root_module.addImport("zlm", zlm.module("zlm"));

    const options = .{ .api = .gl, .version = .@"3.3", .profile = .core };
    const gl_bindings = @import("zigglgen").generateBindingsModule(b, options);
    exe.root_module.addImport("gl", gl_bindings);

    const zstbi = b.dependency("zstbi", .{ .target = target, .optimize = optimize });
    exe.root_module.addImport("zstbi", zstbi.module("root"));
    exe.linkLibrary(zstbi.artifact("zstbi"));

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| run_cmd.addArgs(args);

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
```

## 效果

能够正确下载依赖和进行编译。

## 总结

引入了数学库。

## 附录
