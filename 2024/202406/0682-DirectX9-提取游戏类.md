# 0682-DirectX9-提取游戏类

## 目标

将主函数中的逻辑提取一部分到游戏类中。

## 环境

- Time 2024-12-29
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

相当于一点小重构，很简单，这一节。

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
        var player2 = Sprite.init(&device, win32.zig.L("assets/PlayerPaper.png"));
        player2.position = .{ .x = 80, .y = 200, .z = 0 };

        return .{ .device = device, .player1 = player1, .player2 = player2 };
    }

    pub fn run(self: *Game) void {
        const gameTime: f32 = 0;

        self.update(gameTime);
        self.draw(gameTime);
    }

    fn update(self: Game, delta: f32) void {
        _ = delta;
        _ = self;
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

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Game = @import("game.zig").Game;

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub const UNICODE: bool = true;

const WIDTH = 640;
const HEIGHT = 480;

pub fn main() !void {
    const window = generateWindow();

    var game = Game.init(window);

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

## 效果

效果和上一节一样，没有变动逻辑。

![提取游戏类][1]。

[1]: images/directx027.png

## 附录
