# 0576-DirectX-按钮控件

## 环境

- Time 2024-06-30
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

了解 win32 的按钮控件。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const winmm = @import("winmm.zig");
const ui = win32.ui.windows_and_messaging;
const keyboard = win32.ui.input.keyboard_and_mouse;
const gdi = win32.graphics.gdi;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub const UNICODE: bool = true;
const name = win32.zig.L("游戏编程大师");
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
var hinstance: H = undefined;
var hander: win32.foundation.HWND = undefined;

pub fn mainWindowCallback(
    window: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(WINAPI) win32.foundation.LRESULT {
    var buffer: [1024]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);

    switch (message) {
        ui.WM_CREATE => {
            std.log.info("WM_CREATE", .{});
        },
        ui.WM_COMMAND => {
            const hdc = gdi.GetDC(hander);
            defer _ = gdi.ReleaseDC(hander, hdc);

            _ = gdi.SetBkMode(hdc, .OPAQUE);
            _ = gdi.SetTextColor(hdc, 0x00FF00);
            _ = gdi.SetBkColor(hdc, 0x808080);

            const f1 = "LOWORD(wparam) = {}, HIWORD(wparam) = {}        ";
            const p1 = .{ wParam & 0xFFFF, (wParam >> 16) & 0xFFFF };
            textOut(fba.allocator(), hdc, 100, f1, p1);

            const f2 = "LOWORD(lparam) = 0X{X}, HIWORD(lparam) = 0X{X}  ";
            const p2 = .{ lParam & 0xFFFF, (lParam >> 16) & 0xFFFF };
            textOut(fba.allocator(), hdc, 140, f2, p2);
        },

        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
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
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, name, name, style, //
        ui.CW_USEDEFAULT, ui.CW_USEDEFAULT, //
        @intCast(WIDTH), @intCast(HEIGHT), //
        null, null, h, null);

    const BUTTON_BASE_ID = 100;
    const NUM_BUTTONS = 8;
    const buttonNames: [NUM_BUTTONS][:0]const u16 =
        .{
        win32.zig.L("PUSHBUTTON"),
        win32.zig.L("RADIOBUTTON"),
        win32.zig.L("CHECKBOX"),
        win32.zig.L("3STATE"),
        win32.zig.L("AUTO3STATE"),
        win32.zig.L("AUTOCHECKBOX"),
        win32.zig.L("AUTORADIOBUTTON"),
        win32.zig.L("OWNERDRAW"),
    };
    const buttonTypes: [NUM_BUTTONS]ui.WINDOW_STYLE =
        .{
        @bitCast(ui.BS_PUSHBUTTON),
        @bitCast(ui.BS_RADIOBUTTON),
        @bitCast(ui.BS_CHECKBOX),
        @bitCast(ui.BS_3STATE),
        @bitCast(ui.BS_AUTO3STATE),
        @bitCast(ui.BS_AUTOCHECKBOX),
        @bitCast(ui.BS_AUTORADIOBUTTON),
        @bitCast(ui.BS_OWNERDRAW),
    };

    for (0..NUM_BUTTONS) |button| {
        const ptr: *const ui.HMENU = @ptrCast(&(BUTTON_BASE_ID + button));

        var buttonType = buttonTypes[button];
        buttonType.CHILD = 1;
        buttonType.VISIBLE = 1;
        _ = ui.CreateWindowEx(.{}, // extended style
            win32.zig.L("button"), // class
            buttonNames[button], // title
            buttonType, 10, 10 + @as(i32, @intCast(button)) * 36, //
            @as(i32, @intCast(buttonNames[button].len)) * 16, 24, //
            window, // handle to parent
            ptr.*, // handle to menu
            hinstance, // instance of this application
            null); // extra creation parms
    }

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

fn textOut(
    alloc: std.mem.Allocator,
    hdc: ?win32.graphics.gdi.HDC,
    y: i32,
    comptime fmt: []const u8,
    args: anytype,
) void {
    errdefer unreachable;
    const utf8 = try std.fmt.allocPrint(alloc, fmt, args);
    const utf16 = try std.unicode.utf8ToUtf16LeAllocZ(alloc, utf8);
    _ = gdi.TextOut(hdc, 220, y, utf16, @intCast(utf16.len));
}
```

## 效果

![按钮控件][1]

## 总结

了解 win32 的按钮控件。

[1]: images/directx23.png

## 附录
