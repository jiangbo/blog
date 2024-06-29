# 0573-DirectX-绘制多边形

## 环境

- Time 2024-06-29
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

随机绘制多边形，并将它们显示在屏幕上。

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
    var random = rand.random();
    const hdc = gdi.GetDC(hander);
    defer _ = gdi.ReleaseDC(hander, hdc);

    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }

        const pen = gdi.CreatePen(gdi.PS_SOLID, 1, Color.random(random));
        defer _ = gdi.DeleteObject(pen);
        const brush = gdi.CreateSolidBrush(Color.random(random));
        defer _ = gdi.DeleteObject(brush);

        _ = gdi.SelectObject(hdc, pen);
        _ = gdi.SelectObject(hdc, brush);

        const num = random.intRangeLessThan(usize, 3, 11);
        var points: [10]win32.foundation.POINT = undefined;

        for (0..num) |index| {
            points[index].x = random.intRangeLessThan(i32, 0, WIDTH);
            points[index].y = random.intRangeLessThan(i32, 0, HEIGHT);
        }
        _ = gdi.Polygon(hdc, &points, @intCast(num));

        std.time.sleep(500 * std.time.ns_per_ms);
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

    const black = Color{ .r = 0, .g = 0, .b = 0 };
    const white = Color{ .r = 255, .g = 255, .b = 255 };
    const green = Color{ .r = 0, .g = 255, .b = 0 };

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

![绘制多边形][1]。

## 总结

随机绘制多边形，并将它们显示在屏幕上。

[1]: images/directx20.png

## 附录
