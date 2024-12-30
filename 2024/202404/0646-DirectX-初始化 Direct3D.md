# 0646-DirectX-初始化 Direct3D

## 目标

新建 window 窗口，初始化 Direct3D 环境。

## 环境

- Time 2024-08-09
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》

## 想法

开新系列了，听说这本书写得很好，阅读试试。

## d3d.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const gdi = win32.graphics.gdi;
const ui = win32.ui.windows_and_messaging;
const d3d9 = win32.graphics.direct3d9;
const WINAPI = std.os.windows.WINAPI;
const failed = win32.zig.FAILED;

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

const name = win32.zig.L("Direct9 3D游戏开发编程基础");

pub fn initD3D(width: i32, height: i32) *d3d9.IDirect3DDevice9 {
    //
    // Create the main application window.
    //
    var device: *d3d9.IDirect3DDevice9 = undefined;
    const h = win32.system.library_loader.GetModuleHandle(null).?;
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);

    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1 };
    windowClass.lpszClassName = name;
    windowClass.lpfnWndProc = mainWindowCallback;
    windowClass.hInstance = h;
    windowClass.hbrBackground = gdi.GetStockObject(gdi.BLACK_BRUSH);

    if (ui.RegisterClassEx(&windowClass) == 0) win32Panic();
    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const window = ui.CreateWindowEx(ui.WS_EX_TOPMOST, name, name, style, //
        200, 200, width, height, null, null, h, null).?;

    // Init D3D:
    // Step 1: Create the IDirect3D9 object.
    const d9 = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;

    // Step 2: Check for hardware vp.
    // Step 3: Fill out the D3DPRESENT_PARAMETERS structure.

    const adapter = d3d9.D3DADAPTER_DEFAULT;
    var mode: d3d9.D3DDISPLAYMODE = undefined;
    var hr = d9.IDirect3D9_GetAdapterDisplayMode(adapter, &mode);
    if (failed(hr)) win32Panic();

    var params: d3d9.D3DPRESENT_PARAMETERS = undefined;

    //back buffer information
    params.BackBufferWidth = @intCast(width);
    params.BackBufferHeight = @intCast(height);
    params.BackBufferFormat = mode.Format;
    params.BackBufferCount = 1; //make one back buffer

    //multisampling
    params.MultiSampleType = .NONE;
    params.MultiSampleQuality = 0;

    //swap effect
    params.SwapEffect = .DISCARD;
    params.Windowed = win32.zig.TRUE; //windowed mode

    //destination window
    params.hDeviceWindow = window;

    //depth buffer information
    params.EnableAutoDepthStencil = win32.zig.TRUE;
    params.AutoDepthStencilFormat = .D24S8;

    //flags
    params.Flags = 0;

    //refresh rate and presentation interval
    params.FullScreen_RefreshRateInHz = d3d9.D3DPRESENT_RATE_DEFAULT;
    params.PresentationInterval = d3d9.D3DPRESENT_INTERVAL_DEFAULT;

    //attempt to create a HAL device
    hr = d9.IDirect3D9_CreateDevice(adapter, .HAL, window, //
        d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device));
    if (failed(hr)) win32Panic();

    _ = d9.IUnknown_Release(); // done with d3d9 object
    return device;
}

var lastTime: f32 = 0;

pub fn enterMsgLoop(display: fn (f32) bool) void {
    lastTime = @floatFromInt(win32.media.timeGetTime());
    var message: ui.MSG = std.mem.zeroes(ui.MSG);
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        } else {
            const curTime: f32 = @floatFromInt(win32.media.timeGetTime());
            const delta = (curTime - lastTime) * 0.001;
            _ = display(delta);
            lastTime = curTime;
        }
    }
}

pub fn win32Panic() noreturn {
    const err = win32.foundation.GetLastError();
    std.log.err("win32 panic code {}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const failed = win32.zig.FAILED;

// Globals
pub const UNICODE: bool = true;
var device: *d3d9.IDirect3DDevice9 = undefined;

// Framework Functions
fn setup() bool {
    // Nothing to setup in this sample.
    return true;
}

fn cleanup() void {
    // Nothing to cleanup in this sample.
}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

fn display(_: f32) bool {
    // Instruct the device to set each pixel on the back buffer black -
    // D3DCLEAR_TARGET: 0x00000000 (black) - and to set each pixel on
    // the depth buffer to a value of 1.0 - D3DCLEAR_ZBUFFER: 1.0f.
    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0x00000000, 1, 0);
    if (failed(hr)) win32Panic();

    // Swap the back and front buffers.
    hr = device.IDirect3DDevice9_Present(null, null, null, null);
    if (failed(hr)) win32Panic();
    return true;
}

// main
pub fn main() void {
    device = d3d.initD3D(640, 480);

    if (!setup()) @panic("Setup() - FAILED");

    d3d.enterMsgLoop(display);

    cleanup();
    _ = device.IUnknown_Release();
}
```

## 效果

![初始化 Direct3D][1]

[1]: images/directx80.png

## 附录
