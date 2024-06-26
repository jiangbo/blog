# 0550-DirectX-显示消息框

## 环境

- Time 2024-06-26
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/marlersoft/zigwin32/issues/9>

### 目标

建立 win32 的环境，显示一个消息框。

## 安装依赖

```sh
zig fetch git+https://github.com/marlersoft/zigwin32/ --save
```

## build.zig

```zig
const std = @import("std");
const mach = @import("mach");

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

    const win32 = b.dependency("zigwin32", .{});
    exe.root_module.addImport("win32", win32.module("zigwin32"));

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const exe_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_exe_unit_tests.step);
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const ui = win32.ui.windows_and_messaging;

pub fn main() !void {
    const caption = win32.zig.L("游戏编程");
    const message = win32.zig.L("Windows 游戏编程大师技巧");
    _ = ui.MessageBoxW(null, message, caption, ui.MB_OK);
}
```

## 效果

![消息框][1]

## 总结

使用 Zig 引入 win32 并打开了一个消息框。

[1]: images/directx01.png

## 附录
