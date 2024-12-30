# 0593-DirectX-联合 GDI 和 DirectX

## 目标

在使用 DirectX 的时候，使用 GDI 绘制文字。

## 环境

- Time 2024-07-07
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

前面有两个 256 色调色板技巧的示例，跳过了。一个是闪烁，一个是渐变，感觉调色板用得不多，先不写了。

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
var animationSeq: [4]u32 = .{ 0, 1, 0, 2 };
fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    // print shadowed text using GDI
    const x = zigwin.rand.uintLessThan(u32, zigwin.WIDTH);
    const y = zigwin.rand.uintLessThan(u32, zigwin.HEIGHT);

    // first print shadow
    const text = win32.zig.L("游戏编程大师");
    drawTextGDI(text, x + 4, y + 4, 0x00404040) catch unreachable;

    // now text on top of it
    const color = zigwin.rand.uintAtMost(u24, std.math.maxInt(u24));
    drawTextGDI(text, x, y, color) catch unreachable;

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn drawTextGDI(text: [:0]const u16, x: u32, y: u32, color: u32) !void {

    // the working dc
    var hdc: win32.graphics.gdi.HDC = undefined;
    // get the dc from surface
    if (failed(surface.IDirectDrawSurface7_GetDC(@ptrCast(&hdc)))) win32Panic();
    defer _ = surface.IDirectDrawSurface7_ReleaseDC(hdc);

    // set the colors for the text up
    _ = win32.graphics.gdi.SetTextColor(hdc, color);
    // set background mode to transparent so black isn't copied
    _ = win32.graphics.gdi.SetBkMode(hdc, .TRANSPARENT);

    _ = win32.graphics.gdi.TextOut(hdc, @intCast(x), @intCast(y), text, @intCast(text.len));
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

![GDI 和 DirectX][1]

[1]: images/directx38.png

## 附录
