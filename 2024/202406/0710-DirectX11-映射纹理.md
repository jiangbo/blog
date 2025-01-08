# 0710-DirectX11-映射纹理

## 目标

将一张图片绘制到窗口上。

## 环境

- Time 2025-01-08
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

没有工具类来创建纹理，有点麻烦，之前写过加载位图的，拿过来直接用一下。
写的过程中，一直渲染不出来，排查得知深度缓存配置有问题，直接把深度缓存的删除了，用的时候再加。

## Direct3D.zig

去掉了深度缓存的东西，绘制顶点数量的方法移动到 Model.zig 文件了。

```zig
const std = @import("std");
const win32 = @import("win32");

const dxgi = win32.graphics.dxgi;
const d11 = win32.graphics.direct3d11;

device: *d11.ID3D11Device = undefined,
deviceContext: *d11.ID3D11DeviceContext = undefined,
swapChain: *dxgi.IDXGISwapChain = undefined,
targetView: *d11.ID3D11RenderTargetView = undefined,

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
    self.deviceContext.RSSetViewports(1, @ptrCast(&viewPort));

    self.deviceContext.OMSetRenderTargets(1, @ptrCast(&self.targetView), null);
}

pub fn beginScene(self: *@This(), red: f32, green: f32, blue: f32, alpha: f32) void {
    const color = [_]f32{ red, green, blue, alpha };
    self.deviceContext.ClearRenderTargetView(self.targetView, @ptrCast(&color));
}

pub fn render(self: *@This()) void {
    self.deviceContext.Draw(6, 0);
}

pub fn endScene(self: *@This()) void {
    win32Check(self.swapChain.Present(1, 0));
}

pub fn shutdown(self: *@This()) void {
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

## Shader.zig

创建了采样器，修改了顶点的布局，增加了纹理坐标。

```zig
const std = @import("std");
const win32 = @import("win32");

const d11 = win32.graphics.direct3d11;

vertexShader: *d11.ID3D11VertexShader = undefined,
vertexLayout: *d11.ID3D11InputLayout = undefined,
pixelShader: *d11.ID3D11PixelShader = undefined,
samplerState: *d11.ID3D11SamplerState = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    var self: @This() = .{};

    self.initVertexShaderAndLayout(device);
    self.initPixelShader(device);
    self.initSamplerState(device);

    return self;
}

fn initVertexShaderAndLayout(self: *@This(), device: *d11.ID3D11Device) void {
    const vertex = compileShader(win32.zig.L("vs.hlsl"), "vs_5_0");
    defer _ = vertex.IUnknown.Release();

    const byteCode: [*]u8 = @ptrCast(vertex.GetBufferPointer());
    const size = vertex.GetBufferSize();
    win32Check(device.CreateVertexShader(byteCode, size, null, &self.vertexShader));

    var position = std.mem.zeroes(d11.D3D11_INPUT_ELEMENT_DESC);
    position.SemanticName = "POSITION";
    position.SemanticIndex = 0;
    position.Format = .R32G32B32_FLOAT;
    position.InputSlotClass = .VERTEX_DATA;

    var color = std.mem.zeroes(d11.D3D11_INPUT_ELEMENT_DESC);
    color.SemanticName = "TEXCOORD";
    color.SemanticIndex = 0;
    color.Format = .R32G32_FLOAT;
    color.AlignedByteOffset = d11.D3D11_APPEND_ALIGNED_ELEMENT;
    color.InputSlotClass = .VERTEX_DATA;

    const array = [_]d11.D3D11_INPUT_ELEMENT_DESC{ position, color };
    win32Check(device.CreateInputLayout(&array, array.len, byteCode, size, &self.vertexLayout));
}

fn initPixelShader(self: *@This(), device: *d11.ID3D11Device) void {
    const pixel = compileShader(win32.zig.L("ps.hlsl"), "ps_5_0");

    defer _ = pixel.IUnknown.Release();

    const byteCode: [*]u8 = @ptrCast(pixel.GetBufferPointer());
    const size = pixel.GetBufferSize();
    win32Check(device.CreatePixelShader(byteCode, size, null, &self.pixelShader));
}

