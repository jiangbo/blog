# 0690-DirectX9-2D 相机

## 目标

创建一个 2D 相机，可以跟随玩家。

## 环境

- Time 2024-12-30
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

涉及到一些 3D 的变换的一些，之前学过一点，在这里使用还是不复杂。
之前的显示文本的就跳过了，之前做过了。
这个是最后一个视频，DirectX9的学习应该就到这里了。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Game = @import("game.zig").Game;
const timer = @import("timer.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub const UNICODE: bool = true;

const WIDTH = 800;
const HEIGHT = 600;

pub fn main() !void {
    const window = generateWindow();
    var game = Game.init(window, WIDTH, HEIGHT);

    var message: ui.MSG = std.mem.zeroes(ui.MSG);

    while (true) {
        while (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
        if (message.message == ui.WM_QUIT) break;

        game.run();
    }
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
    Game.win32Check(ui.RegisterClassEx(&windowClass));

    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const name = win32.zig.L("DirectX9 学习");
    const window = ui.CreateWindowEx(ui.WS_EX_LEFT, className, name, //
        style, 200, 200, WIDTH, HEIGHT, null, null, handle, null).?;

    return window;
}
```

## game.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const gfx = @import("gfx.zig");
const Sprite = @import("sprite.zig").Sprite;
const Timer = @import("timer.zig").Timer;
const Camera = @import("camera.zig").Camera;

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub const Game = struct {
    pub const win32Check = gfx.win32Check;
    device: gfx.GraphicsDevice,
    player: Sprite,
    camera: Camera,
    background: Sprite,

    pub fn init(window: win32.foundation.HWND, width: u16, height: u16) Game {
        var game: Game = undefined;
        game.device = gfx.GraphicsDevice.init(window);

        var name = win32.zig.L("assets/PlayerPaper.png");
        game.player = Sprite.init(&game.device, name);
        game.player.position = .{ .x = 100, .y = 200, .z = 0 };

        game.camera = .{
            .width = @as(f32, @floatFromInt(width)),
            .height = @as(f32, @floatFromInt(height)),
            .scale = .{ .x = 1.0, .y = 1.0, .z = 1.0 },
        };
        game.camera.init();

        name = win32.zig.L("assets/bubbles-bg2.png");
        game.background = Sprite.init(&game.device, name);

        return game;
    }

    pub fn run(self: *Game) void {
        const gameTime: f32 = 0;
        self.update(gameTime);
        self.draw(gameTime);
    }

    fn update(self: *Game, delta: f32) void {
        const keyboard = win32.ui.input.keyboard_and_mouse;
        if (keyboard.GetAsyncKeyState('F') != 0 and !self.camera.isFollowing()) {
            self.camera.followSprite(&self.player);
        }

        if (keyboard.GetAsyncKeyState('U') != 0 and self.camera.isFollowing()) {
            self.camera.unfollow();
        }

        self.camera.update();

        self.player.handleInput();
        self.player.update(delta);
    }

    fn draw(self: *Game, delta: f32) void {
        self.device.begin();
        self.device.clear(0x00006464);

        self.camera.setTransform(&self.device);
        self.background.draw(delta);
        self.player.draw(delta);

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
        if (keyboard.GetAsyncKeyState(key) != 0) vector.y -= 4;

        key = @intFromEnum(keyboard.VK_DOWN);
        if (keyboard.GetAsyncKeyState(key) != 0) vector.y += 4;

        key = @intFromEnum(keyboard.VK_LEFT);
        if (keyboard.GetAsyncKeyState(key) != 0) vector.x -= 4;

        key = @intFromEnum(keyboard.VK_RIGHT);
        if (keyboard.GetAsyncKeyState(key) != 0) vector.x += 4;

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

        const flag = d3dx9.D3DXSPRITE_ALPHABLEND | d3dx9.D3DXSPRITE_OBJECTSPACE;
        win32Check(self.sprite.Begin(flag));
        win32Check(self.sprite.Draw(self.texture, null, null, &self.position, self.color));
        win32Check(self.sprite.End());
    }

    pub fn deinit(self: *Sprite) void {
        _ = self.texture.IUnknown.Release();
        _ = self.sprite.IUnknown.Release();
    }
};
```

## camera.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Sprite = @import("sprite.zig").Sprite;
const d3dx9 = @import("d3dx9.zig");
const gfx = @import("gfx.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

const Vector3 = win32.graphics.direct3d.D3DVECTOR;
const Matrix = win32.graphics.direct3d.D3DMATRIX;

pub const Camera = struct {
    width: f32 = 0,
    height: f32 = 0,
    angle: f32 = 0,
    scale: Vector3,

    orthographic: Matrix = undefined,
    identity: Matrix = undefined,
    view: Matrix = undefined,

    follow: ?*Sprite = null,

    pub fn init(self: *Camera) void {
        _ = d3dx9.D3DXMatrixOrthoLH(&self.orthographic, self.width, -self.height, 0.0, 1.0);
        self.identity.Anonymous.m = .{
            1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, 1, 0,
            0, 0, 0, 1,
        };
    }

    pub fn update(self: *Camera) void {
        var cameraX = self.width / 2;
        var cameraY = self.height / 2;
        if (self.follow) |follow| {
            cameraX = follow.position.x;
            cameraY = follow.position.y;
        }

        const cos = @cos(self.angle);
        const sin = @sin(self.angle);
        const m12 = -cameraX * self.scale.x * cos + cameraY * self.scale.y * sin;
        const m13 = -cameraX * self.scale.y * sin - cameraY * self.scale.y * cos;
        self.view.Anonymous.m = .{
            self.scale.x * cos,  self.scale.x * sin, 0,            0,
            -self.scale.y * sin, self.scale.y * cos, 0,            0,
            0,                   0,                  self.scale.z, 0,
            m12,                 m13,                0,            1,
        };
    }

    pub fn followSprite(self: *Camera, following: *Sprite) void {
        self.follow = following;
        std.log.info("follow sprite", .{});
    }

    pub fn unfollow(self: *Camera) void {
        self.follow = null;
        std.log.info("unfollow sprite", .{});
    }

    pub fn isFollowing(self: *Camera) bool {
        return self.follow != null;
    }

    pub fn setTransform(self: *Camera, device: *gfx.GraphicsDevice) void {
        _ = device.device.SetTransform(.PROJECTION, &self.orthographic);
        // 之前忘记这个值是多少了，再看了一次，这个值是 256
        _ = device.device.SetTransform(.WORLD, &self.identity);
        _ = device.device.SetTransform(.VIEW, &self.view);
    }
};
```

## d3dx9.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const d3d9 = win32.graphics.direct3d9;
pub const LPCTSTR = [*:0]const u16;
const HRESULT = win32.foundation.HRESULT;

pub extern fn D3DXLoadSurfaceFromFileW(
    surface: *d3d9.IDirect3DSurface9,
    palette: ?*const win32.graphics.gdi.PALETTEENTRY,
    rect: ?*const win32.foundation.RECT,
    srcFile: LPCTSTR,
    srcRect: ?*const win32.foundation.RECT,
    filter: u32,
    colorkey: u32,
    srcInfo: usize,
) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT;

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
pub const D3DXSPRITE_OBJECTSPACE: u32 = 1 << 2;

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

pub extern fn D3DXMatrixOrthoLH(
    matrix: *win32.graphics.direct3d.D3DMATRIX,
    w: f32,
    h: f32,
    zn: f32,
    zf: f32,
) *win32.graphics.direct3d.D3DMATRIX;
```

## gfx.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub const GraphicsDevice = struct {
    device: *d3d9.IDirect3DDevice9,
    direct3d: *d3d9.IDirect3D9,

    pub fn init(window: win32.foundation.HWND) GraphicsDevice {
        var d3d = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;

        var params = std.mem.zeroes(d3d9.D3DPRESENT_PARAMETERS);
        params.Windowed = win32.zig.TRUE;
        params.SwapEffect = .DISCARD;
        params.hDeviceWindow = window;

        // 创建设备
        var device: *d3d9.IDirect3DDevice9 = undefined;
        win32Check(d3d.CreateDevice(d3d9.D3DADAPTER_DEFAULT, .HAL, window, //
            d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device)));
        return .{ .device = device, .direct3d = d3d };
    }

    pub fn clear(self: *GraphicsDevice, color: u32) void {
        const target = win32.system.system_services.D3DCLEAR_TARGET;
        win32Check(self.device.Clear(0, null, target, color, 1.0, 0));
    }

    pub fn begin(self: *GraphicsDevice) void {
        win32Check(self.device.BeginScene());
    }

    pub fn end(self: *GraphicsDevice) void {
        win32Check(self.device.EndScene());
    }

    pub fn Present(self: *GraphicsDevice) void {
        win32Check(self.device.Present(null, null, null, null));
    }

    pub fn deinit(self: *GraphicsDevice) void {
        _ = self.device.IUnknown.Release();
        _ = self.direct3d.IUnknown.Release();
    }
};

pub fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![2D 相机][1]

[1]: images/directx034.webp

## 附录
