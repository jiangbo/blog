# 0677-DirectX9-DirectSound 播放 wave

## 目标

在上一节的基础上，使用解析出来的 wave 文件格式，设置 DirectSound 进行播放。

## 环境

- Time 2024-10-05
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Teach Yourself Game Programming with DirectX in 21 Days》

## 想法

之前写过一次 DirectSound 播放 wav，之前使用了一个库，这里没有，可以参考之前的例子。

## d3d.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Wave = @import("wave.zig").Wave;

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;

pub fn windowCallback(
    w: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(std.os.windows.WINAPI) win32.foundation.LRESULT {
    switch (message) {
        ui.WM_CREATE => {
            std.log.info("WM_CREATE", .{});
        },
        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
            ui.PostQuitMessage(0);
        },
        ui.WM_KEYDOWN => {
            if (wParam == ' ') {
                win32Check(soundBuffer.IDirectSoundBuffer_Play(0, 0, sound.DSBPLAY_LOOPING));
            }
        },
        else => return ui.DefWindowProc(w, message, wParam, lParam),
    }
    return 0;
}

pub fn initDirectX(width: i32, height: i32) *d3d9.IDirect3DDevice9 {
    const h = win32.system.library_loader.GetModuleHandle(null).?;
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);

    const className = win32.zig.L("TeachYourselfDirectX9");
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1 };
    windowClass.lpszClassName = className;
    windowClass.lpfnWndProc = windowCallback;
    windowClass.hInstance = h;

    win32Check(ui.RegisterClassEx(&windowClass));
    var style = ui.WS_OVERLAPPEDWINDOW;
    style.VISIBLE = 1;
    const name = win32.zig.L("2D 游戏开发");
    window = ui.CreateWindowEx(ui.WS_EX_LEFT, className, name, //
        style, 200, 200, width, height, null, null, h, null).?;

    var d9 = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;
    defer _ = d9.IUnknown_Release();

    const adapter = d3d9.D3DADAPTER_DEFAULT;
    var mode: d3d9.D3DDISPLAYMODE = undefined;
    win32Check(d9.IDirect3D9_GetAdapterDisplayMode(adapter, &mode));

    var params = std.mem.zeroes(d3d9.D3DPRESENT_PARAMETERS);

    // 后备缓冲区信息
    params.BackBufferWidth = @intCast(width);
    params.BackBufferHeight = @intCast(height);
    params.BackBufferFormat = mode.Format;
    params.BackBufferCount = 1; // 使用一个后备缓冲

    // 交换效果
    params.SwapEffect = .DISCARD;
    params.Windowed = win32.zig.TRUE; // 窗口模式

    // 渲染的目的窗口
    params.hDeviceWindow = window;

    // 创建设备
    var device: *d3d9.IDirect3DDevice9 = undefined;
    win32Check(d9.IDirect3D9_CreateDevice(adapter, .HAL, window, //
        d3d9.D3DCREATE_HARDWARE_VERTEXPROCESSING, &params, @ptrCast(&device)));

    return device;
}

var window: win32.foundation.HWND = undefined;
var soundBuffer: *sound.IDirectSoundBuffer8 = undefined;
const sound = win32.media.audio.direct_sound;

pub fn initDirectSound(allocator: std.mem.Allocator) void {

    // 初始化 DirectSound
    var sound8: *sound.IDirectSound8 = undefined;
    win32Check(sound.DirectSoundCreate8(null, @ptrCast(&sound8), null));
    win32Check(sound8.IDirectSound_SetCooperativeLevel(window, sound.DSSCL_NORMAL));

    // 初始化 wave 文件
    var wave = Wave.init(allocator, win32.zig.L("MonsterHit.wav"));
    defer wave.deinit();

    // 设置缓冲区格式
    var soundDesc = std.mem.zeroes(sound.DSBUFFERDESC);
    soundDesc.dwSize = @sizeOf(sound.DSBUFFERDESC);
    soundDesc.dwBufferBytes = @intCast(wave.data.len);
    soundDesc.lpwfxFormat = &wave.format;

    // 创建缓冲区
    win32Check(sound8.IDirectSound_CreateSoundBuffer(&soundDesc, //
        @ptrCast(&soundBuffer), null));

    // 将声音数据复制到缓冲区
    var ptr1: [*]i8, var ptr2: [*]i8 = .{ undefined, undefined };
    var len1: u32, var len2: u32 = .{ undefined, undefined };
    win32Check(soundBuffer.IDirectSoundBuffer_Lock(0, @intCast(wave.data.len), //
        @ptrCast(&ptr1), &len1, @ptrCast(&ptr2), &len2, sound.DSBLOCK_FROMWRITECURSOR));
    @memcpy(ptr1, wave.data);
    win32Check(soundBuffer.IDirectSoundBuffer_Unlock(ptr1, len1, ptr2, len2));
}

