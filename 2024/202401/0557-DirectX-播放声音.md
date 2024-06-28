# 0557-DirectX-播放声音

## 环境

- Time 2024-06-28
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

### 目标

使用 PlaySound 播放声音。

## build.zig

增加 winmm 库。

```zig
...
    exe.addWin32ResourceFile(.{
        .file = b.path("assets/windows.rc"),
        .flags = &.{"/c65001"},
    });
    exe.linkSystemLibrary("winmm");
...
```

## windows.rc

文件都存放在 assets 目录下。

```rc
// note that this file has different types of resources
ICON_T3DX        ICON   t3dx.ico
CURSOR_CROSSHAIR CURSOR crosshair.cur
SOUND_ID_CREATE  WAVE create.wav
SOUND_ID_MUSIC   WAVE techno.wav
```

## winmm.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;
pub const LPCTSTR = [*:0]align(1) const u16;
const BOOL = win32.foundation.BOOL;

pub const SND_RESOURCE: u32 = 0x00040004;
pub const SND_SYNC: u32 = 0x0000;
pub const SND_ASYNC: u32 = 0x0001;
pub const SND_LOOP: u32 = 0x0008;
pub const SND_PURGE: u32 = 0x0040;

pub extern fn PlaySoundW(n: ?LPCTSTR, w: H, f: u32) callconv(WINAPI) BOOL;
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const winmm = @import("winmm.zig");
const ui = win32.ui.windows_and_messaging;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub const UNICODE: bool = true;
const name = win32.zig.L("游戏编程");
var hinstance: H = undefined;
var hander: win32.foundation.HWND = undefined;

pub fn mainWindowCallback(
    window: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(WINAPI) win32.foundation.LRESULT {
    switch (message) {
        ui.WM_CREATE => {
            std.log.info("WM_CREATE", .{});
            _ = winmm.PlaySoundW(win32.zig.L("SOUND_ID_CREATE"), //
                hinstance, winmm.SND_RESOURCE | winmm.SND_SYNC);
            _ = winmm.PlaySoundW(win32.zig.L("SOUND_ID_MUSIC"), hinstance, //
                winmm.SND_RESOURCE | winmm.SND_ASYNC | winmm.SND_LOOP);
        },
        ui.WM_PAINT => {
            var paint: win32.graphics.gdi.PAINTSTRUCT = undefined;
            _ = win32.graphics.gdi.BeginPaint(window, &paint);
            _ = win32.graphics.gdi.EndPaint(window, &paint);
        },
        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
            _ = winmm.PlaySoundW(null, hinstance, winmm.SND_PURGE);
            ui.PostQuitMessage(0);
        },
        else => return ui.DefWindowProc(window, message, wParam, lParam),
    }
    return 0;
}

pub fn wWinMain(h: H, _: ?H, _: [*:0]u16, _: u32) callconv(WINAPI) i32 {
    std.log.info("wWinMain", .{});
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.lpszClassName = name;
    windowClass.lpfnWndProc = mainWindowCallback;
    windowClass.hInstance = h;

    windowClass.hIcon = ui.LoadIcon(h, win32.zig.L("ICON_T3DX"));
    windowClass.hCursor = ui.LoadCursor(h, win32.zig.L("CURSOR_CROSSHAIR"));
    windowClass.hIconSm = ui.LoadIcon(h, win32.zig.L("ICON_T3DX"));

    if (ui.RegisterClassEx(&windowClass) == 0) win32Panic();

    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const window = ui.CreateWindowEx(
        ui.WS_EX_LEFT,
        name,
        name,
        style,
        ui.CW_USEDEFAULT,
        ui.CW_USEDEFAULT,
        640,
        480,
        null,
        null,
        h,
        null,
    );

    hinstance = h;
    hander = window orelse win32Panic();

    var message: ui.MSG = undefined;
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
    }

    std.log.info("wWinMain end", .{});
    return 0;
}

fn win32Panic() noreturn {
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

会出现播放声音的效果。

## 总结

通过 winmm 库，使用 PlaySound 播放声音。

## 附录
