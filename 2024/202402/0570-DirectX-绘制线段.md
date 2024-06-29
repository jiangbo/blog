# 0570-DirectX-绘制线段

## 环境

- Time 2024-06-29
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

随机绘制线段，并将它们显示在屏幕上。

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
    switch (message) {
        ui.WM_CREATE => {
            std.log.info("WM_CREATE", .{});
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
    const time: u64 = @intCast(std.time.milliTimestamp());
    var rand = std.rand.DefaultPrng.init(time);

    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
        const hdc = gdi.GetDC(hander);
        defer _ = gdi.ReleaseDC(hander, hdc);

        const color = Color{
            .r = rand.random().int(u8),
            .g = rand.random().int(u8),
            .b = rand.random().int(u8),
        };
        const hpen = gdi.CreatePen(gdi.PS_SOLID, 1, @bitCast(color));
        defer _ = gdi.DeleteObject(hpen);
        const old = gdi.SelectObject(hdc, hpen);
        defer _ = gdi.SelectObject(hdc, old);

        var x: i32 = rand.random().intRangeLessThan(i32, 0, WIDTH);
        var y: i32 = rand.random().intRangeLessThan(i32, 0, HEIGHT);
        _ = gdi.MoveToEx(hdc, x, y, null);

        x = rand.random().intRangeLessThan(i32, 0, WIDTH);
        y = rand.random().intRangeLessThan(i32, 0, HEIGHT);
        _ = gdi.LineTo(hdc, x, y);
    }

    std.log.info("wWinMain end", .{});
    return 0;
}

fn win32Panic() noreturn {
    @panic(@tagName(win32.foundation.GetLastError()));
}

pub fn keyDown(code: keyboard.VIRTUAL_KEY) bool {
    return !keyUp(code);
}

pub fn keyUp(code: keyboard.VIRTUAL_KEY) bool {
    const key: i32 = @intFromEnum(code);
    return keyboard.GetAsyncKeyState(key) == 0;
}

const Color = extern struct {
    r: u8 = 0,
    g: u8 = 0,
    b: u8 = 0,
    a: u8 = 0,
};
```

## 效果

![绘制线段][1]。

## 总结

将随机生成的线段绘制到屏幕上。

[1]: images/directx17.png

## 附录