fn initSamplerState(self: *@This(), device: *d11.ID3D11Device) void {
    var samplerDesc = std.mem.zeroes(d11.D3D11_SAMPLER_DESC);
    samplerDesc.Filter = .MIN_MAG_MIP_LINEAR;
    samplerDesc.AddressU = .WRAP;
    samplerDesc.AddressV = .WRAP;
    samplerDesc.AddressW = .WRAP;
    samplerDesc.ComparisonFunc = .NEVER;
    samplerDesc.MinLOD = 0;
    samplerDesc.MaxLOD = d11.D3D11_FLOAT32_MAX;

    win32Check(device.CreateSamplerState(&samplerDesc, &self.samplerState));
}

pub fn render(self: *@This(), deviceContext: *d11.ID3D11DeviceContext) void {
    deviceContext.IASetInputLayout(self.vertexLayout);
    deviceContext.IASetPrimitiveTopology(._PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    deviceContext.VSSetShader(self.vertexShader, null, 0);
    deviceContext.PSSetShader(self.pixelShader, null, 0);
    deviceContext.PSSetSamplers(0, 1, @ptrCast(&self.samplerState));
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexShader.IUnknown.Release();
    _ = self.vertexLayout.IUnknown.Release();
    _ = self.pixelShader.IUnknown.Release();
    _ = self.samplerState.IUnknown.Release();
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

## Model.zig

修改了顶点数据，加载了一张纹理。

```zig
const std = @import("std");
const win32 = @import("win32");
const Bitmap = @import("Bitmap.zig");

const d11 = win32.graphics.direct3d11;

vertexBuffer: *d11.ID3D11Buffer = undefined,
textureView: *d11.ID3D11ShaderResourceView = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    var self: @This() = undefined;

    self.initVertexBuffer(device);
    self.initTexture(device);

    return self;
}

fn initVertexBuffer(self: *@This(), device: *d11.ID3D11Device) void {
    const vertices = [_]f32{
        -0.7, -0.7, 0, 0, 1,
        -0.7, 0.7,  0, 0, 0,
        0.7,  0.7,  0, 1, 0,
        0.7,  0.7,  0, 1, 0,
        0.7,  -0.7, 0, 1, 1,
        -0.7, -0.7, 0, 0, 1,
    };

    var bufferDesc = std.mem.zeroes(d11.D3D11_BUFFER_DESC);
    bufferDesc.ByteWidth = @sizeOf(@TypeOf(vertices));
    bufferDesc.BindFlags = d11.D3D11_BIND_VERTEX_BUFFER;

    var initData = std.mem.zeroes(d11.D3D11_SUBRESOURCE_DATA);
    initData.pSysMem = &vertices;

    win32Check(device.CreateBuffer(&bufferDesc, &initData, &self.vertexBuffer));
}

fn initTexture(self: *@This(), device: *d11.ID3D11Device) void {
    var bitmap = Bitmap.init("assets/player24.bmp") catch unreachable;
    defer bitmap.deinit();

    var textureDesc = std.mem.zeroes(d11.D3D11_TEXTURE2D_DESC);
    textureDesc.Width = @intCast(bitmap.infoHeader.biWidth);
    textureDesc.Height = @intCast(bitmap.infoHeader.biHeight);
    textureDesc.MipLevels = 1;
    textureDesc.ArraySize = 1;
    textureDesc.Format = .R8G8B8A8_UNORM;
    textureDesc.SampleDesc.Count = 1;
    textureDesc.Usage = .DEFAULT;
    textureDesc.BindFlags = d11.D3D11_BIND_SHADER_RESOURCE;

    var initialData = std.mem.zeroes(d11.D3D11_SUBRESOURCE_DATA);
    initialData.pSysMem = @ptrCast(bitmap.colors.ptr);
    initialData.SysMemPitch = textureDesc.Width * 4;

    var texture: *d11.ID3D11Texture2D = undefined;
    win32Check(device.CreateTexture2D(&textureDesc, &initialData, &texture));

    var srvDesc = std.mem.zeroes(d11.D3D11_SHADER_RESOURCE_VIEW_DESC);
    srvDesc.Format = textureDesc.Format;
    srvDesc.ViewDimension = ._SRV_DIMENSION_TEXTURE2D;
    srvDesc.Anonymous.Texture2D.MipLevels = 1;

    win32Check(device.CreateShaderResourceView(@ptrCast(texture), &srvDesc, &self.textureView));
}

pub fn render(self: *@This(), deviceContext: *d11.ID3D11DeviceContext) void {
    const strides = [_]u32{@sizeOf(f32) * 5};
    var buffers = [_]?*d11.ID3D11Buffer{self.vertexBuffer};
    deviceContext.IASetVertexBuffers(0, 1, &buffers, &strides, &.{0});
    deviceContext.PSSetShaderResources(0, 1, @ptrCast(&self.textureView));

    deviceContext.Draw(6, 0);
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexBuffer.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## Bitmap.zig

把之前写的加载位图的代码复制过来。

```zig
const std = @import("std");
const win32 = @import("win32");

fileHeader: win32.graphics.gdi.BITMAPFILEHEADER,
infoHeader: win32.graphics.gdi.BITMAPINFOHEADER,
colors: []u32,

const bitmapId: u16 = 0x4D42;

pub fn init(fileName: [:0]const u8) !@This() {
    var bitmap: @This() = undefined;

    const file = win32.storage.file_system;
    const windows = win32.system.windows_programming;

    // open the file if it exists
    var fileData: file.OFSTRUCT = undefined;
    const fileHandle = file.OpenFile(fileName, &fileData, file.OF_READ);
    if (fileHandle == -1) win32Panic();
    defer _ = windows._lclose(fileHandle);

    // now load the bitmap file header
    var len: u32 = @sizeOf(win32.graphics.gdi.BITMAPFILEHEADER);
    _ = windows._lread(fileHandle, &bitmap.fileHeader, len);

    // test if this is a bitmap file
    if (bitmap.fileHeader.bfType != bitmapId) @panic("not bitmap");

    // now we know this is a bitmap, so read in all the sections

    // first the bitmap infoheader

    // now load the bitmap file header
    len = @sizeOf(win32.graphics.gdi.BITMAPINFOHEADER);
    _ = windows._lread(fileHandle, &bitmap.infoHeader, len);

    // now load the color palette if there is one
    std.log.debug("bit count: {d}", .{bitmap.infoHeader.biBitCount});

    // // finally the image data itself
    const end = win32.media.multimedia.SEEK_END;
    const offset: i32 = @intCast(bitmap.infoHeader.biSizeImage);
    _ = windows._llseek(fileHandle, -offset, end);

    // allocate the memory for the image
    len = bitmap.infoHeader.biSizeImage;
    const buffer = try std.heap.page_allocator.alloc(u8, len);
    defer std.heap.page_allocator.free(buffer);

    _ = windows._lread(fileHandle, buffer.ptr, len);

    bitmap.colors = try std.heap.page_allocator.alloc(u32, len / 3);
    for (bitmap.colors, 0..) |*color, i| {
        color.* = @as(u24, @intCast(buffer[3 * i + 2])) << 16 //
        | @as(u24, @intCast(buffer[3 * i + 1])) << 8 | buffer[3 * i];
    }

    // flip the bitmap
    flipBitmap(bitmap.colors, @intCast(bitmap.infoHeader.biHeight));
    return bitmap;
}

fn flipBitmap(image: []u32, height: usize) void {
    // this function is used to flip bottom-up .BMP images

    // allocate the temporary buffer
    const buffer = std.heap.page_allocator.dupe(u32, image) catch unreachable;
    defer std.heap.page_allocator.free(buffer);

    // flip vertically
    const width = image.len / height;
    for (0..height) |index| {
        const source = buffer[index * width ..][0..width];
        const dest = image[(height - index - 1) * width ..][0..width];
        @memcpy(dest, source);
    }
}

pub fn deinit(self: *@This()) void {
    std.heap.page_allocator.free(self.colors);
}

fn win32Panic() void {
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## vs.hlsl

```hlsl
struct VS_INPUT
{
    float3 inPos : POSITION;
    float2 inTexCoord : TEXCOORD;
};

struct VS_OUTPUT
{
    float4 outPosition : SV_POSITION;
    float2 outTexCoord : TEXCOORD;
};

VS_OUTPUT main(VS_INPUT input)
{
    VS_OUTPUT output;
    output.outPosition = float4(input.inPos, 1.0f);
    output.outTexCoord = input.inTexCoord;
    return output;
}
```

## ps.hlsl

```hlsl
struct PS_INPUT
{
    float4 inPosition : SV_POSITION;
    float2 inTexCoord : TEXCOORD;
};

Texture2D objTexture : TEXTURE : register(t0);
SamplerState objSamplerState : SAMPLER : register(s0);

float4 main(PS_INPUT input) : SV_TARGET
{
    float4 s = objTexture.Sample(objSamplerState, input.inTexCoord);
    return float4(s.b, s.g, s.r, 1.0f);
}
```

## 效果

![映射纹理][1]

[1]: images/directx050.png

## 附录
