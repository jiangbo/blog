# 【Rust】树07-平衡二叉树

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
二叉树有个不好的地方，就是在最坏情况下，可能退化成单链表的结构。  
比如按从小到大的顺序插入，节点都在右孩子上，平衡二叉树（AVL）可以解决这种情况。

## 示例

### 引入模块

```rust
pub mod avl_tree;
```

### 结构定义

基于二叉搜索树实现。

```rust
use std::cmp::Ordering;

use super::{binary_search_tree::BinarySearchTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct AvlTree<T> {
    tree: BinarySearchTree<T>,
}
```

### 节点高度

平衡二叉树根据节点的高度来求平衡因子，所以需要一个获取高度的方法。

```rust
fn height(tree: &NodeRef<T>) -> usize {
    match tree {
        Some(node) => {
            let left = Self::height(&node.left);
            let right = Self::height(&node.right);
            1 + std::cmp::max(left, right)
        }
        None => 0,
    }
}
```

### 左旋

在调整树的平衡过程中，需要使用到左旋操作。

```rust
fn left_rotate(root: &mut NodeRef<T>) {
    if let Some(mut node) = root.take() {
        if let Some(mut new_root) = node.right.take() {
            node.right = new_root.left.take();
            new_root.left = Some(node);
            *root = Some(new_root);
        }
    }
}
```

### 右旋

在调整树的平衡过程中，需要使用到右旋操作。

```rust
fn right_rotate(root: &mut NodeRef<T>) {
    if let Some(mut node) = root.take() {
        if let Some(mut new_root) = node.left.take() {
            node.left = new_root.right.take();
            new_root.right = Some(node);
            *root = Some(new_root);
        }
    }
}
```

### 平衡节点

```rust
    fn balance_factor(tree: &NodeRef<T>) -> isize {
        match tree {
            None => 0,
            Some(node) => {
                let left = Self::height(&node.left);
                let right = Self::height(&node.right);
                left as isize - right as isize
            }
        }
    }

    fn balance(tree: &mut NodeRef<T>) {
        let balance_factor = Self::balance_factor(tree);
        if balance_factor == 2 {
            let left = &mut tree.as_mut().unwrap().left;
            if Self::balance_factor(left) == -1 {
                Self::left_rotate(left);
            }
            Self::right_rotate(tree);
        } else if balance_factor == -2 {
            let right = &mut tree.as_mut().unwrap().right;
            if Self::balance_factor(right) == 1 {
                Self::right_rotate(right);
            }
            Self::left_rotate(tree);
        }
    }
```

## 总结

基于二叉搜索树，来实现平衡二叉树，首先定义了一些后续必须使用到的方法。

## 附录

### 源码

```rust
impl<T: Ord> AvlTree<T> {
    fn root_mut(&mut self) -> &mut NodeRef<T> {
        self.tree.root_mut()
    }
    fn left_rotate(root: &mut NodeRef<T>) {
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.right.take() {
                node.right = new_root.left.take();
                new_root.left = Some(node);
                *root = Some(new_root);
            }
        }
    }

    fn right_rotate(root: &mut NodeRef<T>) {
        if let Some(mut node) = root.take() {
            if let Some(mut new_root) = node.left.take() {
                node.left = new_root.right.take();
                new_root.right = Some(node);
                *root = Some(new_root);
            }
        }
    }

    fn height(tree: &NodeRef<T>) -> usize {
        match tree {
            Some(node) => {
                let left = Self::height(&node.left);
                let right = Self::height(&node.right);
                1 + std::cmp::max(left, right)
            }
            None => 0,
        }
    }

    fn balance_factor(tree: &NodeRef<T>) -> isize {
        match tree {
            None => 0,
            Some(node) => {
                let left = Self::height(&node.left);
                let right = Self::height(&node.right);
                left as isize - right as isize
            }
        }
    }

    fn balance(tree: &mut NodeRef<T>) {
        let balance_factor = Self::balance_factor(tree);
        if balance_factor == 2 {
            let left = &mut tree.as_mut().unwrap().left;
            if Self::balance_factor(left) == -1 {
                Self::left_rotate(left);
            }
            Self::right_rotate(tree);
        } else if balance_factor == -2 {
            let right = &mut tree.as_mut().unwrap().right;
            if Self::balance_factor(right) == 1 {
                Self::right_rotate(right);
            }
            Self::left_rotate(tree);
        }
    }
}
```
