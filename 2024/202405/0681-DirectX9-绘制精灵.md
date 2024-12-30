# 0681-DirectX9-绘制精灵

## 目标

在背景板上绘制图片精灵。

## 环境

- Time 2024-12-29
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

加载纹理之前写过了，但是 D3DX 扩展包中的 Sprite 没有，需要自己写。

## d3dx9.zig

```zig
pub extern fn D3DXCreateTextureFromFileW(
    device: *d3d9.IDirect3DDevice9,
    name: LPCTSTR,
    LPDIRECT3DTEXTURE9: ?**d3d9.IDirect3DTexture9,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub extern fn D3DXCreateTextureFromFileExW(
    device: *d3d9.IDirect3DDevice9,
    name: LPCTSTR,
    width: u32,
    height: u32,
    mipLevels: u32,
    usage: u32,
    format: d3d9.D3DFORMAT,
    pool: d3d9.D3DPOOL,
    filter: u32,
    mipFilter: u32,
    colorkey: u32,
    pSrcInfo: usize,
    pPalette: ?*const win32.graphics.gdi.PALETTEENTRY,
    ppTexture: ?**d3d9.IDirect3DTexture9,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub const D3DX_DEFAULT = 0xffffffff;
pub const D3DXSPRITE_ALPHABLEND: u32 = 1 << 4;

pub extern fn D3DXCreateSprite(
    device: *d3d9.IDirect3DDevice9,
    sprite: ?**ID3DXSprite,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

pub const ID3DXSprite = extern union {
    pub const VTable = extern struct {
        base: win32.system.com.IUnknown.VTable,

        GetDevice: usize,

        GetTransform: usize,
        SetTransform: usize,
        SetWorldViewRH: usize,
        SetWorldViewLH: usize,

        Begin: *const fn (self: *ID3DXSprite, flags: u32) //
        callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,
        Draw: *const fn (
            self: *ID3DXSprite,
            texture: *d3d9.IDirect3DTexture9,
            srcRect: ?*const win32.foundation.RECT,
            center: ?*const win32.graphics.direct3d.D3DVECTOR,
            position: ?*const win32.graphics.direct3d.D3DVECTOR,
            color: u32,
        ) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,
        Flush: usize,
        End: *const fn (
            self: *ID3DXSprite,
        ) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,

        OnLostDevice: usize,
        OnResetDevice: usize,
    };

    vtable: *const VTable,
    IUnknown: win32.system.com.IUnknown,

    pub fn Begin(self: *ID3DXSprite, flags: u32) callconv(.Inline) HRESULT {
        return self.vtable.Begin(self, flags);
    }

    pub fn Draw(
        self: *ID3DXSprite,
        texture: *d3d9.IDirect3DTexture9,
        srcRect: ?*const win32.foundation.RECT,
        center: ?*const win32.graphics.direct3d.D3DVECTOR,
        position: ?*const win32.graphics.direct3d.D3DVECTOR,
        color: u32,
    ) callconv(.Inline) HRESULT {
        return self.vtable.Draw(self, texture, srcRect, center, position, color);
    }

    pub fn End(self: *ID3DXSprite) callconv(.Inline) HRESULT {
        return self.vtable.End(self);
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
        _ = self;
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

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const gfx = @import("gfx.zig");
const sprite = @import("sprite.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const win32Check = gfx.win32Check;

pub const UNICODE: bool = true;

const WIDTH = 640;
const HEIGHT = 480;

var player1: sprite.Sprite = undefined;
var player2: sprite.Sprite = undefined;

pub fn main() !void {
    const window = generateWindow();

    var device = gfx.GraphicsDevice.init(window);
    defer device.deinit();

    player1 = sprite.Sprite.init(&device, win32.zig.L("assets/PlayerPaper.png"));
    defer player1.deinit();
    player2 = sprite.Sprite.init(&device, win32.zig.L("assets/PlayerPaper.png"));
    defer player2.deinit();
    player1.position = .{ .x = 100, .y = 200, .z = 0 };
    player2.position = .{ .x = 80, .y = 200, .z = 0 };

    var message: ui.MSG = std.mem.zeroes(ui.MSG);
    while (true) {
        while (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
        if (message.message == ui.WM_QUIT) break;

        update(0);
        draw(&device, 0);
    }
}

fn update(delta: f32) void {
    _ = delta;
}

fn draw(device: *gfx.GraphicsDevice, delta: f32) void {
    device.begin();
    device.clear(0x00006464);

    player1.draw(delta);
    player2.draw(delta);

    device.end();
    device.Present();
}

pub fn windowCallback(
    w: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(std.os.windows.WINAPI) win32.foundation.LRESULT {
    switch (message) {
        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
            ui.PostQuitMessage(0);
        },
        else => {},
    }
    return ui.DefWindowProc(w, message, wParam, lParam);
}

fn generateWindow() win32.foundation.HWND {
    const handle = win32.system.library_loader.GetModuleHandle(null).?;

    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    const className = win32.zig.L("DirectX9");
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1 };
    windowClass.lpszClassName = className;
    windowClass.lpfnWndProc = windowCallback;
    windowClass.hInstance = handle;
    win32Check(ui.RegisterClassEx(&windowClass));

    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const name = win32.zig.L("DirectX9 学习");
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, className, name, //
        style, 200, 200, WIDTH, HEIGHT, null, null, handle, null).?;

    return window;
}
```

## 效果

![绘制精灵][1]

[1]: images/directx027.png

## 附录
