# 0678-DirectX9-提取初始化方法

## 目标

开始编写提取窗口和 Direct3D 的初始化方法到引擎模块中。

## 环境

- Time 2024-10-13
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Teach Yourself Game Programming with DirectX in 21 Days》

## 想法

之前算练习，这里是正式开始编写游戏开发的代码了。
到这里发现没有书本配套的图片和地图数据，做不出来书本上的效果。那就不继续了，直接浏览一下本书，有需要再参考。

## engine.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub fn windowCallback(
    w: win32.foundation.HWND,
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
        else => return ui.DefWindowProc(w, message, wParam, lParam),
    }
    return 0;
}

pub const BookEngine = struct {
    hwnd: win32.foundation.HWND,

    pub fn init(width: i32, height: i32) BookEngine {
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
        return BookEngine{ .hwnd = window };
    }

    pub fn processGame() void {}
    pub fn handleKeys(wParam: std.os.windows.WPARAM) void {
        _ = wParam;
    }
};

pub const Direct3D = struct {
    hwnd: win32.foundation.HWND,
    d3d: *d3d9.IDirect3D9,
    device: *d3d9.IDirect3DDevice9,
    backBuffer: *d3d9.IDirect3DSurface9,

    pub fn init(width: i32, height: i32, hwnd: win32.foundation.HWND) Direct3D {
        var d3d = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;

        const adapter = d3d9.D3DADAPTER_DEFAULT;
        var mode: d3d9.D3DDISPLAYMODE = undefined;
        win32Check(d3d.IDirect3D9_GetAdapterDisplayMode(adapter, &mode));

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
        params.hDeviceWindow = hwnd;

        // 创建设备
        var device: *d3d9.IDirect3DDevice9 = undefined;
        win32Check(d3d.IDirect3D9_CreateDevice(adapter, .HAL, hwnd, //
            d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device)));

        // 获取后备缓冲
        var back: *d3d9.IDirect3DSurface9 = undefined;
        win32Check(device.IDirect3DDevice9_GetBackBuffer(0, 0, .MONO, @ptrCast(&back)));

        return Direct3D{
            .hwnd = hwnd,
            .d3d = d3d,
            .device = device,
            .backBuffer = back,
        };
    }

    pub fn deinit(self: Direct3D) void {
        _ = self.backBuffer.IUnknown_Release();
        _ = self.device.IUnknown_Release();
        _ = self.d3d.IUnknown_Release();
    }
};

pub fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    const err = win32.foundation.GetLastError();
    std.log.err("win32 panic code 0X{0X}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const engine = @import("engine.zig");
const d3dx9 = @import("d3dx9.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const win32Check = engine.win32Check;

pub const UNICODE: bool = true;

const WIDTH = 640;
const HEIGHT = 480;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const bookEngine = engine.BookEngine.init(WIDTH, HEIGHT);

    const direct3D = engine.Direct3D.init(WIDTH, HEIGHT, bookEngine.hwnd);
    defer direct3D.deinit();

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

![初始化][1]。

[1]: images/directx024.png

## 附录
