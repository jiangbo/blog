# 【Rust】树04-二叉搜索树

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
相比较二叉树，二叉搜索树的左节点都比父节点小，右节点都比父节点大。
基于二叉树来实现二叉搜索树，先实现插入和检索方法。

## 示例

### 引入模块

```rust
pub mod binary_search_tree;
```

### 结构定义

基于二叉树实现。

```rust
use std::cmp::Ordering;

use super::{binary_tree::BinaryTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct BinarySearchTree<T> {
    tree: BinaryTree<T>,
}
```

### 自定义方法

```rust
impl<T: Ord> BinarySearchTree<T> {
    pub(crate) fn root_mut(&mut self) -> &mut NodeRef<T> {
        self.tree.root_mut()
    }
    pub(crate) fn root(&self) -> &NodeRef<T> {
        self.tree.root()
    }
}
```

### 插入

```rust
fn insert(&mut self, value: T) {
    let mut current = self.root_mut();
    while let Some(node) = current {
        current = match value.cmp(&node.value) {
            Ordering::Less => &mut node.left,
            Ordering::Greater => &mut node.right,
            Ordering::Equal => return,
        };
    }
    *current = Node::new_node_ref(value)
}
```

### 检索

```rust
fn contains(&mut self, value: &T) -> bool {
    let mut current = self.root();
    while let Some(node) = current {
        current = match value.cmp(&node.value) {
            Ordering::Less => &node.left,
            Ordering::Greater => &node.right,
            Ordering::Equal => return true,
        };
    }
    false
}
```

### 其它方法

```rust
fn pre_order(&self) -> Vec<&T> {
    self.tree.pre_order()
}

fn in_order(&self) -> Vec<&T> {
    self.tree.in_order()
}

fn post_order(&self) -> Vec<&T> {
    self.tree.post_order()
}
```

## 总结

基于二叉树，实现了二叉搜索树的插入、检索方法。

## 附录

### 源码

```rust
use std::cmp::Ordering;

use super::{binary_tree::BinaryTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct BinarySearchTree<T> {
    tree: BinaryTree<T>,
}

impl<T: Ord> Tree<T> for BinarySearchTree<T> {
    fn pre_order(&self) -> Vec<&T> {
        self.tree.pre_order()
    }

    fn in_order(&self) -> Vec<&T> {
        self.tree.in_order()
    }

    fn post_order(&self) -> Vec<&T> {
        self.tree.post_order()
    }

    fn insert(&mut self, value: T) {
        let mut current = self.root_mut();
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                Ordering::Equal => return,
            };
        }
        *current = Node::new_node_ref(value)
    }

    fn contains(&mut self, value: &T) -> bool {
        let mut current = self.root();
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
                Ordering::Equal => return true,
            };
        }
        false
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub(crate) fn root_mut(&mut self) -> &mut NodeRef<T> {
        self.tree.root_mut()
    }
    pub(crate) fn root(&self) -> &NodeRef<T> {
        self.tree.root()
    }
}
```
