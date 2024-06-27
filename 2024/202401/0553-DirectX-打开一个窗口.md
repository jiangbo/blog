# 0553-DirectX-打开一个窗口

## 环境

- Time 2024-06-27
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/marlersoft/zigwin32/issues/9>

### 目标

新建窗口并且进行显示。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const ui = win32.ui.windows_and_messaging;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub const UNICODE: bool = true;
const name = win32.zig.L("游戏编程");

pub fn wWinMain(h: H, _: ?H, _: [*:0]u16, _: u32) callconv(WINAPI) i32 {
    std.debug.print("wWinMain\n", .{});
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.lpszClassName = name;
    windowClass.lpfnWndProc = ui.DefWindowProcW;
    windowClass.hInstance = h;

    if (ui.RegisterClassEx(&windowClass) == 0) win32Panic();
    defer if (ui.UnregisterClass(name, h) == 0) win32Panic();

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
    defer if (ui.DestroyWindow(window) == 0) win32Panic();

    const message = win32.zig.L("Windows 游戏编程大师技巧");
    _ = ui.MessageBox(null, message, name, ui.MB_ICONEXCLAMATION);
    return 0;
}

fn win32Panic() void {
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![打开窗口][1]

## 总结

通过注册的窗口类，新增了一个窗口，并且进行显示。

[1]: images/directx03.png

## 附录
