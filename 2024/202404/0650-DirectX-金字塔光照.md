# 0650-DirectX-金字塔光照

## 目标

建立一个金字塔的顶点数据和法线，实现方向光的照射。

## 环境

- Time 2024-08-13
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》

## 想法

后面的光照使用了 D3DX 中的网格数据，跳过了三个光照的示例。把背景换成了刺眼的紫色，方便观察。

## d3d.zig

无变化。

## d3dx9.zig

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
var world: win32.graphics.direct3d.D3DMATRIX = undefined;

// 三角形的顶点缓存
var buffer: *d3d9.IDirect3DVertexBuffer9 = undefined;
const Vertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    nx: f32 = 0,
    ny: f32 = 0,
    nz: f32 = 0,
};
const fvf = win32.system.system_services.D3DFVF_XYZ | //
    win32.system.system_services.D3DFVF_NORMAL;

// Framework Functions
fn setup() bool {
    const usage = d3d9.D3DUSAGE_WRITEONLY;
    // 创建顶点缓存
    var hr = device.IDirect3DDevice9_CreateVertexBuffer(12 * @sizeOf(Vertex), //
        usage, fvf, .MANAGED, @ptrCast(&buffer), null);
    if (failed(hr)) win32Panic();

    // 填充顶点数据
    var data: [*]Vertex = undefined;
    hr = buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&data), 0);
    if (failed(hr)) win32Panic();

    // front face
    data[0] = .{ .x = -1.0, .z = -1.0, .ny = 0.707, .nz = -0.707 };
    data[1] = .{ .y = 1.0, .ny = 0.707, .nz = -0.707 };
    data[2] = .{ .x = 1.0, .z = -1.0, .ny = 0.707, .nz = -0.707 };

    // left face
    data[3] = .{ .x = -1.0, .z = 1.0, .nx = -0.707, .ny = 0.707 };
    data[4] = .{ .y = 1.0, .nx = -0.707, .ny = 0.707 };
    data[5] = .{ .x = -1, .z = -1.0, .nx = -0.707, .ny = 0.707 };

    // right face
    data[6] = .{ .x = 1.0, .z = -1.0, .nx = 0.707, .ny = 0.707 };
    data[7] = .{ .y = 1.0, .nx = 0.707, .ny = 0.707 };
    data[8] = .{ .x = 1.0, .z = 1.0, .nx = 0.707, .ny = 0.707 };

    // back face
    data[9] = .{ .x = 1.0, .z = 1.0, .ny = 0.707, .nz = 0.707 };
    data[10] = .{ .y = 1.0, .ny = 0.707, .nz = 0.707 };
    data[11] = .{ .x = -1.0, .z = 1.0, .ny = 0.707, .nz = 0.707 };

    hr = buffer.IDirect3DVertexBuffer9_Unlock();
    if (failed(hr)) win32Panic();

    // 创建材质
    const material = d3d9.D3DMATERIAL9{
        .Ambient = .{ .r = 1, .g = 1, .b = 1, .a = 1.0 },
        .Diffuse = .{ .r = 1, .g = 1, .b = 1, .a = 1.0 },
        .Specular = .{ .r = 1, .g = 1, .b = 1, .a = 1.0 },
        .Emissive = .{ .r = 0, .g = 0, .b = 0, .a = 1.0 },
        .Power = 5.0,
    };
    _ = device.IDirect3DDevice9_SetMaterial(&material);

    // 设置方向光
    var light = std.mem.zeroes(d3d9.D3DLIGHT9);
    light.Type = d3d9.D3DLIGHT_DIRECTIONAL;
    light.Diffuse = .{ .r = 1.0, .g = 1.0, .b = 1.0, .a = 1.0 };
    light.Specular = .{ .r = 0.3, .g = 0.3, .b = 0.3, .a = 0.3 };
    light.Ambient = .{ .r = 0.6, .g = 0.6, .b = 0.6, .a = 0.6 };
    light.Direction = .{ .x = 1.0, .y = 0.0, .z = 0.0 };
    _ = device.IDirect3DDevice9_SetLight(0, &light);
    const True = win32.zig.TRUE;
    _ = device.IDirect3DDevice9_LightEnable(0, True);

    // 打开镜面光
    _ = device.IDirect3DDevice9_SetRenderState(.NORMALIZENORMALS, True);
    _ = device.IDirect3DDevice9_SetRenderState(.SPECULARENABLE, True);

    // 设置视图矩阵
    const position = .{ .y = 1, .z = -3.0 };
    const up = .{ .y = 1.0 };
    var view: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixLookAtLH(&view, &position, &.{}, &up);
    _ = device.IDirect3DDevice9_SetTransform(d3d9.D3DTS_VIEW, &view);

    // 设置投影矩阵
    var projection: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixPerspectiveFovLH(
        &projection, // result
        0.5 * std.math.pi, // 90 - degrees
        @as(f32, @floatFromInt(WIDTH)) / @as(f32, @floatFromInt(HEIGHT)),
        1.0, // near plane
        1000.0, // far plane
    );
    _ = device.IDirect3DDevice9_SetTransform(.PROJECTION, &projection);

    return true;
}

fn cleanup() void {
    _ = buffer.IUnknown_Release();
}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

var y: f32 = 0.0;
fn display(timeDelta: f32) bool {

    // 旋转金字塔
    var rotateY: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixRotationY(&rotateY, y);
    y += timeDelta * 0.001;
    if (y >= 6.28) y = 0.0;
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &rotateY);

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    _ = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex));
    _ = device.IDirect3DDevice9_SetFVF(fvf);
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 0, 4);

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

![金字塔光照][1]。

[1]: images/directx84.webp

## 附录
