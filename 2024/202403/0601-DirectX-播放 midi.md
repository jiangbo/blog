# 0601-DirectX-播放 midi

## 目标

播放 midi 文件。

## 环境

- Time 2024-07-08
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

DirectMusic 那些设置在新版里找不到，不清楚应该怎么设置。查看 win32 文件有个播放 midi 的代码，先拿来用。
看这本书就是想了解一下古老的 DirectDraw 的使用方法，到这里差不多了，后面的直接看一遍就可以了。

## win.zig

无变化。

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

    const name = win32.zig.L("battle.mid");
    playMIDIFile(name);
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

const media = win32.media.multimedia;
fn playMIDIFile(filename: [*:0]const u16) void {

    // Open the device by specifying the device and filename.
    // MCI will attempt to choose the MIDI mapper as the output port.
    var mciOpenParams: media.MCI_OPEN_PARMS = undefined;
    mciOpenParams.lpstrDeviceType = win32.zig.L("sequencer");
    mciOpenParams.lpstrElementName = filename;

    const param1 = media.MCI_OPEN_ELEMENT | media.MCI_OPEN_TYPE;
    const param2 = @intFromPtr(&mciOpenParams);
    var r = media.mciSendCommand(0, media.MCI_OPEN, param1, param2);
    std.log.debug("mci oepn command {}", .{r});

    // The device opened successfully; get the device ID.
    const deviceId = mciOpenParams.wDeviceID;

    // Check if the output port is the MIDI mapper.
    var mciStatusParams: media.MCI_STATUS_PARMS = undefined;
    mciStatusParams.dwItem = media.MCI_SEQ_STATUS_PORT;

    r = media.mciSendCommand(deviceId, media.MCI_STATUS, //
        media.MCI_STATUS_ITEM, @intFromPtr(&mciStatusParams));
    std.log.debug("mci status command: {}", .{r});

    // Begin playback. The window procedure function for the parent
    // window will be notified with an MM_MCINOTIFY message when
    // playback is complete. At this time, the window procedure closes
    // the device.
    var mciPlayParams: media.MCI_PLAY_PARMS = undefined;
    mciPlayParams.dwCallback = @intFromPtr(zigwin.instance);
    r = media.mciSendCommand(deviceId, media.MCI_PLAY, //
        media.MCI_NOTIFY, @intFromPtr(&mciPlayParams));
    std.log.debug("mci play command: {}", .{r});
}
```

## 附录
