# 0561-DirectX-处理绘制消息

## 环境

- Time 2024-06-28
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

处理绘制消息，在收到重绘消息时，将绘制的次数显示到窗口中。

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
    var buffer: [200]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);

    switch (message) {
        ui.WM_CREATE => {
            std.log.info("WM_CREATE", .{});
        },
        ui.WM_PAINT => paintHandle(fba.allocator()) catch win32Panic(),
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

fn paintHandle(allocator: std.mem.Allocator) !void {
    var paint: gdi.PAINTSTRUCT = undefined;
    const hdc = gdi.BeginPaint(hander, &paint);
    defer _ = gdi.EndPaint(hander, &paint);

    _ = gdi.SetTextColor(hdc, 0xFFFF00);
    _ = gdi.SetBkColor(hdc, 0);
    _ = gdi.SetBkMode(hdc, .OPAQUE);

    count += 1;
    const utf8 = try std.fmt.allocPrint(allocator, "绘制 {} 次", .{count});
    const text = try std.unicode.utf8ToUtf16LeAllocZ(allocator, utf8);
    _ = gdi.TextOut(hdc, 0, 0, text, @intCast(text.len));
}
```

## 效果

![绘制次数][1]。

## 总结

处理绘制消息，在收到重绘消息时，将绘制的次数显示到窗口中。

[1]: images/directx09.png

## 附录
