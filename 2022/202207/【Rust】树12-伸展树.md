# 【Rust】树12-伸展树

## 环境

- Time 2022-04-25
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
下面实现增加、检索和删除方法。
参考资料：<https://www.geeksforgeeks.org/splay-tree-set-1-insert/>。

## 示例

### 结构定义

基于二叉搜索树实现。

```rust
use std::cmp::Ordering;

use super::{binary_search_tree::BinarySearchTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct SplayTree<T> {
    tree: BinarySearchTree<T>,
}
```

### insert

```rust
fn insert(&mut self, value: T) {
    Self::splay(self.root_mut(), &value);
    let root = self.root_mut().take();
    *self.root_mut() = match root {
        None => Node::new_node_ref(value),
        Some(mut node) => match node.value.cmp(&value) {
            Ordering::Equal => Some(node),
            Ordering::Less => Some(Box::new(Node {
                value,
                right: node.right.take(),
                left: Some(node),
            })),
            Ordering::Greater => Some(Box::new(Node {
                value,
                left: node.left.take(),
                right: Some(node),
            })),
        },
    }
}
```

### remove

```rust
fn remove(&mut self, value: &T) -> Option<T> {
    Self::splay(self.root_mut(), value);
    let node = self.root_mut().as_mut()?;
    match node.value.cmp(value) {
        Ordering::Equal => Node::remove(self.root_mut()),
        _ => None,
    }
}
```

### contains

```rust
fn contains(&mut self, value: &T) -> bool {
    Self::splay(self.root_mut(), value);
    self.root()
        .as_ref()
        .map_or(false, |node| &node.value == value)
}
```

## 总结

实现了伸展树的增加、检索和删除方法。

## 附录

### 源码

```rust
use std::cmp::Ordering;

use super::{binary_search_tree::BinarySearchTree, Node, NodeRef, Tree};

#[derive(Default)]
pub struct SplayTree<T> {
    tree: BinarySearchTree<T>,
}

impl<T: Ord> Tree<T> for SplayTree<T> {
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
        Self::splay(self.root_mut(), &value);
        let root = self.root_mut().take();
        *self.root_mut() = match root {
            None => Node::new_node_ref(value),
            Some(mut node) => match node.value.cmp(&value) {
                Ordering::Equal => Some(node),
                Ordering::Less => Some(Box::new(Node {
                    value,
                    right: node.right.take(),
                    left: Some(node),
                })),
                Ordering::Greater => Some(Box::new(Node {
                    value,
                    left: node.left.take(),
                    right: Some(node),
                })),
            },
        }
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        Self::splay(self.root_mut(), value);
        let node = self.root_mut().as_mut()?;
        match node.value.cmp(value) {
            Ordering::Equal => Node::remove(self.root_mut()),
            _ => None,
        }
    }

    fn contains(&mut self, value: &T) -> bool {
        Self::splay(self.root_mut(), value);
        self.root()
            .as_ref()
            .map_or(false, |node| &node.value == value)
    }
}

impl<T: Ord> SplayTree<T> {
    fn root(&self) -> &NodeRef<T> {
        self.tree.root()
    }
    fn root_mut(&mut self) -> &mut NodeRef<T> {
        self.tree.root_mut()
    }

    fn splay(tree: &mut NodeRef<T>, value: &T) {
        if let Some(grandparent) = tree.as_mut() {
            match grandparent.value.cmp(value) {
                Ordering::Greater => Self::splay_left(tree, value),
                Ordering::Less => Self::splay_right(tree, value),
                Ordering::Equal => (),
            }
        }
    }

    fn splay_left(tree: &mut NodeRef<T>, value: &T) {
        let grandparent = tree.as_mut().unwrap();
        if let Some(parent) = grandparent.left.as_mut() {
            match parent.value.cmp(value) {
                Ordering::Greater => {
                    Self::splay(&mut parent.left, value);
                    Node::right_rotate(tree);
                }
                Ordering::Less => {
                    Self::splay(&mut parent.right, value);
                    Node::left_rotate(tree);
                }
                Ordering::Equal => (),
            }
            Node::right_rotate(tree);
        }
    }

    fn splay_right(tree: &mut NodeRef<T>, value: &T) {
        let grandparent = tree.as_mut().unwrap();
        if let Some(parent) = grandparent.right.as_mut() {
            match parent.value.cmp(value) {
                Ordering::Greater => {
                    Self::splay(&mut parent.left, value);
                    Node::right_rotate(tree);
                }
                Ordering::Less => {
                    Self::splay(&mut parent.right, value);
                    Node::left_rotate(tree);
                }
                Ordering::Equal => (),
            }
            Node::left_rotate(tree);
        }
    }
}
```
