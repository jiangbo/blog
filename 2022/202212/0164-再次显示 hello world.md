# 0164-再次显示 hello world

## 环境

- Time 2022-11-08
- WSL-Ubuntu 22.04
- QEMU 6.2.0
- NASM 2.15.05

## 前言

### 说明

参考：《x86汇编语言:从实模式到保护模式》李忠
参考：<https://wiki.osdev.org/Printing_To_Screen>

### 目标

在屏幕上显示 hello world 字符串。

## 汇编代码

```asm
; 显示 hello world
jmp start

data: db 'h',0x02,'e',0x02,'l',0x02,'l',0x02,'o',0x02,' ',0x02
      db 'w',0x02,'o',0x02,'r',0x02,'l',0x02,'d',0x02

start:

mov ax,0x07c0
mov ds,ax
mov ax,0xb800
mov es,ax

cld
mov si,data
mov di,0
mov cx,(start - data)/2

rep movsw

jmp start

times 510 - $ + $$ db 0
db 0x55
db 0xaa
```

## 数据区

和之前不同，这次将代码和数据进行了分离，将要显示的字符单独出来，形成了 data 段。

## 显示逻辑

使用的是 DS:SI 和 ES:DI 两个地址来控制，第一个控制数据，第二个控制显示。
rep 重复指令会检查 cx 是否为 0，不为 0 则继续重复。movsw 是传送字的指令。

## 标志寄存器

调试时，其中下面就是标志寄存器的内容。cld 控制 DF 方向标志，为 0 表示从小到大。
rep 会检查 ZF 零标志。

```text
OF <0>  DF <0>  IF <1>  TF <0>  SF <0>  ZF <0>  AF <0>  PF <0>  CF <0>
ID <0>  VIP <0> VIF <0> AC <0>  VM <0>  RF <0>  NT <0>  IOPL <0>
```

![显示hello][1]

## 总结

使用另一种方式来显示 hello world 字符串。了解 DS，SI，ES，DI 寄存器。
了解了 rep 和 movsw 指令。

[1]: images/display-hello2.png

## 附录
