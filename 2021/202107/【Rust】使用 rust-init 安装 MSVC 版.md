# 【Rust】使用 rust-init 安装 MSVC 版

## 环境

- Windows 10

## 下载 rust-init

进入官网的下载页面：<https://www.rust-lang.org/tools/install>  
根据 32 还是 64 位系统选择下载不同的 exe 文件，下载完成后双击运行。

![下载 rust-init][1]

## 安装 Rust

### 前置条件说明

默认情况，Rust 依赖 `C++ build tools`，所以有一个提醒，并且提供了安装的方式。

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

### 默认安装

安装继续时，先是说明了会安装哪些工具，并且会安装到哪个目录，可以通过哪些方式修改，这里都直接默认。

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
>1
```

### 验证安装成功

使用命令 `rustc --version` 命令验证是否安装成功。

```txt
C:\Users\jiangbo>rustc --version
rustc 1.54.0 (a178d0322 2021-07-26)
```

## 解决链接错误

### linker `link.exe` not found

如果这时候直接新建项目运行，则会得到一个错误提示，完整的错误提示见附录。

### 下载生成工具

访问 <https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/> 点击下载生成工具。
下载完成后双击安装，然后确定下一步。

### 安装生成工具

安装完成后，提示重启计算机，其实不重启也不影响使用 Rust 的编译。

![install build tool][2]

### 验证运行

![rust compile msvc][3]

## 附录

### error: linker `link.exe` not found

解决方式参考上面的解决链接错误

```txt
> Executing task: C:\Users\JiangBo\.cargo\bin\cargo.exe run --package hello_world --bin hello_world <

   Compiling hello_world v0.1.0 (D:\workspace\rust\hello_world)
error: linker `link.exe` not found
  |
  = note: 系统找不到指定的文件。 (os error 2)

note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option

error: aborting due to previous error

error: could not compile `hello_world`

To learn more, run the command again with --verbose.
The terminal process "C:\Users\JiangBo\.cargo\bin\cargo.exe 'run', '--package', 'hello_world', '--bin', 'hello_world'" terminated with exit code: 101.

Terminal will be reused by tasks, press any key to close it.
```

[1]: images/download-rust-init.png
[2]: images/install-build-tool.png
[3]: images/rustc-compile-msvc.png
