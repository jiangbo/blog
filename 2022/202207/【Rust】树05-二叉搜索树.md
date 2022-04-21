# 【Rust】树05-二叉搜索树

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
相比较二叉树，二叉搜索树的左节点都比父节点小，右节点都比父节点大。
基于二叉树来实现二叉搜索树，实现删除方法。

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

### Node 获取最小节点

```rust
fn get_min(tree: &mut NodeRef<T>) -> Option<T> {
    let mut current = tree;
    while let Some(node) = current {
        current = match node.left {
            Some(_) => &mut current.as_mut()?.left,
            None => break,
        }
    }
    let node = current.take()?;
    *current = node.right;
    Some(node.value)
}
```

### Node 删除节点

```rust
fn remove(tree: &mut NodeRef<T>) -> Option<T> {
    let mut node = tree.take()?;
    *tree = match (node.left.as_ref(), node.right.as_ref()) {
        (None, None) => None,
        (Some(_), None) => node.left.take(),
        (None, Some(_)) => node.right.take(),
        (Some(_), Some(_)) => Some(Box::new(Node {
            value: Self::get_min(&mut node.right)?,
            left: node.left.take(),
            right: node.right.take(),
        })),
    };
    Some(node.value)
}
```

### 删除

```rust
fn remove(&mut self, value: &T) -> Option<T> {
    let mut current = self.root_mut();
    while let Some(node) = current {
        current = match node.value.cmp(value) {
            Ordering::Less => &mut current.as_mut()?.right,
            Ordering::Greater => &mut current.as_mut()?.left,
            Ordering::Equal => break,
        }
    }
    Node::remove(current)
}
```

## 总结

基于二叉树，实现了二叉搜索树的删除方法。

## 附录

### mod.rs 源码

```rust
pub trait Tree<T> {
    fn pre_order(&self) -> Vec<&T>;

    fn in_order(&self) -> Vec<&T>;

    fn post_order(&self) -> Vec<&T>;

    fn insert(&mut self, value: T);

    fn remove(&mut self, value: &T) -> Option<T>;

    fn contains(&mut self, value: &T) -> bool;
}

pub mod binary_search_tree;
pub mod binary_tree;

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

    fn get_min(tree: &mut NodeRef<T>) -> Option<T> {
        let mut current = tree;
        while let Some(node) = current {
            current = match node.left {
                Some(_) => &mut current.as_mut()?.left,
                None => break,
            }
        }
        let node = current.take()?;
        *current = node.right;
        Some(node.value)
    }

    fn remove(tree: &mut NodeRef<T>) -> Option<T> {
        let mut node = tree.take()?;
        *tree = match (node.left.as_ref(), node.right.as_ref()) {
            (None, None) => None,
            (Some(_), None) => node.left.take(),
            (None, Some(_)) => node.right.take(),
            (Some(_), Some(_)) => Some(Box::new(Node {
                value: Self::get_min(&mut node.right)?,
                left: node.left.take(),
                right: node.right.take(),
            })),
        };
        Some(node.value)
    }
}
```

### binary_search_tree.rs 源码

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

    fn remove(&mut self, value: &T) -> Option<T> {
        let mut current = self.root_mut();
        while let Some(node) = current {
            current = match node.value.cmp(value) {
                Ordering::Less => &mut current.as_mut()?.right,
                Ordering::Greater => &mut current.as_mut()?.left,
                Ordering::Equal => break,
            }
        }
        Node::remove(current)
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
