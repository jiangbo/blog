# 0644-DirectX-透明混合

## 目标

当两种颜色叠加到一起时，如果有透明度，则两种颜色进行混合。

## 环境

- Time 2024-07-14
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

把 D3DFVF_DIFFUSE 变量写成了 D3DTA_DIFFUSE，造成一直是白色，排查了很久。

## win.zig

无变化。

## render.zig

无变化。

## d3dx9.zig

无变化。

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

    var hr = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, TRUE);
    if (failed(hr)) win32Panic();
    var s = @intFromEnum(d3d9.D3DBLEND_SRCALPHA);
    hr = device.IDirect3DDevice9_SetRenderState(.SRCBLEND, s);
    if (failed(hr)) win32Panic();
    s = @intFromEnum(d3d9.D3DBLEND_INVSRCALPHA);
    hr = device.IDirect3DDevice9_SetRenderState(.DESTBLEND, s);
    if (failed(hr)) win32Panic();
    s = @intFromEnum(d3d9.D3DBLENDOP_ADD);
    hr = device.IDirect3DDevice9_SetRenderState(.BLENDOP, s);
    if (failed(hr)) win32Panic();

    const xyz = win32.system.system_services.D3DFVF_XYZ;
    const diffuse = win32.system.system_services.D3DFVF_DIFFUSE;
    hr = device.IDirect3DDevice9_SetFVF(xyz | diffuse);
    if (failed(hr)) win32Panic();

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

    // set up vertices
    const r: u32 = 0x80ff0000;
    vert[0][0] = .{ .pos = .{ .x = -1, .z = 0.25 }, .diffuse = r };
    vert[0][1] = .{ .pos = .{ .z = 0.25 }, .diffuse = r };
    vert[0][2] = .{ .pos = .{ .x = -1, .y = -1, .z = 0.25 }, .diffuse = r };
    vert[0][3] = .{ .pos = .{ .y = -1, .z = 0.25 }, .diffuse = r };

    const g: u32 = 0xff00ff00;
    vert[1][0] = .{ .pos = .{ .y = 1, .z = 0.75 }, .diffuse = g };
    vert[1][1] = .{ .pos = .{ .x = 1, .y = 1, .z = 0.75 }, .diffuse = g };
    vert[1][2] = .{ .pos = .{ .z = 0.75 }, .diffuse = g };
    vert[1][3] = .{ .pos = .{ .x = 1, .z = 0.75 }, .diffuse = g };
}

var vert: [2][4]CustomVertex = undefined;
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

fn gameUpdate() void {
    if (win.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xff000000, 0, 0);
    if (failed(hr)) win32Panic();

    // begin the scene
    if (failed(device.IDirect3DDevice9_BeginScene())) win32Panic();

    var index: i32 = 1;
    while (index >= 0) : (index -= 1) {
        const i: usize = @intCast(index);
        // set world matrix
        hr = device.IDirect3DDevice9_SetTransform(.WORLD, &worlds[i]);
        if (failed(hr)) win32Panic();
        // rotate world matrix for next time
        var tmp: win32.graphics.direct3d.D3DMATRIX = undefined;
        _ = d3dx9.D3DXMatrixMultiply(&tmp, &worlds[i], &rotates[i]);
        worlds[i] = tmp;
        // draw the vertices
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

![放射光][1]。

[1]: images/directx78.webp

## 附录
