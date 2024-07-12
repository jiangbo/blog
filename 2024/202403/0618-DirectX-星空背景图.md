# 0618-DirectX-星空背景图

## 目标

综合使用图元绘制，画出一幅星空背景图。

## 环境

- Time 2024-07-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

我记得之前画过这个，就是在随机地点，画上随机的颜色。

## win.zig

无变化。

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

    const g_x: f32 = width / 2.0;
    const g_y: f32 = height / 2.0;

    // set up vertices
    // vertex 0
    vertices[1].x = g_x + @cos(angle) * 20.0;
    vertices[1].y = g_y + @sin(angle) * 20.0;
    // vertex 1
    vertices[2].x = g_x + @cos(angle + 5.0 * PI / 6.0) * 15.0;
    vertices[2].y = g_y + @sin(angle + 5.0 * PI / 6.0) * 15.0;
    // vertex 2
    vertices[3].x = g_x + @cos(angle - 5.0 * PI / 6.0) * 15.0;
    vertices[3].y = g_y + @sin(angle - 5.0 * PI / 6.0) * 15.0;
    // vertex 3
    vertices[0].x = g_x + @cos(angle - PI) * 5.0;
    vertices[0].y = g_y + @sin(angle - PI) * 5.0;

    // increase angle for next time
    angle += (2.0 * PI / 360.0);

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

![星空背景图][1]。

[1]: images/directx53.webp

## 附录
