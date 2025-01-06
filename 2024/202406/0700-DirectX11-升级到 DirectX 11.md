# 0700-DirectX11-升级到 DirectX 11

## 目标

将使用的库从 DirectX 10 升级到 DirectX 11。

## 环境

- Time 2025-01-06
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

好像 DirectX 10 使用并不多，11 更多一些，而且变化不是很大，所以直接学习 DirectX 11。

## build.zig

之前发现系统库已经自带了 DirectX 的库，所以自己引用就去掉相关的库。

```zig
const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);

    const win32 = b.dependency("zigwin32", .{});
    exe.root_module.addImport("win32", win32.module("zigwin32"));

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
```

## Graphics.zig

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

    self.model.render(self.direct3D.deviceContext);
    self.shader.render(self.direct3D.deviceContext);

    self.direct3D.deviceContext.Draw(3, 0);

    self.direct3D.endScene();
    return true;
}

pub fn shutdown(self: *@This()) void {
    self.shader.shutdown();
    self.model.shutdown();
    self.direct3D.shutdown();
}
```

## Direct3D.zig

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

    self.deviceContext.OMSetRenderTargets(1, @ptrCast(&self.targetView), null);

    var viewPort = std.mem.zeroes(d11.D3D11_VIEWPORT);
    viewPort.Width = @floatFromInt(w);
    viewPort.Height = @floatFromInt(h);
    self.deviceContext.RSSetViewports(1, @ptrCast(&viewPort));
}

pub fn beginScene(self: *@This(), red: f32, green: f32, blue: f32, alpha: f32) void {
    const color = [_]f32{ red, green, blue, alpha };
    self.deviceContext.ClearRenderTargetView(self.targetView, @ptrCast(&color));
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

## Shader.zig

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

    var desc = std.mem.zeroes(d11.D3D11_INPUT_ELEMENT_DESC);
    desc.SemanticName = "POSITION";
    desc.SemanticIndex = 0;
    desc.Format = .R32G32_FLOAT;
    desc.InputSlotClass = .VERTEX_DATA;

    const array = [_]d11.D3D11_INPUT_ELEMENT_DESC{desc};
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

## Model.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const d11 = win32.graphics.direct3d11;

vertexBuffer: *d11.ID3D11Buffer = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    const vertices = [_]f32{ -0.5, -0.5, 0.0, 0.5, 0.5, -0.5 };

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
    const strides = [_]u32{@sizeOf(f32) * 2};
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

效果已经和之前一样，没有修改逻辑，只升级使用的库版本。

![显示三角形][1]

[1]: images/directx040.png

## 附录
