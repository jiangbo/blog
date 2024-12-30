# 0560-DirectX-显示文字

## 环境

- Time 2024-06-28
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

通过 textOut 显示文字。

## windows.rc

文件都存放在 assets 目录下。

```rc
// note that this file has different types of resources
ICON_T3DX        ICON   t3dx.ico
CURSOR_CROSSHAIR CURSOR crosshair.cur
```

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
        ui.WM_PAINT => {
            var paint: gdi.PAINTSTRUCT = undefined;
            _ = gdi.BeginPaint(window, &paint);
            _ = gdi.EndPaint(window, &paint);
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

    const time: u64 = @intCast(std.time.milliTimestamp());
    var rand = std.rand.DefaultPrng.init(time);
    const hdc = gdi.GetDC(window).?;
    defer _ = gdi.ReleaseDC(window, hdc);

    var message: ui.MSG = undefined;
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }

        const color = Color{
            .r = rand.random().int(u8),
            .g = rand.random().int(u8),
            .b = rand.random().int(u8),
        };

        _ = gdi.SetTextColor(hdc, @bitCast(color));
        _ = gdi.SetBkColor(hdc, @bitCast(std.mem.zeroes(Color)));
        _ = gdi.SetBkMode(hdc, gdi.TRANSPARENT);

        const x: i32 = rand.random().intRangeLessThan(i32, 0, WIDTH);
        const y: i32 = rand.random().intRangeLessThan(i32, 0, HEIGHT);
        _ = gdi.TextOut(hdc, x, y, name, name.len);

        std.time.sleep(10 * std.time.ns_per_ms);
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
};
```

## 效果

![显示文字][1]

## 总结

使用随机数来生成不同颜色的文字。

[1]: images/directx08.png

## 附录
