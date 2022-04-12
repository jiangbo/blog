# 【Rust】二叉搜索树-查询极值

## 环境

- Time 2022-04-11
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。

### 特点

相比较二叉树，二叉搜索树的左节点都比父节点小，右节点都比父节点大。
使用迭代的方式查询二叉搜索树中的最大和最小值。

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
    fn max_or_min<F>(&self, child: F) -> Option<&T>
    where
        F: Fn(&Box<Node<T>>) -> &NodeRef<T>,
    {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match child(node) {
                Some(_) => child(node),
                None => return Some(&node.value),
            }
        }
        None
    }
}
```

### 最大值

```rust
fn max(&self) -> Option<&T> {
    self.max_or_min(|node| &node.right)
}
```

### 最小值

```rust
fn min(&self) -> Option<&T> {
    self.max_or_min(|node| &node.left)
}
```

### 使用示例

```rust
fn main() {
    let mut tree = BinarySearchTree::new();
    vec![44, 22, 11, 33, 66, 66, 55, 77]
        .into_iter()
        .for_each(|e| tree.insert(e));
    tree.in_order();
    println!("{:?}", tree.search(&88));
    println!("{:?}", tree.search(&77));
    println!("{:?}", tree.max());
    println!("{:?}", tree.min());
}
```

## 总结

使用迭代的方式实现了查询二叉搜索树极值的方法。

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
    println!("{:?}", tree.search(&88));
    println!("{:?}", tree.search(&77));
    println!("{:?}", tree.max());
    println!("{:?}", tree.min());
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
    fn search(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
                Ordering::Equal => return true,
            };
        }
        false
    }

    fn max(&self) -> Option<&T> {
        self.max_or_min(|node| &node.right)
    }
    fn min(&self) -> Option<&T> {
        self.max_or_min(|node| &node.left)
    }

    fn max_or_min<F>(&self, child: F) -> Option<&T>
    where
        F: Fn(&Box<Node<T>>) -> &NodeRef<T>,
    {
        let mut current = &self.root;
        while let Some(node) = current {
            current = match child(node) {
                Some(_) => child(node),
                None => return Some(&node.value),
            }
        }
        None
    }
}
```
