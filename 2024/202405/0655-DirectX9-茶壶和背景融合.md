# 0655-DirectX9-茶壶和背景融合

## 目标

将画出的茶壶和背景图片进行融合，可以通过按键来控制茶壶的透明度。

## 环境

- Time 2024-08-15
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

融合就是两种颜色的混合，通过按键来控制比例，当为 0 时，完全透明，1 时，完全不透明。

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
var texture: *d3d9.IDirect3DTexture9 = undefined;
const True = win32.zig.TRUE;

var teapot: *d3dx9.ID3DXMesh = undefined;
var teapotMaterial: d3d9.D3DMATERIAL9 = undefined;

var bgMaterial: d3d9.D3DMATERIAL9 = d3d.Material.white;

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
    teapotMaterial = d3d.Material.red;
    teapotMaterial.Diffuse.a = 0.5;

    _ = d3dx9.D3DXCreateTeapot(device, &teapot, null);

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

    // 设置方向光
    var light = std.mem.zeroes(d3d9.D3DLIGHT9);
    light.Type = d3d9.D3DLIGHT_DIRECTIONAL;
    light.Ambient = .{ .r = 0.6, .g = 0.6, .b = 0.6, .a = 1 };
    light.Diffuse = .{ .r = 1.0, .g = 1.0, .b = 1.0, .a = 1 };
    light.Specular = .{ .r = 0.2, .g = 0.2, .b = 0.2, .a = 1 };
    light.Direction = .{ .x = 0.707, .y = 0, .z = 0.707 };
    _ = device.IDirect3DDevice9_SetLight(0, &light);
    _ = device.IDirect3DDevice9_LightEnable(0, True);

    // 打开镜面光
    _ = device.IDirect3DDevice9_SetRenderState(.NORMALIZENORMALS, True);
    _ = device.IDirect3DDevice9_SetRenderState(.SPECULARENABLE, True);

    // 创建纹理和过滤器
    const name = win32.zig.L("crate.jpg");
    _ = d3dx9.D3DXCreateTextureFromFileW(device, name, &texture);

    var state: u32 = @intFromEnum(d3d9.D3DTEXF_LINEAR);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MAGFILTER, state);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MINFILTER, state);
    state = @intFromEnum(d3d9.D3DTEXF_POINT);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MIPFILTER, state);

    // 设置透明混合
    // use alpha in material's diffuse component for alpha
    _ = device.IDirect3DDevice9_SetTextureStageState(0, .ALPHAARG1, 0);
    state = @intFromEnum(d3d9.D3DTOP_SELECTARG1);
    _ = device.IDirect3DDevice9_SetTextureStageState(0, .ALPHAOP, state);

    // set blending factors so that alpha component determines transparency
    state = @intFromEnum(d3d9.D3DBLEND_SRCALPHA);
    _ = device.IDirect3DDevice9_SetRenderState(.SRCBLEND, state);
    state = @intFromEnum(d3d9.D3DBLEND_INVSRCALPHA);
    _ = device.IDirect3DDevice9_SetRenderState(.DESTBLEND, state);

    // 设置视图矩阵
    const position = .{ .z = -3 };
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

fn cleanup() void {
    _ = buffer.IUnknown_Release();
}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

var y: f32 = 0.0;
fn display(_: f32) bool {
    const keyboard = win32.ui.input.keyboard_and_mouse;
    // increase/decrease alpha via keyboard input
    if (keyboard.GetAsyncKeyState('A') != 0)
        teapotMaterial.Diffuse.a += 0.01;
    if (keyboard.GetAsyncKeyState('S') != 0)
        teapotMaterial.Diffuse.a -= 0.01;

    // force alpha to [0, 1] interval
    if (teapotMaterial.Diffuse.a > 1.0) teapotMaterial.Diffuse.a = 1.0;
    if (teapotMaterial.Diffuse.a < 0.0) teapotMaterial.Diffuse.a = 0.0;

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    // Draw the background
    const unit: [16]f32 = .{
        1, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 1,
    };
    var world: win32.graphics.direct3d.D3DMATRIX = undefined;
    world.Anonymous.m = unit;
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    _ = device.IDirect3DDevice9_SetFVF(fvf);
    _ = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex));
    _ = device.IDirect3DDevice9_SetMaterial(&bgMaterial);
    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(texture));
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 0, 2);

    // Draw the teapot
    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, True);

    _ = d3dx9.D3DXMatrixScaling(&world, 1.5, 1.5, 1.5);
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    _ = device.IDirect3DDevice9_SetMaterial(&teapotMaterial);
    _ = device.IDirect3DDevice9_SetTexture(0, null);
    _ = teapot.ID3DXBaseMesh_DrawSubset(0);

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

![茶壶背景融合][1]

[1]: images/directx005.webp

## 附录
