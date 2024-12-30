# 0633-DirectX-alpha 混合（d3dx9）

## 目标

通过 DirectX 的扩展库 D3DX9 实现 alpha 混合。

## 环境

- Time 2024-07-13
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》
2. <https://www.microsoft.com/en-in/download/details.aspx?id=6812>

## 想法

查资料发现 d3dx9 提供了一个加载纹理的方法，通过这个方法来加载图片进行 alpha 混合。
书中的位图是 24 位深度，我修改中了 32 位带 alpha 通道的位图。

## build.zig

DirectX SDK 的下载参考上面的链接，增加了 d3dx9.lib 的依赖。

```zig
const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // exe.subsystem = .Windows;
    b.installArtifact(exe);

    const win32 = b.dependency("zigwin32", .{});
    exe.root_module.addImport("win32", win32.module("zigwin32"));

    const dir = "C:/software/Microsoft DirectX SDK (June 2010)/";
    // exe.addIncludePath(.{ .cwd_relative = dir ++ "Include" });
    exe.addObjectFile(.{ .cwd_relative = dir ++ "lib/x64/d3dx9.lib" });

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
```

## win.zig

无变化。

## render.zig

无变化。

## lib.zig

删除不要了，替换成 d3dx9.zig 文件。

## d3dx9.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d9 = win32.graphics.direct3d9;

pub const LPCTSTR = [*:0]align(1) const u16;

pub extern fn D3DXCreateTextureFromFileW(
    device: *d3d9.IDirect3DDevice9,
    name: LPCTSTR,
    LPDIRECT3DTEXTURE9: ?**d3d9.IDirect3DTexture9,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const win = @import("win.zig");
const render = @import("render.zig");
const d3dx9 = @import("d3dx9.zig");

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

const TRUE = win32.zig.TRUE;
const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    render.init(win.WIDTH, win.HEIGHT, win.hander);
    device = render.device;

    var hr = device.IDirect3DDevice9_SetRenderState(.ALPHABLENDENABLE, TRUE);
    if (failed(hr)) win32Panic();
    var v = @intFromEnum(d3d9.D3DBLEND_SRCALPHA);
    hr = device.IDirect3DDevice9_SetRenderState(.SRCBLEND, v);
    if (failed(hr)) win32Panic();
    v = @intFromEnum(d3d9.D3DBLEND_INVSRCALPHA);
    hr = device.IDirect3DDevice9_SetRenderState(.DESTBLEND, v);
    if (failed(hr)) win32Panic();

    const xyzrhw = win32.system.system_services.D3DFVF_XYZRHW;
    const diffuse = win32.system.system_services.D3DFVF_DIFFUSE;
    const uv = win32.system.system_services.D3DFVF_TEX1;
    hr = device.IDirect3DDevice9_SetFVF(xyzrhw | diffuse | uv);
    if (failed(hr)) win32Panic();

    // set up vertex colors
    vertices[0] = .{};
    vertices[1] = .{ .u = 1 };
    vertices[2] = .{ .v = 1 };
    vertices[3] = .{ .u = 1, .v = 1 };

    const name = win32.zig.L("north.bmp");
    hr = d3dx9.D3DXCreateTextureFromFileW(device, name, &texture);
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
    diffuse: u32 = 0xffffffff,
    u: f32 = 0,
    v: f32 = 0,
};

const tSize: u32 = 32;
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
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xffffff00, 0, 0);
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

![alpha 混合][1]

[1]: images/directx67.webp

## 附录
