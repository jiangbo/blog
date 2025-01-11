# 0722-DirectX11-正交投影

## 目标

之前使用的透视投影，这个在 3D 用的多，将其修改为正交投影。

## 环境

- Time 2025-01-11
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

现在还不想深入学习 3D，先把 2D 看看能不能搞明白，等想用 3D 了再学。视频跳过了 3D 的部分，先学 2D。

## Camera.zig

感觉模型变换应该在具体的物体上，不应该在相机上，所以去掉模型矩阵。

```zig
const std = @import("std");
const win32 = @import("win32");
const zm = @import("zm");

const d11 = win32.graphics.direct3d11;

view: zm.Mat,
projection: zm.Mat,
matrixBuffer: *d11.ID3D11Buffer = undefined,

pub fn init(device: *d11.ID3D11Device, width: u16, height: u16) @This() {
    var self: @This() = undefined;

    self.view = zm.identity();
    self.projection = zm.orthographicLh(@floatFromInt(width), @floatFromInt(height), 0, 1);

    var bufferDesc = std.mem.zeroes(d11.D3D11_BUFFER_DESC);
    bufferDesc.ByteWidth = @sizeOf(zm.Mat);
    bufferDesc.BindFlags = d11.D3D11_BIND_CONSTANT_BUFFER;

    win32Check(device.CreateBuffer(&bufferDesc, null, &self.matrixBuffer));

    return self;
}

pub fn render(self: *@This(), deviceContext: *d11.ID3D11DeviceContext) void {
    const mvp = zm.mul(self.view, self.projection);

    deviceContext.UpdateSubresource(@ptrCast(self.matrixBuffer), 0, null, &mvp, 0, 0);
    deviceContext.VSSetConstantBuffers(0, 1, @ptrCast(&self.matrixBuffer));
}

pub fn deinit(self: *@This()) void {
    _ = self.matrixBuffer.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## Model.zig

顶点坐标就不应该是标准化坐标了，而是屏幕坐标。还是反转了纹理的上下坐标。

```zig
const std = @import("std");
const win32 = @import("win32");
const Bitmap = @import("Bitmap.zig");
const zm = @import("zm");

const d11 = win32.graphics.direct3d11;

fn VertexBuffer(T: type) type {
    return struct {
        data: *d11.ID3D11Buffer,
        stride: u32,

        pub fn init(device: *d11.ID3D11Device, data: []const T) @This() {
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

fn IndexBuffer(T: type) type {
    return struct {
        data: *d11.ID3D11Buffer,
        count: u32,
        format: win32.graphics.dxgi.common.DXGI_FORMAT,

        pub fn init(device: *d11.ID3D11Device, data: []const T) @This() {
            var self: @This() = undefined;
            self.count = @as(u32, @intCast(data.len));

            var bufferDesc = std.mem.zeroes(d11.D3D11_BUFFER_DESC);
            bufferDesc.ByteWidth = @sizeOf(T) * self.count;
            bufferDesc.BindFlags = d11.D3D11_BIND_INDEX_BUFFER;

            var initData = std.mem.zeroes(d11.D3D11_SUBRESOURCE_DATA);
            initData.pSysMem = data.ptr;

            win32Check(device.CreateBuffer(&bufferDesc, &initData, &self.data));

            self.format = switch (T) {
                u16 => .R16_UINT,
                u32 => .R32_UINT,
                else => @compileError("unsupported index buffer type"),
            };
            return self;
        }

        pub fn deinit(self: *@This()) void {
            _ = self.data.IUnknown.Release();
        }
    };
}

vertexBuffer: VertexBuffer(Vertex) = undefined,
indexBuffer: IndexBuffer(u16) = undefined,
textureView: *d11.ID3D11ShaderResourceView = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    var self: @This() = undefined;

    const vertices = [_]Vertex{
        .{ .x = -400, .y = -300, .u = 0, .v = 0 },
        .{ .x = -400, .y = 300, .u = 0, .v = 1 },
        .{ .x = 400, .y = 300, .u = 1, .v = 1 },
        .{ .x = 400, .y = -300, .u = 1, .v = 0 },
    };
    self.vertexBuffer = VertexBuffer(Vertex).init(device, &vertices);

    const indices = [_]u16{ 0, 1, 2, 2, 3, 0 };
    self.indexBuffer = IndexBuffer(u16).init(device, &indices);

    self.initTexture(device);
    return self;
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

    deviceContext.IASetIndexBuffer(self.indexBuffer.data, self.indexBuffer.format, 0);

    deviceContext.PSSetShaderResources(0, 1, @ptrCast(&self.textureView));
    deviceContext.DrawIndexed(self.indexBuffer.count, 0, 0);
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexBuffer.deinit();
    _ = self.indexBuffer.deinit();
    _ = self.textureView.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![正交投影][1]

[1]: images/directx062.png

## 附录
