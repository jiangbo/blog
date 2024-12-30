# 0582-DirectX-手动双缓冲

## 环境

- Time 2024-07-02
- Zig 0.12.0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

手动实现双缓冲的切换。

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

var allocator: std.mem.Allocator = undefined;
var draw7: *draw.IDirectDraw7 = undefined;
var surfaceDes: draw.DDSURFACEDESC2 = undefined;
var surface: *draw.IDirectDrawSurface7 = undefined;

var doubleBuffer: []u32 = undefined;

const H = std.os.windows.HINSTANCE;
pub fn wWinMain(h: H, _: ?H, _: [*:0]u16, _: u32) callconv(WINAPI) i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();

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

    const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel(zigwin.hander, style)))
        win32Panic();

    if (failed(draw7.IDirectDraw7_SetDisplayMode(zigwin.WIDTH, //
        zigwin.HEIGHT, 32, 0, 0))) win32Panic();

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);
    surfaceDes.dwFlags = draw.DDSD_CAPS;
    surfaceDes.ddsCaps.dwCaps = draw.DDSCAPS_PRIMARYSURFACE;

    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, //
        @ptrCast(&surface), null))) win32Panic();

    doubleBuffer = allocator.alloc(u32, zigwin.WIDTH * zigwin.HEIGHT) catch unreachable;
}

fn gameUpdate() void {
    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    @memset(doubleBuffer, 0);

    for (0..10000) |_| {
        const color = zigwin.rand.uintAtMost(u24, std.math.maxInt(u24));
        const x = zigwin.rand.uintLessThan(usize, zigwin.WIDTH);
        const y = zigwin.rand.uintLessThan(usize, zigwin.HEIGHT);
        const offset = x + y * zigwin.WIDTH;
        doubleBuffer[offset] = color;
    }

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);

    if (failed(surface.IDirectDrawSurface7_Lock(null, &surfaceDes, //
        draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null))) win32Panic();

    const primaryBuffer: [*]u32 = @ptrCast(@alignCast(surfaceDes.lpSurface));

    @memcpy(primaryBuffer, doubleBuffer);
    _ = surface.IDirectDrawSurface7_Unlock(null);

    // lock to 30 fps
    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = surface.IUnknown_Release();
    _ = draw7.IUnknown_Release();
    allocator.free(doubleBuffer);
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}
```

## 效果

![手动双缓冲][1]

## 总结

手动实现了双缓冲，双缓冲是游戏开发的一项重要技术，可以避免画面闪烁。

[1]: images/directx28.png

## 附录
