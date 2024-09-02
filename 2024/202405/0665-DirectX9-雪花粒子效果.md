# 0665-DirectX9-雪花粒子效果

## 目标

实现雪花粒子效果。

## 环境

- Time 2024-09-02
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

颜色的类型搞错了，导致一直是红色的雪花，排查了好久才找到原因。

## d3d.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3dx9 = @import("d3dx9.zig");

const gdi = win32.graphics.gdi;
const ui = win32.ui.windows_and_messaging;
const d3d9 = win32.graphics.direct3d9;
const WINAPI = std.os.windows.WINAPI;
const failed = win32.zig.FAILED;

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

    const system = win32.system.system_information;
    var prng = std.rand.DefaultPrng.init(system.GetTickCount64());
    rand = prng.random();

    return device;
}

pub var rand: std.Random = undefined;

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
    pub const BLUE = .{ .r = 0, .g = 0, .b = 1, .a = 1.0 };

    pub const white = init(WHITE, WHITE, WHITE, BLACK, 2.0);
    pub const red = init(RED, RED, RED, BLACK, 2.0);
    pub const yellow = init(YELLOW, YELLOW, YELLOW, BLACK, 2.0);
    pub const blue = init(BLUE, BLUE, BLUE, BLACK, 2.0);

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

pub const Vertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    nx: f32 = 0,
    ny: f32 = 0,
    nz: f32 = 0,
    u: f32 = 0,
    v: f32 = 0,
};

// Bounding Objects
pub const BoundingBox = struct {
    min: d3dx9.Vec3,
    max: d3dx9.Vec3,

    pub fn isPointInside(self: BoundingBox, point: d3dx9.Vec3) bool {
        return (point.x >= self.min.x and point.x <= self.max.x and
            point.y >= self.min.y and point.y <= self.max.y and
            point.z >= self.min.z and point.z <= self.max.z);
    }
};

pub const BoundingSphere = struct {
    center: d3dx9.Vec3,
    radius: f32,
};

const fvf = win32.system.system_services.D3DFVF_XYZ | //
    win32.system.system_services.D3DFVF_NORMAL | //
    win32.system.system_services.D3DFVF_TEX1;

