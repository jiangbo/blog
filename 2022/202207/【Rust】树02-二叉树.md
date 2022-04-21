# 【Rust】树02-二叉树

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
实现了二叉树的前序、中序和后序遍历。

## 示例

### 引入模块

```rust
pub mod binary_tree;
```

### 结构定义

```rust
use super::{NodeRef, Tree};

#[derive(Default)]
pub struct BinaryTree<T> {
    root: NodeRef<T>,
}
```

### 前序遍历

```rust
fn pre_order(&self) -> Vec<&T> {
    let mut result = Vec::new();
    let mut stack = vec![&self.root];
    while let Some(node) = stack.pop() {
        if let Some(node) = node {
            stack.push(&node.right);
            stack.push(&node.left);
            result.push(&node.value);
        }
    }
    result
}
```

### 中序遍历

```rust
fn in_order(&self) -> Vec<&T> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut current = &self.root;
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(current);
            current = &node.left;
        }
        current = stack.pop().unwrap();
        result.push(&current.as_ref().unwrap().value);
        current = &current.as_ref().unwrap().right;
    }
    result
}
```

### 后序遍历

```rust
fn post_order(&self) -> Vec<&T> {
    let mut stack = vec![&self.root];
    let mut result = vec![];
    while let Some(node) = stack.pop() {
        if let Some(node) = node {
            result.push(&node.value);
            stack.push(&node.left);
            stack.push(&node.right);
        }
    }
    result.reverse();
    result
}
```

## 其它方法

```rust
impl<T> BinaryTree<T> {
    pub(crate) fn root_mut(&mut self) -> &mut NodeRef<T> {
        &mut self.root
    }

    pub(crate) fn root(&self) -> &NodeRef<T> {
        &self.root
    }
}
```

### 其它未实现方法

```rust
fn insert(&mut self, _: T) {
    unimplemented!()
}

fn remove(&mut self, _: &T) -> Option<T> {
    unimplemented!()
}

fn contains(&mut self, _: &T) -> bool {
    unimplemented!()
}
```

## 总结

实现了二叉树的前序、中序和后序遍历方法，其它方法未实现。

## 附录

### 源码

```rust
use super::{NodeRef, Tree};

#[derive(Default)]
pub struct BinaryTree<T> {
    root: NodeRef<T>,
}

impl<T> Tree<T> for BinaryTree<T> {
    fn pre_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = vec![&self.root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                stack.push(&node.right);
                stack.push(&node.left);
                result.push(&node.value);
            }
        }
        result
    }

    fn in_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = &self.root;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(current);
                current = &node.left;
            }
            current = stack.pop().unwrap();
            result.push(&current.as_ref().unwrap().value);
            current = &current.as_ref().unwrap().right;
        }
        result
    }

    fn post_order(&self) -> Vec<&T> {
        let mut stack = vec![&self.root];
        let mut result = vec![];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                result.push(&node.value);
                stack.push(&node.left);
                stack.push(&node.right);
            }
        }
        result.reverse();
        result
    }

    fn insert(&mut self, _: T) {
        unimplemented!()
    }

    fn remove(&mut self, _: &T) -> Option<T> {
        unimplemented!()
    }

    fn contains(&mut self, _: &T) -> bool {
        unimplemented!()
    }
}

impl<T> BinaryTree<T> {
    pub(crate) fn root_mut(&mut self) -> &mut NodeRef<T> {
        &mut self.root
    }

    pub(crate) fn root(&self) -> &NodeRef<T> {
        &self.root
    }
}
```
