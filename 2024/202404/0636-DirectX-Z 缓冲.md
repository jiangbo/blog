# 0636-DirectX-Z 缓冲

## 目标

使用 Z 缓冲来控制渲染的遮挡问题。

## 环境

- Time 2024-07-14
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

2d 中一般是后渲染会覆盖前面的，不清楚使用 Z 缓冲后，无论什么顺序是不是会好一点。

## win.zig

无变化。

## render.zig

```zig
...
    //depth buffer information
    params.EnableAutoDepthStencil = win32.zig.TRUE;
    params.AutoDepthStencilFormat = .D16;
...
```

## d3dx9.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d9 = win32.graphics.direct3d9;

pub const LPCTSTR = [*:0]align(1) const u16;

pub extern fn D3DXCreateTextureFromFileW(
    device: *d3d9.IDirect3DDevice9,
    name: LPCTSTR,
    LPDIRECT3DTEXTURE9: ?**d3d9.IDirect3DTexture9,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub extern fn D3DXMatrixTranslation(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    x: f32,
    y: f32,
    z: f32,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixScaling(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    x: f32,
    y: f32,
    z: f32,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixRotationZ(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    angle: f32,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixMultiply(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    m1: *win32.graphics.direct3d.D3DMATRIX,
    m2: *win32.graphics.direct3d.D3DMATRIX,
) *win32.graphics.direct3d.D3DMATRIX;
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const win = @import("win.zig");
const render = @import("render.zig");
const d3dx9 = @import("d3dx9.zig");

const d3d9 = win32.graphics.direct3d9;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;
var device: *d3d9.IDirect3DDevice9 = undefined;

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    win.createWindow();

    gameInit();
    win.update(gameUpdate);
    gameShutdown();
}

const TRUE = win32.zig.TRUE;
const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    render.init(win.WIDTH, win.HEIGHT, win.hander);
    device = render.device;

    const xyz = win32.system.system_services.D3DFVF_XYZ;
    const diffuse = win32.system.system_services.D3DFVF_DIFFUSE;
    var hr = device.IDirect3DDevice9_SetFVF(xyz | diffuse);
    if (failed(hr)) win32Panic();

    // set up vertices
    const r: u32 = 0xffff0000;
    vert[0][0] = .{ .pos = .{ .x = -1, .z = 0.25 }, .diffuse = r };
    vert[0][1] = .{ .pos = .{ .z = 0.25 }, .diffuse = r };
    vert[0][2] = .{ .pos = .{ .x = -1, .y = -1, .z = 0.25 }, .diffuse = r };
    vert[0][3] = .{ .pos = .{ .y = -1, .z = 0.25 }, .diffuse = r };

    const g: u32 = 0xff00ff00;
    vert[1][0] = .{ .pos = .{ .y = 1, .z = 0.75 }, .diffuse = g };
    vert[1][1] = .{ .pos = .{ .x = 1, .y = 1, .z = 0.75 }, .diffuse = g };
    vert[1][2] = .{ .pos = .{ .z = 0.75 }, .diffuse = g };
    vert[1][3] = .{ .pos = .{ .x = 1, .z = 0.75 }, .diffuse = g };

    // set up projection matrix
    var p: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixScaling(&p, 2.0 / 640.0, 2.0 / 480.0, 1.0);
    hr = device.IDirect3DDevice9_SetTransform(.PROJECTION, &p);
    if (failed(hr)) win32Panic();

    // set up view matrix
    var v: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = d3dx9.D3DXMatrixScaling(&v, 128, 128, 0.0);

    hr = device.IDirect3DDevice9_SetTransform(.VIEW, &v);
    if (failed(hr)) win32Panic();

    // set up the rotation matrix
    const pi = std.math.pi;
    const unit: [16]f32 = .{
        1, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 1,
    };

    // set up world 0 and rotation 0
    worlds[0].Anonymous.m = unit;
    _ = d3dx9.D3DXMatrixRotationZ(&rotates[0], pi / 180.0);

    // set up world 1 and rotation 1
    worlds[1].Anonymous.m = unit;
    _ = d3dx9.D3DXMatrixRotationZ(&rotates[1], -pi / 180.0);
}

var rotates: [2]win32.graphics.direct3d.D3DMATRIX = undefined;
var worlds: [2]win32.graphics.direct3d.D3DMATRIX = undefined;

const CustomVertex = extern struct {
    pos: Vec3 = .{},
    diffuse: u32 = 0xffffffff,
};

const Vec3 = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
};

var vert: [2][4]CustomVertex = undefined;

fn gameUpdate() void {
    if (win.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xff000000, 1, 0);
    if (failed(hr)) win32Panic();

    // begin the scene
    if (failed(device.IDirect3DDevice9_BeginScene())) win32Panic();

    for (0..2) |i| {

        // set world transformatin
        hr = device.IDirect3DDevice9_SetTransform(.WORLD, &worlds[i]);
        if (failed(hr)) win32Panic();

        var tmp: win32.graphics.direct3d.D3DMATRIX = undefined;
        _ = d3dx9.D3DXMatrixMultiply(&tmp, &worlds[i], &rotates[i]);
        worlds[i] = tmp;

        hr = device.IDirect3DDevice9_DrawPrimitiveUP(.TRIANGLESTRIP, //
            2, &vert[i], @sizeOf(CustomVertex));
        if (failed(hr)) win32Panic();
    }

    // end the scene
    if (failed(device.IDirect3DDevice9_EndScene())) win32Panic();

    hr = device.IDirect3DDevice9_Present(null, null, null, null);
    if (failed(hr)) win32Panic();

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
}

fn win32Panic() noreturn {
    win.win32Panic();
}
```

## 效果

![Z 缓冲][1]。

[1]: images/directx70.webp

## 附录
