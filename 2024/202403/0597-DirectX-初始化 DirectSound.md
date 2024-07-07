# 0597-DirectX-初始化 DirectSound

## 目标

初始化 DirectSound。

## 环境

- Time 2024-07-07
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

第八章的光栅化和变换，之前学习 OpenGL 的时候，已经了解过了，这里就直接看一遍，跳过代码部分。
第九章是 DirectInput，好像可以支持游戏手柄或者其它奇怪的输入设备，这里也暂时跳过，有需要再看。
当前章是声音部分，之前使用了 PlaySound 播放声音，这里再了解一下 DirectSound。
就是效果部分不好展示了，就不上图了，直接贴代码吧。

## win.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const zigwin = @import("win.zig");

const draw = win32.graphics.direct_draw;
const audio = win32.media.audio;
const sound = audio.direct_sound;
const music = audio.direct_music;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;
var draw7: *draw.IDirectDraw7 = undefined;

var sound8: *sound.IDirectSound8 = undefined;
var wavFormat: audio.WAVEFORMATEX = std.mem.zeroes(audio.WAVEFORMATEX);
var soundDesc: sound.DSBUFFERDESC = std.mem.zeroes(sound.DSBUFFERDESC);
var soundBuffer: *sound.IDirectSoundBuffer8 = undefined;

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
    initDirectSound();
}

fn initDirectDraw() void {
    std.log.info("initDirectDraw", .{});

    if (failed(draw.DirectDrawCreateEx(null, @ptrCast(&draw7), //
        draw.IID_IDirectDraw7, null))) win32Panic();

    const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel(zigwin.hander, style)))
        win32Panic();
}

const sndBufferLength: u32 = 64000;

fn initDirectSound() void {
    std.log.info("initDirectSound", .{});

    if (failed(sound.DirectSoundCreate8(null, @ptrCast(&sound8), null)))
        win32Panic();

    // set cooperation level
    if (failed(sound8.IDirectSound_SetCooperativeLevel(zigwin.hander, sound.DSSCL_NORMAL)))
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
