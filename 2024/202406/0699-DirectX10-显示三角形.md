# 0699-DirectX10-显示三角形

## 目标

显示一个白色的三角形。

## 环境

- Time 2025-01-05
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

显示出了第一个三角形，相等于编程的输出了 `hello world` 了吧，还真是不容易。

## Graphics.zig

增加了 Model 和 Shader 的初始化和渲染，增加了一个 draw 方法。

```zig
const std = @import("std");
const win32 = @import("win32");
const Direct3D = @import("Direct3D.zig");
const Model = @import("Model.zig");
const Shader = @import("Shader.zig");

pub const WIDTH: u16 = 800;
pub const HEIGHT: u16 = 600;
pub const VSYNC_ENABLED: bool = true;
pub const SCREEN_DEPTH: f32 = 1000.0;
pub const SCREEN_NEAR: f32 = 0.1;

direct3D: Direct3D,
model: Model,
shader: Shader,

pub fn initialize(window: ?win32.foundation.HWND) @This() {
    var direct = Direct3D{};

    direct.initialize(WIDTH, HEIGHT, window);
    return .{
        .direct3D = direct,
        .model = Model.initialize(direct.device),
        .shader = Shader.initialize(direct.device),
    };
}

pub fn frame(self: *@This()) bool {
    return self.render();
}

pub fn render(self: *@This()) bool {
    self.direct3D.beginScene(1, 0, 1, 1);

    self.model.render(self.direct3D.device);
    self.shader.render(self.direct3D.device);

    self.direct3D.device.Draw(3, 0);

    self.direct3D.endScene();
    return true;
}

pub fn shutdown(self: *@This()) void {
    self.shader.shutdown();
    self.model.shutdown();
    self.direct3D.shutdown();
}
```

## Model.zig

render 方法中，增加了顶点缓冲的绑定。

```zig
const std = @import("std");
const win32 = @import("win32");

const d10 = win32.graphics.direct3d10;

vertexBuffer: *d10.ID3D10Buffer = undefined,

pub fn initialize(device: *d10.ID3D10Device) @This() {
    const vertices = [_]f32{ -0.5, -0.5, 0.0, 0.5, 0.5, -0.5 };

    var bufferDesc = std.mem.zeroes(d10.D3D10_BUFFER_DESC);
    bufferDesc.ByteWidth = @sizeOf(@TypeOf(vertices));
    bufferDesc.BindFlags = @intFromEnum(d10.D3D10_BIND_VERTEX_BUFFER);

    var initData = std.mem.zeroes(d10.D3D10_SUBRESOURCE_DATA);
    initData.pSysMem = &vertices;

    var vertexBuffer: *d10.ID3D10Buffer = undefined;
    win32Check(device.CreateBuffer(&bufferDesc, &initData, @ptrCast(&vertexBuffer)));

    return .{ .vertexBuffer = vertexBuffer };
}

pub fn render(self: *@This(), device: *d10.ID3D10Device) void {
    const strides = [_]u32{@sizeOf(f32) * 2};
    var buffers = [_]?*d10.ID3D10Buffer{self.vertexBuffer};
    device.IASetVertexBuffers(0, 1, &buffers, &strides, &.{0});
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexBuffer.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## Direct3D.zig

初始化的时候，增加了视口。

```zig
const std = @import("std");
const win32 = @import("win32");

const dxgi = win32.graphics.dxgi;
const d10 = win32.graphics.direct3d10;

device: *d10.ID3D10Device = undefined,
swapChain: *dxgi.IDXGISwapChain = undefined,
targetView: *d10.ID3D10RenderTargetView = undefined,

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

    const viewPort = std.mem.zeroInit(d10.D3D10_VIEWPORT, .{ .Width = w, .Height = h });
    self.device.RSSetViewports(1, @ptrCast(&viewPort));
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

![显示三角形][1]

[1]: images/directx039.png

## 附录
