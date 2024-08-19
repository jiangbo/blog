# 0660-DirectX9-ID3DXFont 接口

## 目标

通过 ID3DXFont 接口绘制文字，内部使用的 GDI 绘制。

## 环境

- Time 2024-08-19
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《DirectX 9.0 3D游戏开发编程基础》
2. 书本配套代码：<https://d3dcoder.net/Data/Book1/BookICode.zip>

## 想法

对 D3DX 中继承了解更深了一点，仿照 win32 写了继承，其中方法的顺序和重要。

## d3d.zig

无变化。

## d3dx9.zig

小修改了一点，去掉了 usingnamespace，也可以运行。

```zig
...
pub const ID3DXFont = extern struct {
    pub const VTable = extern struct {
        base: win32.system.com.IUnknown.VTable,
        _: [12]usize, // 其它方法的占位
        DrawTextW: *const fn (
            self: *const ID3DXFont,
            sprite: ?*anyopaque,
            str: LPCTSTR,
            count: i32,
            rect: *win32.foundation.RECT,
            format: u32,
            color: u32,
        ) callconv(std.os.windows.WINAPI) win32.foundation.HRESULT,
    };
    vtable: *const VTable,
    pub inline fn DrawTextW(
        self: *const ID3DXFont,
        sprite: ?*anyopaque,
        str: LPCTSTR,
        count: c_int,
        rect: *win32.foundation.RECT,
        format: u32,
        color: u32,
    ) i32 {
        return @as(*const ID3DXFont.VTable, @ptrCast(self.vtable)).DrawTextW( //
            @as(*const ID3DXFont, @ptrCast(self)), sprite, //
            str, count, rect, format, color);
    }
};
...
```

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

fn display(_: f32) bool {
    const flags = win32.system.system_services.D3DCLEAR_TARGET |
        win32.system.system_services.D3DCLEAR_ZBUFFER;

    _ = device.IDirect3DDevice9_Clear(0, null, flags, 0xffff00ff, 1, 0);
    _ = device.IDirect3DDevice9_BeginScene();

    var rect = std.mem.zeroes(win32.foundation.RECT);
    rect.right = WIDTH;
    rect.bottom = HEIGHT;
    const format: u32 = @bitCast(win32.graphics.gdi.DRAW_TEXT_FORMAT{});
    const str = win32.zig.L("This is D3DX Font");
    _ = font.DrawTextW(null, str, -1, &rect, format, 0xff000000);

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

![D3DXFont 文字][1]。

[1]: images/directx010.png

## 附录
