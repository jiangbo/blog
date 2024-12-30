# 0688-DirectX9-键盘输入方式二

## 目标

使用 `GetAsyncKeyState` 获取按键的状态，适合按键较少的情况。

## 环境

- Time 2024-12-30
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

换了台电脑，发现精灵移动的时候卡一卡的，目前还不知道原因，也不想具体去看是什么问题。

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
        const keyboard = win32.ui.input.keyboard_and_mouse;
        const key: u16 = @intFromEnum(keyboard.VK_DOWN);
        if (keyboard.GetAsyncKeyState(key) != 0) {
            _ = ui.MessageBoxA(null, "down key", null, ui.MB_OK);
        }
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

![键盘输入二][1]

[1]: images/directx032.png

## 附录