pub fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    const err = win32.foundation.GetLastError();
    std.log.err("win32 panic code 0X{0X}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## d3dx9.zig

无变化。

## wave.zig

```zig
const std = @import("std");
const win32 = @import("win32");

pub const Wave = struct {
    allocator: std.mem.Allocator,
    format: win32.media.audio.WAVEFORMATEX,
    data: []i8,

    pub fn init(allocator: std.mem.Allocator, name: [*:0]const u16) Wave {
        const media = win32.media.multimedia;
        const flags = media.MMIO_ALLOCBUF | media.MMIO_READ;
        const mmio = media.mmioOpenW(@constCast(@ptrCast(name)), null, flags);
        if (mmio == null) @panic("failed to open wave file");
        defer _ = media.mmioClose(mmio, 0);

        var riff: media.MMCKINFO = undefined;
        riff.fccType = media.mmioStringToFOURCCW(win32.zig.L("WAVE"), 0);
        var result = media.mmioDescend(mmio, &riff, null, media.MMIO_FINDRIFF);
        if (result != win32.media.MMSYSERR_NOERROR) @panic("failed to descend riff");

        var chunk: media.MMCKINFO = undefined;
        chunk.ckid = media.mmioStringToFOURCCW(win32.zig.L("fmt "), 0);
        result = media.mmioDescend(mmio, &chunk, &riff, media.MMIO_FINDCHUNK);
        if (result != win32.media.MMSYSERR_NOERROR) @panic("failed to descend chunk");

        var wave: Wave = std.mem.zeroInit(Wave, .{ .allocator = allocator });
        const size = @sizeOf(@TypeOf(wave.format));
        var read = media.mmioRead(mmio, @ptrCast(&wave.format), size);
        if (read == -1) @panic("failed to read format");

        result = media.mmioAscend(mmio, &chunk, 0);
        if (result != win32.media.MMSYSERR_NOERROR) @panic("failed to ascend chunk");

        chunk.ckid = media.mmioStringToFOURCCW(win32.zig.L("data"), 0);
        result = media.mmioDescend(mmio, &chunk, &riff, media.MMIO_FINDCHUNK);
        if (result != win32.media.MMSYSERR_NOERROR) @panic("failed to descend chunk");

        wave.data = allocator.alloc(i8, chunk.cksize) catch unreachable;
        read = media.mmioRead(mmio, @ptrCast(wave.data.ptr), @intCast(wave.data.len));
        if (result == -1) @panic("failed to read data");

        return wave;
    }

    pub fn deinit(self: Wave) void {
        self.allocator.free(self.data);
    }
};
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const win32Check = d3d.win32Check;

pub const UNICODE: bool = true;

const Vertex = extern struct {
    x: f32 = 0,
    y: f32 = 0,
    z: f32 = 0,
    rhw: f32 = 1,
    u: f32 = 0,
    v: f32 = 0,
};

const WIDTH = 800;
const HEIGHT = 600;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const device = d3d.initDirectX(800, 600);
    d3d.initDirectSound(gpa.allocator());

    _ = device;
    var message: ui.MSG = std.mem.zeroes(ui.MSG);
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
    }
}
```

## 效果

运行时，按空格键可以听到声音。

## 附录