var floor: *d3d9.IDirect3DVertexBuffer9 = undefined;
var texture: *d3d9.IDirect3DTexture9 = undefined;
var pillar: *d3dx9.ID3DXMesh = undefined;
pub fn drawBasicScene(device: *d3d9.IDirect3DDevice9, scale: f32, preRender: bool) void {
    if (preRender) {
        // they don't exist, create them

        _ = device.IDirect3DDevice9_CreateVertexBuffer(6 * @sizeOf(Vertex), //
            0, fvf, .MANAGED, @ptrCast(&floor), null);

        // 填充顶点数据
        var v: [*]Vertex = undefined;
        _ = floor.IDirect3DVertexBuffer9_Lock(0, 0, @ptrCast(&v), 0);

        v[0] = .{ .x = -20, .y = -2.5, .z = -20, .ny = 1, .v = 1 };
        v[1] = .{ .x = -20, .y = -2.5, .z = 20, .ny = 1 };
        v[2] = .{ .x = 20, .y = -2.5, .z = 20, .ny = 1, .u = 1 };

        v[3] = .{ .x = -20, .y = -2.5, .z = -20, .ny = 1, .v = 1 };
        v[4] = .{ .x = 20, .y = -2.5, .z = 20, .ny = 1, .u = 1 };
        v[5] = .{ .x = 20, .y = -2.5, .z = -20, .ny = 1, .u = 1, .v = 1 };

        _ = floor.IDirect3DVertexBuffer9_Unlock();

        _ = d3dx9.D3DXCreateCylinder(device, 0.5, 0.5, 5.0, 20, 20, &pillar, null);

        const desert = win32.zig.L("desert.bmp");
        _ = d3dx9.D3DXCreateTextureFromFileW(device, desert, &texture);
    } else {
        //
        // Pre-Render Setup
        //
        _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(texture));
        var state: u32 = @intFromEnum(d3d9.D3DTEXF_LINEAR);
        _ = device.IDirect3DDevice9_SetSamplerState(0, .MAGFILTER, state);
        _ = device.IDirect3DDevice9_SetSamplerState(0, .MINFILTER, state);
        state = @intFromEnum(d3d9.D3DTEXF_POINT);
        _ = device.IDirect3DDevice9_SetSamplerState(0, .MIPFILTER, state);

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
        _ = device.IDirect3DDevice9_SetRenderState(.SPECULARENABLE, 1);

        //
        // Render
        //
        var t: win32.graphics.direct3d.D3DMATRIX = undefined;
        var r: win32.graphics.direct3d.D3DMATRIX = undefined;
        var p: win32.graphics.direct3d.D3DMATRIX = undefined;
        var s: win32.graphics.direct3d.D3DMATRIX = undefined;

        _ = d3dx9.D3DXMatrixScaling(&s, scale, scale, scale);

        // used to rotate cylinders to be parallel with world's y-axis
        _ = d3dx9.D3DXMatrixRotationX(&r, std.math.pi * 0.5);

        // draw floor
        const unit: [16]f32 = .{
            1, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 1,
        };
        t.Anonymous.m = unit;
        var world: win32.graphics.direct3d.D3DMATRIX = undefined;
        _ = d3dx9.D3DXMatrixMultiply(&world, &t, &s);

        _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
        _ = device.IDirect3DDevice9_SetMaterial(&Material.white);
        _ = device.IDirect3DDevice9_SetTexture(0, @ptrCast(texture));

        _ = device.IDirect3DDevice9_SetStreamSource(0, floor, 0, @sizeOf(Vertex));
        _ = device.IDirect3DDevice9_SetFVF(fvf);
        _ = device.IDirect3DDevice9_DrawPrimitive(.TRIANGLELIST, 0, 2);

        // draw pillars
        _ = device.IDirect3DDevice9_SetMaterial(&Material.blue);
        _ = device.IDirect3DDevice9_SetTexture(0, null);
        for (0..5) |index| {
            const i: f32 = @floatFromInt(index);
            _ = d3dx9.D3DXMatrixTranslation(&t, -5.0, 0.0, -15.0 + (i * 7.5));
            _ = d3dx9.D3DXMatrixMultiply(&t, &r, &t);
            _ = d3dx9.D3DXMatrixMultiply(&p, &t, &s);

            _ = device.IDirect3DDevice9_SetTransform(.WORLD, &p);
            _ = pillar.ID3DXBaseMesh_DrawSubset(0);

            _ = d3dx9.D3DXMatrixTranslation(&t, 5.0, 0.0, -15.0 + (i * 7.5));
            _ = d3dx9.D3DXMatrixMultiply(&t, &r, &t);
            _ = d3dx9.D3DXMatrixMultiply(&p, &t, &s);

            _ = device.IDirect3DDevice9_SetTransform(.WORLD, &p);
            _ = pillar.ID3DXBaseMesh_DrawSubset(0);
        }
    }
}

