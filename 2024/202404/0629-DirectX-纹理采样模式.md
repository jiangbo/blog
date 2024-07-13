# 0629-DirectX-纹理采样模式

## 目标

将纹理的采样模式设置成镜像 MIRROR 模式，就不太能看出边界了。

## 环境

- Time 2024-07-13
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

这个概念之前也了解过，不过方法变了，应该使用 SetSamplerState 去设置。

## win.zig

无变化。

## render.zig

无变化。

## lib.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const win = @import("win.zig");
const lib = @import("lib.zig");
const render = @import("render.zig");
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

const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    render.init(win.WIDTH, win.HEIGHT, win.hander);
    device = render.device;

    const xyzrhw = win32.system.system_services.D3DFVF_XYZRHW;
    const diffuse = win32.system.system_services.D3DFVF_DIFFUSE;
    const uv = win32.system.system_services.D3DFVF_TEX1;
    var hr = device.IDirect3DDevice9_SetFVF(xyzrhw | diffuse | uv);
    if (failed(hr)) win32Panic();

    // set up vertex colors
    vertices[0] = .{ .diffuse = 0xffff0000 };
    vertices[1] = .{ .diffuse = 0xff00ff00, .u = 2 };
    vertices[2] = .{ .diffuse = 0xff0000ff, .v = 2 };
    vertices[3] = .{ .diffuse = 0xffffffff, .u = 2, .v = 2 };

    // load texture image
    const f = render.mode.Format;
    const surface = lib.loadSourface(allocator, device, f, "texture.bmp");
    defer _ = surface.IUnknown_Release();

    hr = device.IDirect3DDevice9_CreateTexture(tSize, tSize, 1, 0, //
        f, .DEFAULT, @ptrCast(&texture), null);
    if (failed(hr)) win32Panic();

    var dest: ?*d3d9.IDirect3DSurface9 = undefined;
    hr = texture.IDirect3DTexture9_GetSurfaceLevel(0, &dest);
    if (failed(hr)) win32Panic();
    defer _ = dest.?.IUnknown_Release();
    // copy image to texure

    // source and destionation
    var src = std.mem.zeroes(win32.foundation.RECT);
    src.right = tSize;
    src.bottom = tSize;

    const point = std.mem.zeroes(win32.foundation.POINT);

    hr = device.IDirect3DDevice9_UpdateSurface(surface, &src, dest, &point);
    if (failed(hr)) win32Panic();

    const value: i32 = @intFromEnum(d3d9.D3DTADDRESS_MIRROR);
    hr = device.IDirect3DDevice9_SetSamplerState(0, .ADDRESSU, value);
    if (failed(hr)) win32Panic();
    hr = device.IDirect3DDevice9_SetSamplerState(0, .ADDRESSV, value);
    if (failed(hr)) win32Panic();

    // set the texture
    hr = device.IDirect3DDevice9_SetTexture(0, @ptrCast(texture));
    if (failed(hr)) win32Panic();
}

const CustomVertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    rhw: f32 = 1,
    diffuse: u32,
    u: f32 = 0,
    v: f32 = 0,
};

const tSize: u32 = 256;
const PI: f32 = std.math.pi;
const RADIUS: f32 = 200.0;

var vertices: [4]CustomVertex = undefined;
var angle: f32 = 0;
var texture: *d3d9.IDirect3DTexture9 = undefined;

fn gameUpdate() void {
    if (win.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xff000000, 0, 0);
    if (failed(hr)) win32Panic();

    const width: f32 = @floatFromInt(win.WIDTH);
    const height: f32 = @floatFromInt(win.HEIGHT);
    // set up vertices
    // vertex 0
    vertices[0].x = width / 2 + @cos(angle) * RADIUS;
    vertices[0].y = height / 2 + @sin(angle) * RADIUS;
    // vertex 1
    vertices[1].x = width / 2 + @cos(angle + 2.0 * PI / 4.0) * RADIUS;
    vertices[1].y = height / 2 + @sin(angle + 2.0 * PI / 4.0) * RADIUS;
    // vertex 2
    vertices[2].x = width / 2 + @cos(angle - 2.0 * PI / 4.0) * RADIUS;
    vertices[2].y = height / 2 + @sin(angle - 2.0 * PI / 4.0) * RADIUS;
    // vertex 3
    vertices[3].x = width / 2 + @cos(angle + PI) * RADIUS;
    vertices[3].y = height / 2 + @sin(angle + PI) * RADIUS;

    // increase angle for next time
    angle += (2.0 * PI / 360.0);

    // begin the scene
    if (failed(device.IDirect3DDevice9_BeginScene())) win32Panic();

    hr = device.IDirect3DDevice9_DrawPrimitiveUP(.TRIANGLESTRIP, //
        2, &vertices, @sizeOf(CustomVertex));
    if (failed(hr)) win32Panic();

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

![纹理采样][1]。

[1]: images/directx63.webp

## 附录
