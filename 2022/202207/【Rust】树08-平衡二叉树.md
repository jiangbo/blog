# 【Rust】树08-平衡二叉树

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
实现平衡二叉树的插入和删除方法。

## 示例

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

### 插入

```rust
fn insert(&mut self, value: T) {
    match self.root_mut() {
        Some(node) => Self::insert_node(node, value),
        None => *self.root_mut() = Node::new_node_ref(value),
    }
    Self::balance(self.root_mut());
}
```

### 插入节点

```rust
fn insert_node(root: &mut Node<T>, value: T) {
    let target = match value.cmp(&root.value) {
        Ordering::Less => &mut root.left,
        Ordering::Greater => &mut root.right,
        Ordering::Equal => return,
    };

    match target {
        Some(node) => Self::insert_node(node, value),
        None => *target = Node::new_node_ref(value),
    }

    Self::balance(target);
}
```

### 删除

```rust
fn remove(&mut self, value: &T) -> Option<T> {
    let root = self.root_mut();
    let result = Self::remove_node(root, value);
    if root.is_some() {
        Self::balance(root);
    }
    result
}
```

### 删除节点

```rust
fn remove_node(tree: &mut NodeRef<T>, value: &T) -> Option<T> {
    let node = tree.as_mut()?;
    let temp = match node.value.cmp(value) {
        Ordering::Less => &mut node.right,
        Ordering::Greater => &mut node.left,
        Ordering::Equal => return Node::remove(tree),
    };
    let result = Self::remove_node(temp, value);
    if temp.is_some() {
        Self::balance(temp);
    }
    result
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

fn contains(&mut self, value: &T) -> bool {
    self.tree.contains(value)
}
```

## 总结

基于二叉搜索树，来实现平衡二叉树，实现了插入和删除方法。

## 附录

### 源码

```rust
use std::cmp::Ordering;

use super::{binary_search_tree::BinarySearchTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct AvlTree<T> {
    tree: BinarySearchTree<T>,
}

impl<T: Ord> Tree<T> for AvlTree<T> {
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
        match self.root_mut() {
            Some(node) => Self::insert_node(node, value),
            None => *self.root_mut() = Node::new_node_ref(value),
        }
        Self::balance(self.root_mut());
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        let root = self.root_mut();
        let result = Self::remove_node(root, value);
        if root.is_some() {
            Self::balance(root);
        }
        result
    }

    fn contains(&mut self, value: &T) -> bool {
        self.tree.contains(value)
    }
}

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

    fn insert_node(root: &mut Node<T>, value: T) {
        let target = match value.cmp(&root.value) {
            Ordering::Less => &mut root.left,
            Ordering::Greater => &mut root.right,
            Ordering::Equal => return,
        };

        match target {
            Some(node) => Self::insert_node(node, value),
            None => *target = Node::new_node_ref(value),
        }

        Self::balance(target);
    }

    fn remove_node(tree: &mut NodeRef<T>, value: &T) -> Option<T> {
        let node = tree.as_mut()?;
        let temp = match node.value.cmp(value) {
            Ordering::Less => &mut node.right,
            Ordering::Greater => &mut node.left,
            Ordering::Equal => return Node::remove(tree),
        };
        let result = Self::remove_node(temp, value);
        if temp.is_some() {
            Self::balance(temp);
        }
        result
    }
}
```
