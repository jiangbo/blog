# 1101-阳光岛-创建 tiled 模块

## 目标

了解 Tiled 地图导出的数据格式，并且使用 ZON 导入到程序中来。

## 环境

- Time 2026-01-20
- Zig 0.15.1

## 参考

1. <https://www.bilibili.com/video/BV1jf9XYQEhW>

## 想法

先建立一个最简单的 tiled 模块。

## tiled.zig

定义了需要的数据结构。

```zig
const std = @import("std");

pub const Map = struct {
    height: u32,
    width: u32,

    tileWidth: u32,
    tileHeight: u32,
    layers: []const Layer,
    tileSets: []const TileSetRef,
};

pub const LayerEnum = enum { image, tile, object };

pub const Layer = struct {
    id: u32,
    image: u32,
    type: LayerEnum,

    width: u32 = 0,
    height: u32 = 0,
    opacity: f32,
    visible: bool,

    // tile 层特有
    data: []const u32,

    // 对象层特有
    objects: []const Object,

    // 图片层
    parallaxX: f32 = 1.0,
    parallaxY: f32 = 1.0,
    repeatX: bool = false,
    repeatY: bool = false,
};

pub const Object = struct {
    id: u32,
    name: []const u8,
    type: []const u8,

    gid: u32,

    x: f32,
    y: f32,

    width: f32,
    height: f32,

    rotation: f32,
    visible: bool,
};

pub const TileSetRef = struct { firstGid: u32, source: []const u8 };
```

## scene.zig

只贴出了 draw 函数。

```
...
pub fn draw() void {
    batch.beginDraw(.black);

    for (level1.layers) |*layer| {
        if (layer.type != .image) continue;
        batch.draw(layer.image, .zero);
    }

    if (isHelp) drawHelpInfo() else if (isDebug) drawDebugInfo();
    batch.endDraw();
}
...
```

## 效果

和之前一样的效果，只是用了 tiled 模块来加载地图数据。

![开篇][1]

[1]: images/阳光岛01.png

## 附录
