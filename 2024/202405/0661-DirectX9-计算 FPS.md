# 0661-DirectX9-计算 FPS

## 目标

计算程序运行时的 FPS（frames per second），并以文字的形式渲染到界面上。

## 环境

- Time 2024-08-19
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

之前间隔时间这块没有处理好，现在刚好重新梳理一下。

## d3d.zig

通过 `PresentationInterval` 参数设置间隔时间，之前是默认的。
如果想尽快渲染，可以修改成 `0x80000000`，修改后大概有 3000 的 FPS。

```zig
...
pub fn enterMsgLoop(display: fn (f32) bool) void {
    var timer = std.time.Timer.start() catch unreachable;
    var message: ui.MSG = std.mem.zeroes(ui.MSG);
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        } else {
            const delta: f32 = @floatFromInt(timer.lap());
            _ = display(delta / std.time.ns_per_s);
        }
    }
}
...
```

## d3dx9.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const d3d = @import("d3d.zig");
const d3dx9 = @import("d3dx9.zig");

const d3d9 = win32.graphics.direct3d9;
const ui = win32.ui.windows_and_messaging;
const failed = win32.zig.FAILED;

// Globals
// var allocator: std.mem.Allocator = undefined;
pub const UNICODE: bool = true;
var device: *d3d9.IDirect3DDevice9 = undefined;

var font: *d3dx9.ID3DXFont = undefined;

// Framework Functions
fn setup() bool {

    // 初始化字体描述
    var lf = std.mem.zeroes(win32.graphics.gdi.LOGFONTW);

    lf.lfHeight = 25; // in logical units
    lf.lfWidth = 12; // in logical units
    lf.lfWeight = 500; // boldness, range 0(light) - 1000(bold)
    lf.lfCharSet = 1;

    const name = win32.zig.L("Times New Roman");
    @memcpy(lf.lfFaceName[0..name.len], name);

    _ = d3dx9.D3DXCreateFontIndirectW(device, &lf, &font);
    return true;
}

fn cleanup() void {}

var frameCount: f32 = 0;
var timeElapsed: f32 = 0;
var fps: f32 = 0;
var utf8Buffer: [40]u8 = undefined;
var utf16Buffer: [20]u16 = undefined;
var size: usize = 0;
fn display(delta: f32) bool {
    // Update: Compute the frames per second.
    frameCount += 1;
    timeElapsed += delta;

    if (timeElapsed >= 1.0) {
        fps = frameCount / timeElapsed;
        const utf8 = std.fmt.bufPrint(&utf8Buffer, "fps: {d:.3}", .{fps}) catch unreachable;
        size = std.unicode.utf8ToUtf16Le(&utf16Buffer, utf8) catch unreachable;
        timeElapsed = 0.0;
        frameCount = 0;
    }

    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    var rect = std.mem.zeroes(win32.foundation.RECT);
    rect.right = WIDTH;
    rect.bottom = HEIGHT;
    const format: u32 = @bitCast(win32.graphics.gdi.DRAW_TEXT_FORMAT{});
    _ = font.DrawTextW(null, &utf16Buffer, size, &rect, format, 0xff000000);

    _ = device.IDirect3DDevice9_EndScene();
    _ = device.IDirect3DDevice9_Present(null, null, null, null);

    return true;
}

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

// main
pub fn main() void {
    device = d3d.initD3D(WIDTH, HEIGHT);

    if (!setup()) @panic("Setup() - FAILED");

    d3d.enterMsgLoop(display);

    cleanup();
    _ = device.IUnknown_Release();
}
```

## 效果

![渲染 FPS][1]

[1]: images/directx011.png

## 附录
