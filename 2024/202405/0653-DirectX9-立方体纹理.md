# 0653-DirectX9-立方体纹理

## 目标

把立方体的每个面都贴上纹理。

## 环境

- Time 2024-08-14
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

感觉键盘的控制逻辑有点奇怪，不清楚控制的逻辑有没有写正确。

## d3d.zig

无变化。

## d3dx9.zig

无变化。

## cube.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const d3d9 = win32.graphics.direct3d9;

pub const Vertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    nx: f32 = 0,
    ny: f32 = 0,
    nz: f32 = 0,
    u: f32 = 0,
    v: f32 = 0,
};

pub const Cube = struct {
    vertex: *d3d9.IDirect3DVertexBuffer9 = undefined,
    index: *d3d9.IDirect3DIndexBuffer9 = undefined,

    pub fn init(d: *d3d9.IDirect3DDevice9, fvf: u32) Cube {
        var cube = Cube{};

        // 创建顶点缓存
        const usage = d3d9.D3DUSAGE_WRITEONLY;
        _ = d.IDirect3DDevice9_CreateVertexBuffer(24 * @sizeOf(Vertex), //
            usage, fvf, .MANAGED, @ptrCast(&cube.vertex), null);

        // 填充顶点数据
        var v: [*]Vertex = undefined;
        _ = cube.vertex.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&v), 0);

        // fill in the front face vertex data
        v[0] = .{ .x = -1, .y = -1, .z = -1, .nz = -1 };
        v[1] = .{ .x = -1, .y = 1, .z = -1, .nz = -1, .v = 1 };
        v[2] = .{ .x = 1, .y = 1, .z = -1, .nz = -1, .u = 1, .v = 1 };
        v[3] = .{ .x = 1, .y = -1, .z = -1, .nz = -1, .u = 1 };

        // fill in the back face vertex data
        v[4] = .{ .x = -1, .y = -1, .z = 1, .nz = 1 };
        v[5] = .{ .x = 1, .y = -1, .z = 1, .nz = 1, .v = 1 };
        v[6] = .{ .x = 1, .y = 1, .z = 1, .nz = 1, .u = 1, .v = 1 };
        v[7] = .{ .x = -1, .y = 1, .z = 1, .nz = 1, .u = 1 };

        // fill in the top face vertex data
        v[8] = .{ .x = -1, .y = 1, .z = -1, .ny = 1 };
        v[9] = .{ .x = -1, .y = 1, .z = 1, .ny = 1, .v = 1 };
        v[10] = .{ .x = 1, .y = 1, .z = 1, .ny = 1, .u = 1, .v = 1 };
        v[11] = .{ .x = 1, .y = 1, .z = -1, .ny = 1, .u = 1 };

        // fill in the bottom face vertex data
        v[12] = .{ .x = -1, .y = -1, .z = -1, .ny = -1 };
        v[13] = .{ .x = 1, .y = -1, .z = -1, .ny = -1, .v = 1 };
        v[14] = .{ .x = 1, .y = -1, .z = 1, .ny = -1, .u = 1, .v = 1 };
        v[15] = .{ .x = -1, .y = -1, .z = 1, .ny = -1, .u = 1 };

        // fill in the left face vertex data
        v[16] = .{ .x = -1, .y = -1, .z = 1, .nx = -1 };
        v[17] = .{ .x = -1, .y = 1, .z = 1, .nx = -1, .v = 1 };
        v[18] = .{ .x = -1, .y = 1, .z = -1, .nx = -1, .u = 1, .v = 1 };
        v[19] = .{ .x = -1, .y = -1, .z = -1, .nx = -1, .u = 1 };

        // fill in the right face vertex data
        v[20] = .{ .x = 1, .y = -1, .z = -1, .nx = 1 };
        v[21] = .{ .x = 1, .y = 1, .z = -1, .nx = 1, .v = 1 };
        v[22] = .{ .x = 1, .y = 1, .z = 1, .nx = 1, .u = 1, .v = 1 };
        v[23] = .{ .x = 1, .y = -1, .z = 1, .nx = 1, .u = 1 };

        _ = cube.vertex.IDirect3DVertexBuffer9_Unlock();

        _ = d.IDirect3DDevice9_CreateIndexBuffer(36 * @sizeOf(u16), //
            usage, .INDEX16, .MANAGED, @ptrCast(&cube.index), null);

        var i: [*]u16 = undefined;
        _ = cube.index.IDirect3DIndexBuffer9_Lock(0, 0, @ptrCast(&i), 0);

        // fill in the front face index data
        i[0] = 0;
        i[1] = 1;
        i[2] = 2;
        i[3] = 0;
        i[4] = 2;
        i[5] = 3;

        // fill in the back face index data
        i[6] = 4;
        i[7] = 5;
        i[8] = 6;
        i[9] = 4;
        i[10] = 6;
        i[11] = 7;

        // fill in the top face index data
        i[12] = 8;
        i[13] = 9;
        i[14] = 10;
        i[15] = 8;
        i[16] = 10;
        i[17] = 11;

        // fill in the bottom face index data
        i[18] = 12;
        i[19] = 13;
        i[20] = 14;
        i[21] = 12;
        i[22] = 14;
        i[23] = 15;

        // fill in the left face index data
        i[24] = 16;
        i[25] = 17;
        i[26] = 18;
        i[27] = 16;
        i[28] = 18;
        i[29] = 19;

        // fill in the right face index data
        i[30] = 20;
        i[31] = 21;
        i[32] = 22;
        i[33] = 20;
        i[34] = 22;
        i[35] = 23;

        _ = cube.index.IDirect3DIndexBuffer9_Unlock();
        return cube;
    }

    pub fn deinit(self: *Cube) void {
        _ = self.index.IUnknown_Release();
        _ = self.vertex.IUnknown_Release();
    }
};
```

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
var texture: *d3d9.IDirect3DTexture9 = undefined;
var box: cube.Cube = undefined;

const fvf = win32.system.system_services.D3DFVF_XYZ | //
    win32.system.system_services.D3DFVF_NORMAL | //
    win32.system.system_services.D3DFVF_TEX1;

// Framework Functions
fn setup() bool {
    box = cube.Cube.init(device, fvf);

    // 设置方向光
    var light = std.mem.zeroes(d3d9.D3DLIGHT9);
    light.Type = d3d9.D3DLIGHT_DIRECTIONAL;
    light.Ambient = .{ .r = 0.8, .g = 0.8, .b = 0.8, .a = 1 };
    light.Diffuse = .{ .r = 1.0, .g = 1.0, .b = 1.0, .a = 1 };
    light.Specular = .{ .r = 0.2, .g = 0.2, .b = 0.2, .a = 1 };
    light.Direction = .{ .x = 1.0, .y = -1, .z = 0.0 };
    _ = device.IDirect3DDevice9_SetLight(0, &light);
    const True = win32.zig.TRUE;
    _ = device.IDirect3DDevice9_LightEnable(0, True);

    // 打开镜面光
    _ = device.IDirect3DDevice9_SetRenderState(.NORMALIZENORMALS, True);
    _ = device.IDirect3DDevice9_SetRenderState(.SPECULARENABLE, True);

    // 创建纹理和过滤器
    const name = win32.zig.L("crate.jpg");
    _ = d3dx9.D3DXCreateTextureFromFileW(device, name, &texture);

    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(texture));
    const state: u32 = @intFromEnum(d3d9.D3DTEXF_LINEAR);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MAGFILTER, state);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MINFILTER, state);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MIPFILTER, state);

    // 设置投影矩阵
    var p: win32.graphics.direct3d.D3DMATRIX = undefined;
    const w = @as(f32, @floatFromInt(WIDTH));
    const h = @as(f32, @floatFromInt(HEIGHT));
    const fov = 0.5 * std.math.pi;
    _ = d3dx9.D3DXMatrixPerspectiveFovLH(&p, fov, w / h, 1.0, 1000.0);
    _ = device.IDirect3DDevice9_SetTransform(.PROJECTION, &p);

    const material = d3d9.D3DMATERIAL9{
        .Ambient = .{ .r = 1, .g = 1, .b = 1, .a = 1.0 },
        .Diffuse = .{ .r = 1, .g = 1, .b = 1, .a = 1.0 },
        .Specular = .{ .r = 1, .g = 1, .b = 1, .a = 1.0 },
        .Emissive = .{ .r = 0, .g = 0, .b = 0, .a = 1.0 },
        .Power = 2.0,
    };
    _ = device.IDirect3DDevice9_SetMaterial(&material);

    return true;
}

fn cleanup() void {}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

var angle: f32 = 3 * std.math.pi / 2.0;
var height: f32 = 2;

var y: f32 = 0.0;
fn display(delta: f32) bool {
    const timeDelta = delta * 0.001;
    // 设置视图矩阵
    const position = .{ .x = @cos(angle) * 3, .y = height, .z = @sin(angle) * 3 };
    var view: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixLookAtLH(&view, &position, &.{}, &.{ .y = 1.0 });
    _ = device.IDirect3DDevice9_SetTransform(d3d9.D3DTS_VIEW, &view);

    const keyboard = win32.ui.input.keyboard_and_mouse;
    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_LEFT)) != 0)
        angle -= 0.5 * timeDelta;

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_RIGHT)) != 0)
        angle += 0.5 * timeDelta;

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_UP)) != 0)
        height += 5.0 * timeDelta;

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_DOWN)) != 0)
        height -= 5.0 * timeDelta;

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    _ = device.IDirect3DDevice9_SetStreamSource(0, box.vertex, 0, @sizeOf(cube.Vertex));
    _ = device.IDirect3DDevice9_SetIndices(box.index);
    _ = device.IDirect3DDevice9_SetFVF(fvf);
    _ = device.IDirect3DDevice9_DrawIndexedPrimitive(.TRIANGLELIST, 0, 0, 24, 0, 12);

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

![立方体纹理][1]

[1]: images/directx003.webp

## 附录
