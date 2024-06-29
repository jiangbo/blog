# 0563-DirectX-窗口大小修改事件

## 环境

- Time 2024-06-28
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

获取窗口改变事件，并将窗口大小打印到控制台。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const winmm = @import("winmm.zig");
const ui = win32.ui.windows_and_messaging;
const gdi = win32.graphics.gdi;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub const UNICODE: bool = true;
const name = win32.zig.L("游戏编程大师");
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
var hinstance: H = undefined;
var hander: win32.foundation.HWND = undefined;
var count: u32 = 0;

pub fn mainWindowCallback(
    window: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(WINAPI) win32.foundation.LRESULT {
    var buffer: [100]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);

    switch (message) {
        ui.WM_CREATE => {
            std.log.info("WM_CREATE", .{});
        },
        ui.WM_SIZE => {
            const width = 0xFFFF & lParam;
            const height = 0xFFFF & lParam >> 16;
            const hdc = gdi.GetDC(window);
            defer _ = gdi.ReleaseDC(window, hdc);

            _ = gdi.SetTextColor(hdc, 0x00FF00);
            _ = gdi.SetBkColor(hdc, 0x000000);
            _ = gdi.SetBkMode(hdc, .OPAQUE);

            const utf8 = std.fmt.allocPrint(
                fba.allocator(),
                "width: {}, height: {}",
                .{ width, height },
            ) catch win32Panic();
            const T = std.unicode.utf8ToUtf16LeAllocZ;
            const utf16 = T(fba.allocator(), utf8) catch win32Panic();
            _ = gdi.TextOut(hdc, 0, 0, utf16, @intCast(utf16.len));
        },
        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
            _ = winmm.PlaySoundW(null, hinstance, winmm.SND_PURGE);
            ui.PostQuitMessage(0);
        },
        else => return ui.DefWindowProc(window, message, wParam, lParam),
    }
    return 0;
}

pub fn wWinMain(h: H, _: ?H, _: [*:0]u16, _: u32) callconv(WINAPI) i32 {
    std.log.info("wWinMain", .{});
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.lpszClassName = name;
    windowClass.lpfnWndProc = mainWindowCallback;
    windowClass.hInstance = h;

    windowClass.hIcon = ui.LoadIcon(h, win32.zig.L("ICON_T3DX"));
    windowClass.hCursor = ui.LoadCursor(h, win32.zig.L("CURSOR_CROSSHAIR"));
    windowClass.hIconSm = ui.LoadIcon(h, win32.zig.L("ICON_T3DX"));
    windowClass.hbrBackground = gdi.GetStockObject(gdi.BLACK_BRUSH);

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
        @intCast(WIDTH),
        @intCast(HEIGHT),
        null,
        null,
        h,
        null,
    );

    hinstance = h;
    hander = window orelse win32Panic();
    var message: ui.MSG = undefined;
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
    }

    std.log.info("wWinMain end", .{});
    return 0;
}

fn win32Panic() noreturn {
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![窗口大小改变事件][1]。

## 总结

获取窗口大小改变事件，并将窗口大小打印到控制台。

[1]: images/directx11.png

## 附录
