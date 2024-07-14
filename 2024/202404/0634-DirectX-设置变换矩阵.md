# 0634-DirectX-设置变换矩阵

## 目标

通过设置变换矩阵，对一个模型渲染出多个实例。

## 环境

- Time 2024-07-14
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

Zig 的 Win32 绑定里，没有世界变换的那个枚举值，我手动修改源码添加了一个。

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
    // dark square
    const dark: u32 = 0xff808080;
    vert[0][0] = .{ .diffuse = dark };
    vert[0][1] = .{ .pos = .{ .x = 1 }, .diffuse = dark };
    vert[0][2] = .{ .pos = .{ .y = 1 }, .diffuse = dark };
    vert[0][3] = .{ .pos = .{ .x = 1, .y = 1 }, .diffuse = dark };

    // light square
    const light: u32 = 0xffc0c0c0;
    vert[1][0] = .{ .diffuse = light };
    vert[1][1] = .{ .pos = .{ .x = 1 }, .diffuse = light };
    vert[1][2] = .{ .pos = .{ .y = 1 }, .diffuse = light };
    vert[1][3] = .{ .pos = .{ .x = 1, .y = 1 }, .diffuse = light };

    matrix.Anonymous.m = .{
        0.1875, 0,     0, 0, //
        0,      -0.25, 0, 0,
        0,      0,     1, 0,
        -0.75,  1.0,   0, 1,
    };
    // set the projection matrix
    hr = device.IDirect3DDevice9_SetTransform(.PROJECTION, &matrix);
    if (failed(hr)) win32Panic();
}

const CustomVertex = extern struct {
    pos: Vec3 = .{},
    diffuse: u32 = 0xffffffff,
};

const Vec3 = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
};

var matrix: win32.graphics.direct3d.D3DMATRIX = undefined;
var vert: [2][4]CustomVertex = undefined;

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

            // determine color of square
            const oddeven = (x + y) & 1;
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
```

## 效果

![设置变换矩阵][1]。

[1]: images/directx68.png

## 附录
