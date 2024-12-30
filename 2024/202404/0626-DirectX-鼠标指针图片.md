# 0626-DirectX-鼠标指针图片

## 目标

给鼠标指针增加一个图片。

## 环境

- Time 2024-07-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

感觉是加载鼠标的图片格式有问题，出来的鼠标图片感觉很奇怪，不过不想去排查哪里有问题了。
可以使用之前的 windows 窗口创建的时候加载鼠标图片的方法。

## win.zig

无变化。

## render.zig

无变化。

## lib.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const win = @import("win.zig");
const lib = @import("lib.zig");
const render = @import("render.zig");
const d3d9 = win32.graphics.direct3d9;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;

var device: *d3d9.IDirect3DDevice9 = undefined;
var surface: *d3d9.IDirect3DSurface9 = undefined;
var cursor: *d3d9.IDirect3DSurface9 = undefined;

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    win.createWindow();

    gameInit();
    win.update(gameUpdate);
    gameShutdown();
}

const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    render.init(win.WIDTH, win.HEIGHT, win.hander);
    device = render.device;

    // load in image
    const name = "tansdesk.bmp";
    surface = lib.loadSourface(allocator, device, render.mode.Format, name);

    const n = "none.bmp";
    // const mode: d3d9.D3DFORMAT = .A8R8G8B8;
    cursor = lib.loadSourface(allocator, device, render.mode.Format, n);
}

fn gameUpdate() void {
    if (win.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xffffffff, 0, 0);
    if (failed(hr)) win32Panic();

    // source rectangle
    var rect = std.mem.zeroes(win32.foundation.RECT);
    rect.right = win.WIDTH;
    rect.bottom = win.HEIGHT;

    var point = std.mem.zeroes(win32.foundation.POINT);

    // grab back buffer
    var back: ?*d3d9.IDirect3DSurface9 = undefined;
    hr = device.IDirect3DDevice9_GetBackBuffer(0, 0, .MONO, &back);
    if (failed(hr)) win32Panic();

    // copy rectangle
    hr = device.IDirect3DDevice9_UpdateSurface(surface, &rect, back, &point);
    if (failed(hr)) win32Panic();

    if (win.hover) |hover| {
        rect.right = 33;
        rect.bottom = 33;
        point.x = @intCast(hover.x);
        point.y = @intCast(hover.y);
        hr = device.IDirect3DDevice9_UpdateSurface(cursor, &rect, back, &point);
        // if (failed(hr)) win32Panic();
    }
    _ = back.?.IUnknown_Release();

    hr = device.IDirect3DDevice9_Present(null, null, null, null);
    if (failed(hr)) win32Panic();

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
}

fn win32Panic() noreturn {
    win.win32Panic();
}
```

## 效果

![鼠标图片][1]

[1]: images/directx60.png

## 附录
