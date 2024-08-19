# 0659-DirectX9-镜面和阴影效果

## 目标

把之前的两个示例的内容综合到一起，同时具有镜面和阴影效果。

## 环境

- Time 2024-08-15
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

就是把两个例子的代码合并到一起，这个简单，只需要拷贝一下就行。
本书第二部分就算完成了，接下来进入到第三部分。

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
const True = win32.zig.TRUE;
var device: *d3d9.IDirect3DDevice9 = undefined;

var floorTexture: *d3d9.IDirect3DTexture9 = undefined;
var wallTexture: *d3d9.IDirect3DTexture9 = undefined;
var mirrorTexture: *d3d9.IDirect3DTexture9 = undefined;

var floorMaterial: d3d9.D3DMATERIAL9 = d3d.Material.white;
var wallMaterial: d3d9.D3DMATERIAL9 = d3d.Material.white;
var mirrorMaterial: d3d9.D3DMATERIAL9 = d3d.Material.white;

var teapot: *d3dx9.ID3DXMesh = undefined;
var teapotPos: d3dx9.Vec3 = .{ .y = 3, .z = -7.5 };
var teapotMaterial: d3d9.D3DMATERIAL9 = d3d.Material.yellow;

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

    // 墙具有低镜面效果
    wallMaterial.Specular = .{ .r = 0.2, .g = 0.2, .b = 0.2, .a = 0.2 };

    // 创建茶壶
    _ = d3dx9.D3DXCreateTeapot(device, &teapot, null);

    //
    // Create and specify geometry.  For this sample we draw a floor
    // and a wall with a mirror on it.  We put the floor, wall, and
    // mirror geometry in one vertex buffer.
    //
    //   |----|----|----|
    //   |Wall|Mirr|Wall|
    //   |    | or |    |
    //   /--------------/
    //  /   Floor      /
    // /--------------/
    //
    // 创建地板，墙，镜子的顶点缓存
    _ = device.IDirect3DDevice9_CreateVertexBuffer(24 * @sizeOf(Vertex), //
        0, fvf, .MANAGED, @ptrCast(&buffer), null);

    // 填充顶点数据
    var v: [*]Vertex = undefined;
    _ = buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&v), 0);

    // floor
    v[0] = .{ .x = -7.5, .z = -10, .ny = 1, .v = 1 };
    v[1] = .{ .x = -7.5, .ny = 1 };
    v[2] = .{ .x = 7.5, .ny = 1, .u = 1 };

    v[3] = .{ .x = -7.5, .z = -10, .ny = 1, .v = 1 };
    v[4] = .{ .x = 7.5, .ny = 1, .u = 1 };
    v[5] = .{ .x = 7.5, .z = -10, .ny = 1, .u = 1, .v = 1 };

    // wall
    v[6] = .{ .x = -7.5, .nz = -1, .v = 1 };
    v[7] = .{ .x = -7.5, .y = 5, .nz = -1 };
    v[8] = .{ .x = -2.5, .y = 5, .ny = -1, .u = 1 };

    v[9] = .{ .x = -7.5, .nz = -1, .v = 1 };
    v[10] = .{ .x = -2.5, .y = 5, .nz = -1, .u = 1 };
    v[11] = .{ .x = -2.5, .nz = -1, .u = 1, .v = 1 };

    // Note: We leave gap in middle of walls for mirror
    v[12] = .{ .x = 2.5, .nz = -1.0, .v = 1.0 };
    v[13] = .{ .x = 2.5, .y = 5.0, .nz = -1.0 };
    v[14] = .{ .x = 7.5, .y = 5.0, .nz = -1.0, .u = 1.0 };

    v[15] = .{ .x = 2.5, .nz = -1.0, .v = 1.0 };
    v[16] = .{ .x = 7.5, .y = 5.0, .nz = -1.0, .u = 1.0 };
    v[17] = .{ .x = 7.5, .nz = -1.0, .u = 1.0, .v = 1.0 };

    // mirror
    v[18] = .{ .x = -2.5, .nz = -1.0, .v = 1.0 };
    v[19] = .{ .x = -2.5, .y = 5.0, .nz = -1.0 };
    v[20] = .{ .x = 2.5, .y = 5.0, .nz = -1.0, .u = 1.0 };

    v[21] = .{ .x = -2.5, .nz = -1.0, .v = 1.0 };
    v[22] = .{ .x = 2.5, .y = 5.0, .nz = -1.0, .u = 1.0 };
    v[23] = .{ .x = 2.5, .nz = -1.0, .u = 1.0, .v = 1.0 };

    _ = buffer.IDirect3DVertexBuffer9_Unlock();

    // 创建纹理和过滤器
    const t1 = win32.zig.L("checker.jpg");
    _ = d3dx9.D3DXCreateTextureFromFileW(device, t1, &floorTexture);
    const t2 = win32.zig.L("brick0.jpg");
    _ = d3dx9.D3DXCreateTextureFromFileW(device, t2, &wallTexture);
    const t3 = win32.zig.L("ice.bmp");
    _ = d3dx9.D3DXCreateTextureFromFileW(device, t3, &mirrorTexture);

    const state: u32 = @intFromEnum(d3d9.D3DTEXF_LINEAR);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MAGFILTER, state);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MINFILTER, state);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MIPFILTER, state);

    // 设置方向光
    var light = std.mem.zeroes(d3d9.D3DLIGHT9);
    light.Type = d3d9.D3DLIGHT_DIRECTIONAL;
    light.Ambient = .{ .r = 0.4, .g = 0.4, .b = 0.4, .a = 0.4 };
    light.Diffuse = .{ .r = 1, .g = 1, .b = 1, .a = 1 };
    light.Specular = .{ .r = 0.6, .g = 0.6, .b = 0.6, .a = 0.6 };
    light.Direction = .{ .x = 0.707, .y = -0.707, .z = 0.707 };
    _ = device.IDirect3DDevice9_SetLight(0, &light);
    _ = device.IDirect3DDevice9_LightEnable(0, True);

    // 打开镜面光
    _ = device.IDirect3DDevice9_SetRenderState(.NORMALIZENORMALS, True);
    _ = device.IDirect3DDevice9_SetRenderState(.SPECULARENABLE, True);

    // 设置视图矩阵
    const position = .{ .x = -10, .y = 3, .z = -15 };
    var view: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixLookAtLH(&view, &position, &.{}, &.{ .y = 1.0 });
    _ = device.IDirect3DDevice9_SetTransform(d3d9.D3DTS_VIEW, &view);

    // 设置投影矩阵
    var p: win32.graphics.direct3d.D3DMATRIX = undefined;
    const w = @as(f32, @floatFromInt(WIDTH));
    const h = @as(f32, @floatFromInt(HEIGHT));
    const fov = std.math.pi / 4.0;
    _ = d3dx9.D3DXMatrixPerspectiveFovLH(&p, fov, w / h, 1.0, 1000.0);
    _ = device.IDirect3DDevice9_SetTransform(.PROJECTION, &p);

    return true;
}

