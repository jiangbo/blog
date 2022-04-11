# 【Rust】二叉搜索树-插入-迭代

## 环境

- Time 2022-04-11
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。

### 特点

相比较二叉树，二叉搜索树的左节点都比父节点小，右节点都比父节点大。
使用迭代的方式来实现二叉搜索树的节点插入。

## 示例

### 节点定义

```rust
type NodeRef<T> = Option<Box<Node<T>>>;
struct Node<T: Ord + Debug> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}
```

### 节点实现

```rust
impl<T: Ord + Debug> Node<T> {
    fn new_node_ref(value: T) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}
```

### 二叉搜索树定义

```rust
struct BinarySearchTree<T: Ord + Debug> {
    root: NodeRef<T>,
}
```

### 二叉搜索树实现

```rust
impl<T: Ord + Debug> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }
    fn insert(&mut self, value: T) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => current = &mut node.left,
                Ordering::Greater => current = &mut node.right,
                // 相等元素不插入
                Ordering::Equal => return,
            };
        }
        *current = Node::new_node_ref(value)
    }
}
```

### 使用示例

```rust
fn main() {
    let mut tree = BinarySearchTree::new();
    vec![44, 22, 11, 33, 66, 66, 55, 77]
        .into_iter()
        .for_each(|e| tree.insert(e));
    // 中序遍历满足从小到大的顺序
    tree.in_order();
}
```

## 总结

使用迭代的方式，实现了二叉搜索树的插入方法。

## 附录

### 源码

```rust
use std::{cmp::Ordering, fmt::Debug};

fn main() {
    let mut tree = BinarySearchTree::new();
    vec![44, 22, 11, 33, 66, 66, 55, 77]
        .into_iter()
        .for_each(|e| tree.insert(e));
    tree.in_order();
}

type NodeRef<T> = Option<Box<Node<T>>>;
struct Node<T: Ord + Debug> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T: Ord + Debug> Node<T> {
    fn new_node_ref(value: T) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}

struct BinarySearchTree<T: Ord + Debug> {
    root: NodeRef<T>,
}

impl<T: Ord + Debug> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn in_order(&self) {
        let (mut stack, mut current) = (Vec::new(), &self.root);
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(current);
                current = &node.left;
            }
            current = stack.pop().unwrap();
            println!("{:?}", current.as_ref().unwrap().value);
            current = &current.as_ref().unwrap().right;
        }
    }

    fn insert(&mut self, value: T) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                // 相等元素不插入
                Ordering::Equal => return,
            };
        }
        *current = Node::new_node_ref(value)
    }
}
```
