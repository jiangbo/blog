# 0645-DirectX-粒子效果

## 目标

使用很多的点来模拟粒子效果。

## 环境

- Time 2024-07-14
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

这个是本书最后一个例子，刚开始以为使用的 DirectX 8 会导致接口变动进行不下去，没想到大部分兼容的。
这个书内容不多，各方面有涉及，但是不适合入门，很多地方讲得不详细。

## win.zig

无变化。

## render.zig

无变化。

## d3dx9.zig

无变化。

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

    const xyz = win32.system.system_services.D3DFVF_XYZ;
    const diffuse = win32.system.system_services.D3DFVF_DIFFUSE;
    const hr = device.IDirect3DDevice9_SetFVF(xyz | diffuse);
    if (failed(hr)) win32Panic();

    // set up particle lists
    particles = std.mem.zeroes(@TypeOf(particles));
}

var vertices: [1000]ParticleVertex = undefined;
var particles: [vertices.len]Particle = undefined;
var cursor: usize = 0;

const ParticleVertex = extern struct {
    pos: Vec3 = .{},
    diffuse: u32 = 0xffffffff,
};

const Vec3 = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,

    pub fn add(self: Vec3, other: Vec3) Vec3 {
        return .{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
        };
    }

    pub fn mul(self: Vec3, f: f32) Vec3 {
        return .{
            .x = self.x * f,
            .y = self.y * f,
            .z = self.z * f,
        };
    }
};

const Particle = struct {
    position: Vec3 = .{},
    velocity: Vec3 = .{},
    acceleration: Vec3 = .{},
    drag: f32 = 0,
    color: u32 = 0xffffffff,
    left: u32 = 0,
};

fn gameUpdate() void {
    if (win.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xff000000, 0, 0);
    if (failed(hr)) win32Panic();

    // add a particle to the list
    for (0..10) |_| {
        // set up the particle here
        var particle = &particles[cursor];
        cursor += 1;
        if (cursor >= particles.len) cursor = 0;

        var v: f32 = @floatFromInt(win.rand.uintLessThanBiased(u16, 1000));
        particle.position = Vec3{ .x = v * 2 / 1000 - 1, .y = 1 };
        v = @floatFromInt(win.rand.uintLessThanBiased(u16, 50));
        particle.acceleration = Vec3{ .y = v * -0.0001 - 0.005 };
        particle.drag = 15.0 / 16.0;
        particle.color = win.rand.uintLessThanBiased(u32, std.math.maxInt(u32));
        particle.left = 100;
    }

    // begin the scene
    if (failed(device.IDirect3DDevice9_BeginScene())) win32Panic();

    var nVertex: u32 = 0;
    for (&particles) |*particle| {
        if (particle.left == 0) continue;
        // apply drag to velocity
        particle.velocity = particle.velocity.mul(particle.drag);
        // add acceleration to velocity
        particle.velocity = particle.velocity.add(particle.acceleration);
        // add velocity to position
        particle.position = particle.position.add(particle.velocity);
        // decrease the time left for this particle
        particle.left -|= 1;
        // add particle to vertex list
        vertices[nVertex].pos = particle.position;
        vertices[nVertex].diffuse = particle.color;
        nVertex += 1;
    }

    hr = device.IDirect3DDevice9_DrawPrimitiveUP(.POINTLIST, //
        nVertex, &vertices, @sizeOf(ParticleVertex));
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

![粒子效果][1]。

[1]: images/directx79.webp

## 附录
