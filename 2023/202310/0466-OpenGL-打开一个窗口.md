# 0466-OpenGL-打开一个窗口

## 环境

- Time 2024-04-20
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

配置好环境，并打开一个窗口。

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

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn errorPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;

pub fn main() void {
    const window = initWindow();
    defer deinit(window);

    glfw.makeContextCurrent(window);
    defer glfw.makeContextCurrent(null);
    glfw.swapInterval(1);

    if (!glProcs.init(glfw.getProcAddress)) errorPanic();

    gl.makeProcTableCurrent(&glProcs);
    defer gl.makeProcTableCurrent(null);

    while (!window.shouldClose()) {
        processInput(window);
        glfw.pollEvents();

        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gl.COLOR_BUFFER_BIT);

        window.swapBuffers();
    }
}

fn initWindow() glfw.Window {
    glfw.setErrorCallback(logGlfwError);

    if (!glfw.init(.{})) errorPanic();

    return glfw.Window.create(640, 480, "学习 OpenGL", null, null, .{
        .context_version_major = gl.info.version_major,
        .context_version_minor = gl.info.version_minor,
        .opengl_profile = .opengl_core_profile,
    }) orelse errorPanic();
}

fn deinit(window: glfw.Window) void {
    window.destroy();
    glfw.terminate();
}

fn processInput(window: glfw.Window) void {
    if (window.getKey(glfw.Key.escape) == .press)
        window.setShouldClose(true);
}
```

## 效果

![打开一个窗口][1]

## 总结

设置了 OpenGL 的学习环境，打开了一个窗口。

[1]: images/opengl01.png

## 附录
