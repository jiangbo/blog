# 0693-DirectX10-查询显卡信息

## 目标

到初始化 DirectX 了，先查询一下系统中的显卡信息。

## 环境

- Time 2025-01-04
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

一下子抛出的概念也太多了吧，不管现在用不用，全部一股脑全抛出来，也没个详细的解释。
一点一点来吧，先做需要的，不需要的后面用的时候再加。没有修改的地方，就不列出来了。

## Graphics.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Direct3D = @import("Direct3D.zig");

pub const VSYNC_ENABLED: bool = true;
pub const SCREEN_DEPTH: f32 = 1000.0;
pub const SCREEN_NEAR: f32 = 0.1;

direct3D: Direct3D,

pub fn initialize(window: ?win32.foundation.HWND) @This() {
    var direct = Direct3D{};

    direct.initialize(window);
    return .{ .direct3D = direct };
}

pub fn frame(self: *@This()) bool {
    return self.render();
}

pub fn render(self: *@This()) bool {
    self.direct3D.beginScene(0.5, 0.5, 0.5, 1.0);
    self.direct3D.endScene();
    return true;
}

pub fn shutdown(self: *@This()) void {
    self.direct3D.shutdown();
}
```

## Direct3D.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const dxgi = win32.graphics.dxgi;

pub fn initialize(self: *@This(), window: ?win32.foundation.HWND) void {
    _ = window;
    _ = self;
    var factory: *dxgi.IDXGIFactory = undefined;
    win32Check(dxgi.CreateDXGIFactory(dxgi.IID_IDXGIFactory, @ptrCast(&factory)));

    var adapter: *dxgi.IDXGIAdapter = undefined;
    win32Check(factory.EnumAdapters(0, @ptrCast(&adapter)));

    var index: u32 = 0;
    var desc: dxgi.DXGI_ADAPTER_DESC = undefined;
    var buffer: [512]u8 = undefined;
    while (factory.EnumAdapters(index, @ptrCast(&adapter)) != -2005270526) : (index += 1) {
        _ = adapter.GetDesc(&desc);
        const len = std.unicode.utf16LeToUtf8(&buffer, &desc.Description) catch unreachable;
        std.log.info("{s}", .{buffer[0..len]});
    }
}

pub fn beginScene(self: *@This(), red: f32, green: f32, blue: f32, alpha: f32) void {
    _ = self;
    _ = red;
    _ = green;
    _ = blue;
    _ = alpha;
}

pub fn endScene(self: *@This()) void {
    _ = self;
}

pub fn shutdown(self: *@This()) void {
    _ = self;
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![查询显卡信息][1]

[1]: images/directx036.png

## 附录
