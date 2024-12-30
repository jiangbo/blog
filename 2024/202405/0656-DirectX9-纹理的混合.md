# 0656-DirectX9-纹理的混合

## 目标

使用两个纹理进行混合，其中一个纹理带有透明通道。

## 环境

- Time 2024-08-15
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

看起来代码挺多的，但是基本上都是之前练习过的代码，拷贝过来就可以使用。

## d3d.zig

无变化。

## d3dx9.zig

无变化。

## cube.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");
const cube = @import("cube.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const failed = win32.zig.FAILED;

// Globals
// var allocator: std.mem.Allocator = undefined;
pub const UNICODE: bool = true;
var device: *d3d9.IDirect3DDevice9 = undefined;
const True = win32.zig.TRUE;

var box: cube.Cube = undefined;

var crate: *d3d9.IDirect3DTexture9 = undefined;
var backDrop: *d3d9.IDirect3DTexture9 = undefined;

// 三角形的顶点缓存
var buffer: *d3d9.IDirect3DVertexBuffer9 = undefined;
const Vertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    nx: f32 = 0,
    ny: f32 = 0,
    nz: f32 = 0,
    u: f32 = 0,
    v: f32 = 0,
};
const fvf = win32.system.system_services.D3DFVF_XYZ | //
    win32.system.system_services.D3DFVF_NORMAL | //
    win32.system.system_services.D3DFVF_TEX1;

// Framework Functions
fn setup() bool {
    const usage = d3d9.D3DUSAGE_WRITEONLY;
    // 创建顶点缓存
    _ = device.IDirect3DDevice9_CreateVertexBuffer(6 * @sizeOf(Vertex), //
        usage, fvf, .MANAGED, @ptrCast(&buffer), null);

    // 填充顶点数据
    var data: [*]Vertex = undefined;
    _ = buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&data), 0);

    data[0] = .{ .x = -10, .y = -10, .z = 5, .nz = -1, .v = 1 };
    data[1] = .{ .x = -10, .y = 10, .z = 5, .nz = -1 };
    data[2] = .{ .x = 10, .y = 10, .z = 5, .nz = -1, .u = 1 };

    data[3] = .{ .x = -10, .y = -10, .z = 5, .nz = -1, .v = 1 };
    data[4] = .{ .x = 10, .y = 10, .z = 5, .nz = -1, .u = 1 };
    data[5] = .{ .x = 10, .y = -10, .z = 5, .nz = -1, .u = 1, .v = 1 };

    _ = buffer.IDirect3DVertexBuffer9_Unlock();

    // 创建立方体
    box = cube.Cube.init(device, fvf);

    // 创建纹理和过滤器
    const t1 = win32.zig.L("cratewalpha.dds");
    _ = d3dx9.D3DXCreateTextureFromFileW(device, t1, &crate);

    const t2 = win32.zig.L("lobbyxpos.JPG");
    _ = d3dx9.D3DXCreateTextureFromFileW(device, t2, &backDrop);

    var state: u32 = @intFromEnum(d3d9.D3DTEXF_LINEAR);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MAGFILTER, state);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MINFILTER, state);
    state = @intFromEnum(d3d9.D3DTEXF_POINT);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MIPFILTER, state);

    // 设置透明混合
    // use alpha in material's diffuse component for alpha
    state = win32.system.system_services.D3DTA_TEXTURE;
    _ = device.IDirect3DDevice9_SetTextureStageState(0, .ALPHAARG1, state);
    state = @intFromEnum(d3d9.D3DTOP_SELECTARG1);
    _ = device.IDirect3DDevice9_SetTextureStageState(0, .ALPHAOP, state);

    // set blending factors so that alpha component determines transparency
    state = @intFromEnum(d3d9.D3DBLEND_SRCALPHA);
    _ = device.IDirect3DDevice9_SetRenderState(.SRCBLEND, state);
    state = @intFromEnum(d3d9.D3DBLEND_INVSRCALPHA);
    _ = device.IDirect3DDevice9_SetRenderState(.DESTBLEND, state);

    // 关闭光照
    _ = device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_LIGHTING, 0);

    // 设置视图矩阵
    const position = .{ .z = -2.5 };
    var view: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixLookAtLH(&view, &position, &.{}, &.{ .y = 1.0 });
    _ = device.IDirect3DDevice9_SetTransform(d3d9.D3DTS_VIEW, &view);

    // 设置投影矩阵
    var p: win32.graphics.direct3d.D3DMATRIX = undefined;
    const w = @as(f32, @floatFromInt(WIDTH));
    const h = @as(f32, @floatFromInt(HEIGHT));
    const fov = 0.5 * std.math.pi;
    _ = d3dx9.D3DXMatrixPerspectiveFovLH(&p, fov, w / h, 1.0, 1000.0);
    _ = device.IDirect3DDevice9_SetTransform(.PROJECTION, &p);

    return true;
}

fn cleanup() void {}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

var y: f32 = 0.0;
fn display(timeDelta: f32) bool {
    var rotateX: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixRotationX(&rotateX, std.math.pi * 0.2);

    var rotateY: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixRotationY(&rotateY, y);
    y += timeDelta * 0.001;

    if (y >= 6.28) y = 0.0;

    var cubeWorld: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixMultiply(&cubeWorld, &rotateX, &rotateY);

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    // draw back drop
    const unit: [16]f32 = .{
        1, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 1,
    };
    var world: win32.graphics.direct3d.D3DMATRIX = undefined;
    world.Anonymous.m = unit;
    _ = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex));
    _ = device.IDirect3DDevice9_SetFVF(fvf);
    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(backDrop));
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 0, 2);

    // draw cube
    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, True);

    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &cubeWorld);
    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(crate));
    _ = device.IDirect3DDevice9_SetStreamSource(0, box.vertex, 0, @sizeOf(Vertex));
    _ = device.IDirect3DDevice9_SetIndices(box.index);
    _ = device.IDirect3DDevice9_SetFVF(fvf);
    _ = device.IDirect3DDevice9_DrawIndexedPrimitive(.TRIANGLELIST, 0, 0, 24, 0, 12);

    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, 0);

    _ = device.IDirect3DDevice9_EndScene();
    _ = device.IDirect3DDevice9_Present(null, null, null, null);

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

![纹理混合][1]

[1]: images/directx006.webp

## 附录
