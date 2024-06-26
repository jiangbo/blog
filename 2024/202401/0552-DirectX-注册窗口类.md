# 0552-DirectX-注册窗口类

## 环境

- Time 2024-06-26
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/marlersoft/zigwin32/issues/9>

### 目标

实现 win32 窗口类注册。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const ui = win32.ui.windows_and_messaging;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub const UNICODE: bool = true;
const className = win32.zig.L("游戏编程");

pub fn wWinMain(h: H, _: ?H, _: [*:0]u16, _: u32) callconv(WINAPI) i32 {
    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.lpszClassName = className;
    windowClass.lpfnWndProc = ui.DefWindowProcW;
    windowClass.hInstance = h;

    if (ui.RegisterClassEx(&windowClass) == 0) win32Panic();

    const message = win32.zig.L("Windows 游戏编程大师技巧");
    _ = ui.MessageBox(null, message, className, ui.MB_ICONEXCLAMATION);
    return 0;
}

fn win32Panic() void {
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![消息框][1]

## 总结

实现了窗口类的注册，如果发生错误，会打印出错的原因。

[1]: images/directx02.png

## 附录
