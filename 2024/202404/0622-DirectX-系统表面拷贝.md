# 0622-DirectX-系统表面拷贝

## 目标

将系统表面的内容拷贝到后备缓冲区。

## 环境

- Time 2024-07-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

提取了创建设备的代码到 render 模块，main.zig 中的代码看起来少了一点。
很多方法都变了，创建表面的类型需要从 DEFAULT 修改为 SYSTEMMEM。

## win.zig

无变化。

## render.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d9 = win32.graphics.direct3d9;

pub var device: *d3d9.IDirect3DDevice9 = undefined;
pub var mode: d3d9.D3DDISPLAYMODE = undefined;

const HWND = win32.foundation.HWND;
const failed = win32.zig.FAILED;
pub fn init(width: u32, height: u32, h: HWND) void {
    const d9 = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;

    const count = d9.IDirect3D9_GetAdapterCount();
    std.log.debug("adapter count: {d}", .{count});

    var identifier: d3d9.D3DADAPTER_IDENTIFIER9 = undefined;

    for (0..count) |adapter| {
        const i: u32 = @intCast(adapter);
        const r = d9.IDirect3D9_GetAdapterIdentifier(i, 0, &identifier);
        if (failed(r)) win32Panic();

        std.log.debug("adapter Driver: {s}", .{identifier.Driver});
        std.log.debug("adapter name: {s}", .{identifier.Description});
    }

    const adapter = d3d9.D3DADAPTER_DEFAULT;
    var hr = d9.IDirect3D9_GetAdapterDisplayMode(adapter, &mode);
    if (failed(hr)) win32Panic();

    var params: d3d9.D3DPRESENT_PARAMETERS = undefined;

    //back buffer information
    params.BackBufferWidth = width;
    params.BackBufferHeight = height;
    params.BackBufferFormat = mode.Format;
    params.BackBufferCount = 1; //make one back buffer

    //multisampling
    params.MultiSampleType = .NONE;
    params.MultiSampleQuality = 0;

    //swap effect
    params.SwapEffect = .COPY; //we want to copy from back buffer to screen
    params.Windowed = win32.zig.TRUE; //windowed mode

    //destination window
    params.hDeviceWindow = h;

    //depth buffer information
    params.EnableAutoDepthStencil = win32.zig.FALSE;
    params.AutoDepthStencilFormat = .UNKNOWN;

    //flags
    params.Flags = 0;

    //refresh rate and presentation interval
    params.FullScreen_RefreshRateInHz = d3d9.D3DPRESENT_RATE_DEFAULT;
    params.PresentationInterval = d3d9.D3DPRESENT_INTERVAL_DEFAULT;

    //attempt to create a HAL device
    hr = d9.IDirect3D9_CreateDevice(adapter, .HAL, h, //
        d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device));
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetRenderState(.LIGHTING, 0);
    if (failed(hr)) win32Panic();
}

fn win32Panic() noreturn {
    const err = win32.foundation.GetLastError();
    std.log.err("win32 painc code {}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const win = @import("win.zig");
const render = @import("render.zig");
const d3d9 = win32.graphics.direct3d9;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;

var device: *d3d9.IDirect3DDevice9 = undefined;
var surface: *d3d9.IDirect3DSurface9 = undefined;

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

    // set up image surface
    const hr = device.IDirect3DDevice9_CreateOffscreenPlainSurface( //
        win.WIDTH / 2, win.HEIGHT / 2, render.mode.Format, //
        .SYSTEMMEM, @ptrCast(&surface), null);
    if (failed(hr)) win32Panic();
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
    rect.right = win.WIDTH / 2;
    rect.bottom = win.HEIGHT / 2;

    const point = win32.foundation.POINT{
        .x = win.WIDTH / 4,
        .y = win.HEIGHT / 4,
    };

    // grab back buffer
    var backBuffer: ?*d3d9.IDirect3DSurface9 = undefined;
    hr = device.IDirect3DDevice9_GetBackBuffer(0, 0, .MONO, &backBuffer);
    if (failed(hr)) win32Panic();

    // copy rectangle
    hr = device.IDirect3DDevice9_UpdateSurface(surface, &rect, backBuffer, &point);
    if (failed(hr)) win32Panic();
    _ = backBuffer.?.IUnknown_Release();

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

![系统表面拷贝][1]

[1]: images/directx57.png

## 附录
