# 0556-DirectX-图标和鼠标

## 环境

- Time 2024-06-27
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

建立 rc 文件，并使用其中的图片素材。

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

    exe.subsystem = .Windows;
    exe.addWin32ResourceFile(.{
        .file = b.path("asserts/windows.rc"),
        .flags = &.{"/c65001"},
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

## windows.rc

其中 windows.rc、t3dx.ico 和 crosshair.cur 都在 asserts 目录下。

```rc
// note that this file has different types of resources
ICON_T3DX        ICON   t3dx.ico
CURSOR_CROSSHAIR CURSOR crosshair.cur
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const ui = win32.ui.windows_and_messaging;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub const UNICODE: bool = true;
const name = win32.zig.L("游戏编程");

pub fn mainWindowCallback(
    window: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(WINAPI) win32.foundation.LRESULT {
    switch (message) {
        ui.WM_CREATE => std.debug.print("create\n", .{}),
        ui.WM_PAINT => {
            var paint: win32.graphics.gdi.PAINTSTRUCT = undefined;
            _ = win32.graphics.gdi.BeginPaint(window, &paint);
            _ = win32.graphics.gdi.EndPaint(window, &paint);
        },
        // ui.WM_CLOSE => running = false,
        ui.WM_DESTROY => ui.PostQuitMessage(0),
        else => return ui.DefWindowProc(window, message, wParam, lParam),
    }
    return 0;
}

pub fn wWinMain(h: H, _: ?H, _: [*:0]u16, _: u32) callconv(WINAPI) i32 {
    std.debug.print("wWinMain\n", .{});
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.lpszClassName = name;
    windowClass.lpfnWndProc = mainWindowCallback;
    windowClass.hInstance = h;

    windowClass.hIcon = ui.LoadIcon(h, win32.zig.L("ICON_T3DX"));
    windowClass.hCursor = ui.LoadCursor(h, win32.zig.L("CURSOR_CROSSHAIR"));
    windowClass.hIconSm = ui.LoadIcon(h, win32.zig.L("ICON_T3DX"));

    if (ui.RegisterClassEx(&windowClass) == 0) win32Panic();

    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const window = ui.CreateWindowEx(
        ui.WS_EX_LEFT,
        name,
        name,
        style,
        ui.CW_USEDEFAULT,
        ui.CW_USEDEFAULT,
        640,
        480,
        null,
        null,
        h,
        null,
    );

    if (window == null) win32Panic();

    var message: ui.MSG = undefined;
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
    }

    return 0;
}

fn win32Panic() void {
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![图标和鼠标][1]

## 总结

通过使用 rc 文件来引用其中的图标和鼠标样式。

[1]: images/directx05.png

## 附录
