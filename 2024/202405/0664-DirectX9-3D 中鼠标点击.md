# 0664-DirectX9-3D 中鼠标点击

## 目标

在 3D 的场景中，处理鼠标点击和物体的碰撞检测。

## 环境

- Time 2024-09-01
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

之前感觉碰撞还挺简单的，其实是在 2D 的情况下，如果是 3D，确实比较复杂。粒子效果感觉也挺复杂，放到后面去。

## d3d.zig

主要监听了鼠标左键的点击事件。

```zig
const std = @import("std");
const win32 = @import("win32");

const gdi = win32.graphics.gdi;
const ui = win32.ui.windows_and_messaging;
const d3d9 = win32.graphics.direct3d9;
const WINAPI = std.os.windows.WINAPI;
const failed = win32.zig.FAILED;

pub var point: ?isize = null;

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
        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
            ui.PostQuitMessage(0);
        },
        ui.WM_LBUTTONDOWN => point = lParam,
        else => return ui.DefWindowProc(window, message, wParam, lParam),
    }
    return 0;
}

const name = win32.zig.L("DirectX 9.0 3D游戏开发编程基础");

pub fn initD3D(width: i32, height: i32) *d3d9.IDirect3DDevice9 {
    //
    // Create the main application window.
    //
    var device: *d3d9.IDirect3DDevice9 = undefined;
    const h = win32.system.library_loader.GetModuleHandle(null).?;
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);

    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1 };
    windowClass.lpszClassName = name;
    windowClass.lpfnWndProc = mainWindowCallback;
    windowClass.hInstance = h;
    windowClass.hbrBackground = gdi.GetStockObject(gdi.BLACK_BRUSH);

    if (ui.RegisterClassEx(&windowClass) == 0) win32Panic();
    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, name, name, style, //
        200, 200, width, height, null, null, h, null).?;

    // Init D3D:
    // Step 1: Create the IDirect3D9 object.
    const d9 = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;

    // Step 2: Check for hardware vp.
    // Step 3: Fill out the D3DPRESENT_PARAMETERS structure.

    const adapter = d3d9.D3DADAPTER_DEFAULT;
    var mode: d3d9.D3DDISPLAYMODE = undefined;
    var hr = d9.IDirect3D9_GetAdapterDisplayMode(adapter, &mode);
    if (failed(hr)) win32Panic();

    var params: d3d9.D3DPRESENT_PARAMETERS = undefined;

    //back buffer information
    params.BackBufferWidth = @intCast(width);
    params.BackBufferHeight = @intCast(height);
    params.BackBufferFormat = mode.Format;
    params.BackBufferCount = 1; //make one back buffer

    //multisampling
    params.MultiSampleType = .NONE;
    params.MultiSampleQuality = 0;

    //swap effect
    params.SwapEffect = .DISCARD;
    params.Windowed = win32.zig.TRUE; //windowed mode

    //destination window
    params.hDeviceWindow = window;

    //depth buffer information
    params.EnableAutoDepthStencil = win32.zig.TRUE;
    params.AutoDepthStencilFormat = .D24S8;

    //flags
    params.Flags = 0;

    //refresh rate and presentation interval
    params.FullScreen_RefreshRateInHz = d3d9.D3DPRESENT_RATE_DEFAULT;
    params.PresentationInterval = d3d9.D3DPRESENT_INTERVAL_DEFAULT;

    //attempt to create a HAL device
    hr = d9.IDirect3D9_CreateDevice(adapter, .HAL, window, //
        d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device));
    if (failed(hr)) win32Panic();

    _ = d9.IUnknown_Release(); // done with d3d9 object
    return device;
}

var lastTime: u64 = 0;

pub fn enterMsgLoop(display: fn (f32) bool) void {
    var timer = std.time.Timer.start() catch unreachable;
    var message: ui.MSG = std.mem.zeroes(ui.MSG);
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        } else {
            const delta: f32 = @floatFromInt(timer.lap());
            _ = display(delta / std.time.ns_per_s);
        }
    }
}

pub const Material = struct {
    pub const WHITE = .{ .r = 1, .g = 1, .b = 1, .a = 1.0 };
    pub const BLACK = .{ .r = 0, .g = 0, .b = 0, .a = 1.0 };
    pub const RED = .{ .r = 1, .g = 0, .b = 0, .a = 1.0 };
    pub const YELLOW = .{ .r = 1, .g = 1, .b = 0, .a = 1.0 };

    pub const white = init(WHITE, WHITE, WHITE, BLACK, 2.0);
    pub const black = init(BLACK, BLACK, BLACK, BLACK, 2.0);
    pub const red = init(RED, RED, RED, BLACK, 2.0);
    pub const yellow = init(YELLOW, YELLOW, YELLOW, BLACK, 2.0);

    const CV = d3d9.D3DCOLORVALUE;
    fn init(a: CV, d: CV, s: CV, e: CV, p: f32) d3d9.D3DMATERIAL9 {
        return d3d9.D3DMATERIAL9{
            .Ambient = a,
            .Diffuse = d,
            .Specular = s,
            .Emissive = e,
            .Power = p,
        };
    }
};

pub fn win32Panic() noreturn {
    const err = win32.foundation.GetLastError();
    std.log.err("win32 panic code {}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## d3dx9.zig

增加了很多 D3DX 的函数。

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d9 = win32.graphics.direct3d9;

pub const LPCTSTR = [*:0]align(1) const u16;

pub extern fn D3DXMatrixPerspectiveFovLH(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    fovy: f32,
    aspect: f32,
    zn: f32,
    zf: f32,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixLookAtLH(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    eye: *const Vec3,
    at: *const Vec3,
    up: *const Vec3,
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

pub extern fn D3DXMatrixMultiply(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    m1: *const win32.graphics.direct3d.D3DMATRIX,
    m2: *const win32.graphics.direct3d.D3DMATRIX,
) *win32.graphics.direct3d.D3DMATRIX;

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

pub extern fn D3DXMatrixReflect(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    plane: *const Vec4,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXMatrixShadow(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    light: *const Vec4,
    plane: *const Vec4,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXCreateTextureFromFileW(
    device: *d3d9.IDirect3DDevice9,
    name: LPCTSTR,
    LPDIRECT3DTEXTURE9: ?**d3d9.IDirect3DTexture9,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub extern fn D3DXCreateTeapot(
    device: *d3d9.IDirect3DDevice9,
    mesh: **ID3DXMesh,
    buffer: ?**ID3DXBuffer,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

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

pub extern fn D3DXComputeBoundingSphere(
    firstPosition: *const Vec3,
    numVertices: u32,
    stride: u32,
    center: *Vec3,
    radius: *f32,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub extern fn D3DXGetFVFVertexSize(fvf: u32) callconv(std.os.windows.WINAPI) u32;

pub extern fn D3DXCreateSphere(
    device: *d3d9.IDirect3DDevice9,
    radius: f32,
    slices: u32,
    stacks: u32,
    mesh: **ID3DXMesh,
    adjacency: ?**ID3DXBuffer,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub extern fn D3DXMatrixInverse(
    out: *win32.graphics.direct3d.D3DMATRIX,
    determinant: ?*f32,
    matrix: *const win32.graphics.direct3d.D3DMATRIX,
) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXVec3TransformCoord(
    out: *Vec3,
    v: *const Vec3,
    matrix: *const win32.graphics.direct3d.D3DMATRIX,
) *Vec3;

pub extern fn D3DXVec3TransformNormal(
    out: *Vec3,
    v: *const Vec3,
    matrix: *const win32.graphics.direct3d.D3DMATRIX,
) *Vec3;

pub const Vec4 = extern struct { x: f32, y: f32, z: f32, w: f32 };
pub const Vec3 = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,

    pub fn add(self: Vec3, other: Vec3) Vec3 {
        return Vec3{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
        };
    }

    pub fn sub(self: Vec3, other: Vec3) Vec3 {
        return Vec3{
            .x = self.x - other.x,
            .y = self.y - other.y,
            .z = self.z - other.z,
        };
    }

    pub fn dot(self: Vec3, other: Vec3) f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn normalize(self: Vec3) Vec3 {
        const len = @sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        return Vec3{
            .x = self.x / len,
            .y = self.y / len,
            .z = self.z / len,
        };
    }
};

pub const ID3DXMesh = extern struct {
    pub const VTable = extern struct {
        base: ID3DXBaseMesh.VTable,
    };
    vtable: *const VTable,
    pub fn MethodMixin(comptime T: type) type {
        return struct {
            pub inline fn ID3DXBaseMesh_DrawSubset(self: *const T, attribId: u32) i32 {
                return @as(*const ID3DXBaseMesh.VTable, @ptrCast(self.vtable)).DrawSubset(@as(*const ID3DXBaseMesh, @ptrCast(self)), attribId);
            }
            pub inline fn ID3DXBaseMesh_LockVertexBuffer(
                self: *const T,
                flags: u32,
                data: *anyopaque,
            ) i32 {
                return @as(*const ID3DXBaseMesh.VTable, @ptrCast(self.vtable)).LockVertexBuffer(@as(*const ID3DXBaseMesh, @ptrCast(self)), flags, data);
            }
            pub inline fn ID3DXBaseMesh_GetNumVertices(self: *const T) u32 {
                return @as(*const ID3DXBaseMesh.VTable, @ptrCast(self.vtable)).GetNumVertices(@as(*const ID3DXBaseMesh, @ptrCast(self)));
            }
            pub inline fn ID3DXBaseMesh_GetFVF(self: *const T) u32 {
                return @as(*const ID3DXBaseMesh.VTable, @ptrCast(self.vtable)).GetFVF(@as(*const ID3DXBaseMesh, @ptrCast(self)));
            }
            pub inline fn ID3DXBaseMesh_UnlockVertexBuffer(self: *const T) i32 {
                return @as(*const ID3DXBaseMesh.VTable, @ptrCast(self.vtable)).UnlockVertexBuffer(@as(*const ID3DXBaseMesh, @ptrCast(self)));
            }
        };
    }
    pub usingnamespace MethodMixin(@This());
};

pub const ID3DXBaseMesh = extern struct {
    pub const VTable = extern struct {
        base: win32.system.com.IUnknown.VTable,
        DrawSubset: *const fn (
            self: *const ID3DXBaseMesh,
            attribId: u32,
        ) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,
        GetNumFaces: usize,
        GetNumVertices: *const fn (
            self: *const ID3DXBaseMesh,
        ) callconv(std.os.windows.WINAPI) u32,
        GetFVF: *const fn (
            self: *const ID3DXBaseMesh,
        ) callconv(std.os.windows.WINAPI) u32,
        _: [8]usize,
        LockVertexBuffer: *const fn (
            self: *const ID3DXBaseMesh,
            flags: u32,
            data: *anyopaque,
        ) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,
        UnlockVertexBuffer: *const fn (
            self: *const ID3DXBaseMesh,
        ) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,
    };
    vtable: *const VTable,
};

pub const ID3DXBuffer = extern struct {
    pub const VTable = extern struct {
        base: win32.system.com.IUnknown.VTable,
    };
    vtable: *const VTable,
};
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

var teapot: *d3dx9.ID3DXMesh = undefined;
var sphere: *d3dx9.ID3DXMesh = undefined;
var BSphere: BoundingSphere = undefined;

const BoundingSphere = struct {
    center: d3dx9.Vec3,
    radius: f32,
};

const Ray = struct {
    origin: d3dx9.Vec3,
    direction: d3dx9.Vec3,
};

// Framework Functions
fn setup() bool {

    // 创建茶壶
    _ = d3dx9.D3DXCreateTeapot(device, &teapot, null);

    //
    // Compute the bounding sphere.
    //
    var buffer: *d3dx9.Vec3 = undefined;
    _ = teapot.ID3DXBaseMesh_LockVertexBuffer(0, @ptrCast(&buffer));

    _ = d3dx9.D3DXComputeBoundingSphere(
        buffer,
        teapot.ID3DXBaseMesh_GetNumVertices(),
        d3dx9.D3DXGetFVFVertexSize(teapot.ID3DXBaseMesh_GetFVF()),
        &BSphere.center,
        &BSphere.radius,
    );

    _ = teapot.ID3DXBaseMesh_UnlockVertexBuffer();

    //
    // Build a sphere mesh that describes the teapot's bounding sphere.
    //
    _ = d3dx9.D3DXCreateSphere(device, BSphere.radius, 20, 20, &sphere, null);

    // 设置方向光
    var light = std.mem.zeroes(d3d9.D3DLIGHT9);
    light.Type = d3d9.D3DLIGHT_DIRECTIONAL;
    light.Ambient = .{ .r = 0.4, .g = 0.4, .b = 0.4, .a = 0.4 };
    light.Diffuse = .{ .r = 1, .g = 1, .b = 1, .a = 1 };
    light.Specular = .{ .r = 0.6, .g = 0.6, .b = 0.6, .a = 0.6 };
    light.Direction = .{ .x = 0.707, .y = -0.707, .z = 0.707 };
    _ = device.IDirect3DDevice9_SetLight(0, &light);
    _ = device.IDirect3DDevice9_LightEnable(0, 1);

    // 打开镜面光
    _ = device.IDirect3DDevice9_SetRenderState(.NORMALIZENORMALS, 1);
    _ = device.IDirect3DDevice9_SetRenderState(.SPECULARENABLE, 0);

    // 设置视图矩阵
    const position = .{ .z = -10 };
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

var r: f32 = 0.0;
var v: f32 = 1.0;
var angle: f32 = 0.0;
var world: win32.graphics.direct3d.D3DMATRIX = undefined;

fn display(delta: f32) bool {
    if (d3d.point) |point| {
        const x: f32 = @floatFromInt(point & 0xffff);
        const y: f32 = @floatFromInt((point >> 16) & 0xffff);
        d3d.point = null;

        // compute the ray in view space given the clicked screen point
        var ray = calcPickingRay(x, y);

        // transform the ray to world space
        var view: win32.graphics.direct3d.D3DMATRIX = undefined;
        _ = device.IDirect3DDevice9_GetTransform(.VIEW, &view);

        var viewInverse: win32.graphics.direct3d.D3DMATRIX = undefined;
        _ = d3dx9.D3DXMatrixInverse(&viewInverse, null, &view);

        transformRay(&ray, &viewInverse);

        // test for a hit
        if (raySphereIntTest(&ray, &BSphere))
            std.log.debug("Hit", .{});
    }

    //
    // Update: Update Teapot.
    //
    _ = d3dx9.D3DXMatrixTranslation(&world, @cos(angle) * r, @sin(angle) * r, 10.0);

    // transfrom the bounding sphere to match the teapots position in the
    // world.
    BSphere.center = .{ .x = @cos(angle) * r, .y = @sin(angle) * r, .z = 10 };

    r += v * delta;

    if (r >= 8.0)
        v = -v; // reverse direction

    if (r <= 0.0)
        v = -v; // reverse direction

    angle += 1.0 * std.math.pi * delta;
    if (angle >= std.math.pi * 2.0)
        angle = 0.0;

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    // draw teapot
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    _ = device.IDirect3DDevice9_SetMaterial(&d3d.Material.yellow);
    _ = teapot.ID3DXBaseMesh_DrawSubset(0);

    // Render the bounding sphere with alpha blending so we can see
    // through it.
    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, 1);
    var state = @intFromEnum(d3d9.D3DBLEND_SRCALPHA);
    _ = device.IDirect3DDevice9_SetRenderState(.SRCBLEND, state);
    state = @intFromEnum(d3d9.D3DBLEND_INVSRCALPHA);
    _ = device.IDirect3DDevice9_SetRenderState(.DESTBLEND, state);

    var red = d3d.Material.black;
    red.Diffuse.a = 0.25; // 25% opacity
    _ = device.IDirect3DDevice9_SetMaterial(&red);
    _ = sphere.ID3DXBaseMesh_DrawSubset(0);

    _ = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, 0);

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

fn calcPickingRay(x: f32, y: f32) Ray {
    var px: f32 = 0.0;
    var py: f32 = 0.0;

    var vp: d3d9.D3DVIEWPORT9 = undefined;
    _ = device.IDirect3DDevice9_GetViewport(&vp);

    var proj: win32.graphics.direct3d.D3DMATRIX = undefined;
    _ = device.IDirect3DDevice9_GetTransform(.PROJECTION, &proj);

    const w: f32 = @as(f32, @floatFromInt(vp.Width));
    const h: f32 = @as(f32, @floatFromInt(vp.Height));
    px = (((2.0 * x) / w) - 1.0) / proj.Anonymous.Anonymous._11;
    py = (((-2.0 * y) / h) + 1.0) / proj.Anonymous.Anonymous._22;

    return Ray{
        .origin = .{ .x = 0, .y = 0, .z = 0 },
        .direction = .{ .x = px, .y = py, .z = 1.0 },
    };
}

fn transformRay(ray: *Ray, T: *win32.graphics.direct3d.D3DMATRIX) void {
    // transform the ray's origin, w = 1.

    _ = d3dx9.D3DXVec3TransformCoord(&ray.origin, &ray.origin, T);

    // transform the ray's direction, w = 0.
    _ = d3dx9.D3DXVec3TransformNormal(&ray.direction, &ray.direction, T);

    // normalize the direction
    ray.direction = ray.direction.normalize();
}

fn raySphereIntTest(ray: *Ray, s: *BoundingSphere) bool {
    const t = ray.origin.sub(s.center);
    const b = 2.0 * ray.direction.dot(t);

    const c = t.dot(t) - (s.radius * s.radius);

    // find the discriminant
    var discriminant = (b * b) - (4.0 * c);

    // test for imaginary number
    if (discriminant < 0.0)
        return false;

    discriminant = @sqrt(discriminant);

    const s0 = (-b + discriminant) / 2.0;
    const s1 = (-b - discriminant) / 2.0;

    // if a solution is >= 0, then we intersected the sphere
    if (s0 >= 0.0 or s1 >= 0.0)
        return true;

    return false;
}
```

## 效果

好像录制没有鼠标，不太看得出效果。

![3D 中鼠标点击][1]。

[1]: images/directx014.webp

## 附录
