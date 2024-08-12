# 0648-DirectX-线框立方体

## 目标

在上一节的基础上，画出一个线框立方体。

## 环境

- Time 2024-08-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》

## 想法

之前缺少 WORLD 变换，忘记怎么加，现在记录下来。感觉 delta 时间那里还有问题，发现有 0 的情况，后续再看。

## d3d.zig

无变化。

## d3dx9.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d9 = win32.graphics.direct3d9;

pub const LPCTSTR = [*:0]align(1) const u16;

pub extern fn D3DXMatrixPerspectiveFovLH(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    fovy: f32,
    aspect: f32,
    zn: f32,
    zf: f32,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixLookAtLH(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    eye: *const Vec3,
    at: *const Vec3,
    up: *const Vec3,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixRotationX(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    angle: f32,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixRotationY(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    angle: f32,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixRotationZ(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    angle: f32,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixMultiply(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    m1: *const win32.graphics.direct3d.D3DMATRIX,
    m2: *const win32.graphics.direct3d.D3DMATRIX,
) *win32.graphics.direct3d.D3DMATRIX;

pub const Vec4 = extern struct { x: f32, y: f32, z: f32, w: f32 };
pub const Vec3 = extern struct { x: f32 = 0, y: f32 = 0, z: f32 = 0 };
```

## WORLD

增加了一个 WORLD 变换。

```zig
pub const D3DTRANSFORMSTATETYPE = enum(i32) {
    WORLD = 256,
    VIEW = 2,
    PROJECTION = 3,
    TEXTURE0 = 16,
    TEXTURE1 = 17,
    TEXTURE2 = 18,
    TEXTURE3 = 19,
    TEXTURE4 = 20,
    TEXTURE5 = 21,
    TEXTURE6 = 22,
    TEXTURE7 = 23,
    FORCE_DWORD = 2147483647,
};
```

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
var index: *d3d9.IDirect3DIndexBuffer9 = undefined;
const Vertex = packed struct { x: f32 = 0, y: f32 = 0, z: f32 = 0 };
const xyz = win32.system.system_services.D3DFVF_XYZ;

// Framework Functions
fn setup() bool {
    const usage = d3d9.D3DUSAGE_WRITEONLY;
    // 创建顶点缓存
    var hr = device.IDirect3DDevice9_CreateVertexBuffer(8 * @sizeOf(Vertex), //
        usage, xyz, .MANAGED, @ptrCast(&buffer), null);
    if (failed(hr)) win32Panic();

    // 创建索引缓存
    hr = device.IDirect3DDevice9_CreateIndexBuffer(36 * @sizeOf(u16), //
        usage, d3d9.D3DFMT_INDEX16, .MANAGED, @ptrCast(&index), null);
    if (failed(hr)) win32Panic();

    // 填充顶点数据
    var data: [*]Vertex = undefined;
    hr = buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&data), 0);
    if (failed(hr)) win32Panic();
    data[0] = .{ .x = -1.0, .y = -1.0, .z = -1.0 };
    data[1] = .{ .x = -1.0, .y = 1.0, .z = -1.0 };
    data[2] = .{ .x = 1.0, .y = 1.0, .z = -1.0 };
    data[3] = .{ .x = 1.0, .y = -1.0, .z = -1.0 };
    data[4] = .{ .x = -1.0, .y = -1.0, .z = 1.0 };
    data[5] = .{ .x = -1.0, .y = 1.0, .z = 1.0 };
    data[6] = .{ .x = 1.0, .y = 1.0, .z = 1.0 };
    data[7] = .{ .x = 1.0, .y = -1.0, .z = 1.0 };

    hr = buffer.IDirect3DVertexBuffer9_Unlock();
    if (failed(hr)) win32Panic();

    // 填充索引数据
    var indices: [*]u16 = undefined;
    hr = index.IDirect3DIndexBuffer9_Lock(0, 0, @ptrCast(&indices), 0);
    if (failed(hr)) win32Panic();

    // front side
    indices[0] = 0;
    indices[1] = 1;
    indices[2] = 2;
    indices[3] = 0;
    indices[4] = 2;
    indices[5] = 3;

    // back side
    indices[6] = 4;
    indices[7] = 6;
    indices[8] = 5;
    indices[9] = 4;
    indices[10] = 7;
    indices[11] = 6;

    // left side
    indices[12] = 4;
    indices[13] = 5;
    indices[14] = 1;
    indices[15] = 4;
    indices[16] = 1;
    indices[17] = 0;

    // right side
    indices[18] = 3;
    indices[19] = 2;
    indices[20] = 6;
    indices[21] = 3;
    indices[22] = 6;
    indices[23] = 7;

    // top
    indices[24] = 1;
    indices[25] = 5;
    indices[26] = 6;
    indices[27] = 1;
    indices[28] = 6;
    indices[29] = 2;

    // bottom
    indices[30] = 4;
    indices[31] = 0;
    indices[32] = 3;
    indices[33] = 4;
    indices[34] = 3;
    indices[35] = 7;

    hr = index.IDirect3DIndexBuffer9_Unlock();
    if (failed(hr)) win32Panic();

    // 设置视图矩阵
    const position = .{ .z = -5.0 };
    const up = .{ .y = 1.0 };
    var view: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixLookAtLH(&view, &position, &.{}, &up);

    hr = device.IDirect3DDevice9_SetTransform(d3d9.D3DTS_VIEW, &view);
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

    const state: u32 = @intFromEnum(d3d9.D3DFILL_WIREFRAME);
    hr = device.IDirect3DDevice9_SetRenderState(.FILLMODE, state);
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
fn display(delta: f32) bool {
    // 旋转立方体
    var rx = std.mem.zeroes(win32.graphics.direct3d.D3DMATRIX);
    var ry = std.mem.zeroes(win32.graphics.direct3d.D3DMATRIX);

    // rotate 45 degrees on x-axis
    _ = d3dx9.D3DXMatrixRotationX(&rx, 3.14 / 4.0);

    // incremement y-rotation angle each frame
    _ = d3dx9.D3DXMatrixRotationY(&ry, y);
    y += delta * 0.001;

    // reset angle to zero when angle reaches 2*PI
    if (y >= 6.28) y = 0.0;

    // combine x- and y-axis rotation transformations.
    var p: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixMultiply(&p, &rx, &ry);

    var hr = device.IDirect3DDevice9_SetTransform(.WORLD, &p);
    if (failed(hr)) win32Panic();

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;
    hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xffffffff, 1, 0);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_BeginScene();
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex));
    if (failed(hr)) win32Panic();
    hr = device.IDirect3DDevice9_SetIndices(index);
    if (failed(hr)) win32Panic();
    hr = device.IDirect3DDevice9_SetFVF(xyz);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_DrawIndexedPrimitive(.TRIANGLELIST, 0, 0, 8, 0, 12);
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

![线框立方体][1]。

[1]: images/directx82.webp

## 附录