pub fn win32Panic() noreturn {
    const err = win32.foundation.GetLastError();
    std.log.err("win32 panic code {}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## d3dx9.zig

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

pub extern fn D3DXMatrixRotationAxis(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    vec: *const Vec3,
    angle: f32,
) callconv(std.os.windows.WINAPI) *win32.graphics.direct3d.D3DMATRIX;

pub extern fn D3DXVec3TransformCoord(
    out: *Vec3,
    v: *const Vec3,
    m: *const win32.graphics.direct3d.D3DMATRIX,
) callconv(std.os.windows.WINAPI) *Vec3;

pub extern fn D3DXVec3Normalize(
    out: *Vec3,
    v: *const Vec3,
) callconv(std.os.windows.WINAPI) *Vec3;

pub extern fn D3DXMatrixReflect(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
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

pub extern fn D3DXCreateFontIndirectW(
    device: *d3d9.IDirect3DDevice9,
    desc: *win32.graphics.gdi.LOGFONT,
    font: **ID3DXFont,
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

pub extern fn D3DXCreateCylinder(
    device: *d3d9.IDirect3DDevice9,
    radius1: f32,
    radius2: f32,
    length: f32,
    Slices: u32,
    Stacks: u32,
    mesh: **ID3DXMesh,
    adjacency: ?**ID3DXBuffer,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

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

    pub fn mul(self: Vec3, other: f32) Vec3 {
        return Vec3{
            .x = self.x * other,
            .y = self.y * other,
            .z = self.z * other,
        };
    }

    pub fn normalize(self: Vec3) Vec3 {
        const len = @sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        return Vec3{
            .x = self.x / len,
            .y = self.y / len,
            .z = self.z / len,
        };
    }

    pub fn cross(self: Vec3, other: Vec3) Vec3 {
        return Vec3{
            .x = self.y * other.z - self.z * other.y,
            .y = self.z * other.x - self.x * other.z,
            .z = self.x * other.y - self.y * other.x,
        };
    }

    pub fn dot(self: Vec3, other: Vec3) f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
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
    };
    vtable: *const VTable,
};

pub const ID3DXBuffer = extern struct {
    pub const VTable = extern struct {
        base: win32.system.com.IUnknown.VTable,
    };
    vtable: *const VTable,
};

pub const ID3DXFont = extern struct {
    pub const VTable = extern struct {
        base: win32.system.com.IUnknown.VTable,
        _: [12]usize, // 其它方法的占位
        DrawTextW: *const fn (
            self: *const ID3DXFont,
            sprite: ?*anyopaque,
            str: [*]align(1) const u16,
            count: i32,
            rect: *win32.foundation.RECT,
            format: u32,
            color: u32,
        ) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,
    };
    vtable: *const VTable,
    pub inline fn DrawTextW(
        self: *const ID3DXFont,
        sprite: ?*anyopaque,
        str: [*]align(1) const u16,
        count: usize,
        rect: *win32.foundation.RECT,
        format: u32,
        color: u32,
    ) i32 {
        return @as(*const ID3DXFont.VTable, @ptrCast(self.vtable)).DrawTextW( //
            @as(*const ID3DXFont, @ptrCast(self)), sprite, //
            str, @intCast(count), rect, format, color);
    }
};
```

## camera.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");

pub const Camera = struct {
    const Type = enum { landobject, aircraft };

    type: Type = .aircraft,
    pos: d3dx9.Vec3 = .{},
    right: d3dx9.Vec3 = .{ .x = 1 },
    up: d3dx9.Vec3 = .{ .y = 1 },
    look: d3dx9.Vec3 = .{ .z = 1 },

    pub fn walk(self: *Camera, units: f32) void {
        // move only on xz plane for land object
        if (self.type == .landobject) {
            const vec = d3dx9.Vec3{ .x = self.look.x, .z = self.look.z };
            self.pos = self.pos.add(vec.mul(units));
        }

        if (self.type == .aircraft)
            self.pos = self.pos.add(self.look.mul(units));
    }

    pub fn strafe(self: *Camera, units: f32) void {
        // move only on xz plane for land object
        if (self.type == .landobject) {
            const vec = d3dx9.Vec3{ .x = self.right.x, .z = self.right.z };
            self.pos = self.pos.add(vec.mul(units));
        }

        if (self.type == .aircraft)
            self.pos = self.pos.add(self.right.mul(units));
    }

    pub fn fly(self: *Camera, units: f32) void {
        // move only on y-axis for land object
        if (self.type == .landobject)
            self.pos.y += units;

        if (self.type == .aircraft)
            self.pos = self.pos.add(self.up.mul(units));
    }

    pub fn pitch(self: *Camera, angle: f32) void {
        var t: win32.graphics.direct3d.D3DMATRIX = undefined;
        _ = d3dx9.D3DXMatrixRotationAxis(&t, &self.right, angle);

        // rotate _up and _look around _right vector
        _ = d3dx9.D3DXVec3TransformCoord(&self.up, &self.up, &t);
        _ = d3dx9.D3DXVec3TransformCoord(&self.look, &self.look, &t);
    }

    pub fn yaw(self: *Camera, angle: f32) void {
        var t: win32.graphics.direct3d.D3DMATRIX = undefined;
        // rotate around world y (0, 1, 0) always for land object
        if (self.type == .landobject)
            _ = d3dx9.D3DXMatrixRotationY(&t, angle);

        // rotate around own up vector for aircraft
        if (self.type == .aircraft)
            _ = d3dx9.D3DXMatrixRotationAxis(&t, &self.up, angle);

        // rotate _right and _look around _up or y-axis
        _ = d3dx9.D3DXVec3TransformCoord(&self.right, &self.right, &t);
        _ = d3dx9.D3DXVec3TransformCoord(&self.look, &self.look, &t);
    }

    pub fn roll(self: *Camera, angle: f32) void {
        // only roll for aircraft type
        if (self.type == .aircraft) {
            var t: win32.graphics.direct3d.D3DMATRIX = undefined;
            _ = d3dx9.D3DXMatrixRotationAxis(&t, &self.look, angle);

            // rotate _up and _right around _look vector
            _ = d3dx9.D3DXVec3TransformCoord(&self.right, &self.right, &t);
            _ = d3dx9.D3DXVec3TransformCoord(&self.up, &self.up, &t);
        }
    }

    pub fn getViewMatrix(self: *Camera) win32.graphics.direct3d.D3DMATRIX {
        // Keep camera's axes orthogonal to eachother
        self.look = self.look.normalize();

        self.up = self.look.cross(self.right).normalize();
        self.right = self.up.cross(self.look).normalize();

        // Build the view matrix:
        const x = -self.right.dot(self.pos);
        const y = -self.up.dot(self.pos);
        const z = -self.look.dot(self.pos);

        return win32.graphics.direct3d.D3DMATRIX{
            .Anonymous = .{
                .m = .{
                    self.right.x, self.up.x, self.look.x, 0.0, //
                    self.right.y, self.up.y, self.look.y, 0.0,
                    self.right.z, self.up.z, self.look.z, 0.0,
                    x,            y,         z,           1.0,
                },
            },
        };
    }

    pub fn setCameraType(self: *Camera, t: Camera.Type) void {
        self.type = t;
    }
};
```

## particle.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");

const d3d9 = win32.graphics.direct3d9;

pub const Particle = extern struct {
    position: d3dx9.Vec3,
    color: u32,
    const FVF: u32 = win32.system.system_services.D3DFVF_XYZ | //
        win32.system.system_services.D3DFVF_DIFFUSE;
};

pub const Attribute = struct {
    position: d3dx9.Vec3,
    velocity: d3dx9.Vec3,
    acceleration: d3dx9.Vec3 = .{ .x = 0.0, .y = 0.0, .z = 0.0 },
    lifeTime: f32 = 0,
    age: f32 = 0,
    color: u32,
    colorFade: d3d9.D3DCOLORVALUE = d3d.Material.WHITE,
    isAlive: bool = true,

    pub fn reset(boundingBox: d3d.BoundingBox) Attribute {
        var position = randomVector(boundingBox.min, boundingBox.max);
        position.y = boundingBox.max.y;
        return Attribute{
            .isAlive = true,
            .position = position,
            .velocity = .{
                .x = randomFloat(0.0, 1.0) * -3.0,
                .y = randomFloat(0.0, 1.0) * -10.0,
                .z = 0.0,
            },
            .color = 0xffffffff,
        };
    }
};

pub const ParticleSystem = struct {
    device: *d3d9.IDirect3DDevice9 = undefined,
    origin: d3dx9.Vec3 = .{ .x = 0.0, .y = 0.0, .z = 0.0 },
    boundingBox: d3d.BoundingBox,
    emitRate: f32 = 0,
    size: f32 = 0,
    texture: *d3d9.IDirect3DTexture9 = undefined,
    vetexBuffer: *d3d9.IDirect3DVertexBuffer9 = undefined,
    particles: std.ArrayList(Attribute) = undefined,
    maxParticles: usize,

    vbSize: u32,
    vbOffset: u32,
    vbBatchSize: u32,

    fn preRender(self: *ParticleSystem) void {
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_LIGHTING, 0);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSPRITEENABLE, 1);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSCALEENABLE, 1);
        var state: u32 = FtoDw(self.size);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSIZE, state);
        state = FtoDw(0);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSIZE_MIN, state);

        // control the size of the particle relative to distance
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSCALE_A, state);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSCALE_B, state);
        state = FtoDw(1);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSCALE_C, state);

        // use alpha from texture
        state = win32.system.system_services.D3DTA_TEXTURE;
        _ = self.device.IDirect3DDevice9_SetTextureStageState(0, .ALPHAARG1, state);
        state = @intFromEnum(d3d9.D3DTOP_SELECTARG1);
        _ = self.device.IDirect3DDevice9_SetTextureStageState(0, .ALPHAOP, state);

        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_ALPHABLENDENABLE, 1);
        state = @intFromEnum(d3d9.D3DBLEND_SRCALPHA);
        _ = self.device.IDirect3DDevice9_SetRenderState(.SRCBLEND, state);
        state = @intFromEnum(d3d9.D3DBLEND_INVSRCALPHA);
        _ = self.device.IDirect3DDevice9_SetRenderState(.DESTBLEND, state);
    }

    pub fn render(self: *ParticleSystem) void {
        if (self.particles.items.len == 0) return;

        //
        // set render states
        //
        self.preRender();

        _ = self.device.IDirect3DDevice9_SetTexture(0, @ptrCast(self.texture));
        _ = self.device.IDirect3DDevice9_SetFVF(Particle.FVF);
        _ = self.device.IDirect3DDevice9_SetStreamSource(0, self.vetexBuffer, 0, @sizeOf(Particle));

        //
        // render batches one by one
        //

        // start at beginning if we're at the end of the vb
        if (self.vbOffset >= self.vbSize) self.vbOffset = 0;

        var v: [*]Particle = undefined;
        _ = self.vetexBuffer.IDirect3DVertexBuffer9_Lock(
            self.vbOffset * @sizeOf(Particle),
            self.vbBatchSize * @sizeOf(Particle),
            @ptrCast(&v),
            if (self.vbOffset != 0) d3d9.D3DLOCK_NOOVERWRITE else d3d9.D3DLOCK_DISCARD,
        );

        var numParticlesInBatch: u32 = 0;
        // var index: usize = 0;
        //
        // Until all particles have been rendered.
        //
        for (self.particles.items) |*particle| {
            if (!particle.isAlive) continue;

            //
            // Copy a batch of the living particles to the
            // next vertex buffer segment
            //
            v[0].position = particle.position;
            v[0].color = particle.color;

            v += 1; // next element;

            numParticlesInBatch += 1; //increase batch counter

            // if this batch full?
            if (numParticlesInBatch == self.vbBatchSize) {
                //
                // Draw the last batch of particles that was
                // copied to the vertex buffer.
                //
                _ = self.vetexBuffer.IDirect3DVertexBuffer9_Unlock();

                _ = self.device.IDirect3DDevice9_DrawPrimitive(.POINTLIST, self.vbOffset, self.vbBatchSize);

                //
                // While that batch is drawing, start filling the
                // next batch with particles.
                //

                // move the offset to the start of the next batch
                self.vbOffset += self.vbBatchSize;

                // don't offset into memory thats outside the vb's range.
                // If we're at the end, start at the beginning.
                if (self.vbOffset >= self.vbSize)
                    self.vbOffset = 0;

                _ = self.vetexBuffer.IDirect3DVertexBuffer9_Lock(
                    self.vbOffset * @sizeOf(Particle),
                    self.vbBatchSize * @sizeOf(Particle),
                    @ptrCast(&v),
                    if (self.vbOffset != 0) d3d9.D3DLOCK_NOOVERWRITE else d3d9.D3DLOCK_DISCARD,
                );
                numParticlesInBatch = 0; // reset for new batch
            }
        }

        _ = self.vetexBuffer.IDirect3DVertexBuffer9_Unlock();

        // its possible that the LAST batch being filled never
        // got rendered because the condition
        // (numParticlesInBatch == _vbBatchSize) would not have
        // been satisfied.  We draw the last partially filled batch now.

        if (numParticlesInBatch != 0) {
            _ = self.device.IDirect3DDevice9_DrawPrimitive(.POINTLIST, self.vbOffset, numParticlesInBatch);
        }

        // next block
        self.vbOffset += self.vbBatchSize;

        //
        // reset render states
        //

        self.postRender();
    }

    fn postRender(self: *ParticleSystem) void {
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_LIGHTING, 1);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSPRITEENABLE, 0);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_POINTSCALEENABLE, 0);
        _ = self.device.IDirect3DDevice9_SetRenderState(d3d9.D3DRS_ALPHABLENDENABLE, 0);
    }
};

