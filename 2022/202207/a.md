# 【Rust】树03-二叉树测试

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
实现了二叉树的前序、中序和后序遍历的测试。

## 示例

### 测试模块

```rust
mod tests {

    use super::*;
    use crate::tree::Node;
}
```

### 前序遍历

```rust
#[test]
fn 前序遍历() {
    let mut tree = BinaryTree::default();
    let left = Node::new_node_ref(44);
    let right = Node::new_node_ref(55);
    tree.root = Some(Box::new(Node {
        value: 33,
        left,
        right,
    }));
    assert_eq!(tree.pre_order(), vec![&33, &44, &55])
}
```

### 中序遍历

```rust
#[test]
fn 中序遍历() {
    let mut tree = BinaryTree::default();
    let left = Node::new_node_ref(44);
    let right = Node::new_node_ref(55);
    tree.root = Some(Box::new(Node {
        value: 33,
        left,
        right,
    }));
    assert_eq!(tree.in_order(), vec![&44, &33, &55])
}
```

### 后序遍历

```rust
#[test]
fn 后序遍历() {
    let mut tree = BinaryTree::default();
    let left = Node::new_node_ref(44);
    let right = Node::new_node_ref(55);
    tree.root = Some(Box::new(Node {
        value: 33,
        left,
        right,
    }));
    assert_eq!(tree.post_order(), vec![&44, &55, &33])
}
```

### 测试

```text
running 3 tests
test tree::binary_tree::tests::中序遍历 ... ok
test tree::binary_tree::tests::前序遍历 ... ok
test tree::binary_tree::tests::后序遍历 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 总结

测试了二叉树的前序、中序和后序遍历方法。

## 附录

### 源码

```rust
#[cfg(test)]
mod tests {

    use super::*;
    use crate::tree::Node;

    #[test]
    fn 前序遍历() {
        let mut tree = BinaryTree::default();
        let left = Node::new_node_ref(44);
        let right = Node::new_node_ref(55);
        tree.root = Some(Box::new(Node {
            value: 33,
            left,
            right,
        }));
        assert_eq!(tree.pre_order(), vec![&33, &44, &55])
    }
    #[test]
    fn 中序遍历() {
        let mut tree = BinaryTree::default();
        let left = Node::new_node_ref(44);
        let right = Node::new_node_ref(55);
        tree.root = Some(Box::new(Node {
            value: 33,
            left,
            right,
        }));
        assert_eq!(tree.in_order(), vec![&44, &33, &55])
    }
    #[test]
    fn 后序遍历() {
        let mut tree = BinaryTree::default();
        let left = Node::new_node_ref(44);
        let right = Node::new_node_ref(55);
        tree.root = Some(Box::new(Node {
            value: 33,
            left,
            right,
        }));
        assert_eq!(tree.post_order(), vec![&44, &55, &33])
    }
}
```
