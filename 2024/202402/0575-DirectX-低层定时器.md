# 0575-DirectX-低层定时器

## 环境

- Time 2024-06-29
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

使用底层无限循环来将帧率限制在 30 FPS。

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

    const system = win32.system.system_information;
    var prng = std.rand.DefaultPrng.init(system.GetTickCount64());
    var rand = prng.random();
    var colorChangCount: usize = 0;
    var pen: ?gdi.HPEN = null;
    const hdc = gdi.GetDC(hander);
    defer _ = gdi.ReleaseDC(hander, hdc);

    var x1 = rand.intRangeLessThan(i32, 0, WIDTH);
    var y1 = rand.intRangeLessThan(i32, 0, HEIGHT);
    var x2 = rand.intRangeLessThan(i32, 0, WIDTH);
    var y2 = rand.intRangeLessThan(i32, 0, HEIGHT);

    var x1v = rand.intRangeLessThan(i32, -4, 4);
    var y1v = rand.intRangeLessThan(i32, -4, 4);
    var x2v = rand.intRangeLessThan(i32, -4, 4);
    var y2v = rand.intRangeLessThan(i32, -4, 4);

    while (true) {
        const startTime = system.GetTickCount64();

        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }

        colorChangCount += 1;
        if (colorChangCount >= 100) {
            colorChangCount = 0;
            if (pen) |p| _ = gdi.DeleteObject(p);
            pen = gdi.CreatePen(gdi.PS_SOLID, 1, Color.random(rand));
            _ = gdi.SelectObject(hdc, pen);
        }

        x1 += x1v;
        y1 += y1v;
        x2 += x2v;
        y2 += y2v;

        if (x1 < 0 or x1 >= WIDTH) {
            x1v = -x1v;
            x1 += x1v;
        }

        if (y1 < 0 or y1 >= HEIGHT) {
            y1v = -y1v;
            y1 += y1v;
        }

        if (x2 < 0 or x2 >= WIDTH) {
            x2v = -x2v;
            x2 += x2v;
        }

        if (y2 < 0 or y2 >= HEIGHT) {
            y2v = -y2v;
            y2 += y2v;
        }

        _ = gdi.MoveToEx(hdc, x1, y1, null);
        _ = gdi.LineTo(hdc, x2, y2);

        while ((system.GetTickCount64() - startTime) < 33) {}
    }

    std.log.info("wWinMain end", .{});
    return 0;
}

fn win32Panic() noreturn {
    @panic(@tagName(win32.foundation.GetLastError()));
}

const Color = extern struct {
    r: u8 = 0,
    g: u8 = 0,
    b: u8 = 0,
    a: u8 = 0,

    fn random(r: std.Random) u32 {
        const color = Color{
            .r = r.int(u8),
            .g = r.int(u8),
            .b = r.int(u8),
        };
        return @bitCast(color);
    }
};
```

## 效果

![底层定时][1]。

## 总结

使用底层定时来锁定帧率。

[1]: images/directx22.webp

## 附录