const Str = [*:0]align(1) const u16;

pub const Snow = struct {
    particleSystem: ParticleSystem,

    pub fn new(
        allocator: std.mem.Allocator,
        boundingBox: d3d.BoundingBox,
        numParticles: usize,
    ) Snow {
        var system: ParticleSystem = .{
            .boundingBox = boundingBox,
            .size = 0.25,
            .vbSize = 2048,
            .vbOffset = 0,
            .vbBatchSize = 512,
            .particles = std.ArrayList(Attribute)
                .initCapacity(allocator, numParticles) catch unreachable,
            .maxParticles = numParticles,
        };

        for (0..numParticles) |_| {
            const attribute = Attribute.reset(boundingBox);
            system.particles.appendAssumeCapacity(attribute);
        }

        return Snow{ .particleSystem = system };
    }

    pub fn init(self: *Snow, device: *d3d9.IDirect3DDevice9, name: Str) void {
        self.particleSystem.device = device; // save a ptr to the device

        const usage = d3d9.D3DUSAGE_DYNAMIC | d3d9.D3DUSAGE_POINTS | d3d9.D3DUSAGE_WRITEONLY;
        const size = self.particleSystem.vbSize * @sizeOf(Particle);
        _ = device.IDirect3DDevice9_CreateVertexBuffer(size, usage, Particle.FVF, //
            .DEFAULT, @ptrCast(&self.particleSystem.vetexBuffer), null);

        _ = d3dx9.D3DXCreateTextureFromFileW(device, name, &self.particleSystem.texture);
    }

    pub fn update(self: *Snow, deltaTime: f32) void {
        for (self.particleSystem.particles.items) |*particle| {
            particle.position = particle.position.add(particle.velocity.mul(deltaTime));

            if (!self.particleSystem.boundingBox.isPointInside(particle.position)) {
                particle.* = Attribute.reset(self.particleSystem.boundingBox);
            }
        }
    }

    pub fn render(self: *Snow) void {
        self.particleSystem.render();
    }
};

