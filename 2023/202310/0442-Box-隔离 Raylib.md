# 0442-Box-隔离 Raylib

## 环境

- Time 2024-03-14
- Zig 0.12.0-dev.3152+90c1a2c41
- WSL-Ubuntu 22.04.3 LTS
- raylib 5.0

## 前言

### 说明

参考资料：

1. 《游戏开发：世嘉新人培训教材》
2. 图片素材：<https://www.ituring.com.cn/book/1742>

### 目标

当前程序中，对 Raylib 的引用到处都是，修改 file 模块为引擎层 engine，隔离对 Raylib 的引用。

## 效果

游戏该有的效果基本完成。

![box14][1]

## 总结

已经完成得差不多了，进入下一个章节。

[1]: images/box14.gif

## 附录

### 源码

<https://github.com/jiangbo/game/tree/main/zig/box/box6>
