# 0941-教你制作RPG游戏-解析 BMFont 元数据

## 目标

解析 BMFont 软件生成的二进制字体的元数据。

## 环境

- Time 2025-06-03
- Zig 0.14.1

## 参考

1. <https://www.bilibili.com/video/BV1eB4y197zi>
2. <https://angelcode.com/products/bmfont/>

## 想法

游戏中需要显示中文，好像可以通过 BMFont 提前生成文字的图片，然后进行显示。

## font.zig

BMFont 的元数据格式的定义。名称 name 应该在 info 中，不太好解析，所以放到外面。

```zig
const std = @import("std");

pub const Font = struct {
    name: []const u8,
    info: Info,
    common: Common,
    pages: []const []const u8,
    chars: []const Char,
    kerningPairs: []const KerningPair,
};

pub const Info = extern struct {
    fontSize: i16,
    bitField: u8,
    charSet: u8,
    stretchH: u16,
    aa: u8,
    paddingUp: u8,
    paddingRight: u8,
    paddingDown: u8,
    paddingLeft: u8,
    spacingHoriz: u8,
    spacingVert: u8,
    outline: u8,
};

pub const Common = extern struct {
    lineHeight: u16,
    base: u16,
    scaleW: u16,
    scaleH: u16,
    pages: u16,
    bitField: u8,
    alphaChnl: u8,
    redChnl: u8,
    greenChnl: u8,
    blueChnl: u8,
};

pub const Char = extern struct {
    id: u32,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    xOffset: i16,
    yOffset: i16,
    xAdvance: i16,
    page: u8,
    chnl: u8,
};

pub const KerningPair = extern struct {
    first: u32 align(1),
    second: u32 align(1),
    amount: i16 align(1),
};
```

## bmfont.zig

解析元数据的实现，使用的直接读取然后映射到结构体中。

```zig
const std = @import("std");
const font = @import("font.zig");

pub fn parse(allocator: std.mem.Allocator, data: []const u8) font.Font {
    var arena = allocator;
    var result: font.Font = undefined;

    var buffer = data;
    {
        // 验证文件头
        if (!std.mem.eql(u8, buffer[0..3], "BMF"))
            @panic("invalid file header");

        if (buffer[3] != 3) @panic("incompatible version");
        buffer = buffer[4..];
    }
    {
        // info block
        if (buffer[0] != 1) @panic("error info block tag");
        const len: usize = std.mem.readInt(u32, buffer[1..5], .little);
        buffer = buffer[5..];

        result.info = std.mem.bytesToValue(font.Info, buffer);
        std.log.info("info: {any}", .{result.info});

        const name = buffer[@sizeOf(font.Info) .. len - 1];
        result.name = arena.dupe(u8, name) catch unreachable;
        std.log.info("font name: {s}", .{result.name});
        buffer = buffer[len..];
    }
    {
        // common block
        if (buffer[0] != 2) @panic("error common block tag");
        const len: usize = std.mem.readInt(u32, buffer[1..5], .little);
        buffer = buffer[5..];

        result.common = std.mem.bytesToValue(font.Common, buffer);
        std.log.info("common: {any}", .{result.common});
        buffer = buffer[len..];
    }
    {
        // page block
        if (buffer[0] != 3) @panic("error page block tag");
        const len: usize = std.mem.readInt(u32, buffer[1..5], .little);
        buffer = buffer[5..];

        var pages = std.ArrayListUnmanaged([]const u8).empty;
        var readLength: usize = 0;
        while (readLength < len) {
            const name = std.mem.sliceTo(buffer, 0);
            std.log.info("file name: {s}", .{name});
            readLength += name.len + 1;
            pages.append(arena, name) catch unreachable;
        }
        result.pages = pages.toOwnedSlice(arena) catch unreachable;
        buffer = buffer[len..];
    }
    {
        // char block
        if (buffer[0] != 4) @panic("error char block tag");
        const len: usize = std.mem.readInt(u32, buffer[1..5], .little);
        buffer = buffer[5..];

        const charsCount: usize = @divExact(len, @sizeOf(font.Char));
        const chars = arena.alloc(font.Char, charsCount) catch unreachable;
        std.log.info("char number: {d}", .{charsCount});

        for (chars) |*char| {
            char.* = std.mem.bytesToValue(font.Char, buffer);
            buffer = buffer[@sizeOf(font.Char)..];
        }
        result.chars = chars;
    }
    {
        // kerning block
        if (buffer[0] != 5) @panic("error kerning block tag");
        const len: usize = std.mem.readInt(u32, buffer[1..5], .little);
        buffer = buffer[5..];

        const pairsCount: usize = @divExact(len, @sizeOf(font.KerningPair));
        const kerningPairs = arena.alloc(font.KerningPair, pairsCount) catch unreachable;
        std.log.info("kerning pair number: {d}", .{pairsCount});

        for (kerningPairs) |*pair| {
            pair.* = std.mem.bytesToValue(font.KerningPair, buffer);
            buffer = buffer[@sizeOf(font.KerningPair)..];
        }
        result.kerningPairs = kerningPairs;
    }

    if (buffer.len != 0) @panic("unexpected data at the end of file");
    return result;
}
```

## main.zig

验证是否解析成功。

```zig
const std = @import("std");

pub fn main() void {
    const font = @import("bmfont.zig");

    const data = @embedFile("6.fnt");
    const allocator = std.heap.c_allocator;
    var arenaAllocator = std.heap.ArenaAllocator.init(allocator);
    defer arenaAllocator.deinit();

    const result = font.parse(allocator, data);

    std.log.info("char numbers: {d}", .{result.chars.len});
}
```

## 效果

![解析 BMFont][1]

[1]: images/教你制作RPG游戏45.png

## 附录
