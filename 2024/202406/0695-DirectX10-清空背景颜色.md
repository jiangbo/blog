# 0695-DirectX10-清空背景颜色

## 目标

使用固定的颜色，将窗口的背景颜色清空，相当于使用固定的颜色填充。

## 环境

- Time 2025-01-05
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

创建交换链的时候，将驱动类型设置成 NULL 了，导致一直不行，排查了很久。

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
    self.direct3D.beginScene(1, 0, 1, 1);
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
const d10 = win32.graphics.direct3d10;

device: *d10.ID3D10Device = undefined,
swapChain: *dxgi.IDXGISwapChain = undefined,
targetView: *d10.ID3D10RenderTargetView = undefined,
depthStencilBuffer: *d10.ID3D10Texture2D = undefined,
depthStencilState: *d10.ID3D10DepthStencilState = undefined,
depthStencilView: *d10.ID3D10DepthStencilView = undefined,

pub fn initialize(self: *@This(), w: u16, h: u16, window: ?win32.foundation.HWND) void {
    var desc = std.mem.zeroes(dxgi.DXGI_SWAP_CHAIN_DESC);
    desc.BufferDesc.Width = w;
    desc.BufferDesc.Height = h;
    desc.BufferDesc.RefreshRate = .{ .Numerator = 60, .Denominator = 1 };
    desc.BufferDesc.Format = .R8G8B8A8_UNORM;

    desc.SampleDesc = .{ .Count = 1, .Quality = 0 };
    desc.BufferUsage = dxgi.DXGI_USAGE_RENDER_TARGET_OUTPUT;
    desc.BufferCount = 1;
    desc.OutputWindow = window;
    desc.Windowed = win32.zig.TRUE;

    const flags: u16 = @intFromEnum(d10.D3D10_CREATE_DEVICE_DEBUG);
    win32Check(d10.D3D10CreateDeviceAndSwapChain(null, .HARDWARE, null, flags, //
        d10.D3D10_SDK_VERSION, &desc, @ptrCast(&self.swapChain), @ptrCast(&self.device)));

    var back: *d10.ID3D10Texture2D = undefined;
    win32Check(self.swapChain.GetBuffer(0, d10.IID_ID3D10Texture2D, @ptrCast(&back)));
    defer _ = back.IUnknown.Release();

    const target: *?*d10.ID3D10RenderTargetView = @ptrCast(&self.targetView);
    win32Check(self.device.CreateRenderTargetView(@ptrCast(back), null, target));

    self.device.OMSetRenderTargets(1, @ptrCast(&self.targetView), null);
}

pub fn beginScene(self: *@This(), red: f32, green: f32, blue: f32, alpha: f32) void {
    const color = [_]f32{ red, green, blue, alpha };
    self.device.ClearRenderTargetView(self.targetView, @ptrCast(&color));
}

pub fn endScene(self: *@This()) void {
    win32Check(self.swapChain.Present(1, 0));
}

pub fn shutdown(self: *@This()) void {
    _ = self.targetView.IUnknown.Release();
    _ = self.swapChain.IUnknown.Release();
    _ = self.device.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![填充背景][1]

[1]: images/directx037.png

## 附录
