# 【Rust】在 vscode 中编写 Rust

## 环境

- Windows 10
- Rust 1.54.0
- VSCode 1.59.1

## 示例

根据传统，首先编写一个 Hello World 程序，然后使用 vscode 打开并运行。  
创建项目使用 `cargo new hello_world`。

### 使用 vscode 打开项目

![open rust project with vscode][1]

### Rust 插件

Rust 插件在 `2020/5/14` 到现在（2021-09）都没有更新了，暂时先不考虑安装。  
因为它和下面的插件会有冲突，如果两个都安装，会受到一个提示信息，只支持一个。

### rust-analyzer 插件

rust-analyzer 插件也可以支持 Rust，最近也在更新，所以安装它。  
安装完成后，写代码会有提示，在 main 函数上也有快捷的 run 和 debug。

![rust-analyzer in vscode][2]

### Better TOML

这个插件可以帮助编写 toml 配置文件，也可以安装上。

## 总结

使用 Cargo 新建了一个 Hello World 程序。

## 附录

[1]:images/open-with-vscode.png
[2]:images/rust-ra.png