fn randomFloat(lowBound: f32, highBound: f32) f32 {
    if (lowBound >= highBound) return lowBound;

    const float = d3d.rand.float(f32);
    return lowBound + (highBound - lowBound) * float;
}

fn randomVector(min: d3dx9.Vec3, max: d3dx9.Vec3) d3dx9.Vec3 {
    return d3dx9.Vec3{
        .x = randomFloat(min.x, max.x),
        .y = randomFloat(min.y, max.y),
        .z = randomFloat(min.z, max.z),
    };
}

fn FtoDw(f: f32) u32 {
    return @bitCast(f);
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");
const camera = @import("camera.zig");
const particle = @import("particle.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const failed = win32.zig.FAILED;

// Globals
var allocator: std.mem.Allocator = undefined;
pub const UNICODE: bool = true;
var device: *d3d9.IDirect3DDevice9 = undefined;

var snow: particle.Snow = undefined;

// Framework Functions
fn setup() bool {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    allocator = gpa.allocator();
    // Create Snow System.
    const boundingBox: d3d.BoundingBox = .{
        .min = d3dx9.Vec3{ .x = -10.0, .y = -10.0, .z = -10.0 },
        .max = d3dx9.Vec3{ .x = 10.0, .y = 10.0, .z = 10.0 },
    };

    snow = particle.Snow.new(allocator, boundingBox, 5000);
    snow.init(device, win32.zig.L("snowflake.dds"));

    // 绘制基础场景
    d3d.drawBasicScene(device, 1, true);

    // 设置投影矩阵
    var p: win32.graphics.direct3d.D3DMATRIX = undefined;
    const w = @as(f32, @floatFromInt(WIDTH));
    const h = @as(f32, @floatFromInt(HEIGHT));
    const fov = std.math.pi / 4.0;
    _ = d3dx9.D3DXMatrixPerspectiveFovLH(&p, fov, w / h, 1.0, 5000.0);
    _ = device.IDirect3DDevice9_SetTransform(.PROJECTION, &p);

    return true;
}

fn cleanup() void {}

var y: f32 = 0;
var theCamera: camera.Camera = .{ .type = .aircraft };
fn display(timeDelta: f32) bool {
    const keyboard = win32.ui.input.keyboard_and_mouse;
    if (keyboard.GetAsyncKeyState(('W')) != 0)
        theCamera.walk(4.0 * timeDelta);

    if (keyboard.GetAsyncKeyState(('S')) != 0)
        theCamera.walk(-4.0 * timeDelta);

    if (keyboard.GetAsyncKeyState('A') != 0)
        theCamera.strafe(-4.0 * timeDelta);

    if (keyboard.GetAsyncKeyState('D') != 0)
        theCamera.strafe(4.0 * timeDelta);

    if (keyboard.GetAsyncKeyState('R') != 0)
        theCamera.fly(4.0 * timeDelta);

    if (keyboard.GetAsyncKeyState('F') != 0)
        theCamera.fly(-4.0 * timeDelta);

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_UP)) != 0)
        theCamera.pitch(1.0 * timeDelta);

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_DOWN)) != 0)
        theCamera.pitch(-1.0 * timeDelta);

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_LEFT)) != 0)
        theCamera.yaw(-1.0 * timeDelta);

    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_RIGHT)) != 0)
        theCamera.yaw(1.0 * timeDelta);

    if (keyboard.GetAsyncKeyState('N') != 0)
        theCamera.roll(1.0 * timeDelta);

    if (keyboard.GetAsyncKeyState('M') != 0)
        theCamera.roll(-1.0 * timeDelta);

    const view = theCamera.getViewMatrix();
    _ = device.IDirect3DDevice9_SetTransform(d3d9.D3DTS_VIEW, &view);

    snow.update(timeDelta);

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xff000000, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    var world: win32.graphics.direct3d.D3DMATRIX = undefined;
    const unit: [16]f32 = .{
        1, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 1,
    };
    world.Anonymous.m = unit;
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);

    d3d.drawBasicScene(device, 1, false);

    // order important, render snow last.
    _ = device.IDirect3DDevice9_SetTransform(.WORLD, &world);
    snow.render();

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

![雪花粒子效果][1]。

[1]: images/directx015.webp

## 附录
