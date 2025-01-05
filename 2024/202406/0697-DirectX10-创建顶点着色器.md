# 0697-DirectX10-创建顶点着色器

## 目标

创建顶点着色器。

## 环境

- Time 2025-01-05
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

创建着色器，没有使用 VS 来编译着色器，直接使用代码进行着色器的编译。

## vs.hlsl

```hlsl
float4 main(float2 input : POSITION) : SV_POSITION
{
    return float4(input, 0, 1);
}
```

## Shader.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const d10 = win32.graphics.direct3d10;

vertexShader: *d10.ID3D10VertexShader,

pub fn initialize(device: *d10.ID3D10Device) @This() {
    const vertex = compileShader(win32.zig.L("vs.hlsl"), "vs_4_0");
    defer _ = vertex.IUnknown.Release();

    var vs: ?*d10.ID3D10VertexShader = null;
    const byteCode: [*]u8 = @ptrCast(vertex.GetBufferPointer());
    const size = vertex.GetBufferSize();
    win32Check(device.CreateVertexShader(byteCode, size, &vs));

    return .{ .vertexShader = vs.? };
}

pub fn render(self: *@This()) void {
    _ = self;
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexShader.IUnknown.Release();
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

## 效果

如果着色器存在问题，程序会打印出错信息。

![错误提示][1]

[1]: images/directx038.png

## 附录
