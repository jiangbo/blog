# 0330-Tetris-初始化项目环境

## 环境

- Time 2023-08-18
- Zig 0.11.0
- WSL-Ubuntu 22.04.3 LTS

## 前言

### 说明

参考资料：

1. <https://www.youtube.com/watch?v=nF_crEtmpBo>
2. <https://github.com/howprice/sdl2-tetris>

### 目标

在前面，编写了一个模拟器来运行了俄罗斯方块。这里，通过编写代码，直接来实现一个自己的俄罗斯方块游戏。

## 安装依赖库

通过命令 `apt install libsdl2-dev libsdl2-ttf-dev` 安装所需要的两个依赖库。

## 新建项目

建一个文件夹，然后使用 `zig init-exe` 命令来初始化一个新的项目。

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

    exe.linkSystemLibrary("SDL2");
    exe.linkSystemLibrary("SDL2_ttf");
    exe.linkLibC();
    b.installArtifact(exe);
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

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

其中的大部分内容都是自动生成的，其中新增加了如下两行：

```zig
    exe.linkSystemLibrary("SDL2");
    exe.linkSystemLibrary("SDL2_ttf");
```

SDL2 库用来绘制图形界面，SDL2_ttf 用来显示文字，比如游戏的得分。

## main.zig

```zig
const std = @import("std");

pub fn main() !void {
    std.debug.print("hello world\n", .{});
}
```

## 运行

通过命令 `zig build run`，可以看到正常输出 hello world。

## 总结

初始化了一个使用 Zig 语言的 SDL 开发的环境，可以正常编译和输出。

## 附录
