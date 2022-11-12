# 0175-GDB 调试 multiboot 启动

## 环境

- Time 2022-11-12
- WSL-Ubuntu 22.04
- QEMU 6.2.0
- NASM 2.15.05

## 前言

### 说明

参考：<https://os.phil-opp.com/multiboot-kernel/>

### 目标

使用编写好的内核可执行文件，直接从 QEMU 启动，启动时暂停 CPU，使用 GDB 调试。

## 汇编代码

```text
section .multiboot_header
header_start:
    dd 0x1BADB002  ; 魔法数字，固定值
    dd 0
    dd -0x1BADB002 ; 定义的这三个数字相加需要等于0
header_end:

global start
section .text
bits 32
start:
    ; 向屏幕输出 `OK`。
    mov dword [0xb8000], 0x2f4b2f4f
    hlt
```

## 编译 debug 版本

`nasm -f elf32 -g boot.asm` -g 参数可以增加调试信息。

## 链接

linker.ld 文件内容：

```ld
ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}
```

链接命令：`ld -T linker.ld -m elf_i386 boot.o -o kernel.elf`

## 启动 QEMU

`qemu-system-x86_64 -kernel kernel.elf -display curses -s -S`

## 启动 GDB 调试

- 启动命令：`gdb kernel.elf`
- 设置架构：`set architecture i386:x86-64`
- 增加断点：`break start`，在入口增加了一个断点。

```text
root@jiangbo12490:~/git/game# gdb kernel.elf
set architecture i386:x86-64
Reading symbols from kernel.elf...
(gdb) set architecture i386:x86-64
The target architecture is set to "i386:x86-64".
(gdb) target remote :1234
Remote debugging using :1234
0x000000000010001b in ?? ()
(gdb) break start
Breakpoint 1 at 0x100010: file boot.asm, line 13.
```

## 效果

![调试QEMU启动][1]

通过 GDB 调试可以显示源码，单步调试时，自动显示下一条指令。
通过 CR0 寄存器，可以看到 PE，即保护模式已启用。

## 总结

从 QEMU 中启动 心中了调试信息的 kernel.elf，并且通过 GDB 连接调试。

[1]: images/gdb-debug-qemu.png

## 附录
