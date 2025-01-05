# 0698-DirectX10-布局和像素着色器

## 目标

创建顶点的输入布局和像素着色器。

## 环境

- Time 2025-01-05
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

创建输入布局和像素着色器，这两个在一个文件，所以就一起写了。

## ps.hlsl

```hlsl
float4 main() : SV_TARGET
{
    return float4(1.0f, 1.0f, 1.0f, 1.0f);
}
```

## Shader.zig

render 方法也进行了实现。

```zig
const std = @import("std");
const win32 = @import("win32");

const d10 = win32.graphics.direct3d10;

vertexShader: *d10.ID3D10VertexShader,
vertexLayout: *d10.ID3D10InputLayout,
pixelShader: *d10.ID3D10PixelShader,

pub fn initialize(device: *d10.ID3D10Device) @This() {
    const vertex = compileShader(win32.zig.L("vs.hlsl"), "vs_4_0");
    defer _ = vertex.IUnknown.Release();

    var vs: ?*d10.ID3D10VertexShader = null;
    var byteCode: [*]u8 = @ptrCast(vertex.GetBufferPointer());
    var size = vertex.GetBufferSize();
    win32Check(device.CreateVertexShader(byteCode, size, &vs));

    var desc = std.mem.zeroes(d10.D3D10_INPUT_ELEMENT_DESC);
    desc.SemanticName = "POSITION";
    desc.SemanticIndex = 0;
    desc.Format = .R32G32_FLOAT;
    desc.InputSlotClass = .VERTEX_DATA;
    var layout: ?*d10.ID3D10InputLayout = null;

    const array = [_]d10.D3D10_INPUT_ELEMENT_DESC{desc};
    win32Check(device.CreateInputLayout(&array, array.len, byteCode, size, &layout));

    const pixel = compileShader(win32.zig.L("ps.hlsl"), "ps_4_0");
    defer _ = pixel.IUnknown.Release();

    var ps: ?*d10.ID3D10PixelShader = null;
    byteCode = @ptrCast(pixel.GetBufferPointer());
    size = pixel.GetBufferSize();
    win32Check(device.CreatePixelShader(byteCode, size, &ps));

    return .{ .vertexShader = vs.?, .vertexLayout = layout.?, .pixelShader = ps.? };
}

pub fn render(self: *@This(), device: *d10.ID3D10Device) void {
    device.IASetInputLayout(self.vertexLayout);
    device.IASetPrimitiveTopology(._PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    device.VSSetShader(self.vertexShader);
    device.PSSetShader(self.pixelShader);
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

## 附录
