# 0600-DirectX-PlaySound 播放 wav

## 目标

使用之前定义的 PlaySound 方法来播放 wav 文件。

## 环境

- Time 2024-07-08
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

DirectSound 里面播放声音的方法还挺复杂的，后面应该会使用库或者新方法来简化。

## win.zig

无变化。

## winmm.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;
pub const LPCTSTR = [*:0]align(1) const u16;
const BOOL = win32.foundation.BOOL;

pub const SND_FILENAME: u32 = 0x00020000;
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
const zigwin = @import("win.zig");
const winmm = @import("winmm.zig");

const draw = win32.graphics.direct_draw;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;
var draw7: *draw.IDirectDraw7 = undefined;

const H = std.os.windows.HINSTANCE;
pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    zigwin.createWindow();

    gameInit();
    zigwin.update(gameUpdate);
    gameShutdown();
}

const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    initDirectDraw();

    const name = win32.zig.L("flight.wav");
    const flags = winmm.SND_ASYNC | winmm.SND_FILENAME;
    _ = winmm.PlaySoundW(name, zigwin.instance, flags);
}

fn initDirectDraw() void {
    std.log.info("initDirectDraw", .{});

    if (failed(draw.DirectDrawCreateEx(null, @ptrCast(&draw7), //
        draw.IID_IDirectDraw7, null))) win32Panic();

    const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel(zigwin.hander, style)))
        win32Panic();
}

fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = draw7.IUnknown_Release();
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}
```

## 附录
