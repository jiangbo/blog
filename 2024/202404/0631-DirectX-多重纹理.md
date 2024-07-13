# 0631-DirectX-多重纹理

## 目标

根据不同的按键，设置两个纹理不同的混合方式。

## 环境

- Time 2024-07-13
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

两个纹理的颜色的混合方式，根据按键 0 到 9 设置成不同的模式，便于观察。

## win.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const ui = win32.ui.windows_and_messaging;
const gdi = win32.graphics.gdi;
const WINAPI = std.os.windows.WINAPI;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

pub var instance: std.os.windows.HINSTANCE = undefined;
pub var hander: win32.foundation.HWND = undefined;
pub var rand: std.Random = undefined;
pub var windowClosed: bool = false;

pub const Point = struct { x: u32, y: u32 };
pub var point: ?Point = null;
pub var key: ?usize = null;

pub fn mainWindowCallback(
    window: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(WINAPI) win32.foundation.LRESULT {
    switch (message) {
        ui.WM_CREATE => {
            std.log.info("WM_CREATE", .{});
        },
        ui.WM_LBUTTONDOWN => {
            const x: u32 = @intCast(lParam & 0xffff);
            const y: u32 = @intCast((lParam >> 16) & 0xffff);
            point = Point{ .x = x, .y = y };
        },

        ui.WM_KEYDOWN => {
            switch (wParam) {
                '0'...'9' => key = wParam,
                else => {},
            }
        },
        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
            windowClosed = true;
            ui.PostQuitMessage(0);
        },
        else => return ui.DefWindowProc(window, message, wParam, lParam),
    }
    return 0;
}

const name = win32.zig.L("Direct3D 中的 2D 编程");

pub fn createWindow() void {
    std.log.info("wWinMain", .{});

    const h = win32.system.library_loader.GetModuleHandle(null).?;
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    const s = .{ .HREDRAW = 1, .VREDRAW = 1 };

    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = s;
    windowClass.lpszClassName = name;
    windowClass.lpfnWndProc = mainWindowCallback;
    windowClass.hInstance = h;
    windowClass.hbrBackground = gdi.GetStockObject(gdi.BLACK_BRUSH);

    if (ui.RegisterClassEx(&windowClass) == 0) win32Panic();

    var style = ui.WS_OVERLAPPEDWINDOW;
    var rect = std.mem.zeroInit(win32.foundation.RECT, //
        .{ .right = WIDTH, .bottom = HEIGHT });
    _ = ui.AdjustWindowRectEx(&rect, style, 0, ui.WS_EX_LEFT);
    const width = rect.right - rect.left;
    const height = rect.bottom - rect.top;

    style.VISIBLE = 1;
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, name, name, style, //
        200, 200, width, height, null, null, h, null);

    instance = h;
    hander = window orelse win32Panic();

    const system = win32.system.system_information;
    var prng = std.rand.DefaultPrng.init(system.GetTickCount64());
    rand = prng.random();
}

pub fn update(gameUpdate: fn () void) void {
    var message: ui.MSG = undefined;
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }

        gameUpdate();
    }

    std.log.info("wWinMain end", .{});
}

pub fn win32Panic() noreturn {
    const err = win32.foundation.GetLastError();
    std.log.err("win32 panic code {}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

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
    vertices[0] = .{};
    vertices[1] = .{ .u = 1, .u2 = 1 };
    vertices[2] = .{ .v = 1, .v2 = 1 };
    vertices[3] = .{ .u = 1, .v = 1, .u2 = 1, .v2 = 1 };

    // load texture image
    const f = render.mode.Format;
    var surface = lib.loadSourface(allocator, device, f, "texture.bmp");
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

    // set the texture
    hr = device.IDirect3DDevice9_SetTexture(0, @ptrCast(texture));
    if (failed(hr)) win32Panic();

    surface = lib.loadSourface(allocator, device, f, "lightmap.bmp");
    defer _ = surface.IUnknown_Release();

    hr = device.IDirect3DDevice9_CreateTexture(tSize, tSize, 1, 0, //
        f, .DEFAULT, @ptrCast(&texture), null);
    if (failed(hr)) win32Panic();

    hr = texture.IDirect3DTexture9_GetSurfaceLevel(0, &dest);
    if (failed(hr)) win32Panic();
    defer _ = dest.?.IUnknown_Release();
    // copy image to texure

    // source and destionation
    src = std.mem.zeroes(win32.foundation.RECT);
    src.right = tSize;
    src.bottom = tSize;

    hr = device.IDirect3DDevice9_UpdateSurface(surface, &src, dest, &point);
    if (failed(hr)) win32Panic();

    // set the texture
    hr = device.IDirect3DDevice9_SetTexture(1, @ptrCast(texture));
    if (failed(hr)) win32Panic();
}

const CustomVertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    rhw: f32 = 1,
    diffuse: u32 = 0xffffffff,
    u: f32 = 0,
    v: f32 = 0,
    u2: f32 = 0,
    v2: f32 = 0,
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

    changeStageState();

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

fn changeStageState() void {
    if (win.key == null) return;

    std.log.info("click {}", .{win.key.?});
    const value = switch (win.key.?) {
        '1' => d3d9.D3DTOP_DISABLE,
        '2' => d3d9.D3DTOP_SELECTARG1,
        '3' => d3d9.D3DTOP_SELECTARG2,
        '4' => d3d9.D3DTOP_MODULATE,
        '5' => d3d9.D3DTOP_MODULATE2X,
        '6' => d3d9.D3DTOP_MODULATE4X,
        '7' => d3d9.D3DTOP_ADD,
        '8' => d3d9.D3DTOP_ADDSIGNED,
        '9' => d3d9.D3DTOP_ADDSIGNED2X,
        '0' => d3d9.D3DTOP_SUBTRACT,
        else => return,
    };

    const v: u32 = @intCast(@intFromEnum(value));
    const hr = device.IDirect3DDevice9_SetTextureStageState(1, .COLOROP, v);
    if (failed(hr)) win32Panic();

    win.key = null;
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
}

fn win32Panic() noreturn {
    win.win32Panic();
}
```

## 效果

![纹理拉伸][1]。

[1]: images/directx65.webp

## 附录
