# 0709-DirectX11-计算窗口位置

## 目标

之前创建的窗口，虽然指定了大小和位置，但是有菜单这些，导致计算不准确，重新计算。

## 环境

- Time 2025-01-07
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

之前也使用了 `AdjustWindowRect` 函数，当时使用的方式不对，跳过了没有解决，这里重新计算。

## System.zig

所有的窗口的数据都要从 RECT 中去获取。

```zig
const std = @import("std");
const win32 = @import("win32");
const Input = @import("Input.zig");
const Graphics = @import("Graphics.zig");

const ui = win32.ui.windows_and_messaging;

var applicationHandle: *@This() = undefined;
window: ?win32.foundation.HWND = null,
input: Input,
graphics: Graphics,

pub fn initialize() @This() {
    const window = initializeWindows(Graphics.WIDTH, Graphics.HEIGHT);

    return .{
        .window = window,
        .input = Input.initialize(),
        .graphics = Graphics.initialize(window),
    };
}

pub fn run(self: *@This()) void {
    applicationHandle = self;
    var message: ui.MSG = std.mem.zeroes(ui.MSG);

    while (true) {
        while (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
        if (message.message == ui.WM_QUIT) break;
        if (!self.frame()) break;
    }
}

pub fn frame(self: *@This()) bool {
    const key = win32.ui.input.keyboard_and_mouse.VK_ESCAPE;
    if (self.input.isKeyDown(@intFromEnum(key))) {
        return false;
    }

    return self.graphics.frame();
}

pub fn shutdown(self: *@This()) void {
    self.graphics.shutdown();
    _ = ui.DestroyWindow(self.window);
}

fn initializeWindows(width: u16, height: u16) ?win32.foundation.HWND {
    const handle = win32.system.library_loader.GetModuleHandle(null).?;

    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    const className = win32.zig.L("DirectX11");
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1, .OWNDC = 1 };
    windowClass.lpszClassName = className;
    windowClass.lpfnWndProc = windowCallback;
    windowClass.hInstance = handle;

    win32Check(ui.RegisterClassEx(&windowClass));

    // 计算位置
    const posX = @divTrunc(ui.GetSystemMetrics(.CXSCREEN) - width, 2);
    const posY = @divTrunc(ui.GetSystemMetrics(.CYSCREEN) - height, 2);
    var rect: win32.foundation.RECT = .{
        .left = posX,
        .top = posY,
        .right = posX + width,
        .bottom = posY + height,
    };
    const style = ui.WS_OVERLAPPEDWINDOW;
    win32Check(ui.AdjustWindowRect(&rect, style, win32.zig.FALSE));

    //  根据计算的位置创建窗口
    const name = win32.zig.L("DirectX11 学习");
    const window = ui.CreateWindowEx(.{}, className, name, style, rect.left, rect.top, //
        rect.right - rect.left, rect.bottom - rect.top, null, null, handle, null);
    _ = ui.ShowWindow(window, ui.SW_SHOW);
    return window;
}

fn windowCallback(
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
        ui.WM_KEYDOWN => applicationHandle.input.keyDown(wParam),
        ui.WM_KEYUP => applicationHandle.input.keyUp(wParam),
        else => {},
    }
    return ui.DefWindowProc(w, message, wParam, lParam);
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

通过截图获取的尺寸，手动框选的，有点误差。

![计算窗口位置][1]

[1]: images/directx049.png

## 附录
