# 0590-DirectX-使用离屏表面

## 目标

创建背景图片的离屏表面，在游戏循环的过程中，直接将离屏背景拷贝到备用缓冲区。

## 环境

- Time 2024-07-06
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

在创建背景图表面后，忘记了进行 unlock，导致图片一直显示不出来，调试了好久。

## win.zig

无变化。

## lib.zig

新增了一个 lib.zig 来存放加载位图的代码。

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
const zigwin = @import("win.zig");
const lib = @import("lib.zig");

const draw = win32.graphics.direct_draw;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;
var draw7: *draw.IDirectDraw7 = undefined;
var surfaceDes: draw.DDSURFACEDESC2 = undefined;
var surface: *draw.IDirectDrawSurface7 = undefined;
var backup: *draw.IDirectDrawSurface7 = undefined;
var background: *draw.IDirectDrawSurface7 = undefined;

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
        .Anonymous2 = .{ .dwBackBufferCount = 1 },
        .dwFlags = draw.DDSD_CAPS | draw.DDSD_BACKBUFFERCOUNT,
        .ddsCaps = .{
            .dwCaps = draw.DDSCAPS_PRIMARYSURFACE | //
                draw.DDSCAPS_COMPLEX | draw.DDSCAPS_FLIP,
        },
    });

    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, //
        @ptrCast(&surface), null))) win32Panic();

    surfaceDes.ddsCaps.dwCaps = draw.DDSCAPS_BACKBUFFER;
    if (failed(surface.IDirectDrawSurface7_GetAttachedSurface( //
        &surfaceDes.ddsCaps, @ptrCast(&backup)))) win32Panic();

    const bitmap = lib.load8BitmapFile(allocator, "alley8.bmp") catch unreachable;
    background = createSurface(640, 480, false);

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);

    if (failed(background.IDirectDrawSurface7_Lock(null, &surfaceDes, //
        draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null))) win32Panic();

    const buffer: [*]u32 = @ptrCast(@alignCast(surfaceDes.lpSurface));

    @memcpy(buffer[0 .. 640 * 480], bitmap.colors);

    _ = background.IDirectDrawSurface7_Unlock(null);
    allocator.free(bitmap.colors);
}

fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    copySurface(background, 0, 0, 640, 480, backup, false);
    while (failed(surface.IDirectDrawSurface7_Flip(null, draw.DDFLIP_WAIT))) {}

    const ms = 33 -| (system.GetTickCount64() - start);
    std.time.sleep(ms * std.time.ns_per_ms);
}

fn gameShutdown() void {
    std.log.info("gameShutdown", .{});
    _ = surface.IUnknown_Release();
    _ = draw7.IUnknown_Release();
}

const Surface = draw.IDirectDrawSurface7;
fn createSurface(width: u32, height: u32, colorKey: bool) *Surface {
    // this function creates an offscreen plain surface

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);
    surfaceDes.dwFlags = draw.DDSD_CAPS | draw.DDSD_HEIGHT | draw.DDSD_WIDTH;
    surfaceDes.dwWidth = width;
    surfaceDes.dwHeight = height;

    // set surface to offscreen plain
    surfaceDes.ddsCaps.dwCaps = draw.DDSCAPS_OFFSCREENPLAIN;

    // create the surface
    var sur: *Surface = undefined; // temporary surface
    if (failed(draw7.IDirectDraw7_CreateSurface(&surfaceDes, @ptrCast(&sur), null)))
        win32Panic();

    // test if user wants a color key
    if (colorKey) {
        // now set the color key for source blitting
        var key = .{ .dwColorSpaceLowValue = 0, .dwColorSpaceHighValue = 0 };
        _ = sur.IDirectDrawSurface7_SetColorKey(draw.DDCKEY_SRCBLT, &key);
    }
    return sur;
}

fn copySurface(
    source: *draw.IDirectDrawSurface7, // source surface to draw
    x: i32,
    y: i32, // position to draw at
    width: i32,
    height: i32, // size of source surface
    dest: *draw.IDirectDrawSurface7, // surface to draw the surface on
    transparent: bool,
) void {

    // draw a bob at the x,y defined in the BOB
    // on the destination surface defined in dest

    var destRect = win32.foundation.RECT{
        .left = x,
        .top = y,
        .right = x + width - 1,
        .bottom = y + height - 1,
    };

    var sourceDect = std.mem.zeroes(win32.foundation.RECT);
    sourceDect.right = width - 1;
    sourceDect.bottom = height - 1;

    // test transparency flag

    if (transparent) {
        // enable color key blit
        // blt to destination surface
        if (failed(dest.IDirectDrawSurface7_Blt(&destRect, source, //
            &sourceDect, (draw.DDBLT_WAIT | draw.DDBLT_KEYSRC), null)))
            win32Panic();
    } // end if
    else {
        // perform blit without color key
        // blt to destination surface
        if (failed(dest.IDirectDrawSurface7_Blt(&destRect, source, //
            &sourceDect, (draw.DDBLT_WAIT), null))) win32Panic();
    }
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}
```

## 效果

![背景表面][1]。

[1]: images/directx35.png

## 附录
