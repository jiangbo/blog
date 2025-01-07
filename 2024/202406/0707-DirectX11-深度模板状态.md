# 0707-DirectX11-深度模板状态

## 目标

修改深度模板的状态，可以修改深度的比较算法。

## 环境

- Time 2025-01-07
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

修改深度模板的比较算法，可以改变物体的遮挡关系。

## Direct3D.zig

创建了深度模板状态，并进行了绑定。

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
depthState: *d11.ID3D11DepthStencilState = undefined,

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
    var textureDesc = std.mem.zeroes(d11.D3D11_TEXTURE2D_DESC);
    textureDesc.Width = w; // 视口的宽度
    textureDesc.Height = h; // 视口的高度
    textureDesc.MipLevels = 1;
    textureDesc.ArraySize = 1;
    textureDesc.Format = .D24_UNORM_S8_UINT;
    textureDesc.SampleDesc.Count = 1;
    textureDesc.BindFlags = d11.D3D11_BIND_DEPTH_STENCIL;

    var buffer: *d11.ID3D11Texture2D = undefined;
    win32Check(self.device.CreateTexture2D(&textureDesc, null, &buffer));
    defer _ = buffer.IUnknown.Release();

    // 创建深度模板视图
    win32Check(self.device.CreateDepthStencilView(&buffer.ID3D11Resource, null, &self.depthView));

    // 绑定深度模板视图
    self.deviceContext.OMSetRenderTargets(1, @ptrCast(&self.targetView), self.depthView);

    var depthStencilDesc = std.mem.zeroes(d11.D3D11_DEPTH_STENCIL_DESC);
    depthStencilDesc.DepthEnable = win32.zig.TRUE;
    depthStencilDesc.DepthWriteMask = .ALL;
    depthStencilDesc.DepthFunc = .LESS_EQUAL;

    win32Check(self.device.CreateDepthStencilState(&depthStencilDesc, &self.depthState));
}

pub fn beginScene(self: *@This(), red: f32, green: f32, blue: f32, alpha: f32) void {
    const color = [_]f32{ red, green, blue, alpha };
    self.deviceContext.ClearRenderTargetView(self.targetView, @ptrCast(&color));
    const flag = @intFromEnum(d11.D3D11_CLEAR_DEPTH);
    self.deviceContext.ClearDepthStencilView(self.depthView, flag, 1.0, 0);
    self.deviceContext.OMSetDepthStencilState(self.depthState, 0);
}

pub fn render(self: *@This()) void {
    self.deviceContext.Draw(6, 0);
}

pub fn endScene(self: *@This()) void {
    win32Check(self.swapChain.Present(1, 0));
}

pub fn shutdown(self: *@This()) void {
    _ = self.depthState.IUnknown.Release();
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

未修改，不改变绘制的顶点和顺序，修改深度模板状态来改变遮挡。只能看到一个三角形。

## 效果

![深度模板状态][1]

[1]: images/directx047.png

## 附录
