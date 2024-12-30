# 0595-DirectX-在窗口中绘制像素

## 目标

使用窗口模式，并且随机画出像素点到窗口中。

## 环境

- Time 2024-07-07
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

WindowRect 包括了控件和边框，想要的窗口区其实叫做客户区，没想到这里面还有这些问题。

## win.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const zigwin = @import("win.zig");
const lib = @import("lib.zig");

const draw = win32.graphics.direct_draw;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;
var draw7: *draw.IDirectDraw7 = undefined;
var surfaceDes: draw.DDSURFACEDESC2 = undefined;
var surface: *draw.IDirectDrawSurface7 = undefined;

const H = std.os.windows.HINSTANCE;
pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    zigwin.createWindow();

    gameInit();
    zigwin.update(gameUpdate);
    gameShutdown();
}

const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    if (failed(draw.DirectDrawCreateEx(null, @ptrCast(&draw7), //
        draw.IID_IDirectDraw7, null))) win32Panic();

    const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel(zigwin.hander, style)))
        win32Panic();

    surfaceDes = std.mem.zeroInit(draw.DDSURFACEDESC2, .{
        .dwSize = @sizeOf(draw.DDSURFACEDESC2),
        .dwFlags = draw.DDSD_CAPS,
        .ddsCaps = .{ .dwCaps = draw.DDSCAPS_PRIMARYSURFACE },
    });

    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, //
        @ptrCast(&surface), null))) win32Panic();
}
var animationSeq: [4]u32 = .{ 0, 1, 0, 2 };
fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);
    if (failed(surface.IDirectDrawSurface7_Lock(null, &surfaceDes, //
        draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null))) win32Panic();

    var buffer: [*]u32 = @ptrCast(@alignCast(surfaceDes.lpSurface));
    const pitch32: usize = @intCast(surfaceDes.Anonymous1.lPitch >> 2);
    var client = std.mem.zeroes(win32.foundation.RECT);
    _ = win32.ui.windows_and_messaging.GetWindowRect(zigwin.hander, &client);

    for (0..1000) |_| {
        const left: usize = @intCast(client.left);
        const right: usize = @intCast(client.right);
        const top: usize = @intCast(client.top);
        const bottom: usize = @intCast(client.bottom);

        const x = zigwin.rand.intRangeLessThan(usize, left, right);
        const y = zigwin.rand.intRangeLessThan(usize, top, bottom);
        const color: u32 = zigwin.rand.uintAtMost(u24, std.math.maxInt(u24));
        buffer[x * 4 + y * pitch32] = color;
    }

    _ = surface.IDirectDrawSurface7_Unlock(null);

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

![窗口中绘制像素][1]

[1]: images/directx40.png

## 附录
