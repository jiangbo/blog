# 0686-DirectX9-计时器更新精灵

## 目标

使用计时器来更新精灵，移动的速度和帧率就不相关了。

## 环境

- Time 2024-12-29
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

使用计时器来更新精灵。

## sprite.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3dx9 = @import("d3dx9.zig");
const gfx = @import("gfx.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const win32Check = gfx.win32Check;

pub const ObjectStatus = enum { active, dying, dead };

pub const Object = struct {
    name: []const u8,

    position: win32.graphics.direct3d.D3DVECTOR = .{ .x = 0, .y = 0, .z = 0 },
    velocity: win32.graphics.direct3d.D3DVECTOR = .{ .x = 0, .y = 0, .z = 0 },
    rotation: f32 = 0,
    speed: f32 = 0,

    status: ObjectStatus = .active,
    sprite: Sprite = undefined,
    maxSpeed: f32,

    pub fn initSprite(self: *Object, device: *gfx.GraphicsDevice, name: d3dx9.LPCTSTR) void {
        self.sprite = Sprite.init(device, name);
    }

    pub fn update(self: *Object, gameTime: f32) void {
        if (self.status != .active) return;
        self.position.x += self.velocity.x * gameTime;
        self.position.y += self.velocity.y * gameTime;
    }

    pub fn draw(self: *Object, gameTime: f32) void {
        self.sprite.draw(gameTime, self.position);
    }

    pub fn setSpeed(self: *Object, speed: f32) void {
        self.speed = @min(speed, self.maxSpeed);
        self.velocity.x = self.speed * @cos(self.rotation);
        self.velocity.y = self.speed * @sin(self.rotation);
    }

    pub fn deinit(self: *Object) void {
        self.sprite.deinit();
    }
};

pub const Sprite = struct {
    texture: *d3d9.IDirect3DTexture9,
    sprite: *d3dx9.ID3DXSprite,

    color: u32 = 0xffffffff,
    initialised: bool,

    pub fn init(device: *gfx.GraphicsDevice, name: d3dx9.LPCTSTR) Sprite {
        var texture: *d3d9.IDirect3DTexture9 = undefined;
        win32Check(d3dx9.D3DXCreateTextureFromFileW(device.device, name, &texture));

        // win32Check(d3dx9.D3DXCreateTextureFromFileExW(device.device, name, //
        //     d, d, d, 0, .UNKNOWN, .MANAGED, d, d, 0, 0, null, &texture));

        var sprite: *d3dx9.ID3DXSprite = undefined;
        win32Check(d3dx9.D3DXCreateSprite(device.device, &sprite));
        return .{ .texture = texture, .sprite = sprite, .initialised = true };
    }

    pub fn draw(self: *Sprite, gameTime: f32, position: win32.graphics.direct3d.D3DVECTOR) void {
        _ = gameTime;

        if (!self.initialised) return;
        win32Check(self.sprite.Begin(d3dx9.D3DXSPRITE_ALPHABLEND));
        win32Check(self.sprite.Draw(self.texture, null, null, &position, self.color));
        win32Check(self.sprite.End());
    }

    pub fn deinit(self: *Sprite) void {
        _ = self.texture.IUnknown.Release();
        _ = self.sprite.IUnknown.Release();
    }
};
```

## game.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const gfx = @import("gfx.zig");
const Object = @import("sprite.zig").Object;
const Timer = @import("timer.zig").Timer;

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub const Game = struct {
    pub const win32Check = gfx.win32Check;
    device: gfx.GraphicsDevice,
    player1: Object,
    player2: Object,
    timer: Timer,

    pub fn init(window: win32.foundation.HWND) Game {
        var device = gfx.GraphicsDevice.init(window);

        var player1: Object = .{
            .name = "Player1",
            .rotation = @as(f32, std.math.pi) / 4,
            .position = .{ .x = 100, .y = 200, .z = 0 },
            .maxSpeed = 90,
        };
        player1.setSpeed(90);
        player1.initSprite(&device, win32.zig.L("assets/PlayerPaper.png"));

        var player2: Object = .{
            .name = "Player2",
            .position = .{ .x = 100, .y = 200, .z = 0 },
            .maxSpeed = 90,
        };
        player2.setSpeed(90);
        player2.initSprite(&device, win32.zig.L("assets/PlayerPaper.png"));

        return .{
            .device = device,
            .player1 = player1,
            .player2 = player2,
            .timer = Timer.init(),
        };
    }

    pub fn run(self: *Game) void {
        self.timer.update();
        self.update(self.timer.elapsed);
        self.draw(self.timer.elapsed);
    }

    fn update(self: *Game, delta: f32) void {
        self.player1.update(delta);
        self.player2.update(delta);
    }

    fn draw(self: *Game, delta: f32) void {
        self.device.begin();
        self.device.clear(0x00006464);

        self.player1.draw(delta);
        self.player2.draw(delta);

        self.device.end();
        self.device.Present();
    }

    pub fn deinit(self: *Game) void {
        self.device.deinit();
        self.player1.deinit();
        self.player2.deinit();
    }
};
```

## 效果

![计时器更新精灵][1]。

[1]: images/directx030.webp

## 附录
