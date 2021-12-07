# 【Serde】特性标签

## 环境

- Time 2021-12-07
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/feature-flags.html>  

## 示例

serde 有几个特性标签，可以在 Cargo 引入的时候，选择性打开。类似如下的方式：

```toml
serde = {version = "1", features = ["derive"]}
```

### derive

派生，如果需要使用属性宏自动生成序列化和反序列化方法，需要打开。

### std

默认，依赖 Rust 的标准库，实现了 Vec，HashMap 和其它类型的转换。

### unstable

不稳定功能版本。

### alloc

选择不依赖标准库，而是依赖核心库。

### rc

对引用计数类型 Rc 和 Arc 的支持。

## 总结

介绍了 serde 中的功能特性有哪几种可以选择。

## 附录
