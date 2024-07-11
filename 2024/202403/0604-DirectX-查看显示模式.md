# 0604-DirectX-查看显示模式

## 目标

查看显示适配器支持的显示模式。

## 环境

- Time 2024-07-11
- Zig 0.13.0-dev.351+64ef45eb0

## 参考

1. 《Direct3D 中的 2D 编程》

## 想法

接口有一个点变化，版本 8 的接口不需要 format 参数，版本 9 的接口需要，直接遍历给它。

## win.zig

无变化。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const zigwin = @import("win.zig");

const d3d9 = win32.graphics.direct3d9;

pub const UNICODE: bool = true;

var allocator: std.mem.Allocator = undefined;
var d9: *d3d9.IDirect3D9 = undefined;

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

    d9 = d3d9.Direct3DCreate9(d3d9.D3D_SDK_VERSION).?;

    var count = d9.IDirect3D9_GetAdapterCount();
    std.log.debug("adapter count: {d}", .{count});

    var identifier: d3d9.D3DADAPTER_IDENTIFIER9 = undefined;

    for (0..count) |adapter| {
        const i: u32 = @intCast(adapter);
        const r = d9.IDirect3D9_GetAdapterIdentifier(i, 0, &identifier);
        if (failed(r)) win32Panic();

        std.log.debug("adapter Driver: {s}", .{identifier.Driver});
        std.log.debug("adapter name: {s}", .{identifier.Description});
    }

    const adapter = d3d9.D3DADAPTER_DEFAULT;

    var caps: d3d9.D3DCAPS9 = undefined;
    var r = d9.IDirect3D9_GetDeviceCaps(adapter, .HAL, &caps);
    if (failed(r)) win32Panic();

    const formats = std.enums.values(d3d9.D3DFORMAT);
    var mode: d3d9.D3DDISPLAYMODE = undefined;
    for (formats) |format| {
        count = d9.IDirect3D9_GetAdapterModeCount(adapter, format);
        if (count == 0) continue;
        std.log.debug("adapter mode format: {} count: {}", .{ format, count });

        for (0..count) |value| {
            const c: u32 = @intCast(value);
            r = d9.IDirect3D9_EnumAdapterModes(adapter, format, c, &mode);
            if (failed(r)) win32Panic();
            std.log.debug("adapter display mode: {any}", .{mode});
        }
    }
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
    _ = d9.IUnknown_Release();
}

fn win32Panic() noreturn {
    zigwin.win32Panic();
}
```

## 附录
