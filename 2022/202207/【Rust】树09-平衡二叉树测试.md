
# 【Rust】树09-平衡二叉树测试

## 环境

- Time 2022-04-21
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
实现平衡二叉树的插入、检索和删除方法的测试。

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
fn avl_插入() {
    let mut tree = AvlTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
    let expected = vec![&3, &1, &0, &2, &7, &5, &4, &6, &8, &9];
    assert_eq!(tree.pre_order(), expected)
}
```

### 检索

```rust
#[test]
fn avl_检索() {
    let mut tree = AvlTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert!(tree.contains(&4));
}
```

### 删除

```rust
#[test]
fn avl_删除() {
    let mut tree = AvlTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert_eq!(tree.remove(&0), Some(0));
    let mut expected = vec![&3, &1, &2, &7, &5, &4, &6, &8, &9];
    assert_eq!(tree.pre_order(), expected);
    assert_eq!(tree.remove(&2), Some(2));
    expected = vec![&7, &3, &1, &5, &4, &6, &8, &9];
    assert_eq!(tree.pre_order(), expected);
    assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
    assert_eq!(tree.remove(&10), None);
}
```

### 测试

```text
running 9 tests
test tree::avl_tree::tests::avl_删除 ... ok
test tree::avl_tree::tests::avl_插入 ... ok
test tree::avl_tree::tests::avl_检索 ... ok
test tree::binary_search_tree::tests::bst_删除 ... ok
test tree::binary_search_tree::tests::bst_插入 ... ok
test tree::binary_search_tree::tests::bst_检索 ... ok
test tree::binary_tree::tests::中序遍历 ... ok
test tree::binary_tree::tests::前序遍历 ... ok
test tree::binary_tree::tests::后序遍历 ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 总结

实现了平衡二叉树的插入和删除方法的测试。

## 附录

### 源码

```rust

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avl_插入() {
        let mut tree = AvlTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
        let expected = vec![&3, &1, &0, &2, &7, &5, &4, &6, &8, &9];
        assert_eq!(tree.pre_order(), expected)
    }

    #[test]
    fn avl_删除() {
        let mut tree = AvlTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert_eq!(tree.remove(&0), Some(0));
        let mut expected = vec![&3, &1, &2, &7, &5, &4, &6, &8, &9];
        assert_eq!(tree.pre_order(), expected);
        assert_eq!(tree.remove(&2), Some(2));
        expected = vec![&7, &3, &1, &5, &4, &6, &8, &9];
        assert_eq!(tree.pre_order(), expected);
        assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]))
    }

    #[test]
    fn avl_检索() {
        let mut tree = AvlTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert!(tree.contains(&4));
    }
}
```
