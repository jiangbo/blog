# 0555-DirectX-非阻塞消息

## 环境

- Time 2024-06-27
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/marlersoft/zigwin32/issues/9>

### 目标

使用 `GetMessage`，如果没有消息会阻塞，使用 `PeekMessage` 替换。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const ui = win32.ui.windows_and_messaging;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub const UNICODE: bool = true;
const name = win32.zig.L("游戏编程");
var running = true;

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
        ui.WM_CLOSE => running = false,
        ui.WM_DESTROY => running = false,
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

    while (running) {
        var message: ui.MSG = undefined;
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
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

![消息循环][1]

## 总结

通过使用 `PeekMessage` 来避免消息阻塞。

[1]: images/directx04.png

## 附录
