# 0620-DirectX-缓冲三角形扇

## 目标

将之前未使用缓冲的三角形扇图元修改成使用缓冲的形式。

## 环境

- Time 2024-07-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

之前了解过 OpenGL，这里的概念还是挺清楚的，buffer 应该就是一块显存，然后将内存的数据拷贝过去。

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
var buffer: *d3d9.IDirect3DVertexBuffer9 = undefined;

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
    hr = device.IDirect3DDevice9_CreateVertexBuffer(@sizeOf(@TypeOf(vertices)), //
        0, xyzrhw | diffuse, .DEFAULT, @ptrCast(&buffer), null);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetStreamSource(0, buffer, //
        0, @sizeOf(CustomVertex));
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetFVF(xyzrhw | diffuse);
    if (failed(hr)) win32Panic();

    hr = device.IDirect3DDevice9_SetRenderState(.LIGHTING, 0);
    if (failed(hr)) win32Panic();

    // set up vertex colors
    vertices[0] = .{ .diffuse = 0xffff0000 };
    vertices[1] = .{ .diffuse = 0xff00ff00 };
    vertices[2] = .{ .diffuse = 0xff0000ff };
    vertices[3] = .{ .diffuse = 0xff00ff00 };
    vertices[4] = .{ .diffuse = 0xff0000ff };
    vertices[5] = .{ .diffuse = 0xff00ff00 };
    vertices[6] = .{ .diffuse = 0xff0000ff };
    vertices[7] = .{ .diffuse = 0xff00ff00 };
    vertices[8] = .{ .diffuse = 0xff0000ff };
    vertices[9] = .{ .diffuse = 0xff00ff00 };
    vertices[10] = .{ .diffuse = 0xff0000ff };
    vertices[11] = .{ .diffuse = 0xff00ff00 };
}

const PI: f32 = std.math.pi;
const RADIUS: f32 = 200.0;

var vertices: [12]CustomVertex = undefined;
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

    // set up vertices
    // vertex 0
    vertices[0].x = width / 2;
    vertices[0].y = height / 2;
    // vertex 1
    vertices[1].x = width / 2 + @cos(angle) * RADIUS;
    vertices[1].y = height / 2 + @sin(angle) * RADIUS;
    // vertex 2
    vertices[2].x = width / 2 + @cos(angle + 2.0 * PI / 10.0) * RADIUS / 2.0;
    vertices[2].y = height / 2 + @sin(angle + 2.0 * PI / 10.0) * RADIUS / 2.0;
    // vertex 3
    vertices[3].x = width / 2 + @cos(angle + 2.0 * 2.0 * PI / 10.0) * RADIUS;
    vertices[3].y = height / 2 + @sin(angle + 2.0 * 2.0 * PI / 10.0) * RADIUS;
    // vertex 4
    vertices[4].x = width / 2 + @cos(angle + 3.0 * 2.0 * PI / 10.0) * RADIUS / 2.0;
    vertices[4].y = height / 2 + @sin(angle + 3.0 * 2.0 * PI / 10.0) * RADIUS / 2.0;
    // vertex 5
    vertices[5].x = width / 2 + @cos(angle + 4.0 * 2.0 * PI / 10.0) * RADIUS;
    vertices[5].y = height / 2 + @sin(angle + 4.0 * 2.0 * PI / 10.0) * RADIUS;
    // vertex 6
    vertices[6].x = width / 2 + @cos(angle + 5.0 * 2.0 * PI / 10.0) * RADIUS / 2.0;
    vertices[6].y = height / 2 + @sin(angle + 5.0 * 2.0 * PI / 10.0) * RADIUS / 2.0;
    // vertex 7
    vertices[7].x = width / 2 + @cos(angle + 6.0 * 2.0 * PI / 10.0) * RADIUS;
    vertices[7].y = height / 2 + @sin(angle + 6.0 * 2.0 * PI / 10.0) * RADIUS;
    // vertex 8
    vertices[8].x = width / 2 + @cos(angle + 7.0 * 2.0 * PI / 10.0) * RADIUS / 2.0;
    vertices[8].y = height / 2 + @sin(angle + 7.0 * 2.0 * PI / 10.0) * RADIUS / 2.0;
    // vertex 9
    vertices[9].x = width / 2 + @cos(angle + 8.0 * 2.0 * PI / 10.0) * RADIUS;
    vertices[9].y = height / 2 + @sin(angle + 8.0 * 2.0 * PI / 10.0) * RADIUS;
    // vertex 10
    vertices[10].x = width / 2 + @cos(angle + 9.0 * 2.0 * PI / 10.0) * RADIUS / 2.0;
    vertices[10].y = height / 2 + @sin(angle + 9.0 * 2.0 * PI / 10.0) * RADIUS / 2.0;
    // vertex 11
    vertices[11].x = width / 2 + @cos(angle) * RADIUS;
    vertices[11].y = height / 2 + @sin(angle) * RADIUS;

    // increase angle for next time
    angle += (2.0 * PI / 360.0);

    // lock vertex buffer
    var data: [*]CustomVertex = undefined;
    var hr = buffer.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&data), 0);
    if (failed(hr)) win32Panic();

    // copy information to vertex buffer
    @memcpy(data[0..vertices.len], &vertices);

    // unlock vertex buffer
    hr = buffer.IDirect3DVertexBuffer9_Unlock();
    if (failed(hr)) win32Panic();

    // begin the scene
    if (failed(device.IDirect3DDevice9_BeginScene())) win32Panic();

    hr = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLEFAN, 0, 10);
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

![缓冲三角形扇][1]

[1]: images/directx55.webp

## 附录
