# 0623-DirectX-显示 24 位位图

## 目标

加载 24 位的位图，并且将其显示在窗口上。

## 环境

- Time 2024-07-12
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

书本中的例子挺复杂的，正好之前写过一个加载位图的代码，直接拷贝过来用。

## win.zig

无变化。

## render.zig

无变化。

## lib.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const Allocator = std.mem.Allocator;

pub const BitMap = struct {
    fileHeader: win32.graphics.gdi.BITMAPFILEHEADER,
    infoHeader: win32.graphics.gdi.BITMAPINFOHEADER,
    colors: []u32,
};

const bitmapId: u16 = 0x4D42;

pub fn load8BitmapFile(allocator: Allocator, fileName: [:0]const u8) !BitMap {
    const file = win32.storage.file_system;
    const windows = win32.system.windows_programming;

    // open the file if it exists
    var fileData: file.OFSTRUCT = undefined;
    const fileHandle = file.OpenFile(fileName, &fileData, file.OF_READ);
    if (fileHandle == -1) win32Panic();
    defer _ = windows._lclose(fileHandle);

    var bitmap: BitMap = undefined;
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
    flipBitmap(allocator, bitmap.colors, @intCast(bitmap.infoHeader.biHeight));
    return bitmap;
}

pub fn load24BitmapFile(allocator: Allocator, fileName: [:0]const u8) !BitMap {
    const file = win32.storage.file_system;
    const windows = win32.system.windows_programming;

    // open the file if it exists
    var fileData: file.OFSTRUCT = undefined;
    const fileHandle = file.OpenFile(fileName, &fileData, file.OF_READ);
    if (fileHandle == -1) win32Panic();
    defer _ = windows._lclose(fileHandle);

    var bitmap: BitMap = undefined;
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

    // // finally the image data itself
    const end = win32.media.multimedia.SEEK_END;
    const offset: i32 = @intCast(bitmap.infoHeader.biSizeImage);
    _ = windows._llseek(fileHandle, -offset, end);

    // allocate the memory for the image
    len = bitmap.infoHeader.biSizeImage;
    const buffer = try allocator.alloc(u8, len);
    defer allocator.free(buffer);

    _ = windows._lread(fileHandle, buffer.ptr, len);

    bitmap.colors = try allocator.alloc(u32, len / 3);
    for (bitmap.colors, 0..) |*color, i| {
        color.* = @as(u24, @intCast(buffer[3 * i + 2])) << 16 //
        | @as(u24, @intCast(buffer[3 * i + 1])) << 8 | buffer[3 * i];
    }

    // flip the bitmap
    flipBitmap(allocator, bitmap.colors, @intCast(bitmap.infoHeader.biHeight));
    return bitmap;
}

fn flipBitmap(allocator: Allocator, image: []u32, height: usize) void {
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

pub fn win32Panic() noreturn {
    const err = win32.foundation.GetLastError();
    std.log.err("win32 painc code {}", .{@intFromEnum(err)});
    @panic(@tagName(err));
}
```

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const win = @import("win.zig");
const lib = @import("lib.zig");
const render = @import("render.zig");
const d3d9 = win32.graphics.direct3d9;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;

var device: *d3d9.IDirect3DDevice9 = undefined;
var surface: *d3d9.IDirect3DSurface9 = undefined;

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    win.createWindow();

    gameInit();
    win.update(gameUpdate);
    gameShutdown();
}

const failed = win32.zig.FAILED;
fn gameInit() void {
    std.log.info("gameInit", .{});

    render.init(win.WIDTH, win.HEIGHT, win.hander);
    device = render.device;

    // load in image
    const name = "tansdesk.bmp";
    const bmp = lib.load24BitmapFile(allocator, name) catch unreachable;
    defer allocator.free(bmp.colors);

    // set up image surface
    const width: u32 = @intCast(bmp.infoHeader.biWidth);
    const height: u32 = @intCast(bmp.infoHeader.biHeight);
    var hr = device.IDirect3DDevice9_CreateOffscreenPlainSurface( //
        width, height, render.mode.Format, //
        .SYSTEMMEM, @ptrCast(&surface), null);
    if (failed(hr)) win32Panic();

    var rect: d3d9.D3DLOCKED_RECT = undefined;
    hr = surface.IDirect3DSurface9_LockRect(&rect, null, 0);
    if (failed(hr)) win32Panic();

    var dest: [*]u32 = @ptrCast(@alignCast(rect.pBits));
    @memcpy(dest[0 .. width * height], bmp.colors);

    hr = surface.IDirect3DSurface9_UnlockRect();
    if (failed(hr)) win32Panic();
}

fn gameUpdate() void {
    if (win.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    const flags = win32.system.system_services.D3DCLEAR_TARGET;
    var hr = device.IDirect3DDevice9_Clear(0, null, flags, 0xffffffff, 0, 0);
    if (failed(hr)) win32Panic();

    // source rectangle
    var rect = std.mem.zeroes(win32.foundation.RECT);
    rect.right = win.WIDTH;
    rect.bottom = win.HEIGHT;

    const point = std.mem.zeroes(win32.foundation.POINT);

    // grab back buffer
    var back: ?*d3d9.IDirect3DSurface9 = undefined;
    hr = device.IDirect3DDevice9_GetBackBuffer(0, 0, .MONO, &back);
    if (failed(hr)) win32Panic();

    // copy rectangle
    hr = device.IDirect3DDevice9_UpdateSurface(surface, &rect, back, &point);
    if (failed(hr)) win32Panic();
    _ = back.?.IUnknown_Release();

    hr = device.IDirect3DDevice9_Present(null, null, null, null);
    if (failed(hr)) win32Panic();

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
}

fn win32Panic() noreturn {
    win.win32Panic();
}
```

## 效果

![显示位图][1]。

[1]: images/directx58.png

## 附录
