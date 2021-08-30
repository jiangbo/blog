# 【Rust】使用 rust-init 安装 GNU 版

## 环境

- Windows 10

## 下载 rust-init

进入官网的下载页面：<https://www.rust-lang.org/tools/install>  
根据 32 还是 64 位系统选择下载不同的 exe 文件，下载完成后双击运行。

![下载 rust-init][1]

## 安装 Rust

### 前置条件说明

默认情况，Rust 依赖 `C++ build tools`，因为检查到电脑没有安装，所以有一个提醒，并且提供了安装的方式。  
最后也说明了，如果基于 `GNU ABI` 就可以不安装，并且询问是否继续，输入 `y` 回车继续。

```txt
Rust Visual C++ prerequisites

Rust requires the Microsoft C++ build tools for Visual Studio 2013 or later, but they don't seem to be installed.

The easiest way to acquire the build tools is by installing Microsoft Visual C++ Build Tools 2019 which provides just the Visual C++ build tools:

  https://visualstudio.microsoft.com/visual-cpp-build-tools/

Please ensure the Windows 10 SDK and the English language pack components are included when installing the Visual C++ Build Tools.

Alternately, you can install Visual Studio 2019, Visual Studio 2017, Visual Studio 2015, or Visual Studio 2013 and during install select the "C++ tools":

  https://visualstudio.microsoft.com/downloads/

Install the C++ build tools before proceeding.

If you will be targeting the GNU ABI or otherwise know what you are doing then it is fine to continue installation without the build tools, but otherwise, install the C++ build tools before proceeding.

Continue? (y/N) y
```

### 选择自定义安装

安装继续时，先是说明了会安装哪些工具，并且会安装到哪个目录，可以通过哪些方式修改，这里都直接默认。  
在最后，有三个选项，由于需要安装基于 `GNU` 的方式，所以选择 2，自定义安装。

```txt
Welcome to Rust!

This will download and install the official compiler for the Rust programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup home directory, located at:

  C:\Users\jiangbo\.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory located at:

  C:\Users\jiangbo\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to Cargo's bin directory, located at:

  C:\Users\jiangbo\.cargo\bin

This path will then be added to your PATH environment variable by modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and these changes will be reverted.

Current installation options:


   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>2
```

### 自定义安装配置

将 `x86_64-pc-windows-msvc` 修改为 `x86_64-pc-windows-gnu`，其他的可以直接选择默认。  
最后会确认安装信息，如果没有问题，直接回车安装，这时会到网上下载文件，要保证网络正常。

```txt
I'm going to ask you the value of each of these installation options. You may simply press the Enter key to leave unchanged.

Default host triple? [x86_64-pc-windows-msvc]
x86_64-pc-windows-gnu

Default toolchain? (stable/beta/nightly/none) [stable]


Profile (which tools and data to install)? (minimal/default/complete) [default]
complete

Modify PATH variable? (Y/n)
y


Current installation options:


   default host triple: x86_64-pc-windows-gnu
     default toolchain: stable
               profile: complete
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>

```

### 查看安装过程

```txt
info: profile set to 'complete'
info: setting default host triple to x86_64-pc-windows-gnu
info: syncing channel updates for 'stable-x86_64-pc-windows-gnu'
info: latest update on 2021-07-29, rust version 1.54.0 (a178d0322 2021-07-26)
warning: Force-skipping unavailable component 'miri-x86_64-pc-windows-gnu'
warning: Force-skipping unavailable component 'rust-analyzer-preview-x86_64-pc-windows-gnu'
info: downloading component 'cargo'
  6.7 MiB /   6.7 MiB (100 %)   5.5 MiB/s in  1s ETA:  0s
info: downloading component 'clippy'
info: downloading component 'llvm-tools-preview'
 69.6 MiB /  69.6 MiB (100 %)   5.5 MiB/s in 16s ETA:  0s
info: downloading component 'rls'
 11.1 MiB /  11.1 MiB (100 %)   3.6 MiB/s in  4s ETA:  0s
info: downloading component 'rust-analysis'
  2.6 MiB /   2.6 MiB (100 %)   1.9 MiB/s in  2s ETA:  0s
info: downloading component 'rust-docs'
 16.8 MiB /  16.8 MiB (100 %)   5.5 MiB/s in  3s ETA:  0s
info: downloading component 'rust-mingw'
info: downloading component 'rust-src'
info: downloading component 'rust-std'
 28.4 MiB /  28.4 MiB (100 %)   5.5 MiB/s in  5s ETA:  0s
info: downloading component 'rustc'
145.1 MiB / 145.1 MiB (100 %)   5.5 MiB/s in 28s ETA:  0s
info: downloading component 'rustfmt'
  6.0 MiB /   6.0 MiB (100 %)   5.5 MiB/s in  1s ETA:  0s
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'llvm-tools-preview'
 69.6 MiB /  69.6 MiB (100 %)  10.4 MiB/s in  6s ETA:  0s
info: installing component 'rls'
 11.1 MiB /  11.1 MiB (100 %)  10.8 MiB/s in  1s ETA:  0s
info: installing component 'rust-analysis'
info: installing component 'rust-docs'
 16.8 MiB /  16.8 MiB (100 %)   1.7 MiB/s in  9s ETA:  0s
info: installing component 'rust-mingw'
info: installing component 'rust-src'
info: installing component 'rust-std'
 28.4 MiB /  28.4 MiB (100 %)  10.7 MiB/s in  3s ETA:  0s
info: installing component 'rustc'
145.1 MiB / 145.1 MiB (100 %)   9.7 MiB/s in 14s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-pc-windows-gnu'

  stable-x86_64-pc-windows-gnu installed - (timeout reading rustc version)

Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload its PATH environment variable to include
Cargo's bin directory (%USERPROFILE%\.cargo\bin).

Press the Enter key to continue.
```

### 验证安装成功

使用命令 `rustc --version` 命令验证是否安装成功。

```txt
C:\Users\jiangbo1446>rustc --version
rustc 1.54.0 (a178d0322 2021-07-26)
```

## 附录

[1]: images/download-rust-init.png
