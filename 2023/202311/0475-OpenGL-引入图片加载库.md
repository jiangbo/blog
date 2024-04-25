# 0475-OpenGL-引入图片加载库

## 环境

- Time 2024-04-25
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

引入图片加载库 zstbi：<https://github.com/zig-gamedev/zig-gamedev/tree/main/libs/zstbi>。
可以根据 readme 中的说明使用 zstbi 来加载图片。

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

## main.zig

```zig
const std = @import("std");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const shader = @import("shader.zig");
const zstbi = @import("zstbi");

...
zstbi.init(gpa.allocator());
defer zstbi.deinit();
zstbi.setFlipVerticallyOnLoad(true);
...
var image = try zstbi.Image.loadFromFile("assets/wall.jpg", 0);
...
image.deinit();
```

## 效果

无

## 总结

引入了 zstbi 图片加载库。

## 附录
