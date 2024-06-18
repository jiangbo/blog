# 0551-GStreamer-Zig 语言环境

## 环境

- Time 2024-06-18
- WSL2 Ubuntu 22.04.4 LTS
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. <https://gstreamer.freedesktop.org/documentation/installing/on-linux.html>
2. <https://gstreamer.freedesktop.org/documentation/tutorials/basic/hello-world.html>

### 目标

建立 GStreamer Zig 语言环境。

## 安装依赖

```sh
apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio
```

## build.zig.zon

```zig
.{
    .name = "gstreamer",
    .version = "0.0.0",
    .dependencies = .{},
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
        .name = "gstreamer",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    exe.addIncludePath(.{ .cwd_relative = "/usr/include/gstreamer-1.0" });
    exe.linkSystemLibrary("gstreamer-1.0");
    exe.linkLibC();

    b.installArtifact(exe);
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
const c = @cImport(@cInclude("gst/gst.h"));

pub fn main() !void {
    var argc: c_int = @intCast(std.os.argv.len);
    var argv: [*c][*c]u8 = @ptrCast(std.os.argv.ptr);
    c.gst_init(&argc, &argv);

    const uri = "https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm";
    const pipeline = c.gst_parse_launch("playbin uri=" ++ uri, null);
    defer c.gst_object_unref(pipeline);

    // Start playing
    _ = c.gst_element_set_state(pipeline, c.GST_STATE_PLAYING);

    //  Wait until error or EOS
    const bus = c.gst_element_get_bus(pipeline);
    defer c.gst_object_unref(bus);

    const msg = c.gst_bus_timed_pop_filtered(bus, //
        c.GST_CLOCK_TIME_NONE, c.GST_MESSAGE_ERROR | c.GST_MESSAGE_EOS);
    defer c.gst_message_unref(msg);

    //  See next tutorial for proper error message handling/parsing
    if (c.GST_MESSAGE_TYPE(msg) == c.GST_MESSAGE_ERROR) {
        c.g_printerr("An error occurred! Re-run with the GST_DEBUG=*:WARN " ++
            "environment variable set for more details.\n");
    }

    _ = c.gst_element_set_state(pipeline, c.GST_STATE_NULL);

    std.log.debug("run end", .{});
}
```

## 构建和运行

```sh
zig build run
```

## 效果

![播放视频][1]

## 总结

使用 GStreamer 播放视频。

[1]: images/gstreamer02.png

## 附录
