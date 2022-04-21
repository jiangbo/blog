# 【Rust】树06-二叉搜索树测试

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
实现了二叉搜索树的插入、检索和删除测试。

## 示例

### 测试模块

```rust
#[cfg(test)]
mod tests {
    use super::*;
}
```

### 插入

```rust
#[test]
fn bst_插入() {
    let mut tree = BinarySearchTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]))
}
```

### 检索

```rust
#[test]
fn bst_检索() {
    let mut tree = BinarySearchTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert!(tree.contains(&4));
}
```

### 删除

```rust
fn bst_删除() {
    let mut tree = BinarySearchTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert_eq!(tree.remove(&4), Some(4));
    assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
    assert_eq!(tree.remove(&4), None);
}
```

### 测试

```text
running 6 tests
test tree::binary_search_tree::tests::bst_删除 ... ok
test tree::binary_search_tree::tests::bst_插入 ... ok
test tree::binary_search_tree::tests::bst_检索 ... ok
test tree::binary_tree::tests::中序遍历 ... ok
test tree::binary_tree::tests::前序遍历 ... ok
test tree::binary_tree::tests::后序遍历 ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 总结

测试了二叉树的插入、检索和删除方法。

## 附录

### 源码

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst_插入() {
        let mut tree = BinarySearchTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]))
    }

    #[test]
    fn bst_删除() {
        let mut tree = BinarySearchTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert_eq!(tree.remove(&4), Some(4));
        assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(tree.remove(&4), None);
    }

    #[test]
    fn bst_检索() {
        let mut tree = BinarySearchTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert!(tree.contains(&4));
    }
}
```