fn cleanup() void {}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

var radius: f32 = 20;
var angle: f32 = (3.0 * std.math.pi) / 2.0;
fn display(delta: f32) bool {
    const timeDelta = delta / 1000.0;
    const keyboard = win32.ui.input.keyboard_and_mouse;
    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_LEFT)) != 0)
        teapotPos.x -= 3 * timeDelta;

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_RIGHT)) != 0)
        teapotPos.x += 3 * timeDelta;

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_UP)) != 0)
        radius += 2 * timeDelta;

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_DOWN)) != 0)
        radius -= 2 * timeDelta;

    if (keyboard.GetAsyncKeyState(('A')) != 0)
        angle -= 0.5 * timeDelta;

    if (keyboard.GetAsyncKeyState(('S')) != 0)
        angle += 0.5 * timeDelta;

    // 变换视图矩阵
    const position = .{ .x = @cos(angle) * radius, .y = 3, .z = @sin(angle) * radius };
    var view: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixLookAtLH(&view, &position, &.{}, &.{ .y = 1.0 });
    _ = device.IDirect3DDevice9_SetTransform(d3d9.D3DTS_VIEW, &view);

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER |
        win32.system.system_services.D3DCLEAR_STENCIL;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    renderScene();
    // draw shadow before mirror because the depth buffer hasn't been cleared
    // yet and we need the depth buffer so that the shadow is blended correctly.
    // That is, if an object obscures the shadow, we don't want to write the shadow
    // pixel.  Alternatively, we could redraw the scene to rewrite the depth buffer.
    // (RenderMirror clears the depth buffer).
    // 应该先画阴影，再画镜面
    renderShadow();
    renderMirror();

    _ = device.IDirect3DDevice9_EndScene();
    _ = device.IDirect3DDevice9_Present(null, null, null, null);

    return true;
}

