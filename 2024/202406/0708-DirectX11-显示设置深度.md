# 0708-DirectX11-显示设置深度

## 目标

绘制的时候，显示设置每个图形的深度。

## 环境

- Time 2025-01-07
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

在代码中显示设置绘制的深度，就不用考虑哪个图形先绘制，哪个后绘制。

## Direct3D.zig

去掉了深度目标状态，使用默认的。

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

修改了顶点数据，增加 Z 轴。绘制 5 个点修改成了 6 个。

```zig
const std = @import("std");
const win32 = @import("win32");

const d11 = win32.graphics.direct3d11;

vertexBuffer: *d11.ID3D11Buffer = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    const vertices = [_]f32{
        -0.4, -0.4, 0.4, 0, 1, 0,
        0,    0.4,  0.4, 0, 1, 0,
        0.4,  -0.4, 0.4, 0, 1, 0,
        // -0.4, -0.4, 0.9, 0, 1, 0,
        // 0,    0.4,  0.9, 0, 1, 0,
        // 0.4,  -0.4, 0.9, 0, 1, 0,
        -0.8, -0.8, 0.8, 1, 0, 0,
        0,    0.8,  0.8, 1, 0, 0,
        0.8,  -0.8, 0.8, 1, 0, 0,
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
    const strides = [_]u32{@sizeOf(f32) * 6};
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

## Shader.zig

修改了顶点布局。

```zig
const std = @import("std");
const win32 = @import("win32");

const d11 = win32.graphics.direct3d11;

vertexShader: *d11.ID3D11VertexShader = undefined,
vertexLayout: *d11.ID3D11InputLayout = undefined,
pixelShader: *d11.ID3D11PixelShader = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    var self: @This() = .{};

    const vertex = compileShader(win32.zig.L("vs.hlsl"), "vs_5_0");
    defer _ = vertex.IUnknown.Release();

    var byteCode: [*]u8 = @ptrCast(vertex.GetBufferPointer());
    var size = vertex.GetBufferSize();
    win32Check(device.CreateVertexShader(byteCode, size, null, &self.vertexShader));

    var position = std.mem.zeroes(d11.D3D11_INPUT_ELEMENT_DESC);
    position.SemanticName = "POSITION";
    position.SemanticIndex = 0;
    position.Format = .R32G32B32_FLOAT;
    position.InputSlotClass = .VERTEX_DATA;

    var color = std.mem.zeroes(d11.D3D11_INPUT_ELEMENT_DESC);
    color.SemanticName = "COLOR";
    color.SemanticIndex = 0;
    color.Format = .R32G32B32_FLOAT;
    color.AlignedByteOffset = d11.D3D11_APPEND_ALIGNED_ELEMENT;
    color.InputSlotClass = .VERTEX_DATA;

    const array = [_]d11.D3D11_INPUT_ELEMENT_DESC{ position, color };
    win32Check(device.CreateInputLayout(&array, array.len, byteCode, size, &self.vertexLayout));

    const pixel = compileShader(win32.zig.L("ps.hlsl"), "ps_5_0");
    defer _ = pixel.IUnknown.Release();

    byteCode = @ptrCast(pixel.GetBufferPointer());
    size = pixel.GetBufferSize();
    win32Check(device.CreatePixelShader(byteCode, size, null, &self.pixelShader));

    return self;
}

pub fn render(self: *@This(), deviceContext: *d11.ID3D11DeviceContext) void {
    deviceContext.IASetInputLayout(self.vertexLayout);
    deviceContext.IASetPrimitiveTopology(._PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    deviceContext.VSSetShader(self.vertexShader, null, 0);
    deviceContext.PSSetShader(self.pixelShader, null, 0);
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexShader.IUnknown.Release();
    _ = self.vertexLayout.IUnknown.Release();
    _ = self.pixelShader.IUnknown.Release();
}

const ID3DBlob = win32.graphics.direct3d.ID3DBlob;
const fxc = win32.graphics.direct3d.fxc;
pub fn compileShader(srcName: [*:0]const u16, target: [*:0]const u8) *ID3DBlob {
    var r: ?*ID3DBlob = null;
    var blob: ?*ID3DBlob = null;

    const flags = fxc.D3DCOMPILE_ENABLE_STRICTNESS //
    | fxc.D3DCOMPILE_DEBUG | fxc.D3DCOMPILE_SKIP_OPTIMIZATION;
    _ = fxc.D3DCompileFromFile(srcName, null, null, "main", target, flags, 0, &r, &blob);
    shaderCheck(blob);
    return r.?;
}

fn shaderCheck(errorBlob: ?*ID3DBlob) void {
    if (errorBlob) |blob| {
        const msg: [*]u8 = @ptrCast(blob.GetBufferPointer());
        @panic(msg[0..blob.GetBufferSize()]);
    }
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## vs.hlsl

修改了输入的顶点格式。

```hlsl
struct VS_INPUT
{
    float3 inPos : POSITION;
    float3 inColor : COLOR;
};

struct VS_OUTPUT
{
    float4 outPosition : SV_POSITION;
    float3 outColor : COLOR;
};

VS_OUTPUT main(VS_INPUT input)
{
    VS_OUTPUT output;
    output.outPosition = float4(input.inPos, 1.0f);
    output.outColor = input.inColor;
    return output;
}
```

## 效果

![设置深度][1]

[1]: images/directx048.png

## 附录
