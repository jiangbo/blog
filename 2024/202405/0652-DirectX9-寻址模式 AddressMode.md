# 0652-DirectX9-寻址模式 AddressMode

## 目标

通过按键切换不同的 AddressMode。

## 环境

- Time 2024-08-14
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>
3. <https://learn.microsoft.com/zh-cn/windows/uwp/graphics-concepts/texture-addressing-modes>

## 想法

寻址模式之前已经了解过了，不算太难，看一下就懂，可以参考第三点参考链接。

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

    data[0] = .{ .x = -1, .y = -1, .z = 1.25, .nz = -1, .v = 3 };
    data[1] = .{ .x = -1, .y = 1, .z = 1.25, .nz = -1 };
    data[2] = .{ .x = 1, .y = 1, .z = 1.25, .nz = -1, .u = 3 };

    data[3] = .{ .x = -1, .y = -1, .z = 1.25, .nz = -1, .v = 3 };
    data[4] = .{ .x = 1, .y = 1, .z = 1.25, .nz = -1, .u = 3 };
    data[5] = .{ .x = 1, .y = -1, .z = 1.25, .nz = -1, .u = 3, .v = 3 };

    _ = buffer.IDirect3DVertexBuffer9_Unlock();

    // 创建纹理和过滤器
    const name = win32.zig.L("dx5_logo.bmp");
    _ = d3dx9.D3DXCreateTextureFromFileW(device, name, &texture);

    _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(texture));
    var state: u32 = @intFromEnum(d3d9.D3DTEXF_LINEAR);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MAGFILTER, state);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MINFILTER, state);
    state = @intFromEnum(d3d9.D3DTEXF_POINT);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .MIPFILTER, state);

    // 关闭光照
    _ = device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_LIGHTING, 0);

    // 设置投影矩阵
    var p: win32.graphics.direct3d.D3DMATRIX = undefined;
    const w = @as(f32, @floatFromInt(WIDTH));
    const h = @as(f32, @floatFromInt(HEIGHT));
    const fov = 0.5 * std.math.pi;
    _ = d3dx9.D3DXMatrixPerspectiveFovLH(&p, fov, w / h, 1.0, 1000.0);
    _ = device.IDirect3DDevice9_SetTransform(.PROJECTION, &p);

    _ = device.IDirect3DDevice9_SetSamplerState(0, .BORDERCOLOR, 0x000000ff);

    return true;
}

fn cleanup() void {
    _ = buffer.IUnknown_Release();
}

pub fn win32Panic() noreturn {
    d3d.win32Panic();
}

var samplerState: d3d9.D3DTEXTUREADDRESS = .WRAP;

var y: f32 = 0.0;
fn display(_: f32) bool {
    const keyboard = win32.ui.input.keyboard_and_mouse;
    // set wrap address mode
    if (keyboard.GetAsyncKeyState('W') != 0) samplerState = .WRAP;

    // set border color address mode
    if (keyboard.GetAsyncKeyState('B') != 0) samplerState = .BORDER;

    // set clamp address mode
    if (keyboard.GetAsyncKeyState('C') != 0) samplerState = .CLAMP;

    // set mirror address mode
    if (keyboard.GetAsyncKeyState('M') != 0) samplerState = .MIRROR;

    const state: u32 = @intCast(@intFromEnum(samplerState));
    _ = device.IDirect3DDevice9_SetSamplerState(0, .ADDRESSU, state);
    _ = device.IDirect3DDevice9_SetSamplerState(0, .ADDRESSV, state);

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    _ = device.IDirect3DDevice9_SetStreamSource(0, buffer, 0, @sizeOf(Vertex));
    _ = device.IDirect3DDevice9_SetFVF(fvf);
    _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 0, 2);

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

![寻址模式][1]。

[1]: images/directx002.webp

## 附录
