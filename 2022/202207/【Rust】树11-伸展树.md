# 【Rust】树11-伸展树

## 环境

- Time 2022-04-25
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
伸展树也是一种可以自平衡的二叉搜索树，并且可以不计算节点的高度和平衡因子信息。
下面实现伸展方法，参考资料：<https://www.geeksforgeeks.org/splay-tree-set-1-insert/>。
因为也需要使用到左旋和右旋的操作，将之前 AVL 树中的旋转操作转移到 Node 定义上。

## 示例

### 引入模块

```rust
pub mod splay_tree;
```

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

### splay

```rust
fn splay(tree: &mut NodeRef<T>, value: &T) {
    if let Some(grandparent) = tree.as_mut() {
        match grandparent.value.cmp(value) {
            Ordering::Greater => Self::splay_left(tree, value),
            Ordering::Less => Self::splay_right(tree, value),
            Ordering::Equal => (),
        }
    }
}
```

### splay_left

```rust
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
```

### splay_right

```rust
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
```

### 其它方法

```rust
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
}

impl<T: Ord> SplayTree<T> {
    fn root(&self) -> &NodeRef<T> {
        self.tree.root()
    }
    fn root_mut(&mut self) -> &mut NodeRef<T> {
        self.tree.root_mut()
    }
}
```

## 总结

实现伸展树的伸展方法，调整了左旋和右旋的代码结构。

## 附录

### Node源码

```rust
impl<T> Node<T> {
    fn new_node_ref(value: T) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }))
    }

    fn children(value: T, left: NodeRef<T>, right: NodeRef<T>) -> NodeRef<T> {
        Some(Box::new(Node { value, left, right }))
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
            (Some(_), Some(_)) => Node::children(
                Self::get_min(&mut node.right)?,
                node.left.take(),
                node.right.take(),
            ),
        };
        Some(node.value)
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
}
```

### SplayTree 源码

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
