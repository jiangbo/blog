# 0676-DirectX9-解析 wave 文件

## 目标

读取 wave 文件，并解析文件中的结构。

## 环境

- Time 2024-10-05
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Teach Yourself Game Programming with DirectX in 21 Days》

## 想法

跟着书中的代码写好了步骤，为什么这么写还理解不到，可以先不弄懂，跟着例子继续往下。

## d3d.zig

无变化。

## d3dx9.zig

无变化。

## wave.zig

```zig
const std = @import("std");
const win32 = @import("win32");

pub const Wave = struct {
    allocator: std.mem.Allocator,
    format: win32.media.audio.WAVEFORMATEX,
    size: u32,
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
const Wave = @import("wave.zig").Wave;

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

    const wave = Wave.init(gpa.allocator(), win32.zig.L("MonsterHit.wav"));
    defer wave.deinit();
    std.log.debug("wave file init format: {any}", .{wave.format});

    const device = d3d.initDirectX(800, 600);
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

打印解析后的 wave 格式信息。

```text
debug: wave file init format: win32.media.audio.WAVEFORMATEX{
    .wFormatTag = 1,
    .nChannels = 1,
    .nSamplesPerSec = 11025,
    .nAvgBytesPerSec = 11025,
    .nBlockAlign = 1,
    .wBitsPerSample = 8,
    .cbSize = 24932
}
```

## 附录
