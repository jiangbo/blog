# 0654-DirectX9-茶壶线框模式

## 目标

画出茶壶的线框模式。

## 环境

- Time 2024-08-15
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>
3. <https://github.com/marlersoft/zigwin32>

## 想法

之前想跳过 D3DX 中复杂的网格，因为有继承，使用 Zig 不会写。
但是后面还有茶壶这些，没有办法，根据参考链接第三点仿照写了一个，居然可以用。

## d3dx9.zig

```zig
...
pub const ID3DXMesh = extern struct {
    pub const VTable = extern struct {
        base: ID3DXBaseMesh.VTable,
    };
    vtable: *const VTable,
    pub fn MethodMixin(comptime T: type) type {
        return struct {
            pub inline fn ID3DXBaseMesh_DrawSubset(self: *const T, attribId: u32) i32 {
                return @as(*const ID3DXBaseMesh.VTable, @ptrCast(self.vtable)).DrawSubset(@as(*const ID3DXBaseMesh, @ptrCast(self)), attribId);
            }
        };
    }
    pub usingnamespace MethodMixin(@This());
};

pub const ID3DXBaseMesh = extern struct {
    pub const VTable = extern struct {
        base: win32.system.com.IUnknown.VTable,
        DrawSubset: *const fn (
            self: *const ID3DXBaseMesh,
            attribId: u32,
        ) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,
    };
    vtable: *const VTable,
};

pub const ID3DXBuffer = extern struct {
    pub const VTable = extern struct {
        base: win32.system.com.IUnknown.VTable,
    };
    vtable: *const VTable,
};
...
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
const True = win32.zig.TRUE;

var teapot: *d3dx9.ID3DXMesh = undefined;

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
    _ = d3dx9.D3DXCreateTeapot(device, &teapot, null);

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

    const state: u32 = @intFromEnum(d3d9.D3DFILL_WIREFRAME);
    _ = device.IDirect3DDevice9_SetRenderState(.FILLMODE, state);

    return true;
}

fn cleanup() void {}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

var y: f32 = 0.0;
fn display(timeDelta: f32) bool {
    var world: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixRotationY(&world, y);

    y += timeDelta * 0.001;
    if (y >= 6.28) y = 0.0;

    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    // Draw the teapot
    _ = teapot.ID3DXBaseMesh_DrawSubset(0);

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

![茶壶线框模式][1]

[1]: images/directx004.webp

## 附录
