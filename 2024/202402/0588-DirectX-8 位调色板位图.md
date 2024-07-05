# 0588-DirectX-8 位调色板位图

## 目标

加载带有 8 位调色板的位图，将其转化成 32 位 RGB 模式显示到屏幕上。

## 环境

- Time 2024-07-05
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

增加了想法，当日记写。书本里使用的调色板模式来显示，和 RGB 模式还是有点区别。
不过逻辑还是很清楚的，就是根据调色板查询到的 RGB 颜色直接设置到缓冲里就行。
使用 64 位编译的时候，偶尔程序崩溃，也不清楚到底是什么原因，大概 Windows 老的 API 对 64 位支持有点问题？

## win.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const zigwin = @import("win.zig");

const draw = win32.graphics.direct_draw;

pub const UNICODE: bool = true;
const WINAPI = std.os.windows.WINAPI;

var allocator: std.mem.Allocator = undefined;
var draw7: *draw.IDirectDraw7 = undefined;
var surfaceDes: draw.DDSURFACEDESC2 = undefined;
var surface: *draw.IDirectDrawSurface7 = undefined;

pub fn main() u8 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();

    zigwin.createWindow();

    gameInit();
    zigwin.update(gameUpdate);
    gameShutdown();

    return 0;
}

const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    if (failed(draw.DirectDrawCreateEx(null, @ptrCast(&draw7), //
        draw.IID_IDirectDraw7, null))) win32Panic();

    const style = draw.DDSCL_FULLSCREEN | draw.DDSCL_ALLOWMODEX |
        draw.DDSCL_EXCLUSIVE | draw.DDSCL_ALLOWREBOOT;

    // const style = draw.DDSCL_NORMAL;
    if (failed(draw7.IDirectDraw7_SetCooperativeLevel(zigwin.hander, style)))
        win32Panic();

    if (failed(draw7.IDirectDraw7_SetDisplayMode(zigwin.WIDTH, //
        zigwin.HEIGHT, 32, 0, 0))) win32Panic();

    surfaceDes = std.mem.zeroInit(draw.DDSURFACEDESC2, .{
        .dwSize = @sizeOf(draw.DDSURFACEDESC2),
        .dwFlags = draw.DDSD_CAPS,
        .ddsCaps = .{ .dwCaps = draw.DDSCAPS_PRIMARYSURFACE },
    });

    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, //
        @ptrCast(&surface), null))) win32Panic();

    loadBitmapFile("bitmap8.bmp") catch unreachable;
}

fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);

    if (failed(surface.IDirectDrawSurface7_Lock(null, &surfaceDes, //
        draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null))) win32Panic();

    const pitch32: usize = @intCast(surfaceDes.Anonymous1.lPitch >> 2);
    const buffer: [*]u32 = @ptrCast(@alignCast(surfaceDes.lpSurface));

    const width: usize = @intCast(bitmap.infoHeader.biWidth);
    for (0..@intCast(bitmap.infoHeader.biHeight)) |index| {
        const dest = buffer[index * pitch32 ..][0..width];
        const source = bitmap.colors[index * width ..][0..width];
        @memcpy(dest, source);
    }

    _ = surface.IDirectDrawSurface7_Unlock(null);

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    allocator.free(bitmap.colors);
    _ = surface.IUnknown_Release();
    _ = draw7.IUnknown_Release();
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}

const BitMap = struct {
    fileHeader: win32.graphics.gdi.BITMAPFILEHEADER,
    infoHeader: win32.graphics.gdi.BITMAPINFOHEADER,
    colors: []u32,
};

const bitmapId: u16 = 0x4D42;
var bitmap: BitMap = undefined;

fn loadBitmapFile(fileName: [:0]const u8) !void {
    const file = win32.storage.file_system;
    const windows = win32.system.windows_programming;

    // open the file if it exists
    var fileData: file.OFSTRUCT = undefined;
    const fileHandle = file.OpenFile(fileName, &fileData, file.OF_READ);
    if (fileHandle == -1) win32Panic();
    defer _ = windows._lclose(fileHandle);

    // now load the bitmap file header
    var len: u32 = @sizeOf(win32.graphics.gdi.BITMAPFILEHEADER);
    _ = windows._lread(fileHandle, &bitmap.fileHeader, len);

    // test if this is a bitmap file
    if (bitmap.fileHeader.bfType != bitmapId) @panic("not bitmap");

    // now we know this is a bitmap, so read in all the sections

    // first the bitmap infoheader

    // now load the bitmap file header
    len = @sizeOf(win32.graphics.gdi.BITMAPINFOHEADER);
    _ = windows._lread(fileHandle, &bitmap.infoHeader, len);

    // now load the color palette if there is one
    std.log.debug("bit count: {d}", .{bitmap.infoHeader.biBitCount});

    var palettes: [256]win32.graphics.gdi.PALETTEENTRY = undefined;
    _ = windows._lread(fileHandle, &palettes, @sizeOf(@TypeOf(palettes)));

    // // finally the image data itself
    const end = win32.media.multimedia.SEEK_END;
    const offset: i32 = @intCast(bitmap.infoHeader.biSizeImage);
    _ = windows._llseek(fileHandle, -offset, end);

    // allocate the memory for the image
    len = bitmap.infoHeader.biSizeImage;
    const buffer = try allocator.alloc(u8, len);
    defer allocator.free(buffer);

    _ = windows._lread(fileHandle, buffer.ptr, len);

    bitmap.colors = try allocator.alloc(u32, len);
    for (buffer, bitmap.colors) |value, *color| {
        const palette = palettes[value];
        color.* = @as(u24, @intCast(palette.peBlue)) << 16 //
        | @as(u24, @intCast(palette.peGreen)) << 8 | palette.peRed;
    }

    // flip the bitmap
    flipBitmap(bitmap.colors, @intCast(bitmap.infoHeader.biHeight));
}

fn flipBitmap(image: []u32, height: usize) void {
    // this function is used to flip bottom-up .BMP images

    // allocate the temporary buffer
    const buffer = allocator.dupe(u32, image) catch unreachable;
    defer allocator.free(buffer);

    // flip vertically
    const width = image.len / height;
    for (0..height) |index| {
        const source = buffer[index * width ..][0..width];
        const dest = image[(height - index - 1) * width ..][0..width];
        @memcpy(dest, source);
    }
}
```

## 效果

![8 位调色板位图][1]。

[1]: images/directx33.png

## 附录
