# 0702-DirectX11-彩色三角形

## 目标

画出一个彩色的三角形。

## 环境

- Time 2025-01-06
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://rastertek.com/tutdx11win10.html>
2. <https://enjoyphysics.cn/Soft/Program>
3. <https://www.youtube.com/playlist?list=PLcacUGyBsOIBlGyQQWzp6D1Xn6ZENx9Y2>

## 想法

我感觉第一个参考资料不适合来学习，每一章节太多，太杂了。第三个更适合用来入门，每一节很短，只讲一件事。

## Graphics.zig

将顶点数修改成三个，背景颜色修改为黑色。

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
    self.direct3D.beginScene(0, 0, 0, 1);

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

## Shader.zig

将基元修改成三角形，定义了输入布局，增加颜色相关的描述。

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
    position.Format = .R32G32_FLOAT;
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

## Model.zig

修改了顶点数据，增加了颜色。

```zig
const std = @import("std");
const win32 = @import("win32");

const d11 = win32.graphics.direct3d11;

vertexBuffer: *d11.ID3D11Buffer = undefined,

pub fn initialize(device: *d11.ID3D11Device) @This() {
    const vertices = [_]f32{
        -0.5, -0.5, 1, 0, 0,
        0,    0.5,  0, 1, 0,
        0.5,  -0.5, 0, 0, 1,
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

## vs.hlsl

```hlsl
struct VS_INPUT
{
    float2 inPos : POSITION;
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
    output.outPosition = float4(input.inPos, 0.0f, 1.0f);
    output.outColor = input.inColor;
    return output;
}
```

## ps.hlsl

```hlsl
struct PS_INPUT
{
    float4 inPosition : SV_POSITION;
    float3 inColor : COLOR;
};

float4 main(PS_INPUT input) : SV_TARGET
{
    return float4(input.inColor, 1.0f);
}
```

## 效果

![彩色三角形][1]

[1]: images/directx042.png

## 附录
