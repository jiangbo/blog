# 0649-DirectX-颜色三角形

## 目标

画出两种不同着色模式的三角形。

## 环境

- Time 2024-08-13
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》

## 想法

前一章使用 D3DX 画出了茶壶等网格，但是有继承，在 Zig 中不好实现，所以跳过了两个示例。

## d3d.zig

无变化。

## d3dx9.zig

增加了一个 `D3DXMatrixTranslation` 矩阵变换函数。

```zig
...
pub extern fn D3DXMatrixTranslation(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    x: f32,
    y: f32,
    z: f32,
) *win32.graphics.direct3d.D3DMATRIX;
...
```

## main.zig

每次需要检测 `HRESULT` 结果有点烦，后面练习的过程中可能会不检测返回结果了。

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const failed = win32.zig.FAILED;

// Globals
// var allocator: std.mem.Allocator = undefined;
pub const UNICODE: bool = true;
var device: *d3d9.IDirect3DDevice9 = undefined;
var world: win32.graphics.direct3d.D3DMATRIX = undefined;

// 三角形的顶点缓存
var buffer: *d3d9.IDirect3DVertexBuffer9 = undefined;
const ColorVertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    color: u32 = 0xffffffff,
};
const fvf = win32.system.system_services.D3DFVF_XYZ | //
    win32.system.system_services.D3DFVF_DIFFUSE;

// Framework Functions
fn setup() bool {
    const usage = d3d9.D3DUSAGE_WRITEONLY;
    // 创建顶点缓存
    var hr = device.IDirect3DDevice9_CreateVertexBuffer(3 * @sizeOf(ColorVertex), //
        usage, fvf, .MANAGED, @ptrCast(&buffer), null);
    if (failed(hr)) win32Panic();

    // 填充顶点数据
    var data: [*]ColorVertex = undefined;
    hr = buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&data), 0);
    if (failed(hr)) win32Panic();
    data[0] = .{ .x = -1.0, .y = 0.0, .z = 2.0, .color = 0xffff0000 };
    data[1] = .{ .x = 0.0, .y = 1.0, .z = 2.0, .color = 0xff00ff00 };
    data[2] = .{ .x = 1.0, .y = 0.0, .z = 2.0, .color = 0xff0000ff };

    hr = buffer.IDirect3DVertexBuffer9_Unlock();
    if (failed(hr)) win32Panic();

    // 设置投影矩阵
    var projection: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixPerspectiveFovLH(
        &projection, // result
        0.5 * std.math.pi, // 90 - degrees
        @as(f32, @floatFromInt(WIDTH)) / @as(f32, @floatFromInt(HEIGHT)),
        1.0, // near plane
        1000.0, // far plane
    );
    hr = device.IDirect3DDevice9_SetTransform(.PROJECTION, &projection);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetRenderState(.LIGHTING, 0);
    if (failed(hr)) win32Panic();

    return true;
}

fn cleanup() void {
    _ = buffer.IUnknown_Release();
}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

var y: f32 = 0.0;
fn display(_: f32) bool {
    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xffffffff, 1, 0);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_BeginScene();
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(ColorVertex));
    if (failed(hr)) win32Panic();
    hr = device.IDirect3DDevice9_SetFVF(fvf);
    if (failed(hr)) win32Panic();

    // 左边的三角形使用 FLAT 着色
    _ = d3dx9.D3DXMatrixTranslation(&world, -1.25, 0.0, 0.0);
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    var state: u32 = @intFromEnum(d3d9.D3DSHADE_FLAT);
    _ = device.IDirect3DDevice9_SetRenderState(.SHADEMODE, state);
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 0, 1);

    // 右边的三角形使用 GOURAUD 着色
    _ = d3dx9.D3DXMatrixTranslation(&world, 1.25, 0.0, 0.0);
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    state = @intFromEnum(d3d9.D3DSHADE_GOURAUD);
    _ = device.IDirect3DDevice9_SetRenderState(.SHADEMODE, state);
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 0, 1);

    hr = device.IDirect3DDevice9_EndScene();
    if (failed(hr)) win32Panic();

    // Swap the back and front buffers.
    hr = device.IDirect3DDevice9_Present(null, null, null, null);
    if (failed(hr)) win32Panic();
    return true;
}

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

// main
pub fn main() void {
    device = d3d.initD3D(WIDTH, HEIGHT);

    if (!setup()) @panic("Setup() - FAILED");

    d3d.enterMsgLoop(display);

    cleanup();
    _ = device.IUnknown_Release();
}
```

## 效果

![颜色三角形][1]。

[1]: images/directx83.png

## 附录
