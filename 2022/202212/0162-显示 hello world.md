# 0162-显示 hello world

## 环境

- Time 2022-11-07
- WSL-Ubuntu 22.04
- QEMU 6.2.0
- NASM 2.15.05

## 前言

### 说明

参考：《x86汇编语言:从实模式到保护模式》李忠
参考：<https://wiki.osdev.org/Printing_To_Screen>

### 目标

在屏幕上显示 hello 字符串。

## 显存

彩色的文本显示内存的地址是 0xB800，只要向这个地址写入字符和颜色，就可以直接显示。
其中使用两个字节来显示一个字，第一个字节对应字符的 ASCII 码，第二个对应具体的颜色。

## 颜色

| Color number | Color name | RGB value | Hex value |
| --- | --- | --- | --- |
| 0 | Black | 0 0 0 | 00 00 00 |
| 1 | Blue | 0 0 170 | 00 00 AA |
| 2 | Green | 0 170 0 | 00 AA 00 |
| 3 | Cyan | 0 170 170 | 00 AA AA |
| 4 | Red | 170 0 0 | AA 00 00 |
| 5 | Purple | 170 0 170 | AA 00 AA |
| 6 | Brown | 170 85 0 | AA 55 00 |
| 7 | Gray | 170 170 170 | AA AA AA |
| 8 | Dark Gray | 85 85 85 | 55 55 55 |
| 9 | Light Blue | 85 85 255 | 55 55 FF |
| 10 | Light Green | 85 255 85 | 55 FF 55 |
| 11 | Light Cyan | 85 255 255 | 55 FF FF |
| 12 | Light Red | 255 85 85 | FF 55 55 |
| 13 | Light Purple | 255 85 255 | FF 55 FF |
| 14 | Yellow | 255 255 85 | FF FF 55 |
| 15 | White | 255 255 255 | FF FF FF |

## 汇编代码

```asm
;显示 hello 字符串
mov ax,0xb800
mov ds,ax

mov byte[0x00],'h'
mov byte[0x01],0x02
mov byte[0x02],'e'
mov byte[0x03],0x02
mov byte[0x04],'l'
mov byte[0x05],0x02
mov byte[0x06],'l'
mov byte[0x07],0x02
mov byte[0x08],'o'
mov byte[0x09],0x02

times 510 - $ + $$ db 0
db 0x55
db 0xaa
```

在这里，借助了数据段 ds 的地址，不指定数据段的情况下，默认使用 ds 访问。

## 启动 QEMU

之前是通过 `-nographic` 启动的，因为现在需要显示，所以需要换个参数：

`qemu-system-i386 -s -S -drive format=raw,file=main -curses`

## 效果

可以看到，在最开始的地方，绿色的颜色显示了 hello 字符串。

![显示hello][1]

## 总结

介绍了 CPU 的十六位寄存器，了解了指令执行的时候的寻址方式，了解什么是大小端存储。

[1]: images/display-hello.png

## 附录
