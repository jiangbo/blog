# 【Tokio】mini-redis 服务器

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://tokio.rs/tokio/tutorial/setup>  

## 示例

### 查看版本

```text
C:\Users\jiangbo\work\rust\game>rustc --version
rustc 1.57.0 (f1edd0429 2021-11-29)
```

### 安装

`cargo install mini-redis`

### 启动

`mini-redis-server`

### 访问

```text
C:\Users\jiangbo\work\rust\game>mini-redis-cli get foo
(nil)
```

## 总结

搭建 mini redis server 环境。

## 附录
