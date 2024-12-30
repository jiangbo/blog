# 0687-DirectX9-键盘输入方式一

## 目标

使用消息来获取键盘的输入。

## 环境

- Time 2024-12-29
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

1. <https://www.youtube.com/watch?v=K9wWTP0Dgv0>

## 想法

这种方式之前还没有见过，不过视频里说这种走消息的延迟高，不好，还有第二种，那么这种了解一下就可以了。

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

    var game = Game.init(window);

    initializeInput();
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

fn initializeInput() void {
    var rid = std.mem.zeroInit(win32.ui.input.RAWINPUTDEVICE, .{
        .usUsagePage = 0x01, //Generic desktop controls
        .usUsage = 0x06, //Keyboard
    });

    const size = @sizeOf(win32.ui.input.RAWINPUTDEVICE);
    Game.win32Check(win32.ui.input.RegisterRawInputDevices(@ptrCast(&rid), 1, size));
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

        ui.WM_INPUT => {
            const input = win32.ui.input;
            //We should not use this method.  The message queue is far too slow for a game.
            var dwSize: u32 = undefined;
            std.log.info("dw size: {}", .{dwSize});
            //RID_INPUT - Gets raw data from the rawinput structure
            //dwSize - size of the data in pdata
            const RID_INPUT = input.RID_INPUT;
            const rawInput: input.HRAWINPUT = @ptrFromInt(@as(usize, @intCast(lParam)));
            const size = @sizeOf(input.RAWINPUTHEADER);
            _ = input.GetRawInputData(rawInput, RID_INPUT, null, &dwSize, size);

            var buffer: [128]u8 = undefined;
            if (buffer.len < dwSize) @panic("buffer too small");

            //Get raw input data again, this time with the BYTE array we made above
            _ = win32.ui.input.GetRawInputData(rawInput, RID_INPUT, &buffer, &dwSize, size);

            const raw: *win32.ui.input.RAWINPUT = @alignCast(@ptrCast(&buffer));

            if (raw.header.dwType == @intFromEnum(win32.ui.input.RIM_TYPEKEYBOARD)) {
                if (raw.data.keyboard.Message == ui.WM_KEYDOWN //
                or raw.data.keyboard.Message == ui.WM_SYSKEYDOWN) {
                    std.log.info("ok", .{});
                }

                const down = @intFromEnum(win32.ui.input.keyboard_and_mouse.VK_DOWN);
                if (raw.data.keyboard.VKey == down)
                    _ = ui.MessageBoxA(null, "Down Arrow", null, ui.MB_OK);
            }
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

![键盘输入][1]

[1]: images/directx031.png

## 附录