fn renderScene() void {
    // draw teapot
    _ = device.IDirect3DDevice9_SetMaterial(&teapotMaterial);
    _ = device.IDirect3DDevice9_SetTexture(0, null);

    var world: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixTranslation(&world, teapotPos.x, teapotPos.y, teapotPos.z);

    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    _ = teapot.ID3DXBaseMesh_DrawSubset(0);

    const unit: [16]f32 = .{
        1, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 1,
    };
    world.Anonymous.m = unit;
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);

    _ = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex));
    _ = device.IDirect3DDevice9_SetFVF(fvf);

    // draw the floor
    _ = device.IDirect3DDevice9_SetMaterial(&floorMaterial);
    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(floorTexture));
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 0, 2);

    // draw the walls
    _ = device.IDirect3DDevice9_SetMaterial(&wallMaterial);
    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(wallTexture));
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 6, 4);

    // draw the mirror
    _ = device.IDirect3DDevice9_SetMaterial(&mirrorMaterial);
    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(mirrorTexture));
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 18, 2);
}

fn renderMirror() void {
    //
    // Draw Mirror quad to stencil buffer ONLY.  In this way
    // only the stencil bits that correspond to the mirror will
    // be on.  Therefore, the reflected teapot can only be rendered
    // where the stencil bits are turned on, and thus on the mirror
    // only.
    //
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILENABLE, True);
    var state: u32 = @intFromEnum(d3d9.D3DCMP_ALWAYS);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILFUNC, state);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILREF, 0x1);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILMASK, 0xffffffff);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILWRITEMASK, 0xffffffff);
    state = @intFromEnum(d3d9.D3DSTENCILOP_KEEP);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILZFAIL, state);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILFAIL, state);
    state = @intFromEnum(d3d9.D3DSTENCILOP_REPLACE);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILPASS, state);

    // disable writes to the depth and back buffers
    _ = device.IDirect3DDevice9_SetRenderState(.ZWRITEENABLE, 0);
    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, True);
    state = @intFromEnum(d3d9.D3DBLEND_ZERO);
    _ = device.IDirect3DDevice9_SetRenderState(.SRCBLEND, state);
    state = @intFromEnum(d3d9.D3DBLEND_ONE);
    _ = device.IDirect3DDevice9_SetRenderState(.DESTBLEND, state);

    // draw the mirror to the stencil buffer
    _ = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex));
    _ = device.IDirect3DDevice9_SetFVF(fvf);
    _ = device.IDirect3DDevice9_SetMaterial(&mirrorMaterial);
    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(mirrorTexture));
    const unit: [16]f32 = .{
        1, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 1,
    };
    var world: win32.graphics.direct3d.D3DMATRIX = undefined;
    world.Anonymous.m = unit;
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 18, 2);

    // re-enable depth writes
    _ = device.IDirect3DDevice9_SetRenderState(.ZWRITEENABLE, True);

    // only draw reflected teapot to the pixels where the mirror
    // was drawn to.
    state = @intFromEnum(d3d9.D3DCMP_EQUAL);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILFUNC, state);
    state = @intFromEnum(d3d9.D3DSTENCILOP_KEEP);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILPASS, state);

    // position reflection
    const zero = std.mem.zeroes(win32.graphics.direct3d.D3DMATRIX);
    var w: win32.graphics.direct3d.D3DMATRIX = zero;
    var t: win32.graphics.direct3d.D3DMATRIX = zero;
    var r: win32.graphics.direct3d.D3DMATRIX = zero;
    const plane: d3dx9.Vec4 = .{ .x = 0, .y = 0, .z = 1, .w = 0 };

    _ = d3dx9.D3DXMatrixReflect(&r, &plane);

    _ = d3dx9.D3DXMatrixTranslation(&t, teapotPos.x, teapotPos.y, teapotPos.z);

    _ = d3dx9.D3DXMatrixMultiply(&w, &t, &r);

    // clear depth buffer and blend the reflected teapot with the mirror
    const flags = win32.system.system_services.D3DCLEAR_ZBUFFER;
    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0, 1, 0);
    state = @intFromEnum(d3d9.D3DBLEND_DESTCOLOR);
    _ = device.IDirect3DDevice9_SetRenderState(.SRCBLEND, state);
    state = @intFromEnum(d3d9.D3DBLEND_ZERO);
    _ = device.IDirect3DDevice9_SetRenderState(.DESTBLEND, state);

    // Finally, draw the reflected teapot
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &w);
    _ = device.IDirect3DDevice9_SetMaterial(&teapotMaterial);
    _ = device.IDirect3DDevice9_SetTexture(0, null);

    state = @intFromEnum(d3d9.D3DCULL_CW);
    _ = device.IDirect3DDevice9_SetRenderState(.CULLMODE, state);
    _ = teapot.ID3DXBaseMesh_DrawSubset(0);

    // Restore render states.
    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, 0);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILENABLE, 0);
    state = @intFromEnum(d3d9.D3DCULL_CCW);
    _ = device.IDirect3DDevice9_SetRenderState(.CULLMODE, state);
}

