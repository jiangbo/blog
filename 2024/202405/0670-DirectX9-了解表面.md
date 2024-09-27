# 0670-DirectX9-了解表面

## 目标

使用表面 Surface 来显示图片。

## 环境

- Time 2024-09-27
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Teach Yourself Game Programming with DirectX in 21 Days》

## 想法

之前使用的纹理来显示的图片，这里使用表面来显示，还没有写过，创建离屏表面要使用 SYSTEMMEM 类型，不然报错。
这个图片是网上随便找的一张图片，只要和后备缓冲的大小一样，就可以。

## d3d.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3dx9 = @import("d3dx9.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub fn windowCallback(
    window: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(std.os.windows.WINAPI) win32.foundation.LRESULT {
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

pub fn initDirectX(width: i32, height: i32) void {
    const h = win32.system.library_loader.GetModuleHandle(null).?;
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);

    const className = win32.zig.L("TeachYourselfDirectX9");
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1 };
    windowClass.lpszClassName = className;
    windowClass.lpfnWndProc = windowCallback;
    windowClass.hInstance = h;

    win32Check(ui.RegisterClassEx(&windowClass));
    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const name = win32.zig.L("2D 游戏开发");
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, className, name, //
        style, 200, 200, width, height, null, null, h, null).?;

    var d9 = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;
    defer _ = d9.IUnknown_Release();

    const adapter = d3d9.D3DADAPTER_DEFAULT;
    var mode: d3d9.D3DDISPLAYMODE = undefined;
    win32Check(d9.IDirect3D9_GetAdapterDisplayMode(adapter, &mode));

    var params = std.mem.zeroes(d3d9.D3DPRESENT_PARAMETERS);

    // 后备缓冲区信息
    params.BackBufferWidth = @intCast(width);
    params.BackBufferHeight = @intCast(height);
    params.BackBufferFormat = mode.Format;
    params.BackBufferCount = 1; // 使用一个后备缓冲

    // 交换效果
    params.SwapEffect = .DISCARD;
    params.Windowed = win32.zig.TRUE; // 窗口模式

    // 渲染的目的窗口
    params.hDeviceWindow = window;

    // 创建设备
    var device: *d3d9.IDirect3DDevice9 = undefined;
    win32Check(d9.IDirect3D9_CreateDevice(adapter, .HAL, window, //
        d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device)));
    defer _ = device.IUnknown_Release();

    // 创建源表面
    var surface: *d3d9.IDirect3DSurface9 = undefined;
    win32Check(device.IDirect3DDevice9_CreateOffscreenPlainSurface(params.BackBufferWidth, //
        params.BackBufferHeight, mode.Format, .SYSTEMMEM, @ptrCast(&surface), null));
    defer _ = surface.IUnknown_Release();

    // 加载图片到源表面
    const fileName = win32.zig.L("dashangu.jpg");
    const filter = std.math.maxInt(u32);
    win32Check(d3dx9.D3DXLoadSurfaceFromFileW(surface, null, null, fileName, null, filter, 0, 0));

    // 获取后备缓冲
    var back: *d3d9.IDirect3DSurface9 = undefined;
    win32Check(device.IDirect3DDevice9_GetBackBuffer(0, 0, .MONO, @ptrCast(&back)));
    defer _ = back.IUnknown_Release();

    // const flags = win32.system.system_services.D3DCLEAR_TARGET;
    // win32Check(device.IDirect3DDevice9_Clear(0, null, flags, 0x00ff00ff, 0, 0));
    // 将源表面绘制到后备缓冲
    win32Check(device.IDirect3DDevice9_UpdateSurface(surface, null, back, null));
    win32Check(device.IDirect3DDevice9_Present(null, null, null, null));

    var message: ui.MSG = std.mem.zeroes(ui.MSG);
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
    }
}

pub fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    const err = win32.foundation.GetLastError();
    std.log.err("win32 panic code 0X{0X}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## d3dx9.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const d3d9 = win32.graphics.direct3d9;
pub const LPCTSTR = [*:0]const u16;

pub extern fn D3DXLoadSurfaceFromFileW(
    surface: *d3d9.IDirect3DSurface9,
    palette: ?*const win32.graphics.gdi.PALETTEENTRY,
    rect: ?*const win32.foundation.RECT,
    srcFile: LPCTSTR,
    srcRect: ?*const win32.foundation.RECT,
    filter: u32,
    colorkey: u32,
    srcInfo: usize,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;
```

## main.zig

无变化。

## 效果

![了解表面][1]。

[1]: images/directx020.png

## 附录
