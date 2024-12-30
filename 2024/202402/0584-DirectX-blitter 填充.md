# 0584-DirectX-blitter 填充

## 目标

使用 DirectDraw 的 blt 方法向屏幕填充不同颜色的矩形。

## 环境

- Time 2024-07-03
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## win.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const zigwin = @import("win.zig");

const draw = win32.graphics.direct_draw;

pub const UNICODE: bool = true;
const WINAPI = std.os.windows.WINAPI;

var draw7: *draw.IDirectDraw7 = undefined;
var surfaceDes: draw.DDSURFACEDESC2 = undefined;
var surface: *draw.IDirectDrawSurface7 = undefined;

const H = std.os.windows.HINSTANCE;
pub fn wWinMain(h: H, _: ?H, _: [*:0]u16, _: u32) callconv(WINAPI) i32 {
    zigwin.createWindow(h);

    gameInit();
    zigwin.update(gameUpdate);
    gameShutdown();

    return 0;
}

const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    if (failed(draw.DirectDrawCreateEx(null, @ptrCast(&draw7), //
        draw.IID_IDirectDraw7, null))) win32Panic();

    const style = draw.DDSCL_FULLSCREEN | draw.DDSCL_ALLOWMODEX |
        draw.DDSCL_EXCLUSIVE | draw.DDSCL_ALLOWREBOOT;

    // const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel(zigwin.hander, style)))
        win32Panic();

    if (failed(draw7.IDirectDraw7_SetDisplayMode(zigwin.WIDTH, //
        zigwin.HEIGHT, 32, 0, 0))) win32Panic();

    surfaceDes = std.mem.zeroInit(draw.DDSURFACEDESC2, .{
        .dwSize = @sizeOf(draw.DDSURFACEDESC2),
        .dwFlags = draw.DDSD_CAPS,
        .ddsCaps = .{ .dwCaps = draw.DDSCAPS_PRIMARYSURFACE },
    });

    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, //
        @ptrCast(&surface), null))) win32Panic();
}

fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    var ddbltfx = std.mem.zeroes(draw.DDBLTFX);
    ddbltfx.dwSize = @sizeOf(draw.DDBLTFX);
    ddbltfx.Anonymous5.dwFillColor = zigwin.rand.uintAtMost(u24, std.math.maxInt(u24));

    var rect = win32.foundation.RECT{
        .left = zigwin.rand.uintLessThan(u16, zigwin.WIDTH),
        .top = zigwin.rand.uintLessThan(u16, zigwin.HEIGHT),
        .right = zigwin.rand.uintLessThan(u16, zigwin.WIDTH),
        .bottom = zigwin.rand.uintLessThan(u16, zigwin.HEIGHT),
    };

    _ = surface.IDirectDrawSurface7_Blt(&rect, null, null, //
        draw.DDBLT_COLORFILL | draw.DDBLT_WAIT, &ddbltfx);

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = surface.IUnknown_Release();
    _ = draw7.IUnknown_Release();
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}
```

## 效果

![blt 填充][1]

[1]: images/directx30.png

## 附录
