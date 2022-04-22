# 【Rust】树10-平衡二叉树

## 环境

- Time 2022-04-22
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
之前使用递归的方式实现平衡二叉树的插入和删除方法，下面使用迭代实现。

## 示例

### 插入

```rust
    fn insert(&mut self, value: T) {
        let mut stack: Vec<*mut NodeRef<T>> = vec![self.tree.root_mut()];
        let mut current = self.tree.root_mut();
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                Ordering::Equal => return,
            };
            stack.push(current);
        }
        *current = Node::new_node_ref(value);
        while let Some(node) = stack.pop() {
            Self::balance(unsafe { &mut *node })
        }
    }
```

### 删除

```rust
    fn remove(&mut self, value: &T) -> Option<T> {
        let mut stack: Vec<*mut NodeRef<T>> = vec![self.tree.root_mut()];
        let mut current = self.tree.root_mut();
        while let Some(node) = current {
            current = match node.value.cmp(value) {
                Ordering::Less => &mut current.as_mut()?.right,
                Ordering::Greater => &mut current.as_mut()?.left,
                Ordering::Equal => break,
            };
            stack.push(current);
        }

        let result = Node::remove(current);
        while let Some(node) = stack.pop() {
            Self::balance(unsafe { &mut *node })
        }
        result
    }
```

### 测试

```text
running 9 tests
test tree::avl_tree::tests::avl_插入 ... ok
test tree::avl_tree::tests::avl_删除 ... ok
test tree::avl_tree::tests::avl_检索 ... ok
test tree::binary_search_tree::tests::bst_删除 ... ok
test tree::binary_search_tree::tests::bst_插入 ... ok
test tree::binary_search_tree::tests::bst_检索 ... ok
test tree::binary_tree::tests::中序遍历 ... ok
test tree::binary_tree::tests::前序遍历 ... ok
test tree::binary_tree::tests::后序遍历 ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests (target\debug\deps\game-de7eb3498b4e9a50.exe)
```

## 总结

使用迭代的方式，实现了平衡二叉树的插入和删除方法。

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
        let mut stack: Vec<*mut NodeRef<T>> = vec![self.tree.root_mut()];
        let mut current = self.tree.root_mut();
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                Ordering::Equal => return,
            };
            stack.push(current);
        }
        *current = Node::new_node_ref(value);
        while let Some(node) = stack.pop() {
            Self::balance(unsafe { &mut *node })
        }
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        let mut stack: Vec<*mut NodeRef<T>> = vec![self.tree.root_mut()];
        let mut current = self.tree.root_mut();
        while let Some(node) = current {
            current = match node.value.cmp(value) {
                Ordering::Less => &mut current.as_mut()?.right,
                Ordering::Greater => &mut current.as_mut()?.left,
                Ordering::Equal => break,
            };
            stack.push(current);
        }

        let result = Node::remove(current);
        while let Some(node) = stack.pop() {
            Self::balance(unsafe { &mut *node })
        }
        result
    }

    fn contains(&mut self, value: &T) -> bool {
        self.tree.contains(value)
    }
}

impl<T: Ord> AvlTree<T> {
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
