# 0174-QEMU 从 kernel 启动

## 环境

- Time 2022-11-12
- WSL-Ubuntu 22.04
- QEMU 6.2.0
- NASM 2.15.05

## 前言

### 说明

参考：<https://os.phil-opp.com/multiboot-kernel/>
参考：<https://megtechcorner.medium.com/a-tutorial-on-os-and-compiler-3-5d14f7448415>

### 目标

使用编写好的内核可执行文件，直接从 QEMU 启动。

## kernel 参数

```text
-kernel bzImage
              Use bzImage as kernel image. The kernel can be either a Linux kernel or in multiboot format.
```

可以看到，可以直接从 multiboot 格式直接启动。
目前还不支持 multiboot2 格式的文件：<https://gitlab.com/qemu-project/qemu/-/issues/389>

## multiboot 格式

multiboot 格式和之前的 multiboot2 格式不太一样，需要重新编写程序。
规范参数文档：
<https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Specification>

其中的 3.1.1 和 3.1.2 节，对需要的字段和值进行了说明。

## 汇编代码

```asm
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

## 编译和链接

```text
root@jiangbo12490:~/git/game# nasm -f elf32 boot.asm
root@jiangbo12490:~/git/game# ld -T linker.ld -m elf_i386 boot.o  -o kernel.elf
```

## 启动 QEMU

```sh
qemu-system-x86_64 -kernel kernel.elf -display curses
```

可以看到和之前的制作的 ISO 文件启动效果一样。

## 总结

创建了一个内核可执行程序 kernel.elf，直接从 QEMU 中启动。

## 附录

### Remove restriction that prevents bootimg elf64 images

如果启动的时候，出现这个问题，表示 QEMU 还不支持从 elf64 启动。
可以参考这篇文件的编译和链接部分，将文件转化为 elf32 格式。
