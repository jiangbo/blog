# 0551-DirectX-wWinMain 主函数

## 环境

- Time 2024-06-26
- Zig 0.13.0-dev.351+64ef45eb0

## 前言

### 说明

参考资料：

1. 《Windows 游戏编程大师技巧》
2. <https://github.com/marlersoft/zigwin32/issues/9>

### 目标

从 main 主函数切换为 wWinMain 主函数。

## main.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const ui = win32.ui.windows_and_messaging;

const H = std.os.windows.HINSTANCE;
const WINAPI = std.os.windows.WINAPI;

pub fn wWinMain(_: H, _: ?H, _: [*:0]u16, _: u32) callconv(WINAPI) i32 {
    const caption = win32.zig.L("游戏编程");
    const message = win32.zig.L("Windows 游戏编程大师技巧");
    _ = ui.MessageBoxW(null, message, caption, ui.MB_OK);
    return 0;
}
```

## 效果

![消息框][1]

## 总结

效果和之前一样，但是将主函数进行了切换，从 main 函数修改为 wWinMain 函数。

[1]: images/directx01.png

## 附录
