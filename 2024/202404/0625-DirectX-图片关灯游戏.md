# 0625-DirectX-图片关灯游戏

## 目标

之前实现了一个关灯游戏，使用的是简单的矩形，现在加上图片。

## 环境

- Time 2024-07-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

窗口上下的黑边对鼠标点击还是有很大的影响，有时候点击不到目标。
还是不知道怎么解决这个问题，先记录这里吧。

## win.zig

增加了鼠标相关的信息。

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
pub var hover: ?Point = null;

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

        // mouse has been moved
        ui.WM_MOUSEMOVE => {
            // grab mouse coordinates
            const x: u32 = @intCast(lParam & 0xffff);
            const y: u32 = @intCast((lParam >> 16) & 0xffff);
            hover = if (x < WIDTH and y < HEIGHT)
                Point{ .x = x, .y = y }
            else
                null;
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

var surfaces: [4]*d3d9.IDirect3DSurface9 = undefined;
var dest: [10][7]win32.foundation.POINT = undefined;
var src: win32.foundation.RECT = undefined;
var board: [10][7]bool = undefined;

const imageWidth: u32 = 64;
const imageHeight: u32 = 64;

const Cell = struct {
    rect: d3d9.D3DRECT,
    light: bool,
};

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

    // load the image surfaces
    const f = render.mode.Format;
    surfaces[0] = lib.loadSourface(allocator, device, f, "tails.bmp");
    surfaces[1] = lib.loadSourface(allocator, device, f, "heads.bmp");
    surfaces[2] = lib.loadSourface(allocator, device, f, "tailsnegative.bmp");
    surfaces[3] = lib.loadSourface(allocator, device, f, "headsnegative.bmp");

    // set up points
    for (0..dest.len) |x| {
        for (0..dest[0].len) |y| {
            const tx = win.WIDTH / 2 - dest.len * imageWidth / 2;
            const ty = win.HEIGHT / 2 - dest[0].len * imageHeight / 2;

            dest[x][y].x = @intCast(imageWidth * x + tx);
            dest[x][y].y = @intCast(imageHeight * y + ty);
        }
    }

    // set up source rectangle
    src = std.mem.zeroes(win32.foundation.RECT);
    src.right = imageWidth;
    src.bottom = imageHeight;

    // clear the board
    board = std.mem.zeroes(@TypeOf(board));

    for (0..10) |_| {
        const x = win.rand.uintLessThanBiased(u32, 10);
        const y = win.rand.uintLessThanBiased(u32, 7);
        makeMove(x, y);
    }

    std.log.debug("board: {any}", .{board});
}

fn gameUpdate() void {
    if (win.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var r = device.IDirect3DDevice9_Clear(0, null, flags, 0xff000000, 0, 0);
    if (failed(r)) win32Panic();

    // grab back buffer
    var back: ?*d3d9.IDirect3DSurface9 = undefined;
    var hr = device.IDirect3DDevice9_GetBackBuffer(0, 0, .MONO, &back);
    if (failed(hr)) win32Panic();

    for (0..dest[0].len) |y| {
        for (0..dest.len) |x| {
            // check board
            var image: u8 = if (board[x][y]) 1 else 0;

            // check for hovering
            if (win.point) |point| {
                const mx = point.x / imageWidth;
                const my = point.y / imageHeight;
                if (mx == x and my == y) {
                    makeMove(@intCast(mx), @intCast(my));
                    win.point = null;
                }
            }

            // check for hovering
            if (win.hover) |hover| {
                if (hover.x / imageWidth == x and hover.y / imageHeight == y)
                    image += 2;
            }

            // copy rectangle
            hr = device.IDirect3DDevice9_UpdateSurface(surfaces[image], &src, back, &dest[x][y]);
            if (failed(hr)) win32Panic();
        }
    }

    _ = back.?.IUnknown_Release();
    hr = device.IDirect3DDevice9_Present(null, null, null, null);
    if (failed(hr)) win32Panic();

    r = device.IDirect3DDevice9_Present(null, null, null, null);
    if (failed(r)) win32Panic();

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
}

fn win32Panic() noreturn {
    win.win32Panic();
}

fn makeMove(x: u32, y: u32) void {
    std.log.debug("make move {d},{d}", .{ x, y });
    // change square x,y
    board[x][y] = !board[x][y];

    // change square x+1,y
    if (x + 1 < dest.len) board[x + 1][y] = !board[x + 1][y];

    // change square x-1,y
    board[x -| 1][y] = !board[x -| 1][y];

    // change square x,y+1
    if (y + 1 < dest[0].len) board[x][y + 1] = !board[x][y + 1];

    // change square x,y-1
    board[x][y -| 1] = !board[x][y -| 1];
}
```

## 效果

![图片关灯游戏][1]

[1]: images/directx59.webp

## 附录
