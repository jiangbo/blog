# 【Rust】树01-操作接口

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。

### 特点

学习树的过程中，一般先学简单的二叉树，前面已经学习过二叉树了。  
各种树越来越多，需要将代码分布到不同文件去。

## 示例

### 操作定义

```rust
pub trait Tree<T> {
    fn pre_order(&self) -> Vec<&T>;
    fn in_order(&self) -> Vec<&T>;
    fn post_order(&self) -> Vec<&T>;
    fn insert(&mut self, value: T);
    fn remove(&mut self, value: &T) -> Option<T>;
    fn contains(&mut self, value: &T) -> bool;
}
```

### 文件结构

```text
src
    - lib.rs
    tree
        - mod.rs
        - xxx.rs
```

### lib.rs

```rust
pub mod tree;
```

### 模块引入

```rust
pub mod xxx;
pub mod binary_tree;
```

### Node 定义

```rust
type NodeRef<T> = Option<Box<Node<T>>>;
pub(crate) struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T> Node<T> {
    fn new_node_ref(value: T) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}
```

## 总结

创建实现树结构的目录结构和定义了相关接口。

## 附录

### lib.rs 源码

```rust
pub mod tree;
```

### mod.rs 源码

```rust
type NodeRef<T> = Option<Box<Node<T>>>;
pub(crate) struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T> Node<T> {
    fn new_node_ref(value: T) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}
```
