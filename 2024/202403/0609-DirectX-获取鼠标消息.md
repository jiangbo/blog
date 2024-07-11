# 0609-DirectX-获取鼠标消息

## 目标

获取鼠标左键点击的消息，并且记录点击的位置。

## 环境

- Time 2024-07-11
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

获取得到的鼠标坐标，感觉不准，应该是窗体的坐标也算上了，也就这样，后面看能否解决这个问题。
当点击左下角和右下角的时候，看到 y 坐标超过了 480。

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
    style.VISIBLE = 1;
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, name, name, style, 0, 0, //
        @intCast(WIDTH), @intCast(HEIGHT), null, null, h, null);

    var rect = std.mem.zeroInit(win32.foundation.RECT, //
        .{ .right = WIDTH, .bottom = HEIGHT });
    _ = ui.AdjustWindowRectEx(&rect, style, 1, ui.WS_EX_LEFT);

    const width = rect.right - rect.left;
    const height = rect.bottom - rect.top;
    _ = ui.MoveWindow(window, 200, 200, width, height, 1);

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

var map: [10][10]Cell = undefined;

const cellWidth: u32 = zigwin.WIDTH / map.len;
const cellHeight: u32 = zigwin.HEIGHT / map.len;

const Cell = struct {
    rect: d3d9.D3DRECT,
    light: bool,
};

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
    var r = d9.IDirect3D9_GetAdapterDisplayMode(adapter, &mode);
    if (failed(r)) win32Panic();

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
    r = d9.IDirect3D9_CreateDevice(adapter, .HAL, zigwin.hander, //
        d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device));
    if (failed(r)) win32Panic();

    for (0..map.len) |x| {
        for (0..map.len) |y| {
            map[x][y].rect.x1 = @intCast(cellWidth * x + 1);
            map[x][y].rect.y1 = @intCast(cellHeight * y + 1);
            map[x][y].rect.x2 = map[x][y].rect.x1 + cellWidth - 2;
            map[x][y].rect.y2 = map[x][y].rect.y1 + cellHeight - 2;
            map[x][y].light = false;
        }
    }

    for (0..10) |_| {
        const x = zigwin.rand.uintLessThan(usize, map.len);
        const y = zigwin.rand.uintLessThan(usize, map.len);
        makeMove(x, y);
    }
}

fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var r = device.IDirect3DDevice9_Clear(0, null, flags, 0xff808080, 0, 0);
    if (failed(r)) win32Panic();

    // clear the cells
    for (0..map.len) |x| {
        for (0..map.len) |y| {
            const item = map[x][y];
            const color: u32 = if (item.light) 0xffffff00 else 0xff0000ff;

            // clear the viewport
            r = device.IDirect3DDevice9_Clear(1, &item.rect, flags, color, 0, 0);
            if (failed(r)) win32Panic();
        }
    }

    if (zigwin.point) |point| {
        std.log.debug("mouse click: {},{}", .{ point.x, point.y });
        zigwin.point = null;
    }

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

fn makeMove(x: usize, y: usize) void {
    // toggle center cell
    map[x][y].light = !map[x][y].light;

    // toggle cell to left
    if (x > 0) map[x - 1][y].light = !map[x - 1][y].light;

    // toggle cell to right
    if (x < map.len - 1) map[x + 1][y].light = !map[x + 1][y].light;

    // toggle cell above
    if (y > 0) map[x][y - 1].light = !map[x][y - 1].light;

    // toggle cell below
    if (y < map.len - 1) map[x][y + 1].light = !map[x][y + 1].light;
}
```

## 效果

![鼠标消息][1]。

[1]: images/directx43.png

## 附录
