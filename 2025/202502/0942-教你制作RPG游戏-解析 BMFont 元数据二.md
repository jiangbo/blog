# 0942-教你制作RPG游戏-解析 BMFont 元数据二

## 目标

解析 BMFont 软件生成的二进制字体的元数据。

## 环境

- Time 2025-06-03
- Zig 0.14.1

## 参考

1. <https://www.bilibili.com/video/BV1eB4y197zi>
2. <https://angelcode.com/products/bmfont/>

## 想法

前一节的解析，要求结构体紧凑，这一节解析稍微复杂一点，但是不要求结构体字段紧凑。

## font.zig

去掉了 struct 前的 extern 关键字，不再要求结构体紧凑。

```zig
const std = @import("std");

pub const Font = struct {
    info: Info,
    common: Common,
    pages: []const []const u8,
    chars: []const Char,
    kerningPairs: []const KerningPair,
};

pub const Info = struct {
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
    name: []const u8 = &.{},
};

pub const Common = struct {
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

pub const Char = struct {
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

pub const KerningPair = struct { first: u32, second: u32, amount: i16 };
```

## bmfont.zig

解析元数据的实现，使用的直接读取然后映射到结构体中。

```zig
const std = @import("std");
const font = @import("font.zig");

pub fn parse(allocator: std.mem.Allocator, data: []const u8) void {
    arena = allocator;
    var stream = std.io.fixedBufferStream(data);
    doParse(stream.reader()) catch unreachable;
}

const BlockTag = enum(u8) { none, info, common, page, char, kerning };

fn doParse(reader: anytype) !void {
    {
        // 验证文件头
        var headerBuffer: [3]u8 = undefined;
        const index = try reader.readAll(&headerBuffer);

        if (!std.mem.eql(u8, headerBuffer[0..index], "BMF")) {
            return error.badHeader;
        }

        if (try reader.readByte() != 3) {
            return error.incompatibleVersion;
        }
    }

    try parseInfo(reader, try parseSize(reader, .info));
    try parseCommon(reader, try parseSize(reader, .common));
    try parsePage(reader, try parseSize(reader, .page));
    try parseChar(reader, try parseSize(reader, .char));
    try parseKerningPairs(reader, try parseSize(reader, .kerning));
}

fn parseSize(reader: anytype, tag: BlockTag) !usize {
    const actual = try reader.readEnum(BlockTag, .little);
    if (actual != tag) return error.unexpectedBlock;

    const len = try reader.readInt(i32, .little);
    std.log.info("block type: {} , size: {}", .{ tag, len });
    return @intCast(len);
}

pub var bmfont: font.Font = undefined;
pub var arena: std.mem.Allocator = undefined;

fn parseInfo(reader: anytype, _: usize) !void {
    bmfont.info = .{
        .fontSize = try reader.readInt(i16, .little),
        .bitField = try reader.readInt(u8, .little),
        .charSet = try reader.readInt(u8, .little),
        .stretchH = try reader.readInt(u16, .little),
        .aa = try reader.readInt(u8, .little),
        .paddingUp = try reader.readInt(u8, .little),
        .paddingRight = try reader.readInt(u8, .little),
        .paddingDown = try reader.readInt(u8, .little),
        .paddingLeft = try reader.readInt(u8, .little),
        .spacingHoriz = try reader.readInt(u8, .little),
        .spacingVert = try reader.readInt(u8, .little),
        .outline = try reader.readInt(u8, .little),
    };
    std.log.info("info: {any}", .{bmfont.info});

    const name = try reader.readUntilDelimiterAlloc(arena, 0, 256);
    std.log.info("font name: {s}", .{name});
    bmfont.info.name = name;
}

fn parseCommon(reader: anytype, _: usize) !void {
    bmfont.common = .{
        .lineHeight = try reader.readInt(u16, .little),
        .base = try reader.readInt(u16, .little),
        .scaleW = try reader.readInt(u16, .little),
        .scaleH = try reader.readInt(u16, .little),
        .pages = try reader.readInt(u16, .little),
        .bitField = try reader.readInt(u8, .little),
        .alphaChnl = try reader.readInt(u8, .little),
        .redChnl = try reader.readInt(u8, .little),
        .greenChnl = try reader.readInt(u8, .little),
        .blueChnl = try reader.readInt(u8, .little),
    };
    std.log.info("common: {any}", .{bmfont.common});
}

fn parsePage(reader: anytype, size: usize) !void {
    var len: usize = 0;

    var pages = std.ArrayListUnmanaged([]const u8).empty;

    while (len + 1 < size) {
        const name = try reader.readUntilDelimiterAlloc(arena, 0, 256);
        std.log.info("file name: {s}", .{name});
        len += name.len + 1;
        try pages.append(arena, name);
    }

    bmfont.pages = pages.toOwnedSlice(arena) catch unreachable;
}

fn parseChar(reader: anytype, size: usize) !void {
    const len: usize = @intCast(@divExact(size, 20));

    const chars = try arena.alloc(font.Char, len);
    std.log.info("char number: {d}", .{len});

    for (chars) |*char| {
        char.* = font.Char{
            .id = try reader.readInt(u32, .little),
            .x = try reader.readInt(u16, .little),
            .y = try reader.readInt(u16, .little),
            .width = try reader.readInt(u16, .little),
            .height = try reader.readInt(u16, .little),
            .xOffset = try reader.readInt(i16, .little),
            .yOffset = try reader.readInt(i16, .little),
            .xAdvance = try reader.readInt(i16, .little),
            .page = try reader.readInt(u8, .little),
            .chnl = try reader.readInt(u8, .little),
        };
    }
    bmfont.chars = chars;
}

fn parseKerningPairs(reader: anytype, size: usize) !void {
    const pairs: usize = @intCast(@divExact(size, 10));

    const kerningPairs = try arena.alloc(font.KerningPair, pairs);
    std.log.info("kerning pair number: {d}", .{pairs});

    for (kerningPairs) |*pair| {
        pair.* = font.KerningPair{
            .first = try reader.readInt(u32, .little),
            .second = try reader.readInt(u32, .little),
            .amount = try reader.readInt(i16, .little),
        };
    }
    bmfont.kerningPairs = kerningPairs;
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

    std.log.info("char numbers: {d}", .{font.bmfont.chars.len});
}
```

## 效果

![解析 BMFont][1]

[1]: images/教你制作RPG游戏46.png

## 附录
