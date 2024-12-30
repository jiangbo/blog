# 0666-DirectX9-激光粒子效果

## 目标

实现激光粒子效果。

## 环境

- Time 2024-09-02
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

在之前的基础上来做的话，工作量轻松不少。从配套代码看，激光是弯曲的，自己实现的是直的，不清楚是不是哪里错了。

## d3d.zig

无变化。

## d3dx9.zig

无变化。

## camera.zig

加了两个方法。

```zig
...
    pub fn getPos(self: Camera) d3dx9.Vec3 {
        return self.pos;
    }

    pub fn getLook(self: Camera) d3dx9.Vec3 {
        return self.look;
    }
...
```

## particle.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");
const camera = @import("camera.zig");

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

    fn reset(boundingBox: d3d.BoundingBox) Attribute {
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
    boundingBox: d3d.BoundingBox = .{},
    emitRate: f32 = 0,
    size: f32 = 0,
    texture: *d3d9.IDirect3DTexture9 = undefined,
    vetexBuffer: *d3d9.IDirect3DVertexBuffer9 = undefined,
    particles: std.ArrayList(Attribute) = undefined,
    maxParticles: usize = 0,

    vbSize: u32,
    vbOffset: u32,
    vbBatchSize: u32,

    pub fn init(self: *ParticleSystem, device: *d3d9.IDirect3DDevice9, name: Str) void {
        self.device = device; // save a ptr to the device

        const usage = d3d9.D3DUSAGE_DYNAMIC | d3d9.D3DUSAGE_POINTS | d3d9.D3DUSAGE_WRITEONLY;
        const size = self.vbSize * @sizeOf(Particle);
        _ = device.IDirect3DDevice9_CreateVertexBuffer(size, usage, Particle.FVF, //
            .DEFAULT, @ptrCast(&self.vetexBuffer), null);

        _ = d3dx9.D3DXCreateTextureFromFileW(device, name, &self.texture);
    }

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

    fn removeDeadParticles(self: *ParticleSystem) void {
        var i: usize = 0;
        while (i < self.particles.items.len) : (i += 1) {
            if (!self.particles.items[i].isAlive) {
                _ = self.particles.swapRemove(i);
                // 因为 swapRemove 会将数组的最后一个元素移动到当前索引，所以不增加 i
                continue;
            }
        }
    }
};

const Str = [*:0]align(1) const u16;

pub const Snow = struct {
    particleSystem: ParticleSystem,

    pub fn new(allocator: std.mem.Allocator, box: d3d.BoundingBox, numParticles: usize) Snow {
        var system: ParticleSystem = .{
            .boundingBox = box,
            .size = 0.25,
            .vbSize = 2048,
            .vbOffset = 0,
            .vbBatchSize = 512,
            .particles = std.ArrayList(Attribute)
                .initCapacity(allocator, numParticles) catch unreachable,
            .maxParticles = numParticles,
        };

        for (0..numParticles) |_| {
            const attribute = Attribute.reset(box);
            system.particles.appendAssumeCapacity(attribute);
        }

        return Snow{ .particleSystem = system };
    }

    pub fn init(self: *Snow, device: *d3d9.IDirect3DDevice9, name: Str) void {
        self.particleSystem.init(device, name);
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

pub const ParticleGun = struct {
    particleSystem: ParticleSystem,
    camera: camera.Camera,

    pub fn new(allocator: std.mem.Allocator, c: camera.Camera) ParticleGun {
        const system: ParticleSystem = .{
            .size = 0.8,
            .vbSize = 2048,
            .vbOffset = 0,
            .vbBatchSize = 512,
            .particles = std.ArrayList(Attribute)
                .initCapacity(allocator, 5000) catch unreachable,
        };
        return ParticleGun{ .particleSystem = system, .camera = c };
    }

    pub fn init(self: *ParticleGun, device: *d3d9.IDirect3DDevice9, name: Str) void {
        self.particleSystem.init(device, name);
    }

    pub fn addParticle(self: *ParticleGun) void {
        var pos = self.camera.getPos();
        pos.y -= 1;
        const attribute: Attribute = .{
            .position = pos,
            .velocity = self.camera.getLook().mul(100),
            .color = 0x0000ff00,
            .lifeTime = 1,
        };
        self.particleSystem.particles.appendAssumeCapacity(attribute);
    }

    pub fn update(self: *ParticleGun, deltaTime: f32) void {
        for (self.particleSystem.particles.items) |*particle| {
            particle.position = particle.position.add(particle.velocity.mul(deltaTime));

            particle.age += deltaTime;
            if (particle.age > particle.lifeTime) particle.isAlive = false;
        }
        self.particleSystem.removeDeadParticles();
    }

    pub fn render(self: *ParticleGun) void {
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
// var allocator: std.mem.Allocator = undefined;
pub const UNICODE: bool = true;
var device: *d3d9.IDirect3DDevice9 = undefined;

// var snow: particle.Snow = undefined;
var gun: particle.ParticleGun = undefined;

// Framework Functions
fn setup() bool {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    gun = particle.ParticleGun.new(gpa.allocator(), theCamera);
    gun.init(device, win32.zig.L("flare_alpha.dds"));

    // snow = particle.Snow.new(allocator, boundingBox, 5000);
    // snow.init(device, win32.zig.L("flare_alpha.dds"));

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
    if (keyboard.GetAsyncKeyState(@intFromEnum(keyboard.VK_SPACE)) != 0)
        gun.addParticle();

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

    gun.update(timeDelta);

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
    gun.render();

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

![激光粒子效果][1]

[1]: images/directx016.webp

## 附录
