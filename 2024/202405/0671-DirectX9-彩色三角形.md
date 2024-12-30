# 0671-DirectX9-彩色三角形

## 目标

使用顶点缓冲区来渲染一个彩色的三角形。

## 环境

- Time 2024-09-28
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Teach Yourself Game Programming with DirectX in 21 Days》

## 想法

画出彩色的三角形之前已经做过多次，把之前的代码拷贝过来就行。

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

const Vertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    rhw: f32 = 0,
    color: u32 = 0xffffffff,
};

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

    // 创建顶点缓存
    var buffer: *d3d9.IDirect3DVertexBuffer9 = undefined;
    defer _ = buffer.IUnknown_Release();
    const fvf = win32.system.system_services.D3DFVF_XYZRHW |
        win32.system.system_services.D3DFVF_DIFFUSE;
    win32Check(device.IDirect3DDevice9_CreateVertexBuffer(3 * @sizeOf(Vertex), //
        d3d9.D3DUSAGE_WRITEONLY, fvf, .DEFAULT, @ptrCast(&buffer), null));

    var data: [*]Vertex = undefined;
    win32Check(buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&data), 0));
    data[0] = .{ .x = 400, .y = 180, .color = 0xffff0000 };
    data[1] = .{ .x = 500, .y = 380, .color = 0xff00ff00 };
    data[2] = .{ .x = 300, .y = 380, .color = 0xff0000ff };
    win32Check(buffer.IDirect3DVertexBuffer9_Unlock());

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    win32Check(device.IDirect3DDevice9_Clear(0, null, flags, 0x00000000, 0, 0));

    win32Check(device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex)));
    win32Check(device.IDirect3DDevice9_SetFVF(fvf));
    win32Check(device.IDirect3DDevice9_BeginScene());

    win32Check(device.IDirect3DDevice9_DrawPrimitive(d3d9.D3DPT_TRIANGLELIST, 0, 1));

    win32Check(device.IDirect3DDevice9_EndScene());
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

## main.zig

无变化。

## 效果

![彩色三角形][1]

[1]: images/directx021.png

## 附录
