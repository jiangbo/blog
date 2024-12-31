# 0692-DirectX10-创建框架和窗口

## 目标

新增 System，Input，Graphics 模块，显示一个窗口。

## 环境

- Time 2024-12-31
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

全屏弄了一下，有点不显示的问题，没有做全屏。其它地方搭了一个框架，不确定是否存在问题，有问题遇到再修改。

## main.zig

```zig
const std = @import("std");
const System = @import("System.zig");

pub const UNICODE: bool = true;

pub fn main() !void {
    var system = System.initialize();
    defer system.shutdown();

    system.run();
}
```

## System.zig

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
    const window = initializeWindows(800, 600);

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
        if (!self.Frame()) break;
    }
}

pub fn Frame(self: *@This()) bool {
    const key = win32.ui.input.keyboard_and_mouse.VK_ESCAPE;
    if (self.input.isKeyDown(@intFromEnum(key))) {
        return false;
    }

    return self.graphics.Frame();
}

pub fn shutdown(self: *@This()) void {
    self.graphics.shutdown();
    _ = ui.DestroyWindow(self.window);
}

fn initializeWindows(width: u16, height: u16) ?win32.foundation.HWND {
    const handle = win32.system.library_loader.GetModuleHandle(null).?;

    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    const className = win32.zig.L("DirectX10");
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1, .OWNDC = 1 };
    windowClass.lpszClassName = className;
    windowClass.lpfnWndProc = windowCallback;
    windowClass.hInstance = handle;

    win32Check(ui.RegisterClassEx(&windowClass));

    const posX = @divTrunc(ui.GetSystemMetrics(.CXSCREEN) - width, 2);
    const posY = @divTrunc(ui.GetSystemMetrics(.CYSCREEN) - height, 2);
    const name = win32.zig.L("DirectX10 学习");
    const window = ui.CreateWindowEx(ui.WS_EX_APPWINDOW, className, name, //
        ui.WS_OVERLAPPEDWINDOW, posX, posY, width, height, null, null, handle, null);
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

## Input.zig

```zig
const std = @import("std");
const win32 = @import("win32");

keys: [256]bool,

pub fn initialize() @This() {
    return .{ .keys = .{false} ** 256 };
}

pub fn keyDown(self: *@This(), input: usize) void {
    self.keys[input] = true;
}

pub fn keyUp(self: *@This(), input: usize) void {
    self.keys[input] = false;
}

pub fn isKeyDown(self: *@This(), input: u32) bool {
    return self.keys[input];
}
```

## Graphics.zig

```zig
const std = @import("std");
const win32 = @import("win32");

pub const VSYNC_ENABLED: bool = true;
pub const SCREEN_DEPTH: f32 = 1000.0;
pub const SCREEN_NEAR: f32 = 0.1;

pub fn initialize(window: ?win32.foundation.HWND) @This() {
    _ = window;
    return .{};
}

pub fn shutdown(self: *@This()) void {
    _ = self;
}

pub fn Frame(self: *@This()) bool {
    _ = self;
    return true;
}

pub fn Render(self: *@This()) bool {
    _ = self;
    return true;
}
```

## 效果

![创建窗口][1]

[1]: images/directx035.png

## 附录
