# 【Rust】二叉搜索树-删除

## 环境

- Time 2022-04-12
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。

### 特点

相比较二叉树，二叉搜索树的左节点都比父节点小，右节点都比父节点大。
使用迭代的方式删除二叉搜索树中的某个节点。

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

不返回删除的节点

```rust
impl<T: Ord + Debug> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn remove(&mut self, value: &T) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            match node.value.cmp(value) {
                Ordering::Less => current = &mut current.as_mut().unwrap().right,
                Ordering::Greater => current = &mut current.as_mut().unwrap().left,
                Ordering::Equal => {
                    match (node.left.as_mut(), node.right.as_mut()) {
                        (None, None) => *current = None,
                        (Some(_), None) => *current = node.left.take(),
                        (None, Some(_)) => *current = node.right.take(),
                        (Some(_), Some(_)) => {
                            current.as_mut().unwrap().value =
                                Node::get_min(&mut node.right).unwrap()
                        }
                    };
                }
            }
        }
    }
}
```

### 删除1

返回删除的节点，并且不新增节点，复用之前的节点。

```rust
fn remove(&mut self, value: &T) -> Option<T> {
    let mut current = &mut self.root;
    while let Some(node) = current {
        current = match node.value.cmp(value) {
            Ordering::Less => &mut current.as_mut()?.right,
            Ordering::Greater => &mut current.as_mut()?.left,
            Ordering::Equal => break,
        }
    }

    let mut node = current.take()?;
    *current = match (node.left.as_ref(), node.right.as_ref()) {
        (None, None) => None,
        (Some(_), None) => node.left.take(),
        (None, Some(_)) => node.right.take(),
        (Some(_), Some(_)) => {
            let old = replace(&mut node.value, Node::get_min(&mut node.right)?);
            *current = Some(node);
            return Some(old);
        }
    };
    Some(node.value)
}
```

### 删除2

返回删除的节点，新增一个节点。

```rust
fn remove(&mut self, value: &T) -> Option<T> {
    let mut current = &mut self.root;
    while let Some(node) = current {
        current = match node.value.cmp(value) {
            Ordering::Less => &mut current.as_mut()?.right,
            Ordering::Greater => &mut current.as_mut()?.left,
            Ordering::Equal => break,
        }
    }

    let mut node = current.take()?;
    *current = match (node.left.as_ref(), node.right.as_ref()) {
        (None, None) => None,
        (Some(_), None) => node.left.take(),
        (None, Some(_)) => node.right.take(),
        (Some(_), Some(_)) => Some(Box::new(Node {
            value: Node::get_min(&mut node.right)?,
            left: node.left.take(),
            right: node.right.take(),
        })),
    };
    Some(node.value)
}
```

### 删除3

迭代删除方式

```rust
fn remove(tree: &mut NodeRef<T>, value: &T) -> Option<T> {
    let node = tree.as_mut()?;
    let current = match node.value.cmp(value) {
        Ordering::Less => &mut node.right,
        Ordering::Greater => &mut node.left,
        Ordering::Equal => return Node::remove_node(tree),
    };
    Node::remove(current, value)
}

fn remove_node(target: &mut NodeRef<T>) -> Option<T> {
    let mut node = target.take()?;
    *target = match (node.left.as_ref(), node.right.as_ref()) {
        (None, None) => None,
        (Some(_), None) => node.left.take(),
        (None, Some(_)) => node.right.take(),
        (Some(_), Some(_)) => Some(Box::new(Node {
            value: Node::get_min(&mut node.right)?,
            left: node.left.take(),
            right: node.right.take(),
        })),
    };
    Some(node.value)
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
    println!("{:?}", tree.get_max());
    println!("{:?}", tree.get_min());
    tree.in_order();
    tree.remove(&44);
    tree.in_order();
}
```

## 总结

使用迭代的方式实现了删除二叉搜索树中节点的方法。

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
    println!("{:?}", tree.get_max());
    println!("{:?}", tree.get_min());
    tree.in_order();
    println!("{:?}", tree.remove(&44));
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

    fn get_max(root: &mut NodeRef<T>) -> Option<T> {
        let mut current = root;
        while let Some(node) = current {
            current = match node.right {
                Some(_) => &mut current.as_mut()?.right,
                None => break,
            }
        }
        let node = current.take()?;
        *current = node.left;
        Some(node.value)
    }

    fn get_min(root: &mut NodeRef<T>) -> Option<T> {
        let mut current = root;
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

    fn get_max(&mut self) -> Option<T> {
        Node::get_max(&mut self.root)
    }

    fn get_min(&mut self) -> Option<T> {
        Node::get_min(&mut self.root)
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        let mut current = &mut self.root;
        while let Some(node) = current {
            current = match node.value.cmp(value) {
                Ordering::Less => &mut current.as_mut()?.right,
                Ordering::Greater => &mut current.as_mut()?.left,
                Ordering::Equal => break,
            }
        }

        let mut node = current.take()?;
        *current = match (node.left.as_ref(), node.right.as_ref()) {
            (None, None) => None,
            (Some(_), None) => node.left.take(),
            (None, Some(_)) => node.right.take(),
            (Some(_), Some(_)) => Some(Box::new(Node {
                value: Node::get_min(&mut node.right)?,
                left: node.left.take(),
                right: node.right.take(),
            })),
        };
        Some(node.value)
    }
}
```
