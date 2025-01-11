# 0723-DirectX11-新增纹理封装

## 目标

Model 文件中的内容太多了，将纹理部分的内容单独提取出来。

## 环境

- Time 2025-01-11
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

把纹理封装到 Texture 文件中，增加了一个模型矩阵，目前还没有用，感觉有用。

## Graphics.zig

新增 Texture 文件的封装，加入了一个 texture 属性。

```zig
const std = @import("std");
const win32 = @import("win32");
const Direct3D = @import("Direct3D.zig");
const Model = @import("Model.zig");
const Shader = @import("Shader.zig");
const Camera = @import("Camera.zig");
const Texture = @import("Texture.zig");

pub const WIDTH: u16 = 800;
pub const HEIGHT: u16 = 600;

direct3D: Direct3D,
model: Model,
shader: Shader,
camera: Camera,
texture: Texture,

pub fn initialize(window: ?win32.foundation.HWND) @This() {
    var direct = Direct3D{};

    direct.initialize(WIDTH, HEIGHT, window);
    return .{
        .direct3D = direct,
        .model = Model.initialize(direct.device),
        .shader = Shader.initialize(direct.device),
        .camera = Camera.init(direct.device, WIDTH, HEIGHT),
        .texture = Texture.init(direct.device, "assets/player32.bmp"),
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
    self.texture.draw(self.direct3D.deviceContext);
    self.direct3D.render();

    self.direct3D.endScene();
    return true;
}

pub fn shutdown(self: *@This()) void {
    self.shader.shutdown();
    self.model.shutdown();
    self.texture.deinit();
    self.camera.deinit();
    self.direct3D.shutdown();
}
```

## Model.zig

删除了纹理相关的内容。

```zig
const std = @import("std");
const win32 = @import("win32");

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

    return self;
}

pub fn render(self: *@This(), deviceContext: *d11.ID3D11DeviceContext) void {
    const strides = [_]u32{self.vertexBuffer.stride};
    var buffers = [_]?*d11.ID3D11Buffer{self.vertexBuffer.data};
    deviceContext.IASetVertexBuffers(0, 1, &buffers, &strides, &.{0});

    deviceContext.IASetIndexBuffer(self.indexBuffer.data, self.indexBuffer.format, 0);

    deviceContext.DrawIndexed(self.indexBuffer.count, 0, 0);
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexBuffer.deinit();
    _ = self.indexBuffer.deinit();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## Texture.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Bitmap = @import("Bitmap.zig");
const zm = @import("zm");

const d11 = win32.graphics.direct3d11;

model: zm.Mat,
textureView: *d11.ID3D11ShaderResourceView,

pub fn init(device: *d11.ID3D11Device, name: [:0]const u8) @This() {
    var bitmap = Bitmap.init(name) catch unreachable;
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

    var textureView: *d11.ID3D11ShaderResourceView = undefined;
    win32Check(device.CreateShaderResourceView(@ptrCast(texture), &srvDesc, &textureView));

    return .{ .model = zm.identity(), .textureView = textureView };
}

pub fn draw(self: *@This(), context: *d11.ID3D11DeviceContext) void {
    context.PSSetShaderResources(0, 1, @ptrCast(&self.textureView));
}

pub fn deinit(self: *@This()) void {
    _ = self.textureView.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![封装纹理][1]

[1]: images/directx063.png

## 附录
