# 0680-DirectX9-初始化 DirectX

## 目标

初始化 DirectX 9。

## 环境

- Time 2024-12-29
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

初始化之前也写过了，直接拷贝过来修改一下。

## gfx.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub const GraphicsDevice = struct {
    device: *d3d9.IDirect3DDevice9,
    direct3d: *d3d9.IDirect3D9,

    pub fn init(window: win32.foundation.HWND) GraphicsDevice {
        var d3d = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;

        var params = std.mem.zeroes(d3d9.D3DPRESENT_PARAMETERS);
        params.Windowed = win32.zig.TRUE;
        params.SwapEffect = .DISCARD;
        params.hDeviceWindow = window;

        // 创建设备
        var device: *d3d9.IDirect3DDevice9 = undefined;
        win32Check(d3d.CreateDevice(d3d9.D3DADAPTER_DEFAULT, .HAL, window, //
            d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device)));
        return .{ .device = device, .direct3d = d3d };
    }

    pub fn clear(self: *GraphicsDevice, color: u32) void {
        const target = win32.system.system_services.D3DCLEAR_TARGET;
        win32Check(self.device.Clear(0, null, target, color, 1.0, 0));
    }

    pub fn begin(self: *GraphicsDevice) void {
        win32Check(self.device.BeginScene());
    }

    pub fn end(self: *GraphicsDevice) void {
        win32Check(self.device.EndScene());
    }

    pub fn Present(self: *GraphicsDevice) void {
        win32Check(self.device.Present(null, null, null, null));
    }

    pub fn deinit(self: *GraphicsDevice) void {
        _ = self.device.IUnknown.Release();
        _ = self.direct3d.IUnknown.Release();
    }
};

pub fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const gfx = @import("gfx.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const win32Check = gfx.win32Check;

pub const UNICODE: bool = true;

const WIDTH = 640;
const HEIGHT = 480;

pub fn main() !void {
    const window = generateWindow();

    var device = gfx.GraphicsDevice.init(window);
    defer device.deinit();

    var message: ui.MSG = std.mem.zeroes(ui.MSG);
    while (true) {
        while (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
        if (message.message == ui.WM_QUIT) break;

        update(0);
        draw(&device, 0);
    }
}

fn update(delta: f32) void {
    _ = delta;
}

fn draw(device: *gfx.GraphicsDevice, delta: f32) void {
    device.begin();
    device.clear(0x00FF00FF);
    device.end();
    device.Present();
    _ = delta;
}

pub fn windowCallback(
    w: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(std.os.windows.WINAPI) win32.foundation.LRESULT {
    switch (message) {
        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
            ui.PostQuitMessage(0);
        },
        else => {},
    }
    return ui.DefWindowProc(w, message, wParam, lParam);
}

fn generateWindow() win32.foundation.HWND {
    const handle = win32.system.library_loader.GetModuleHandle(null).?;

    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    const className = win32.zig.L("DirectX9");
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1 };
    windowClass.lpszClassName = className;
    windowClass.lpfnWndProc = windowCallback;
    windowClass.hInstance = handle;
    win32Check(ui.RegisterClassEx(&windowClass));

    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const name = win32.zig.L("DirectX9 学习");
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, className, name, //
        style, 200, 200, WIDTH, HEIGHT, null, null, handle, null).?;

    return window;
}
```

## 效果

![初始化 DirectX][1]。

[1]: images/directx026.png

## 附录
