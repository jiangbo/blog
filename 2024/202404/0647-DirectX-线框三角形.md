# 0647-DirectX-线框三角形

## 目标

上一节初始化了环境，这里画出一个线框三角形。

## 环境

- Time 2024-08-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》

## 想法

感觉上知识跨度比较大，没有循序渐进。因为之前已经了解这方面的知识，所以还好，如果是第一次了解这些，感觉有点难度。

## d3d.zig

无变化。

## main.zig

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

// 三角形的顶点缓存
var buffer: *d3d9.IDirect3DVertexBuffer9 = undefined;
const Vertex = packed struct { x: f32 = 0, y: f32 = 0, z: f32 = 0 };
const xyz = win32.system.system_services.D3DFVF_XYZ;

// Framework Functions
fn setup() bool {

    // 创建顶点缓存
    var hr = device.IDirect3DDevice9_CreateVertexBuffer(3 * @sizeOf(Vertex), //
        d3d9.D3DUSAGE_WRITEONLY, xyz, .MANAGED, @ptrCast(&buffer), null);
    if (failed(hr)) win32Panic();

    var data: [*]Vertex = undefined;
    hr = buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&data), 0);
    if (failed(hr)) win32Panic();
    data[0] = .{ .x = -1.0, .y = 0.0, .z = 2.0 };
    data[1] = .{ .x = 0.0, .y = 1.0, .z = 2.0 };
    data[2] = .{ .x = 1.0, .y = 0.0, .z = 2.0 };
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
    hr = device.IDirect3DDevice9_SetTransform(d3d9.D3DTS_PROJECTION, &projection);
    if (failed(hr)) win32Panic();

    const state: u32 = @intFromEnum(d3d9.D3DFILL_WIREFRAME);
    hr = device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_FILLMODE, state);
    if (failed(hr)) win32Panic();

    return true;
}

fn cleanup() void {
    _ = buffer.IUnknown_Release();
}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

fn display(_: f32) bool {
    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xffffffff, 1, 0);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_BeginScene();
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex));
    if (failed(hr)) win32Panic();
    hr = device.IDirect3DDevice9_SetFVF(xyz);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_DrawPrimitive(d3d9.D3DPT_TRIANGLELIST, 0, 1);
    if (failed(hr)) win32Panic();

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

![线框三角形][1]。

[1]: images/directx81.png

## 附录
