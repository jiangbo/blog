# 0592-DirectX-显示缩放位图

## 目标

在上一节的基础上，对外星人进行任意缩放，然后显示到屏幕上。

## 环境

- Time 2024-07-07
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/erikyuzwa/windows-game-programming-gurus-source>

## 想法

加载原图片的时候没有变换，将其显示到屏幕上时，通过 blt 方法进行缩放的。

## win.zig

无变化。

## lib.zig

无变化。

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

const Alien = struct {
    frames: [3]*draw.IDirectDrawSurface7 = undefined, // 3 frames of animation for complete walk cycle
    x: i32,
    y: i32, // position of alien
    width: i32,
    height: i32,
    velocity: i32, // x-velocity
    current_frame: u32 = 0, // current frame of animation
    counter: i32 = 0, // used to time animation
};

var aliens: [3]Alien = undefined;

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

    var bitmap = lib.load8Bitmap(allocator, "alley8.bmp");
    background = createSurface(640, 480, false);

    surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
    surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);

    if (failed(background.IDirectDrawSurface7_Lock(null, &surfaceDes, //
        draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null))) win32Panic();

    var buffer: [*]u32 = @ptrCast(@alignCast(surfaceDes.lpSurface));

    @memcpy(buffer[0 .. 640 * 480], bitmap.colors);

    _ = background.IDirectDrawSurface7_Unlock(null);
    allocator.free(bitmap.colors);

    // initialize all the aliens

    // alien on level 1 of complex

    aliens[0] = Alien{
        .x = zigwin.rand.uintLessThan(u16, 640),
        .y = 116 - 72,
        .width = zigwin.rand.intRangeAtMost(i32, 36, 144),
        .height = zigwin.rand.intRangeAtMost(i32, 40, 160),
        .velocity = zigwin.rand.intRangeAtMost(i32, 2, 5),
    };
    aliens[0].y += (80 - aliens[0].height);

    // alien on level 2 of complex
    aliens[1] = Alien{
        .x = zigwin.rand.uintLessThan(u16, 640),
        .y = 246 - 72,
        .width = zigwin.rand.intRangeAtMost(i32, 36, 144),
        .height = zigwin.rand.intRangeAtMost(i32, 40, 160),
        .velocity = zigwin.rand.intRangeAtMost(i32, 2, 5),
    };
    aliens[1].y += (80 - aliens[1].height);

    // alien on level 3 of complex

    aliens[2] = Alien{
        .x = zigwin.rand.uintLessThan(u16, 640),
        .y = 382 - 72,
        .width = zigwin.rand.intRangeAtMost(i32, 36, 144),
        .height = zigwin.rand.intRangeAtMost(i32, 40, 160),
        .velocity = zigwin.rand.intRangeAtMost(i32, 2, 5),
    };
    aliens[2].y += (80 - aliens[2].height);

    // now load the bitmap containing the alien imagery
    // then scan the images out into the surfaces of alien[0]
    // and copy then into the other two, be careful of reference counts!

    // load the 8-bit image
    bitmap = lib.load8Bitmap(allocator, "Dedsp0.bmp");

    // create each surface and load bits

    const width = 72;
    for (0..aliens[0].frames.len) |index| {
        // create surface to hold image
        aliens[0].frames[index] = createSurface(width, 80, true);

        surfaceDes = std.mem.zeroes(draw.DDSURFACEDESC2);
        surfaceDes.dwSize = @sizeOf(draw.DDSURFACEDESC2);

        if (failed(aliens[0].frames[index].IDirectDrawSurface7_Lock(null, &surfaceDes, //
            draw.DDLOCK_SURFACEMEMORYPTR | draw.DDLOCK_WAIT, null))) win32Panic();

        buffer = @ptrCast(@alignCast(surfaceDes.lpSurface));
        for (0..80) |value| {
            const offset = 1 + index + index * width;
            const source = bitmap.colors[(value + 1) * 640 + offset ..][0..width];
            @memcpy(buffer[(value * width)..][0..width], source);
        }
        _ = aliens[0].frames[index].IDirectDrawSurface7_Unlock(null);
    }

    // unload the bitmap file, we no longer need it
    allocator.free(bitmap.colors);

    // now for the tricky part. There is no need to create more surfaces with the same
    // data, so I'm going to copy the surface pointers member for member to each alien
    // however, be careful, since the reference counts do NOT go up, you still only need
    // to release() each surface once!

    for (0..aliens.len) |index| {
        aliens[2].frames[index] = aliens[0].frames[index];
        aliens[1].frames[index] = aliens[2].frames[index];
    }
}
var animationSeq: [4]u32 = .{ 0, 1, 0, 2 };
fn gameUpdate() void {
    if (zigwin.windowClosed) return;

    // get the time
    const system = win32.system.system_information;
    const start = system.GetTickCount64();

    copySurface(background, 0, 0, 640, 480, backup, false);

    // move objects around
    for (&aliens) |*alien| {

        // move each object to the right at its given velocity
        alien.x += alien.velocity;

        // test if off screen edge, and wrap around
        if (alien.x > 640) alien.x = 80;

        // animate bot
        alien.counter += 1;
        if (alien.counter >= (8 - alien.velocity)) {
            // reset counter
            alien.counter = 0;

            // advance to next frame
            alien.current_frame += 1;
            if (alien.current_frame > 3) alien.current_frame = 0;
        } // end if
    } // end for index

    // // draw all the bots
    for (aliens) |alien| {
        // draw objects
        copySurfaceScale(alien.frames[animationSeq[alien.current_frame]], //
            alien.x, alien.y, 72, 80, alien.width, alien.height, backup, true);
    }

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
        .right = x + width,
        .bottom = y + height,
    };

    var sourceDect = std.mem.zeroes(win32.foundation.RECT);
    sourceDect.right = width;
    sourceDect.bottom = height;

    // test transparency flag

    if (transparent) {
        // enable color key blit
        // blt to destination surface
        if (failed(dest.IDirectDrawSurface7_Blt(&destRect, source, //
            &sourceDect, draw.DDBLT_WAIT | draw.DDBLT_KEYSRC, null)))
            win32Panic();
    } // end if
    else {
        // perform blit without color key
        // blt to destination surface
        if (failed(dest.IDirectDrawSurface7_Blt(&destRect, source, //
            &sourceDect, draw.DDBLT_WAIT, null))) win32Panic();
    }
}

