# 0674-DirectX9-播放声音

## 目标

链接 winmm 库来进行声音的播放。

## 环境

- Time 2024-10-05
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Teach Yourself Game Programming with DirectX in 21 Days》

## 想法

之前已经使用过这个方法，参考之前的拷贝过来，声音的效果不好展示。

## d3d.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const winmm = @import("winmm.zig");

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
        ui.WM_KEYDOWN => {
            switch (wParam) {
                '1' => playSound(),
                else => {},
            }
        },
        else => return ui.DefWindowProc(window, message, wParam, lParam),
    }
    return 0;
}

fn playSound() void {
    _ = winmm.sndPlaySoundW(win32.zig.L("MonsterHit.wav"), winmm.SND_ASYNC);
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

## build.zig

链接 winmm 库。

```zig
...
exe.linkSystemLibrary("winmm");
...
```

## winmm.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;
pub const LPCTSTR = [*:0]align(1) const u16;
const BOOL = win32.foundation.BOOL;

pub const SND_RESOURCE: u32 = 0x00040004;
pub const SND_FILENAME: u32 = 0x00020000;
pub const SND_SYNC: u32 = 0x0000;
pub const SND_ASYNC: u32 = 0x0001;
pub const SND_LOOP: u32 = 0x0008;
pub const SND_PURGE: u32 = 0x0040;

pub extern fn PlaySoundW(n: ?LPCTSTR, w: H, f: u32) callconv(WINAPI) BOOL;

pub extern fn sndPlaySoundW(lpszSound: ?LPCTSTR, fuSound: u32) callconv(WINAPI) BOOL;
```

## d3dx9.zig

无变化。

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

    _ = device;
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

声音效果不好展示。

## 附录
