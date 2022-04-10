# 【Rust】二叉树-后续-迭代

## 环境

- Time 2022-04-10
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构。

### 特点

之前使用递归的方式实现了二叉树的后序遍历，这里使用迭代的方式。

## 示例

### 节点定义

```rust
type NodeRef<T> = Option<Box<Node<T>>>;

struct Node<T: Debug> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

impl<T: Debug> Node<T> {
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
struct BinaryTree<T: Debug> {
    root: NodeRef<T>,
}

impl<T: Debug> BinaryTree<T> {
    fn post_order(&self) {
        let mut stack = vec![&self.root];
        let mut result = vec![];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                result.push(&node.value);
                stack.push(&node.left);
                stack.push(&node.right);
            }
        }
        result.iter().rev().for_each(|node| println!("{node:?}",));
    }
}
```

### 后序遍历

```rust
fn main() {
    let tree = BinaryTree {
        root: Some(Box::new(Node {
            value: 44,
            left: Some(Box::new(Node {
                value: 22,
                left: Node::new_node_ref(11),
                right: Node::new_node_ref(33),
            })),
            right: Some(Box::new(Node {
                value: 66,
                left: Node::new_node_ref(55),
                right: Node::new_node_ref(77),
            })),
        })),
    };

    tree.post_order();
}
```

## 总结

使用迭代的方式实现了二叉树的后序遍历。

## 附录