fn renderShadow() void {
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILENABLE, True);
    var state: u32 = @intFromEnum(d3d9.D3DCMP_EQUAL);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILFUNC, state);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILREF, 0x0);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILMASK, 0xffffffff);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILWRITEMASK, 0xffffffff);
    state = @intFromEnum(d3d9.D3DSTENCILOP_KEEP);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILZFAIL, state);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILFAIL, state);
    state = @intFromEnum(d3d9.D3DSTENCILOP_INCR);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILPASS, state);

    // position shadow

    const light: d3dx9.Vec4 = .{ .x = 0.707, .y = -0.707, .z = 0.707, .w = 0 };
    const plane: d3dx9.Vec4 = .{ .x = 0, .y = -1, .z = 0, .w = 0 };
    var s: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixShadow(&s, &light, &plane);

    var t: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixTranslation(&t, teapotPos.x, teapotPos.y, teapotPos.z);

    var w: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixMultiply(&w, &t, &s);
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &w);

    // alpha blend the shadow
    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, True);
    state = @intFromEnum(d3d9.D3DBLEND_SRCALPHA);
    _ = device.IDirect3DDevice9_SetRenderState(.SRCBLEND, state);
    state = @intFromEnum(d3d9.D3DBLEND_INVSRCALPHA);
    _ = device.IDirect3DDevice9_SetRenderState(.DESTBLEND, state);

    var black = d3d.Material.black;
    black.Power = 0;
    black.Diffuse.a = 0.5; // 50% transparency.

    // Disable depth buffer so that z-fighting doesn't occur when we
    // render the shadow on top of the floor.
    _ = device.IDirect3DDevice9_SetRenderState(.ZENABLE, 0);
    _ = device.IDirect3DDevice9_SetMaterial(&black);
    _ = device.IDirect3DDevice9_SetTexture(0, null);
    _ = teapot.ID3DXBaseMesh_DrawSubset(0);

    _ = device.IDirect3DDevice9_SetRenderState(.ZENABLE, True);
    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, 0);
    _ = device.IDirect3DDevice9_SetRenderState(.STENCILENABLE, 0);
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

![阴影和镜面效果][1]。

[1]: images/directx009.webp

## 附录
