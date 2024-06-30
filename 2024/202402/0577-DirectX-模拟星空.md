# 0577-DirectX-模拟星空

## 环境

- Time 2024-06-30
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

使用 gameInit、gameUpdate、gameShutdown 函数实现模拟星空。

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

const Star = struct { x: i32, y: i32, vel: u32, color: u32 };

var instance: H = undefined;
var hander: win32.foundation.HWND = undefined;
var rand: std.Random = undefined;
var globalDc: gdi.HDC = undefined;
var stars: [4096]Star = undefined;

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
    const s = .{ .DBLCLKS = 1, .OWNDC = 1, .HREDRAW = 1, .VREDRAW = 1 };

    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = s;
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

    instance = h;
    hander = window orelse win32Panic();
    var message: ui.MSG = undefined;

    gameInit();
    defer gameShutdown();

    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }

        gameUpdate();
    }

    std.log.info("wWinMain end", .{});
    return 0;
}

const system = win32.system.system_information;
fn gameInit() void {
    std.log.info("gameInit", .{});
    globalDc = gdi.GetDC(hander) orelse win32Panic();
    var prng = std.rand.DefaultPrng.init(system.GetTickCount64());
    rand = prng.random();

    for (&stars) |*star| {
        // select random position
        star.x = rand.intRangeLessThan(i32, 0, WIDTH);
        star.y = rand.intRangeLessThan(i32, 0, HEIGHT);
        star.vel = rand.intRangeAtMost(u32, 1, 16);

        const c: u8 = 15 * (17 - @as(u8, @intCast(star.vel)));
        star.color = @bitCast(Color{ .r = c, .g = c, .b = c });
    }
}

fn gameUpdate() void { // get the time
    const start = system.GetTickCount64();

    // erase the stars
    for (&stars) |*star| _ = gdi.SetPixel(globalDc, star.x, star.y, 0);

    // move the stars
    for (&stars) |*star| {
        star.x += @intCast(star.vel);
        if (star.x >= WIDTH) star.x -= WIDTH;
    }

    // draw the stars
    for (&stars) |star|
        _ = gdi.SetPixel(globalDc, star.x, star.y, star.color);

    // lock to 30 fps
    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = gdi.ReleaseDC(hander, globalDc);
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

![模拟星空][1]。

## 总结

使用 SetPixel 方法绘制星光，来实现模拟星空。

[1]: images/directx24.webp

## 附录
