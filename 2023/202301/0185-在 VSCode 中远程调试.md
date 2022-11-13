# 0185-在 VSCode 中远程调试

## 环境

- Time 2022-11-13
- WSL-Ubuntu 22.04
- QEMU 6.2.0
- Rust 1.67.0-nightly
- VSCode 1.73.1

## 前言

### 说明

参考：<https://os.phil-opp.com/minimal-rust-kernel/>

### 目标

将上一节编写的可执行文件制作成 QEMU 可以启动的镜像。使用 QEMU 启动后，使用 VSCode 远程调试。

## 启动 QEMU 脚本

```bash
#! /usr/bin/bash

cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/mos/debug/bootimage-mos.bin \
    -display curses -s -S
```

## VSCode 调试配置

```json
{
    "configurations": [
        {
            "name": "mos",
            "type": "cppdbg",
            "request": "launch",
            "program": "${workspaceFolder}/target/mos/debug/mos",
            "MIMode": "gdb",
            "miDebuggerPath": "rust-gdb",
            "miDebuggerServerAddress": "localhost:1234",
            "cwd": ".",
            "setupCommands": [
                {
                    "description": "Enable pretty-printing for gdb",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                },
                {
                    "description": "Set Disassembly Flavor to Intel",
                    "text": "-gdb-set disassembly-flavor intel",
                    "ignoreFailures": true
                }
            ]
        }
    ]
}
```

## 效果

![VSCode调试][1]

## 总结

使用 Rust 编写了一个在 x64 平台上的独立可执行程序，使用 VSCode 远程调试它。

[1]: images/vscode-debug.png

## 附录
