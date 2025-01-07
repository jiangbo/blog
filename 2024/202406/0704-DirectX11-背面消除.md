# 0704-DirectX11-背面消除

## 目标

验证背面消除的效果。

## 环境

- Time 2025-01-07
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

交换顶点的顺序，将顺时针修改为逆时针，验证背面消除的效果。

## Direct3D.zig

将背面消除修改为 NONE。

```zig
const std = @import("std");
const win32 = @import("win32");

const dxgi = win32.graphics.dxgi;
const d11 = win32.graphics.direct3d11;

device: *d11.ID3D11Device = undefined,
deviceContext: *d11.ID3D11DeviceContext = undefined,
swapChain: *dxgi.IDXGISwapChain = undefined,
targetView: *d11.ID3D11RenderTargetView = undefined,
rasterState: *d11.ID3D11RasterizerState = undefined,

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

    const flags = d11.D3D11_CREATE_DEVICE_DEBUG;
    win32Check(d11.D3D11CreateDeviceAndSwapChain(null, .HARDWARE, null, flags, null, 0, //
        d11.D3D11_SDK_VERSION, &desc, @ptrCast(&self.swapChain), //
        @ptrCast(&self.device), null, @ptrCast(&self.deviceContext)));

    var back: *d11.ID3D11Texture2D = undefined;
    win32Check(self.swapChain.GetBuffer(0, d11.IID_ID3D11Texture2D, @ptrCast(&back)));
    defer _ = back.IUnknown.Release();

    const target: **d11.ID3D11RenderTargetView = @ptrCast(&self.targetView);
    win32Check(self.device.CreateRenderTargetView(@ptrCast(back), null, target));

    self.deviceContext.OMSetRenderTargets(1, @ptrCast(&self.targetView), null);

    var viewPort = std.mem.zeroes(d11.D3D11_VIEWPORT);
    viewPort.Width = @floatFromInt(w);
    viewPort.Height = @floatFromInt(h);
    self.deviceContext.RSSetViewports(1, @ptrCast(&viewPort));

    // 创建光栅化状态
    var rasterDesc = std.mem.zeroes(d11.D3D11_RASTERIZER_DESC);
    rasterDesc.CullMode = .NONE;
    rasterDesc.FillMode = .SOLID;

    win32Check(self.device.CreateRasterizerState(&rasterDesc, &self.rasterState));
}

pub fn beginScene(self: *@This(), red: f32, green: f32, blue: f32, alpha: f32) void {
    const color = [_]f32{ red, green, blue, alpha };
    self.deviceContext.ClearRenderTargetView(self.targetView, @ptrCast(&color));
}

pub fn render(self: *@This()) void {
    self.deviceContext.RSSetState(self.rasterState);
    self.deviceContext.Draw(3, 0);
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

## Model.zig

交换了后面两个顶点的顺序，从顺时针变成了逆时针。

```zig
const std = @import("std");
const win32 = @import("win32");

const d11 = win32.graphics.direct3d11;

vertexBuffer: *d11.ID3D11Buffer = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    const vertices = [_]f32{
        -0.5, -0.5, 1, 0, 0,
        0.5,  -0.5, 0, 0, 1,
        0,    0.5,  0, 1, 0,
    };

    var bufferDesc = std.mem.zeroes(d11.D3D11_BUFFER_DESC);
    bufferDesc.ByteWidth = @sizeOf(@TypeOf(vertices));
    bufferDesc.BindFlags = d11.D3D11_BIND_VERTEX_BUFFER;

    var initData = std.mem.zeroes(d11.D3D11_SUBRESOURCE_DATA);
    initData.pSysMem = &vertices;

    var vertexBuffer: *d11.ID3D11Buffer = undefined;
    win32Check(device.CreateBuffer(&bufferDesc, &initData, @ptrCast(&vertexBuffer)));

    return .{ .vertexBuffer = vertexBuffer };
}

pub fn render(self: *@This(), deviceContext: *d11.ID3D11DeviceContext) void {
    const strides = [_]u32{@sizeOf(f32) * 5};
    var buffers = [_]?*d11.ID3D11Buffer{self.vertexBuffer};
    deviceContext.IASetVertexBuffers(0, 1, &buffers, &strides, &.{0});
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexBuffer.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

如果不将背面消除去掉，则看不到三角形。去掉背面消除就可以看到三角形。

![背面消除][1]

[1]: images/directx044.png

## 附录
