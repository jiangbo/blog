# 0694-DirectX10-创建交换链

## 目标

创建交换。

## 环境

- Time 2025-01-04
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

其中涉及到的概念，可以看龙书，这里就不介绍这些概念信息了。
发现 build.zig 中加入的链接库好像没有用，后面去掉看看报错不。

## Graphics.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Direct3D = @import("Direct3D.zig");

pub const WIDTH: u16 = 800;
pub const HEIGHT: u16 = 600;
pub const VSYNC_ENABLED: bool = true;
pub const SCREEN_DEPTH: f32 = 1000.0;
pub const SCREEN_NEAR: f32 = 0.1;

direct3D: Direct3D,

pub fn initialize(window: ?win32.foundation.HWND) @This() {
    var direct = Direct3D{};

    direct.initialize(WIDTH, HEIGHT, window);
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
const direct3d10 = win32.graphics.direct3d10;

device: *direct3d10.ID3D10Device = undefined,
swapChain: *dxgi.IDXGISwapChain = undefined,

pub fn initialize(self: *@This(), width: u16, height: u16, window: ?win32.foundation.HWND) void {
    var desc = std.mem.zeroes(dxgi.DXGI_SWAP_CHAIN_DESC);
    desc.BufferDesc.Width = width;
    desc.BufferDesc.Height = height;
    desc.BufferDesc.RefreshRate = .{ .Numerator = 60, .Denominator = 1 };
    desc.BufferDesc.Format = .R8G8B8A8_UNORM;

    desc.SampleDesc = .{ .Count = 1, .Quality = 0 };
    desc.BufferUsage = dxgi.DXGI_USAGE_RENDER_TARGET_OUTPUT;
    desc.BufferCount = 1;
    desc.OutputWindow = window;
    desc.Windowed = win32.zig.TRUE;
    desc.SwapEffect = .DISCARD;

    win32Check(direct3d10.D3D10CreateDeviceAndSwapChain(null, .NULL, null, 0, //
        direct3d10.D3D10_SDK_VERSION, &desc, @ptrCast(&self.swapChain), @ptrCast(&self.device)));
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
    _ = self.swapChain.IUnknown.Release();
    _ = self.device.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 附录