fn copySurfaceScale(
    source: *draw.IDirectDrawSurface7, // source surface to draw
    x: i32,
    y: i32, // position to draw at
    srcWidth: i32,
    srcHeight: i32, // size of source surface
    destWidth: i32,
    destHeight: i32, // size of dests surface
    dest: *draw.IDirectDrawSurface7, // surface to draw the surface on
    transparent: bool,
) void {

    // draw a bob at the x,y defined in the BOB
    // on the destination surface defined in dest

    var destRect = win32.foundation.RECT{
        .left = x,
        .top = y,
        .right = x + destWidth,
        .bottom = y + destHeight,
    };

    var sourceDect = std.mem.zeroes(win32.foundation.RECT);
    sourceDect.right = srcWidth;
    sourceDect.bottom = srcHeight;

    // test transparency flag

    if (transparent) {
        // enable color key blit
        // blt to destination surface
        if (failed(dest.IDirectDrawSurface7_Blt(&destRect, source, //
            &sourceDect, draw.DDBLT_WAIT | draw.DDBLT_KEYSRC, null)))
            win32Panic();
    } // end if
    else {
        // perform blit without color key
        // blt to destination surface
        if (failed(dest.IDirectDrawSurface7_Blt(&destRect, source, //
            &sourceDect, draw.DDBLT_WAIT, null))) win32Panic();
    }
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}
```

## 效果

![位图缩放显示][1]

[1]: images/directx37.png

## 附录
