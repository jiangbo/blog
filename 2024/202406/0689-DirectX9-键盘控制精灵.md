# 0689-DirectX9-键盘控制精灵

## 目标

使用 `GetAsyncKeyState` 来控制精灵的移动。

## 环境

- Time 2024-12-30
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

去掉了 timer 的东西，移动正常了，应该加了时间相关的导致卡顿。

## game.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const gfx = @import("gfx.zig");
const Sprite = @import("sprite.zig").Sprite;
const Timer = @import("timer.zig").Timer;

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub const Game = struct {
    pub const win32Check = gfx.win32Check;
    device: gfx.GraphicsDevice,
    player1: Sprite,

    pub fn init(window: win32.foundation.HWND) Game {
        var device = gfx.GraphicsDevice.init(window);

        var player1 = Sprite.init(&device, win32.zig.L("assets/PlayerPaper.png"));
        player1.position = .{ .x = 100, .y = 200, .z = 0 };
        player1.velocity = .{ .x = 2, .y = 1, .z = 0 };

        return .{ .device = device, .player1 = player1 };
    }

    pub fn run(self: *Game) void {
        const gameTime: f32 = 0;
        self.update(gameTime);
        self.draw(gameTime);
    }

    fn update(self: *Game, delta: f32) void {
        self.player1.handleInput();
        self.player1.update(delta);
    }

    fn draw(self: *Game, delta: f32) void {
        self.device.begin();
        self.device.clear(0x00006464);

        self.player1.draw(delta);

        self.device.end();
        self.device.Present();
    }

    pub fn deinit(self: *Game) void {
        self.device.deinit();
        self.player1.deinit();
    }
};
```

## sprite.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3dx9 = @import("d3dx9.zig");
const gfx = @import("gfx.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const win32Check = gfx.win32Check;

pub const Sprite = struct {
    texture: *d3d9.IDirect3DTexture9,
    sprite: *d3dx9.ID3DXSprite,

    position: win32.graphics.direct3d.D3DVECTOR = .{ .x = 0, .y = 0, .z = 0 },
    velocity: win32.graphics.direct3d.D3DVECTOR = .{ .x = 0, .y = 0, .z = 0 },
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

    pub fn handleInput(self: *Sprite) void {
        const keyboard = win32.ui.input.keyboard_and_mouse;

        var vector: win32.graphics.direct3d.D3DVECTOR = .{ .x = 0, .y = 0, .z = 0 };
        var key: u16 = @intFromEnum(keyboard.VK_UP);
        if (keyboard.GetAsyncKeyState(key) != 0) vector.y -= 1;

        key = @intFromEnum(keyboard.VK_DOWN);
        if (keyboard.GetAsyncKeyState(key) != 0) vector.y += 1;

        key = @intFromEnum(keyboard.VK_LEFT);
        if (keyboard.GetAsyncKeyState(key) != 0) vector.x -= 1;

        key = @intFromEnum(keyboard.VK_RIGHT);
        if (keyboard.GetAsyncKeyState(key) != 0) vector.x += 1;

        self.velocity = vector;
    }

    pub fn update(self: *Sprite, gameTime: f32) void {
        _ = gameTime;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    pub fn draw(self: *Sprite, gameTime: f32) void {
        _ = gameTime;

        if (!self.initialised) return;
        win32Check(self.sprite.Begin(d3dx9.D3DXSPRITE_ALPHABLEND));
        win32Check(self.sprite.Draw(self.texture, null, null, &self.position, self.color));
        win32Check(self.sprite.End());
    }

    pub fn deinit(self: *Sprite) void {
        _ = self.texture.IUnknown.Release();
        _ = self.sprite.IUnknown.Release();
    }
};
```

## 效果

![精灵控制][1]

[1]: images/directx033.webp

## 附录
