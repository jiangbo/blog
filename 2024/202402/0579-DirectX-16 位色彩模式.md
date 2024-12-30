# 0579-DirectX-16 位颜色

## 环境

- Time 2024-07-02
- Zig 0.12.0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>
3. <https://github.com/dege-diosg/dgVoodoo2>

问题：

1. 使用 Zig 语言编译 32 位模式，0.12 可以成功，0.13 失败，所以先退回 0.12。
2. 运行不起来，下载了一个 Voodoo2 将起放到运行目录中，运行成功。
3. 640x480 的分辨率会导致电脑分辨率混乱，将其修改成真实的屏幕分辨率。

### 目标

去除了 DirectDraw 中使用调色板的功能，直接使用 16 位色彩模式。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const ui = win32.ui.windows_and_messaging;
const draw = win32.graphics.direct_draw;
const gdi = win32.graphics.gdi;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub const UNICODE: bool = true;
const name = win32.zig.L("游戏编程大师");
const WIDTH: u32 = 2560;
const HEIGHT: u32 = 1080;

var instance: H = undefined;
var hander: win32.foundation.HWND = undefined;
var rand: std.Random = undefined;

var draw7: *draw.IDirectDraw7 = undefined;
var surfaceDes: draw.DDSURFACEDESC2 = undefined;
var surface: *draw.IDirectDrawSurface7 = undefined;

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
    windowClass.hbrBackground = gdi.GetStockObject(gdi.BLACK_BRUSH);

    if (ui.RegisterClassEx(&windowClass) == 0) win32Panic();

    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, name, name, style, 0, 0, //
        @intCast(WIDTH), @intCast(HEIGHT), null, null, h, null);

    instance = h;
    hander = window orelse win32Panic();

    gameInit();
    defer gameShutdown();

    var message: ui.MSG = undefined;
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
const failed = win32.zig.FAILED;
const system = win32.system.system_information;
fn gameInit() void {
    std.log.info("gameInit", .{});

    var prng = std.rand.DefaultPrng.init(system.GetTickCount64());
    rand = prng.random();

    if (failed(draw.DirectDrawCreateEx(null, @ptrCast(&draw7), //
        draw.IID_IDirectDraw7, null))) win32Panic();

    const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel(hander, style)))
        win32Panic();

    if (failed(draw7.IDirectDraw7_SetDisplayMode(WIDTH, HEIGHT, 16, 0, 0)))
        win32Panic();

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);
    surfaceDes.dwFlags = draw.DDSD_CAPS;
    surfaceDes.ddsCaps.dwCaps = draw.DDSCAPS_PRIMARYSURFACE;

    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, //
        @ptrCast(&surface), null))) win32Panic();

    var pixel = std.mem.zeroes(draw.DDPIXELFORMAT);
    pixel.dwSize = @sizeOf(draw.DDPIXELFORMAT);
    if (failed(surface.IDirectDrawSurface7_GetPixelFormat(&pixel)))
        win32Panic();

    if (pixel.dwFlags & draw.DDPF_RGB != 0) {
        std.log.info("RGB bit count: {any}", .{pixel.Anonymous1.dwRGBBitCount});
    } else if (pixel.dwFlags & draw.DDPF_PALETTEINDEXED8 != 0) {
        std.log.info("palette indexed 8", .{});
    } else {
        std.log.info("unknown pixel format", .{});
    }
}

fn gameUpdate() void {
    // get the time
    const start = system.GetTickCount64();

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);

    if (failed(surface.IDirectDrawSurface7_Lock(null, &surfaceDes, //
        draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null))) win32Panic();

    const pitch16: usize = @intCast(surfaceDes.Anonymous1.lPitch >> 1);
    const buffer: [*]u16 = @ptrCast(@alignCast(surfaceDes.lpSurface));

    for (0..100000) |_| {
        const color: u16 = rand.uintAtMost(u16, std.math.maxInt(u16));
        const x = rand.uintLessThan(usize, WIDTH);
        const y = rand.uintLessThan(usize, HEIGHT);
        const offset = x + y * pitch16;
        buffer[offset] = color;
    }

    _ = surface.IDirectDrawSurface7_Unlock(null);

    // lock to 30 fps
    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = surface.IUnknown_Release();
    _ = draw7.IUnknown_Release();
}

fn win32Panic() noreturn {
    const err = win32.foundation.GetLastError();
    std.log.err("win32 painc code {}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## 效果

![DirectDraw 16 位色彩][1]

## 总结

使用 DirectDraw 的 16 位色彩模式，随机生成了一些点。

[1]: images/directx26.png

## 附录
