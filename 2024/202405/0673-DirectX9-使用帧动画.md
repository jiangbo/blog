# 0673-DirectX9-使用帧动画

## 目标

通过时间来控制图片的显示区域，形成帧动画。

## 环境

- Time 2024-10-05
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Teach Yourself Game Programming with DirectX in 21 Days》

## 想法

是基于表面来做的帧动画，不清楚后面是不是会使用纹理来做。

## d3d.zig

无变化。

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
    const device = d3d.initDirectX(800, 600);

    // 创建源表面
    var surface: *d3d9.IDirect3DSurface9 = undefined;
    win32Check(device.IDirect3DDevice9_CreateOffscreenPlainSurface(192, //
        384, .X8R8G8B8, .SYSTEMMEM, @ptrCast(&surface), null));
    defer _ = surface.IUnknown_Release();

    // 加载图片到源表面
    const fileName = win32.zig.L("player.bmp");
    const filter = std.math.maxInt(u32);
    win32Check(d3dx9.D3DXLoadSurfaceFromFileW(surface, null, null, fileName, null, filter, 0, 0));

    var index: i32 = 1;
    var message: ui.MSG = std.mem.zeroes(ui.MSG);
    while (true) {
        if (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            if (message.message == ui.WM_QUIT) break;
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }

        // get the time
        const system = win32.system.system_information;
        const start = system.GetTickCount64();

        const flags = win32.system.system_services.D3DCLEAR_TARGET;
        win32Check(device.IDirect3DDevice9_Clear(0, null, flags, 0x00000000, 0, 0));

        // 获取后备缓冲
        var back: *d3d9.IDirect3DSurface9 = undefined;
        win32Check(device.IDirect3DDevice9_GetBackBuffer(0, 0, .MONO, @ptrCast(&back)));

        // 将源表面绘制到后备缓冲
        const src = std.mem.zeroInit(win32.foundation.RECT, .{
            .left = 64 * (index - 1),
            .right = 64 * index,
            .bottom = 96,
        });
        const dest: win32.foundation.POINT = .{ .x = 350, .y = 200 };
        win32Check(device.IDirect3DDevice9_UpdateSurface(surface, &src, back, &dest));
        _ = back.IUnknown_Release();

        win32Check(device.IDirect3DDevice9_Present(null, null, null, null));

        index += 1;
        if (index > 3) index = 1;
        const ms = 200 -| (system.GetTickCount64() - start);
        std.time.sleep(ms * std.time.ns_per_ms);
    }
}
```

## 效果

![帧动画][1]

[1]: images/directx023.webp

## 附录
