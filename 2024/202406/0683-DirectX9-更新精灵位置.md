# 0683-DirectX9-更新精灵位置

## 目标

更新精灵的位置，使其在屏幕上移动。

## 环境

- Time 2024-12-29
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

这一节也简单，只增加了一点点东西。

## game.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const gfx = @import("gfx.zig");
const Sprite = @import("sprite.zig").Sprite;

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub const Game = struct {
    pub const win32Check = gfx.win32Check;
    device: gfx.GraphicsDevice,
    player1: Sprite,
    player2: Sprite,

    pub fn init(window: win32.foundation.HWND) Game {
        var device = gfx.GraphicsDevice.init(window);

        var player1 = Sprite.init(&device, win32.zig.L("assets/PlayerPaper.png"));
        player1.position = .{ .x = 100, .y = 200, .z = 0 };
        player1.velocity = .{ .x = 2, .y = 1, .z = 0 };
        var player2 = Sprite.init(&device, win32.zig.L("assets/PlayerPaper.png"));
        player2.position = .{ .x = 80, .y = 200, .z = 0 };
        player2.velocity = .{ .x = 4, .y = -1, .z = 0 };

        return .{ .device = device, .player1 = player1, .player2 = player2 };
    }

    pub fn run(self: *Game) void {
        const gameTime: f32 = 0;

        self.update(gameTime);
        self.draw(gameTime);
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

![更新精灵位置][1]。

[1]: images/directx028.webp

## 附录
