# 0713-DirectX11-封装顶点缓冲区

## 目标

将顶点缓冲区封装成结构体，并且支持泛型。

## 环境

- Time 2025-01-08
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

刚刚学习 Zig 中的泛型，边学边了解。

## Model.zig

封装顶点缓冲区。

```zig
const std = @import("std");
const win32 = @import("win32");
const Bitmap = @import("Bitmap.zig");

const d11 = win32.graphics.direct3d11;

fn VertexBuffer(T: type) type {
    return struct {
        data: *d11.ID3D11Buffer,
        stride: u32,

        pub fn init(device: *d11.ID3D11Device, data: []T) @This() {
            var self: @This() = undefined;
            self.stride = @sizeOf(T);

            var bufferDesc = std.mem.zeroes(d11.D3D11_BUFFER_DESC);
            bufferDesc.ByteWidth = self.stride * @as(u32, @intCast(data.len));
            bufferDesc.BindFlags = d11.D3D11_BIND_VERTEX_BUFFER;

            var initData = std.mem.zeroes(d11.D3D11_SUBRESOURCE_DATA);
            initData.pSysMem = data.ptr;

            win32Check(device.CreateBuffer(&bufferDesc, &initData, &self.data));
            return self;
        }

        pub fn deinit(self: *@This()) void {
            _ = self.data.IUnknown.Release();
        }
    };
}

const Vertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    u: f32 = 0,
    v: f32 = 0,
};

vertexBuffer: VertexBuffer(Vertex) = undefined,
indexBuffer: *d11.ID3D11Buffer = undefined,
textureView: *d11.ID3D11ShaderResourceView = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    var self: @This() = undefined;

    self.initVertexBuffer(device);
    self.initIndexBuffer(device);
    self.initTexture(device);
    return self;
}

fn initVertexBuffer(self: *@This(), device: *d11.ID3D11Device) void {
    var vertices = [_]Vertex{
        .{ .x = -0.7, .y = -0.7 },
        .{ .x = -0.7, .y = 0.7, .v = 1 },
        .{ .x = 0.7, .y = 0.7, .u = 1, .v = 1 },
        .{ .x = 0.7, .y = -0.7, .u = 1 },
    };

    self.vertexBuffer = VertexBuffer(Vertex).init(device, &vertices);
}

fn initIndexBuffer(self: *@This(), device: *d11.ID3D11Device) void {
    const indices = [_]u16{ 0, 1, 2, 2, 3, 0 };

    var bufferDesc = std.mem.zeroes(d11.D3D11_BUFFER_DESC);
    bufferDesc.ByteWidth = @sizeOf(@TypeOf(indices));
    bufferDesc.BindFlags = d11.D3D11_BIND_INDEX_BUFFER;

    var initData = std.mem.zeroes(d11.D3D11_SUBRESOURCE_DATA);
    initData.pSysMem = &indices;

    win32Check(device.CreateBuffer(&bufferDesc, &initData, &self.indexBuffer));
}

fn initTexture(self: *@This(), device: *d11.ID3D11Device) void {
    var bitmap = Bitmap.init("assets/player32.bmp") catch unreachable;
    defer bitmap.deinit();

    var textureDesc = std.mem.zeroes(d11.D3D11_TEXTURE2D_DESC);
    textureDesc.Width = @intCast(bitmap.infoHeader.biWidth);
    textureDesc.Height = @intCast(bitmap.infoHeader.biHeight);
    textureDesc.MipLevels = 1;
    textureDesc.ArraySize = 1;
    textureDesc.Format = .B8G8R8X8_UNORM;
    textureDesc.SampleDesc.Count = 1;
    textureDesc.Usage = .DEFAULT;
    textureDesc.BindFlags = d11.D3D11_BIND_SHADER_RESOURCE;

    var initialData = std.mem.zeroes(d11.D3D11_SUBRESOURCE_DATA);
    initialData.pSysMem = @ptrCast(bitmap.buffer.ptr);
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
    const strides = [_]u32{self.vertexBuffer.stride};
    var buffers = [_]?*d11.ID3D11Buffer{self.vertexBuffer.data};
    deviceContext.IASetVertexBuffers(0, 1, &buffers, &strides, &.{0});
    deviceContext.IASetIndexBuffer(self.indexBuffer, .R16_UINT, 0);
    deviceContext.PSSetShaderResources(0, 1, @ptrCast(&self.textureView));
    deviceContext.DrawIndexed(6, 0, 0);
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexBuffer.deinit();
    _ = self.indexBuffer.IUnknown.Release();
    _ = self.textureView.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![封装顶点缓冲区][1]

[1]: images/directx053.png

## 附录
