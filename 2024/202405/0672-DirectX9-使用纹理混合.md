# 0672-DirectX9-使用纹理混合

## 目标

使用纹理混合，将一个纹理去掉透明部分，渲染到背景上。

## 环境

- Time 2024-10-04
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Teach Yourself Game Programming with DirectX in 21 Days》

## 想法

之前的纹理加载也练习过了，不过使用的是一个简单的函数，这次使用的一个扩展函数。

## d3d.zig

分离了 d3d 中的功能，将其移动到了 main.zig 文件中。

```zig
const std = @import("std");
const win32 = @import("win32");

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

pub fn initDirectX(width: i32, height: i32) *d3d9.IDirect3DDevice9 {
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

    return device;
}

pub fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    const err = win32.foundation.GetLastError();
    std.log.err("win32 panic code 0X{0X}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## d3dx9.zig

新增了创建纹理的函数。

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

pub extern fn D3DXCreateTextureFromFileW(
    device: *d3d9.IDirect3DDevice9,
    name: LPCTSTR,
    LPDIRECT3DTEXTURE9: ?**d3d9.IDirect3DTexture9,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub extern fn D3DXCreateTextureFromFileExW(
    device: *d3d9.IDirect3DDevice9,
    name: LPCTSTR,
    width: u32,
    height: u32,
    mipLevels: u32,
    usage: u32,
    format: d3d9.D3DFORMAT,
    pool: d3d9.D3DPOOL,
    filter: u32,
    mipFilter: u32,
    colorkey: u32,
    pSrcInfo: usize,
    pPalette: ?*const win32.graphics.gdi.PALETTEENTRY,
    ppTexture: ?**d3d9.IDirect3DTexture9,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub const D3DX_DEFAULT = 0xffffffff;
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const win32Check = d3d.win32Check;

pub const UNICODE: bool = true;

const Vertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    rhw: f32 = 1,
    u: f32 = 0,
    v: f32 = 0,
};

const WIDTH = 800;
const HEIGHT = 600;

pub fn main() !void {
    const device = d3d.initDirectX(800, 600);

    // 创建源表面
    var surface: *d3d9.IDirect3DSurface9 = undefined;
    win32Check(device.IDirect3DDevice9_CreateOffscreenPlainSurface(WIDTH, //
        HEIGHT, .X8R8G8B8, .SYSTEMMEM, @ptrCast(&surface), null));
    defer _ = surface.IUnknown_Release();

    // 加载图片到源表面
    const fileName = win32.zig.L("dashangu.jpg");
    const filter = std.math.maxInt(u32);
    win32Check(d3dx9.D3DXLoadSurfaceFromFileW(surface, null, null, fileName, null, filter, 0, 0));

    const d = d3dx9.D3DX_DEFAULT;
    var texture: *d3d9.IDirect3DTexture9 = undefined;
    win32Check(d3dx9.D3DXCreateTextureFromFileExW(device, win32.zig.L("player.bmp"), d, //
        d, 1, 0, .A8R8G8B8, .MANAGED, d, d, 0xffffeebb, 0, null, &texture));

    // 创建顶点缓存
    var buffer: *d3d9.IDirect3DVertexBuffer9 = undefined;
    defer _ = buffer.IUnknown_Release();
    const fvf = win32.system.system_services.D3DFVF_XYZRHW |
        win32.system.system_services.D3DFVF_TEX1;
    win32Check(device.IDirect3DDevice9_CreateVertexBuffer(3 * @sizeOf(Vertex), //
        d3d9.D3DUSAGE_WRITEONLY, fvf, .DEFAULT, @ptrCast(&buffer), null));

    var data: [*]Vertex = undefined;
    win32Check(buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&data), 0));

    // 设置顶点数据
    data[0] = .{ .x = 250, .y = 392, .u = 0.0, .v = 1.0 };
    data[1] = .{ .x = 250, .y = 200, .u = 0.0, .v = 0.0 };
    data[2] = .{ .x = 346, .y = 392, .u = 1.0, .v = 1.0 };
    data[3] = .{ .x = 346, .y = 200, .u = 1.0, .v = 0.0 };

    win32Check(buffer.IDirect3DVertexBuffer9_Unlock());

    // 获取后备缓冲
    var back: *d3d9.IDirect3DSurface9 = undefined;
    win32Check(device.IDirect3DDevice9_GetBackBuffer(0, 0, .MONO, @ptrCast(&back)));
    defer _ = back.IUnknown_Release();

    // 将源表面绘制到后备缓冲
    win32Check(device.IDirect3DDevice9_UpdateSurface(surface, null, back, null));

    // 设置透明混合
    var state: u32 = @intFromEnum(d3d9.D3DTOP_SELECTARG1);
    win32Check(device.IDirect3DDevice9_SetTextureStageState(0, .COLOROP, state));
    win32Check(device.IDirect3DDevice9_SetTextureStageState(0, .ALPHAOP, state));

    state = win32.system.system_services.D3DTA_TEXTURE;
    win32Check(device.IDirect3DDevice9_SetTextureStageState(0, .COLORARG1, state));
    win32Check(device.IDirect3DDevice9_SetTextureStageState(0, .ALPHAARG1, state));

    state = @intFromEnum(d3d9.D3DBLEND_SRCALPHA);
    win32Check(device.IDirect3DDevice9_SetRenderState(.SRCBLEND, state));
    state = @intFromEnum(d3d9.D3DBLEND_INVSRCALPHA);
    win32Check(device.IDirect3DDevice9_SetRenderState(.DESTBLEND, state));
    win32Check(device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, 1));

    // 设置纹理
    win32Check(device.IDirect3DDevice9_SetFVF(fvf));
    win32Check(device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex)));
    win32Check(device.IDirect3DDevice9_SetTexture(0, @ptrCast(texture)));

    // 渲染，使用的是三角形带
    win32Check(device.IDirect3DDevice9_BeginScene());
    win32Check(device.IDirect3DDevice9_DrawPrimitive(d3d9.D3DPT_TRIANGLESTRIP, 0, 2));
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
```

## 效果

![纹理混合][1]

[1]: images/directx022.png

## 附录
