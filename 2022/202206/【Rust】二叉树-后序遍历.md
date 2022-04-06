# 【Rust】二叉树-后序遍历

## 环境

- Time 2022-04-06
- Rust 1.59.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构。

### 特点

二叉树是一个节点最多有两个分支的树。到目前为止，标准库还没有内置支持。

## 示例

### 节点定义

```rust
type NodeRef<T> = Option<Box<Node<T>>>;
struct Node<T: PartialOrd + Debug> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T: PartialOrd + Debug> Node<T> {
    fn post_order(&self) {
        
        if let Some(left) = &self.left {
            left.post_order();
        }

        if let Some(right) = &self.right {
            right.post_order();
        }
        print!("{:?} ", self.value);
    }

    fn new_node_ref(value: T) -> NodeRef<T> {
        Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}
```

### 二叉树定义

```rust
struct BinaryTree<T: PartialOrd + Debug> {
    root: NodeRef<T>,
}

impl<T: PartialOrd + Debug> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root: None }
    }

    fn post_order(&self) {
        if let Some(root) = &self.root {
            root.post_order();
        }
    }
}
```

### 后序遍历

```rust
fn main() {
    let mut tree = BinaryTree::new();
    let left = Node::new_node_ref(44);
    let right = Node::new_node_ref(55);
    tree.root = Some(Box::new(Node {
        value: 33,
        left,
        right,
    }));
    tree.post_order();
}
```

## 总结

实现了二叉树的前序遍历。

## 附录
