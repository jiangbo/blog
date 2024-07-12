# 0619-DirectX-小飞船控制

## 目标

在之前的星空背景图上，显示一个小飞船，并且可以通过键盘控制。

## 环境

- Time 2024-07-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

可以进行前进和后退，可以控制旋转，不可以开火。

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

pub var controls: [5]bool = .{false} ** 5;

pub const ControlEnum = enum {
    UP,
    LEFT,
    DOWN,
    RIGHT,
    FIRE,
    COUNT,
};

pub fn mainWindowCallback(
    window: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(WINAPI) win32.foundation.LRESULT {
    const keyboard = win32.ui.input.keyboard_and_mouse;
    switch (message) {
        ui.WM_CREATE => {
            std.log.info("WM_CREATE", .{});
        },
        ui.WM_LBUTTONDOWN => {
            const x: u32 = @intCast(lParam & 0xffff);
            const y: u32 = @intCast((lParam >> 16) & 0xffff);
            point = Point{ .x = x, .y = y };
        },
        // a key has been pressed
        ui.WM_KEYDOWN => {
            // which key was pressed
            const key: keyboard.VIRTUAL_KEY = @enumFromInt(wParam);
            const action: ?ControlEnum = switch (key) {
                keyboard.VK_LEFT => .LEFT,
                keyboard.VK_RIGHT => .RIGHT,
                keyboard.VK_UP => .UP,
                keyboard.VK_DOWN => .DOWN,
                keyboard.VK_SPACE => .FIRE,
                else => null,
            };
            if (action) |a| controls[@intFromEnum(a)] = true;
        },
        // which key was released
        ui.WM_KEYUP => {
            // which key was pressed
            const key: keyboard.VIRTUAL_KEY = @enumFromInt(wParam);
            const action: ?ControlEnum = switch (key) {
                keyboard.VK_LEFT => .LEFT,
                keyboard.VK_RIGHT => .RIGHT,
                keyboard.VK_UP => .UP,
                keyboard.VK_DOWN => .DOWN,
                keyboard.VK_SPACE => .FIRE,
                else => null,
            };
            if (action) |a| controls[@intFromEnum(a)] = false;
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
    const s = .{ .DBLCLKS = 1, .OWNDC = 1, .HREDRAW = 1, .VREDRAW = 1 };

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
    _ = ui.AdjustWindowRectEx(&rect, style, 1, ui.WS_EX_LEFT);
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
    std.log.err("win32 painc code {}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const zigwin = @import("win.zig");

const d3d9 = win32.graphics.direct3d9;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;
var d9: *d3d9.IDirect3D9 = undefined;
var device: *d3d9.IDirect3DDevice9 = undefined;

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    zigwin.createWindow();

    gameInit();
    zigwin.update(gameUpdate);
    gameShutdown();
}

const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    d9 = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;

    const count = d9.IDirect3D9_GetAdapterCount();
    std.log.debug("adapter count: {d}", .{count});

    var identifier: d3d9.D3DADAPTER_IDENTIFIER9 = undefined;

    for (0..count) |adapter| {
        const i: u32 = @intCast(adapter);
        const r = d9.IDirect3D9_GetAdapterIdentifier(i, 0, &identifier);
        if (failed(r)) win32Panic();

        std.log.debug("adapter Driver: {s}", .{identifier.Driver});
        std.log.debug("adapter name: {s}", .{identifier.Description});
    }

    const adapter = d3d9.D3DADAPTER_DEFAULT;
    var mode: d3d9.D3DDISPLAYMODE = undefined;
    var hr = d9.IDirect3D9_GetAdapterDisplayMode(adapter, &mode);
    if (failed(hr)) win32Panic();

    var params: d3d9.D3DPRESENT_PARAMETERS = undefined;

    //back buffer information
    params.BackBufferWidth = zigwin.WIDTH;
    params.BackBufferHeight = zigwin.HEIGHT;
    params.BackBufferFormat = mode.Format;
    params.BackBufferCount = 1; //make one back buffer

    //multisampling
    params.MultiSampleType = .NONE;
    params.MultiSampleQuality = 0;

    //swap effect
    params.SwapEffect = .COPY; //we want to copy from back buffer to screen
    params.Windowed = win32.zig.TRUE; //windowed mode

    //destination window
    params.hDeviceWindow = zigwin.hander;

    //depth buffer information
    params.EnableAutoDepthStencil = win32.zig.FALSE;
    params.AutoDepthStencilFormat = .UNKNOWN;

    //flags
    params.Flags = 0;

    //refresh rate and presentation interval
    params.FullScreen_RefreshRateInHz = d3d9.D3DPRESENT_RATE_DEFAULT;
    params.PresentationInterval = d3d9.D3DPRESENT_INTERVAL_DEFAULT;

    //attempt to create a HAL device
    hr = d9.IDirect3D9_CreateDevice(adapter, .HAL, zigwin.hander, //
        d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device));
    if (failed(hr)) win32Panic();

    const xyzrhw = win32.system.system_services.D3DFVF_XYZRHW;
    const diffuse = win32.system.system_services.D3DFVF_DIFFUSE;
    hr = device.IDirect3DDevice9_SetFVF(xyzrhw | diffuse);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetRenderState(.LIGHTING, 0);
    if (failed(hr)) win32Panic();

    // set up vertex colors
    vertices[0] = .{ .diffuse = 0xff00ffff };
    vertices[1] = .{ .diffuse = 0xff00ffff };
    vertices[2] = .{ .diffuse = 0xff00ffff };
    vertices[3] = .{ .diffuse = 0xff00ffff };

    // set up the stars
    for (&stars) |*star| {
        star.* = std.mem.zeroInit(CustomVertex, .{ .rhw = 1 });
        // random x,y position
        star.x = @floatFromInt(zigwin.rand.uintLessThan(u32, zigwin.WIDTH));
        star.y = @floatFromInt(zigwin.rand.uintLessThan(u32, zigwin.HEIGHT));
        // random gray color
        star.diffuse = zigwin.rand.uintAtMost(u32, std.math.maxInt(u24));
    }
}

const PI: f32 = std.math.pi;
const RADIUS: f32 = 200.0;

var vertices: [4]CustomVertex = undefined;
const indices: [6]u16 = .{ 0, 1, 2, 0, 3, 1 };
var stars: [1000]CustomVertex = undefined;
var angle: f32 = 0;

const CustomVertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    rhw: f32 = 1,
    diffuse: u32,
};

var gx: f32 = zigwin.WIDTH / 2;
var gy: f32 = zigwin.HEIGHT / 2;
var gvx: f32 = 0;
var gvy: f32 = 0;

fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var r = device.IDirect3DDevice9_Clear(0, null, flags, 0xff000000, 0, 0);
    if (failed(r)) win32Panic();

    const width: f32 = @floatFromInt(zigwin.WIDTH);
    const height: f32 = @floatFromInt(zigwin.HEIGHT);

    // update position
    gx += gvx;
    gy += gvy;

    // bounds checking
    while (gx < 0.0) gx += width;
    while (gy < 0.0) gy += height;
    while (gx >= width) gx -= width;
    while (gy >= height) gy -= height;

    // friction
    gvx *= (15.0 / 16.0);
    gvy *= (15.0 / 16.0);

    // thrust
    if (zigwin.controls[@intFromEnum(zigwin.ControlEnum.UP)]) {
        gvx += @cos(angle);
        gvy += @sin(angle);
    }

    // reverse thrust
    if (zigwin.controls[@intFromEnum(zigwin.ControlEnum.DOWN)]) {
        gvx += @cos(angle - PI);
        gvy += @sin(angle - PI);
    }

    // left turn
    if (zigwin.controls[@intFromEnum(zigwin.ControlEnum.LEFT)]) {
        angle -= (PI / 45.0);
    }

    // right turn
    if (zigwin.controls[@intFromEnum(zigwin.ControlEnum.RIGHT)]) {
        angle += (PI / 45.0);
    }

    // set up vertices
    // vertex 0
    vertices[1].x = gx + @cos(angle) * 20.0;
    vertices[1].y = gy + @sin(angle) * 20.0;
    // vertex 1
    vertices[2].x = gx + @cos(angle + 5.0 * PI / 6.0) * 15.0;
    vertices[2].y = gy + @sin(angle + 5.0 * PI / 6.0) * 15.0;
    // vertex 2
    vertices[3].x = gx + @cos(angle - 5.0 * PI / 6.0) * 15.0;
    vertices[3].y = gy + @sin(angle - 5.0 * PI / 6.0) * 15.0;
    // vertex 3
    vertices[0].x = gx + @cos(angle - PI) * 5.0;
    vertices[0].y = gy + @sin(angle - PI) * 5.0;

    // begin the scene
    if (failed(device.IDirect3DDevice9_BeginScene())) win32Panic();

    var hr = device.IDirect3DDevice9_DrawPrimitiveUP(.POINTLIST, //
        stars.len, &stars, @sizeOf(CustomVertex));
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_DrawIndexedPrimitiveUP(.TRIANGLELIST, 0, //
        indices.len, 3, &indices, .INDEX16, &vertices, @sizeOf(CustomVertex));
    if (failed(hr)) win32Panic();

    // end the scene
    if (failed(device.IDirect3DDevice9_EndScene())) win32Panic();

    r = device.IDirect3DDevice9_Present(null, null, null, null);
    if (failed(r)) win32Panic();

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = d9.IUnknown_Release();
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}
```

## 效果

![控制小飞船][1]。

[1]: images/directx54.webp

## 附录
