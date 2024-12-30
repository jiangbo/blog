# 0578-DirectX-DirectDraw 简单使用

## 环境

- Time 2024-07-02
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

问题：

<https://ziggit.dev/t/win32-setdisplaymode-api-return-a-error/4932>
Zig 中使用 DirectDraw 会出现问题，参见上面的链接。那么现在只要求编译通过就行，运行时异常不处理。
对于运行时的效果，将 C++ 代码放到 Visual Studio 中编译，然后运行。

### 目标

使用 DirectDraw 中的表面描述、表面、调色板、加锁和解锁。

## main.zig

注意，以下代码只能编译通过，运行时将出现异常。

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
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

var instance: H = undefined;
var hander: win32.foundation.HWND = undefined;
var rand: std.Random = undefined;

var draw7: *draw.IDirectDraw7 = undefined;
var surfaceDes: draw.DDSURFACEDESC2 = undefined;
var surface: *draw.IDirectDrawSurface7 = undefined;
var palettes: [256]win32.graphics.gdi.PALETTEENTRY = undefined;
var palette: *draw.IDirectDrawPalette = undefined;

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
const failed = win32.zig.FAILED;
const system = win32.system.system_information;
fn gameInit() void {
    std.log.info("gameInit", .{});

    var prng = std.rand.DefaultPrng.init(system.GetTickCount64());
    rand = prng.random();

    if (failed(draw.DirectDrawCreateEx(null, @ptrCast(&draw7), //
        draw.IID_IDirectDraw7, null))) win32Panic();

    // const style = draw.DDSCL_FULLSCREEN | draw.DDSCL_ALLOWMODEX |
    //     draw.DDSCL_EXCLUSIVE | draw.DDSCL_ALLOWREBOOT;

    const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel( //
        hander, style))) win32Panic();

    // var result = draw7.IDirectDraw7_SetDisplayMode( //
    //     WIDTH, HEIGHT, 8, 0, 0);
    // std.log.info("SetDisplayMode result {}", .{result});
    var result: i32 = undefined;
    if (failed(draw7.IDirectDraw7_SetDisplayMode( //
        WIDTH, HEIGHT, 8, 0, 0))) win32Panic();

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);
    surfaceDes.dwFlags = draw.DDSD_CAPS;
    surfaceDes.ddsCaps.dwCaps = draw.DDSCAPS_PRIMARYSURFACE;

    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, //
        @ptrCast(&surface), null))) win32Panic();

    for (palettes[1..255]) |*value| {
        value.* = win32.graphics.gdi.PALETTEENTRY{
            .peRed = rand.uintAtMost(u8, 255),
            .peGreen = rand.uintAtMost(u8, 255),
            .peBlue = rand.uintAtMost(u8, 255),
            .peFlags = win32.graphics.gdi.PC_NOCOLLAPSE,
        };
    }

    const f = win32.graphics.gdi.PC_NOCOLLAPSE;
    const T = win32.graphics.gdi.PALETTEENTRY;
    palettes[0] = std.mem.zeroInit(T, .{ .peFlags = f });
    palettes[255] = std.mem.zeroInit(T, .{ 255, 255, 255, f });

    // create the palette object
    if (failed(draw7.IDirectDraw7_CreatePalette(draw.DDPCAPS_8BIT | //
        draw.DDPCAPS_ALLOW256 | draw.DDPCAPS_INITIALIZE, //
        @ptrCast((&palettes).ptr), @ptrCast(&palette), null))) win32Panic();

    // finally attach the palette to the primary surface
    result = surface.IDirectDrawSurface7_SetPalette(palette);
    std.log.info("SetPalette result {}", .{result});
}

fn gameUpdate() void { // get the time
    const start = system.GetTickCount64();

    // plot 1000 random pixels to the primary surface and return
    // clear ddsd and set size, never assume it's clean
    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);

    _ = surface.IDirectDrawSurface7_Lock(null, &surfaceDes, //
        draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null);

    for (0..1000) |_| {
        const color: u8 = rand.uintAtMost(u8, 255);
        const x = rand.uintLessThan(usize, 640);
        const y = rand.uintLessThan(usize, 480);
        const pitch: usize = @intCast(surfaceDes.Anonymous1.lPitch);
        const offset = x + y * pitch;
        const buffer: [*]u8 = @ptrCast(surfaceDes.lpSurface);
        buffer[offset] = color;
    }
    _ = surface.IDirectDrawSurface7_Unlock(null);

    // lock to 30 fps
    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = palette.IUnknown_Release();
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

![DirectDraw 简单使用][1]

## 总结

使用 DirectDraw 绘制了简单的随机颜色的表面。

[1]: images/directx25.webp

## 附录
