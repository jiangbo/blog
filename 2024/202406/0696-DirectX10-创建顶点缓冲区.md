# 0696-DirectX10-创建顶点缓冲区

## 目标

创建顶点缓冲区。

## 环境

- Time 2025-01-05
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <http://rastertek.com/tutdx10.html>
2. <https://enjoyphysics.cn/Soft/Program>

## 想法

感觉到这里，和之前学习的 OpenGL 有一点对的上了，有些概念有相似的地方。

## Model.zig

定义了三个点，还没有集成到一起，后面如果有错误，再修改。

```zig
const std = @import("std");
const win32 = @import("win32");

const d10 = win32.graphics.direct3d10;

vertexBuffer: *d10.ID3D10Buffer = undefined,

pub fn initialize(device: *d10.ID3D10Device) @This() {
    const vertices = [_]f32{
        -0.5, -0.5,
        0.5,  -0.5,
        0.0,  0.5,
    };

    var bufferDesc = std.mem.zeroes(d10.D3D10_BUFFER_DESC);
    bufferDesc.ByteWidth = @sizeOf(@TypeOf(vertices));
    bufferDesc.BindFlags = @intFromEnum(d10.D3D10_BIND_VERTEX_BUFFER);

    var initData = std.mem.zeroes(d10.D3D10_SUBRESOURCE_DATA);
    initData.pSysMem = &vertices;

    var vertexBuffer: *d10.ID3D10Buffer = undefined;
    win32Check(device.CreateBuffer(&bufferDesc, &initData, @ptrCast(&vertexBuffer)));

    return .{ .vertexBuffer = vertexBuffer };
}

pub fn render(self: *@This()) void {
    _ = self;
}

pub fn shutdown(self: *@This()) void {
    _ = self.vertexBuffer.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 附录
