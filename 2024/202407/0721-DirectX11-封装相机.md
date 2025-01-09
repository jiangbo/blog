# 0721-DirectX11-封装相机

## 目标

将变换相关的代码提取到相机中。

## 环境

- Time 2025-01-09
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

先简单把模型类中的变换的提取到相机中，后面缺少什么方法的话，再增加。

## Model.zig

删除了变换的东西，和矩阵的缓冲区。

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
        .{ .x = -0.7, .y = -0.7 },
        .{ .x = -0.7, .y = 0.7, .v = 1 },
        .{ .x = 0.7, .y = 0.7, .u = 1, .v = 1 },
        .{ .x = 0.7, .y = -0.7, .u = 1 },
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

## Camera.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const zm = @import("zm");

const d11 = win32.graphics.direct3d11;

model: zm.Mat,
view: zm.Mat,
projection: zm.Mat,
matrixBuffer: *d11.ID3D11Buffer = undefined,

pub fn init(device: *d11.ID3D11Device, width: u16, height: u16) @This() {
    var self: @This() = undefined;
    // 模型矩阵
    self.model = zm.identity();

    // 视图矩阵
    const eve = zm.f32x4(0, 0, -2, 0);
    const look = zm.f32x4(0, 0, 0, 0);
    const up = zm.f32x4(0, 1, 0, 0);
    self.view = zm.lookAtLh(eve, look, up);

    // 投影矩阵
    const fov = std.math.pi / 2.0;
    const aspect = @as(f32, @floatFromInt(width)) / @as(f32, @floatFromInt(height));
    const near = 0.1;
    const far = 1000.0;
    self.projection = zm.perspectiveFovLh(fov, aspect, near, far);

    var bufferDesc = std.mem.zeroes(d11.D3D11_BUFFER_DESC);
    bufferDesc.ByteWidth = @sizeOf(zm.Mat);
    bufferDesc.BindFlags = d11.D3D11_BIND_CONSTANT_BUFFER;

    win32Check(device.CreateBuffer(&bufferDesc, null, &self.matrixBuffer));

    return self;
}

pub fn render(self: *@This(), deviceContext: *d11.ID3D11DeviceContext) void {
    const mvp = zm.mul(zm.mul(self.model, self.view), self.projection);

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

## graphics.zig

增加了相机相关的代码。

```zig
const std = @import("std");
const win32 = @import("win32");
const Direct3D = @import("Direct3D.zig");
const Model = @import("Model.zig");
const Shader = @import("Shader.zig");
const Camera = @import("Camera.zig");

pub const WIDTH: u16 = 800;
pub const HEIGHT: u16 = 600;

direct3D: Direct3D,
model: Model,
shader: Shader,
camera: Camera,

pub fn initialize(window: ?win32.foundation.HWND) @This() {
    var direct = Direct3D{};

    direct.initialize(WIDTH, HEIGHT, window);
    return .{
        .direct3D = direct,
        .model = Model.initialize(direct.device),
        .shader = Shader.initialize(direct.device),
        .camera = Camera.init(direct.device, WIDTH, HEIGHT),
    };
}

pub fn frame(self: *@This()) bool {
    return self.render();
}

pub fn render(self: *@This()) bool {
    self.direct3D.beginScene(0, 0, 0, 1);

    self.shader.render(self.direct3D.deviceContext);
    self.model.render(self.direct3D.deviceContext);
    self.camera.render(self.direct3D.deviceContext);
    self.direct3D.render();

    self.direct3D.endScene();
    return true;
}

pub fn shutdown(self: *@This()) void {
    self.shader.shutdown();
    self.model.shutdown();
    self.camera.deinit();
    self.direct3D.shutdown();
}
```

## 效果

![提取相机][1]

[1]: images/directx061.png

## 附录
