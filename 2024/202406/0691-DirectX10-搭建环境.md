# 0691-DirectX10-搭建环境

## 目标

搭建 DirectX 10 的开发环境。

## 环境

- Time 2024-12-31
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>

## 想法

DirectX9 就这样了，看看 DirectX10 有哪些变化，听说挺大的。

## build.zig.zon

```zig
.{
    .name = "demo",
    .version = "0.0.0",
    .dependencies = .{
        .zigwin32 = .{
            .url = "git+https://github.com/marlersoft/zigwin32",
            .hash = "1220adcf9ec0447c6a170ed069ed9d52c999b3dcae3557b3647878bf65ee59a2f5d0",
        },
    },

    .paths = .{""},
}
```

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

    // exe.subsystem = .Windows;
    b.installArtifact(exe);

    const win32 = b.dependency("zigwin32", .{});
    exe.root_module.addImport("win32", win32.module("zigwin32"));

    const dir = "C:/software/Microsoft DirectX SDK (June 2010)/";
    // exe.addIncludePath(.{ .cwd_relative = dir ++ "Include" });
    exe.addObjectFile(.{ .cwd_relative = dir ++ "lib/x64/d3dx10.lib" });
    exe.addObjectFile(.{ .cwd_relative = dir ++ "lib/x64/d3dx10d.lib" });

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");

pub const UNICODE: bool = true;

pub fn main() !void {
    std.log.info("", .{});
}
```

## 附录
