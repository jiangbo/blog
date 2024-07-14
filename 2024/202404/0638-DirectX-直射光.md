# 0638-DirectX-直射光

## 目标

一个使用直射光源的例子。

## 环境

- Time 2024-07-14
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

书上原本的例子变换太不明显了，增加了每次旋转的角度，能更容易看出变化。

## win.zig

无变化。

## render.zig

无变化。

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

pub const Vec4 = extern struct { x: f32, y: f32, z: f32, w: f32 };

pub extern fn D3DXVec3Transform(
    vec: *Vec4,
    v: *const win32.graphics.direct3d.D3DVECTOR,
    m: *const win32.graphics.direct3d.D3DMATRIX,
) callconv(std.os.windows.WINAPI) *Vec4;

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
    const normal = win32.system.system_services.D3DFVF_NORMAL;
    var hr = device.IDirect3DDevice9_SetFVF(xyz | normal);
    if (failed(hr)) win32Panic();

    // set up vertices
    // dark square
    vert[0][0] = .{ .normal = .{ .z = -1 } };
    vert[0][1] = .{ .pos = .{ .x = 1 }, .normal = .{ .z = -1 } };
    vert[0][2] = .{ .pos = .{ .y = 1 }, .normal = .{ .z = -1 } };
    vert[0][3] = .{ .pos = .{ .x = 1, .y = 1 }, .normal = .{ .z = -1 } };

    // light square
    vert[1][0] = .{ .normal = .{ .z = -1 } };
    vert[1][1] = .{ .pos = .{ .x = 1 }, .normal = .{ .z = -1 } };
    vert[1][2] = .{ .pos = .{ .y = 1 }, .normal = .{ .z = -1 } };
    vert[1][3] = .{ .pos = .{ .x = 1, .y = 1 }, .normal = .{ .z = -1 } };

    // set up materials
    material = std.mem.zeroes(@TypeOf(material));
    material[0].Diffuse.r = 0.75;
    material[0].Diffuse.g = 0.75;
    material[0].Diffuse.b = 0.75;
    material[1].Diffuse.r = 0.5;
    material[1].Diffuse.g = 0.5;
    material[1].Diffuse.b = 0.5;

    // set up light
    light = std.mem.zeroes(d3d9.D3DLIGHT9);
    light.Type = .DIRECTIONAL;
    light.Diffuse.r = 1.0;
    light.Diffuse.g = 1.0;
    light.Diffuse.b = 1.0;
    light.Direction.x = 0;
    light.Direction.y = 0;
    light.Direction.z = 1;

    // // set the light
    hr = device.IDirect3DDevice9_SetLight(0, &light);
    if (failed(hr)) win32Panic();
    hr = device.IDirect3DDevice9_LightEnable(0, win32.zig.TRUE);
    if (failed(hr)) win32Panic();

    _ = d3dx9.D3DXMatrixRotationX(&rotates[0], DEGREE);
    _ = d3dx9.D3DXMatrixRotationX(&rotates[1], -DEGREE);
    _ = d3dx9.D3DXMatrixRotationY(&rotates[2], DEGREE);
    _ = d3dx9.D3DXMatrixRotationY(&rotates[3], -DEGREE);

    matrix.Anonymous.m = .{
        0.1875, 0,     0, 0,
        0,      -0.25, 0, 0,
        0,      0,     1, 0,
        -0.75,  1.0,   0, 1,
    };
    // set the projection matrix
    hr = device.IDirect3DDevice9_SetTransform(.PROJECTION, &matrix);
    if (failed(hr)) win32Panic();
}

var matrix: win32.graphics.direct3d.D3DMATRIX = undefined;
var vert: [2][4]CustomVertex = undefined;
var material: [2]d3d9.D3DMATERIAL9 = undefined;
var light: d3d9.D3DLIGHT9 = undefined;
var rotates: [4]win32.graphics.direct3d.D3DMATRIX = undefined;
const DEGREE: f32 = 18.0 / 180.0;

const CustomVertex = extern struct {
    pos: Vec3 = .{},
    normal: Vec3 = .{},
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
    changeLight();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xff000000, 0, 0);
    if (failed(hr)) win32Panic();

    // begin the scene
    if (failed(device.IDirect3DDevice9_BeginScene())) win32Panic();

    for (0..8) |y| {
        for (0..8) |x| {
            const fx: f32 = @floatFromInt(x);
            const fy: f32 = @floatFromInt(y);
            // set up world matrix
            var world: win32.graphics.direct3d.D3DMATRIX = undefined;
            _ = d3dx9.D3DXMatrixTranslation(&world, fx, fy, 0.0);

            // set world transformatin
            hr = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
            if (failed(hr)) win32Panic();

            const oddeven = (x + y) & 1;
            hr = device.IDirect3DDevice9_SetMaterial(&material[oddeven]);
            if (failed(hr)) win32Panic();

            // draw the square
            hr = device.IDirect3DDevice9_DrawPrimitiveUP(.TRIANGLESTRIP, //
                2, @ptrCast(&vert[oddeven]), @sizeOf(CustomVertex));
            if (failed(hr)) win32Panic();
        }
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

fn changeLight() void {
    if (win.key == null) return;

    std.log.info("click {}", .{win.key.?});
    const board = win32.ui.input.keyboard_and_mouse;

    const index: usize = switch (win.key.?) {
        @intFromEnum(board.VK_UP) => 2,
        @intFromEnum(board.VK_DOWN) => 3,
        @intFromEnum(board.VK_LEFT) => 0,
        @intFromEnum(board.VK_RIGHT) => 1,
        else => {
            win.key = null;
            return;
        },
    };

    var v: d3dx9.Vec4 = undefined;
    const old = light.Direction;
    _ = d3dx9.D3DXVec3Transform(&v, &old, &rotates[index]);
    light.Direction.x = v.x;
    light.Direction.y = v.y;
    light.Direction.z = v.z;

    const hr = device.IDirect3DDevice9_SetLight(0, &light);
    if (failed(hr)) win32Panic();
    win.key = null;
}
```

## 效果

![直射光源][1]。

[1]: images/directx72.webp

## 附录
