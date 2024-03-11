# 0430-Box-使用 DDS 素材

## 环境

- Time 2024-03-11
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- raylib 5.0

## 前言

### 说明

参考资料：

1. 《游戏开发：世嘉新人培训教材》
2. 图片素材：<https://www.ituring.com.cn/book/1742>

### 目标

上一节实现了加载和显示 DDS 图片的功能，这里替换之前的 png 图片，使用 DDS 格式的素材。

## init

```zig
    pub fn init(allocator: std.mem.Allocator, level: usize) ?Stage {
        texture = ray.LoadTexture("data/image/box.dds");
        return doInit(allocator, level) catch |e| {
            std.log.err("init stage error: {}", .{e});
            return null;
        };
    }
```

将 init 方法中的加载图片的格式修改为 DDS。

## mapItemToIndex

```zig
fn mapItemToIndex(item: MapItem) f32 {
    return switch (item) {
        .SPACE => 5,
        .WALL => 1,
        .BLOCK => 2,
        .GOAL => 4,
        .BLOCK_ON_GOAL => 3,
        .MAN => 0,
        .MAN_ON_GOAL => 0,
    };
}
```

图片素材的索引有点变化，所以修改索引值来匹配 DDS 图片素材。

## 效果

![box6][1]

## 总结

将 png 图片的素材，替换成了 DDS 格式的素材。

[1]: images/box6.png

## 附录
