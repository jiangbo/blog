# 0587-DirectX-DirectDraw 裁剪

## 目标

使用 DirectDraw 的裁剪功能。

## 环境

- Time 2024-07-03
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 说明

这个裁剪有点复杂，代码能通过编译，逻辑是否正确未知。先就这样，后面如果还有再看。

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
var backup: *draw.IDirectDrawSurface7 = undefined;
var clipper: *draw.IDirectDrawClipper = undefined;

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

    const style = draw.DDSCL_FULLSCREEN | draw.DDSCL_ALLOWMODEX |
        draw.DDSCL_EXCLUSIVE | draw.DDSCL_ALLOWREBOOT;

    // const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel(zigwin.hander, style)))
        win32Panic();

    if (failed(draw7.IDirectDraw7_SetDisplayMode(zigwin.WIDTH, //
        zigwin.HEIGHT, 32, 0, 0))) win32Panic();

    surfaceDes = std.mem.zeroInit(draw.DDSURFACEDESC2, .{
        .dwSize = @sizeOf(draw.DDSURFACEDESC2),
        .Anonymous2 = .{ .dwBackBufferCount = 1 },
        .dwFlags = draw.DDSD_CAPS | draw.DDSD_BACKBUFFERCOUNT,
        .ddsCaps = .{
            .dwCaps = draw.DDSCAPS_PRIMARYSURFACE | //
                draw.DDSCAPS_COMPLEX | draw.DDSCAPS_FLIP,
        },
    });

    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, //
        @ptrCast(&surface), null))) win32Panic();

    surfaceDes.ddsCaps.dwCaps = draw.DDSCAPS_BACKBUFFER;
    if (failed(surface.IDirectDrawSurface7_GetAttachedSurface( //
        &surfaceDes.ddsCaps, @ptrCast(&backup)))) win32Panic();

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);

    if (failed(backup.IDirectDrawSurface7_Lock(null, &surfaceDes, //
        draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null))) win32Panic();

    var backBuffer: [*]u32 = @ptrCast(@alignCast(surfaceDes.lpSurface));
    for (backBuffer[0 .. zigwin.WIDTH * zigwin.HEIGHT]) |*value| {
        value.* = zigwin.rand.uintAtMost(u24, std.math.maxInt(u24));
    }

    if (failed(backup.IDirectDrawSurface7_Unlock(null))) win32Panic();

    // now create and attach clipper
    drawAttachClipper();
}

fn drawAttachClipper() void {
    var clips = [_]win32.foundation.RECT{
        .{ .left = 0, .top = 0, .right = 100, .bottom = 100 },
        .{ .left = 100, .top = 100, .right = 200, .bottom = 200 },
        .{ .left = 300, .top = 300, .right = 500, .bottom = 450 },
    };
    // first create the direct draw clipper
    if (failed(draw7.IDirectDraw7_CreateClipper(0, @ptrCast(&clipper), null)))
        win32Panic();

    // now create the clip list from the sent data

    // first allocate memory for region data
    const RGNDATA = win32.graphics.gdi.RGNDATA;
    const length = clips.len * @sizeOf(win32.foundation.RECT);
    const regions = allocator.alignedAlloc(RGNDATA, 128, 1);
    defer allocator.free(regions catch unreachable);

    var region = (regions catch unreachable)[0];

    const regionBuffer: [*]u8 = @ptrCast(&region.Buffer);
    var source: [*]u8 = @alignCast(@ptrCast(&clips));

    @memcpy(regionBuffer[0..length], source[0..length]);

    // set up fields of header
    region.rdh.dwSize = @sizeOf(win32.graphics.gdi.RGNDATAHEADER);
    region.rdh.iType = win32.graphics.gdi.RDH_RECTANGLES;
    region.rdh.nCount = clips.len;
    region.rdh.nRgnSize = length;

    region.rdh.rcBound.left = 64000;
    region.rdh.rcBound.top = 64000;
    region.rdh.rcBound.right = -64000;
    region.rdh.rcBound.bottom = -64000;

    // find bounds of all clipping regions
    for (0..clips.len) |index| {
        // test if the next rectangle unioned with the current bound is larger
        if (clips[index].left < region.rdh.rcBound.left)
            region.rdh.rcBound.left = clips[index].left;

        if (clips[index].right > region.rdh.rcBound.right)
            region.rdh.rcBound.right = clips[index].right;

        if (clips[index].top < region.rdh.rcBound.top)
            region.rdh.rcBound.top = clips[index].top;

        if (clips[index].bottom > region.rdh.rcBound.bottom)
            region.rdh.rcBound.bottom = clips[index].bottom;
    }

    // now we have computed the bounding rectangle region and set up the data
    // now let's set the clipping list
    if (failed(clipper.IDirectDrawClipper_SetClipList(&region, 0)))
        win32Panic();

    if (failed(surface.IDirectDrawSurface7_SetClipper(clipper)))
        win32Panic();
}

fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    var source = win32.foundation.RECT{
        .left = zigwin.rand.uintLessThan(u16, zigwin.WIDTH),
        .top = zigwin.rand.uintLessThan(u16, zigwin.HEIGHT),
        .right = zigwin.rand.uintLessThan(u16, zigwin.WIDTH),
        .bottom = zigwin.rand.uintLessThan(u16, zigwin.HEIGHT),
    };
    var dest = win32.foundation.RECT{
        .left = zigwin.rand.uintLessThan(u16, zigwin.WIDTH),
        .top = zigwin.rand.uintLessThan(u16, zigwin.HEIGHT),
        .right = zigwin.rand.uintLessThan(u16, zigwin.WIDTH),
        .bottom = zigwin.rand.uintLessThan(u16, zigwin.HEIGHT),
    };

    _ = surface.IDirectDrawSurface7_Blt(&dest, backup, &source, //
        draw.DDBLT_WAIT, null);

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = backup.IUnknown_Release();
    _ = surface.IUnknown_Release();
    _ = draw7.IUnknown_Release();
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}
```

## 附录
