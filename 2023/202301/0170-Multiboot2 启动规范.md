# 0170-Multiboot2 启动规范

## 环境

- Time 2022-11-11
- WSL-Ubuntu 22.04
- QEMU 6.2.0
- NASM 2.15.05

## 前言

### 说明

参考：<https://os.phil-opp.com/multiboot-kernel/>

### 目标

编写一个符合 multiboot2 规范的启动文件。

## multiboot2 规范

<https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Header-tags>
规范定义文档如上，其中的 3.1.1，3.1.2，3.1.3 介绍了启动文件需要符合的格式。

| Field | Type | Value |
| --- | --- | --- |
| magic number | u32 | `0xE85250D6` |
| architecture | u32 | `0` for i386, `4` for MIPS |
| header length | u32 | total header size, including tags |
| checksum | u32 | `-(magic + architecture + header_length)` |
| tags | variable |  |
| end tag | (u16, u16, u32) | `(0, 0, 8)` |

可以看到上面定义的都是无符号数，其中的 checksum（校验和）+ magic + architecture + header_length 需要等于零。要使无符号数 u32 等于 0，可以使其刚好产生溢出，结果回到 0，即（0x100000000）。

## 汇编代码

```asm
section .multiboot_header
header_start:
    dd 0xe85250d6                ; 魔法数字，固定值
    dd 0                         ; 0 表示进入 32 位保护模式
    dd header_end - header_start ; 头文件的长度
    ; 校验和，因为都是使其加起来一共等于 0
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; 可选的标签

    ; 结束标签
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:
```

## 编译和查看机器码

```text
root@jiangbo12490:~/git/game# nasm main.asm
root@jiangbo12490:~/git/game# hexdump -x main
0000000    50d6    e852    0000    0000    0018    0000    af12    17ad
0000010    0000    0000    0008    0000
0000018
root@jiangbo12490:~/git/game#
```

## 总结

了解了 multiboot2 的启动规范，定义和实现了其 header 汇编程序。

## 附录
