# 0662-DirectX9-3D 字体

## 目标

使用 `D3DXCreateTextW` 方法创建 3D 字体。

## 环境

- Time 2024-08-19
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

创建了一个 3D 的字体网格，然后进行光照渲染。

## d3d.zig

无变化。

## d3dx9.zig

```zig
...
pub extern fn D3DXCreateTextW(
    device: *d3d9.IDirect3DDevice9,
    hdc: win32.graphics.gdi.HDC,
    str: [*]align(1) const u16,
    deviation: f32,
    extrusion: f32,
    mesh: **ID3DXMesh,
    adjacency: ?**ID3DXBuffer,
    glyphMetrics: ?*win32.everything.GLYPHMETRICSFLOAT,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;
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

var text: *d3dx9.ID3DXMesh = undefined;

// Framework Functions
fn setup() bool {
    const hdc = win32.graphics.gdi.CreateCompatibleDC(null);

    // 初始化字体描述
    var lf = std.mem.zeroes(win32.graphics.gdi.LOGFONTW);

    lf.lfHeight = 25; // in logical units
    lf.lfWidth = 12; // in logical units
    lf.lfWeight = 500; // boldness, range 0(light) - 1000(bold)
    lf.lfCharSet = 1;

    const name = win32.zig.L("Times New Roman");
    @memcpy(lf.lfFaceName[0..name.len], name);

    const font = win32.graphics.gdi.CreateFontIndirectW(&lf).?;
    const old = win32.graphics.gdi.SelectObject(hdc, font);

    // Create the text mesh based on the selected font in the HDC.
    const str = win32.zig.L("Direct3D");
    _ = d3dx9.D3DXCreateTextW(device, hdc, str, 0.001, 0.4, &text, null, null);

    // Restore the old font and free the acquired HDC.
    _ = win32.graphics.gdi.SelectObject(hdc, old);
    _ = win32.graphics.gdi.DeleteObject(font);
    _ = win32.graphics.gdi.DeleteDC(hdc);

    // 设置方向光
    var light = std.mem.zeroes(d3d9.D3DLIGHT9);
    light.Type = d3d9.D3DLIGHT_DIRECTIONAL;
    light.Ambient = .{ .r = 0.4, .g = 0.4, .b = 0.4, .a = 0.4 };
    light.Diffuse = .{ .r = 1, .g = 1, .b = 1, .a = 1 };
    light.Specular = .{ .r = 0.6, .g = 0.6, .b = 0.6, .a = 0.6 };
    light.Direction = .{ .x = 0, .y = -0.5, .z = 1 };
    _ = device.IDirect3DDevice9_SetLight(0, &light);
    _ = device.IDirect3DDevice9_LightEnable(0, 1);

    // 打开镜面光
    _ = device.IDirect3DDevice9_SetRenderState(.NORMALIZENORMALS, 1);
    _ = device.IDirect3DDevice9_SetRenderState(.SPECULARENABLE, 1);

    // 设置视图矩阵
    const position = .{ .y = 1.5, .z = -3.3 };
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

var y: f32 = 0;
fn display(delta: f32) bool {
    var r = std.mem.zeroes(win32.graphics.direct3d.D3DMATRIX);
    _ = d3dx9.D3DXMatrixRotationY(&r, y);
    y += delta;

    if (y >= 6.28) y = 0.0;

    var t = std.mem.zeroes(win32.graphics.direct3d.D3DMATRIX);
    _ = d3dx9.D3DXMatrixTranslation(&t, -1.6, 0.0, 0.0);

    var world = std.mem.zeroes(win32.graphics.direct3d.D3DMATRIX);
    _ = d3dx9.D3DXMatrixMultiply(&world, &t, &r);

    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    _ = device.IDirect3DDevice9_SetMaterial(&d3d.Material.white);
    _ = text.ID3DXBaseMesh_DrawSubset(0);

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

![3D 字体][1]。

[1]: images/directx012.png

## 附录
