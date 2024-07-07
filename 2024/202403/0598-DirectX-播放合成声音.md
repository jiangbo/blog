# 0598-DirectX-播放合成声音

## 目标

使用 DirectSound 播放合成声音。

## 环境

- Time 2024-07-07
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

感觉逻辑上还挺复杂的，可能是不懂声音相关的东西，花了挺久时间来调试和理解这部分代码。

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

    // this example does everything: it sets up directsound
    // creates a secondary buffer, loads it with a synthesizer
    // sine wave and plays it

    // allocate memory for buffer
    var sndBuffer = allocator.alloc(i8, sndBufferLength) catch unreachable;
    defer allocator.free(sndBuffer);

    // we need some data for the buffer, you could load a .VOC or .WAV
    // but as an example, lets synthesize the data

    // fill buffer with a synthesized 100hz sine wave
    for (0..sndBufferLength) |index| {
        const f1: f32 = @floatFromInt(index % 110);
        const f2 = 127.0 * @sin(6.28 * f1 / 110.0);
        sndBuffer[index] = @intFromFloat(@trunc(f2));
    }

    // note the math, 127 is the scale or amplitude
    // 6.28 is to convert to radians
    // (index % 110) read below
    // we are playing at 11025 hz or 11025 cycles/sec therefore, in 1 sec
    // we want 100 cycles of our synthesized sound, thus 11025/100 is approx.
    // 110, thus we want the waveform to repeat each 110 clicks of index, so
    // normalize to 110

    if (failed(sound.DirectSoundCreate8(null, @ptrCast(&sound8), null)))
        win32Panic();

    // set cooperation level
    if (failed(sound8.IDirectSound_SetCooperativeLevel(zigwin.hander, sound.DSSCL_NORMAL)))
        win32Panic();

    // // set up the format data structure

    wavFormat.wFormatTag = audio.WAVE_FORMAT_PCM;
    wavFormat.nChannels = 1;
    wavFormat.nSamplesPerSec = 11025;
    wavFormat.nBlockAlign = 1;
    wavFormat.nAvgBytesPerSec = wavFormat.nSamplesPerSec * wavFormat.nBlockAlign;
    wavFormat.wBitsPerSample = 8;
    wavFormat.cbSize = 0;

    soundDesc.dwSize = @sizeOf(sound.DSBUFFERDESC);
    soundDesc.dwFlags = //
        sound.DSBCAPS_STATIC | sound.DSBCAPS_LOCSOFTWARE;
    soundDesc.dwBufferBytes = sndBufferLength + 1;
    soundDesc.lpwfxFormat = &wavFormat;

    if (failed(sound8.IDirectSound_CreateSoundBuffer(&soundDesc, //
        @ptrCast(&soundBuffer), null))) win32Panic();

    // copy data into sound buffer
    var ptr1: [*]i8 = undefined;
    var ptr2: [*]i8 = undefined;
    var len1: u32 = undefined;
    var len2: u32 = undefined;
    if (failed(soundBuffer.IDirectSoundBuffer_Lock(0, sndBufferLength, //
        @ptrCast(&ptr1), &len1, @ptrCast(&ptr2), &len2, //
        sound.DSBLOCK_FROMWRITECURSOR))) win32Panic();

    // copy first section of circular buffer
    @memcpy(ptr1[0..len1], sndBuffer[0..len1]);
    // copy last section of circular buffer
    @memcpy(ptr2[0..len2], sndBuffer[len1..][0..len2]);

    // unlock the buffer
    if (failed(soundBuffer.IDirectSoundBuffer_Unlock(ptr1, len1, ptr2, len2)))
        win32Panic();

    // play the sound in looping mode
    if (failed(soundBuffer.IDirectSoundBuffer_Play(0, 0, sound.DSBPLAY_LOOPING)))
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
