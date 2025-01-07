# 0706-DirectX11-深度模板视图

## 目标

为了避免先后绘制的影响，引入深度模板。

## 环境

- Time 2025-01-07
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

深度模板相当于有个 Z 轴，来描述物体的远近，根据远近来确定遮挡而不是绘制的先后顺序。

## Direct3D.zig

创建了深度模板视图，并进行了绑定。

```zig
const std = @import("std");
const win32 = @import("win32");

const dxgi = win32.graphics.dxgi;
const d11 = win32.graphics.direct3d11;

device: *d11.ID3D11Device = undefined,
deviceContext: *d11.ID3D11DeviceContext = undefined,
swapChain: *dxgi.IDXGISwapChain = undefined,
targetView: *d11.ID3D11RenderTargetView = undefined,
depthView: *d11.ID3D11DepthStencilView = undefined,

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

    var viewPort = std.mem.zeroes(d11.D3D11_VIEWPORT);
    viewPort.Width = @floatFromInt(w);
    viewPort.Height = @floatFromInt(h);
    viewPort.MaxDepth = 1;
    self.deviceContext.RSSetViewports(1, @ptrCast(&viewPort));

    // 创建深度模板缓存
    var depthStencilDesc = std.mem.zeroes(d11.D3D11_TEXTURE2D_DESC);
    depthStencilDesc.Width = w; // 视口的宽度
    depthStencilDesc.Height = h; // 视口的高度
    depthStencilDesc.MipLevels = 1;
    depthStencilDesc.ArraySize = 1;
    depthStencilDesc.Format = .D24_UNORM_S8_UINT;
    depthStencilDesc.SampleDesc.Count = 1;
    depthStencilDesc.BindFlags = d11.D3D11_BIND_DEPTH_STENCIL;

    var buffer: *d11.ID3D11Texture2D = undefined;
    win32Check(self.device.CreateTexture2D(&depthStencilDesc, null, &buffer));
    defer _ = buffer.IUnknown.Release();

    // 创建深度模板视图
    win32Check(self.device.CreateDepthStencilView(&buffer.ID3D11Resource, null, &self.depthView));
    // 绑定深度模板视图
    self.deviceContext.OMSetRenderTargets(1, @ptrCast(&self.targetView), self.depthView);
}

pub fn beginScene(self: *@This(), red: f32, green: f32, blue: f32, alpha: f32) void {
    const color = [_]f32{ red, green, blue, alpha };
    self.deviceContext.ClearRenderTargetView(self.targetView, @ptrCast(&color));
    const flag = @intFromEnum(d11.D3D11_CLEAR_DEPTH);
    self.deviceContext.ClearDepthStencilView(self.depthView, flag, 1.0, 0);
}

pub fn render(self: *@This()) void {
    self.deviceContext.Draw(6, 0);
}

pub fn endScene(self: *@This()) void {
    win32Check(self.swapChain.Present(1, 0));
}

pub fn shutdown(self: *@This()) void {
    _ = self.depthView.IUnknown.Release();
    _ = self.targetView.IUnknown.Release();
    _ = self.swapChain.IUnknown.Release();
    _ = self.device.IUnknown.Release();

    var debug: *d11.ID3D11Debug = undefined;
    win32Check(self.device.IUnknown.QueryInterface(d11.IID_ID3D11Debug, @ptrCast(&debug)));
    defer _ = debug.IUnknown.Release();

    win32Check(debug.ReportLiveDeviceObjects(d11.D3D11_RLDO_DETAIL));
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## Model.zig

先绘制小的三角形，再绘制大的三角形，但是先绘制的遮挡了后绘制的。

```zig
const std = @import("std");
const win32 = @import("win32");

const d11 = win32.graphics.direct3d11;

vertexBuffer: *d11.ID3D11Buffer = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    const vertices = [_]f32{
        -0.4, -0.4, 0, 1, 0,
        0,    0.4,  0, 1, 0,
        0.4,  -0.4, 0, 1, 0,
        -0.8, -0.8, 1, 0, 0,
        0,    0.8,  1, 0, 0,
        0.8,  -0.8, 1, 0, 0,
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

![深度模板视图][1]

[1]: images/directx046.png

## 附录
