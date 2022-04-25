# 【Rust】树13-伸展树测试

## 环境

- Time 2022-04-25
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
实现伸展树的插入、检索和删除方法的测试。

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
fn splay_插入() {
    let mut tree = SplayTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
}
```

### 检索

```rust
#[test]
fn splay_检索() {
    let mut tree = SplayTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert!(tree.contains(&0));
    let mut expected = vec![&0, &8, &6, &4, &2, &1, &3, &5, &7, &9];
    assert_eq!(tree.pre_order(), expected);
    assert!(tree.contains(&9));
    expected = vec![&9, &8, &0, &6, &4, &2, &1, &3, &5, &7];
    assert_eq!(tree.pre_order(), expected);
    assert!(!tree.contains(&-1));
    expected = vec![&0, &8, &6, &4, &2, &1, &3, &5, &7, &9];
    assert_eq!(tree.pre_order(), expected);
}
```

### 删除

```rust
#[test]
fn splay_删除() {
    let mut tree = SplayTree::default();
    (0..10).for_each(|e| tree.insert(e));
    assert_eq!(tree.remove(&0), Some(0));
    let arr = [8, 6, 4, 2, 1, 3, 5, 7, 9];
    assert_eq!(tree.pre_order(), arr.iter().collect::<Vec<_>>());
    assert_eq!(tree.remove(&10), None);
}
```

### 测试

```text
running 12 tests
test tree::avl_tree::tests::avl_删除 ... ok
test tree::avl_tree::tests::avl_插入 ... ok
test tree::avl_tree::tests::avl_检索 ... ok
test tree::binary_search_tree::tests::bst_删除 ... ok
test tree::binary_search_tree::tests::bst_插入 ... ok
test tree::binary_search_tree::tests::bst_检索 ... ok
test tree::binary_tree::tests::中序遍历 ... ok
test tree::binary_tree::tests::前序遍历 ... ok
test tree::binary_tree::tests::后序遍历 ... ok
test tree::splay_tree::tests::splay_删除 ... ok
test tree::splay_tree::tests::splay_插入 ... ok
test tree::splay_tree::tests::splay_检索 ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running unittests (target\debug\deps\game-de7eb3498b4e9a50.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 总结

实现了伸展树的插入、检索和删除方法的测试。

## 附录

### 源码

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splay_插入() {
        let mut tree = SplayTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert!(tree.in_order().windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn splay_检索() {
        let mut tree = SplayTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert!(tree.contains(&0));
        let mut expected = vec![&0, &8, &6, &4, &2, &1, &3, &5, &7, &9];
        assert_eq!(tree.pre_order(), expected);
        assert!(tree.contains(&9));
        expected = vec![&9, &8, &0, &6, &4, &2, &1, &3, &5, &7];
        assert_eq!(tree.pre_order(), expected);
        assert!(!tree.contains(&-1));
        expected = vec![&0, &8, &6, &4, &2, &1, &3, &5, &7, &9];
        assert_eq!(tree.pre_order(), expected);
    }

    #[test]
    fn splay_删除() {
        let mut tree = SplayTree::default();
        (0..10).for_each(|e| tree.insert(e));
        assert_eq!(tree.remove(&0), Some(0));
        let arr = [8, 6, 4, 2, 1, 3, 5, 7, 9];
        assert_eq!(tree.pre_order(), arr.iter().collect::<Vec<_>>());
        assert_eq!(tree.remove(&10), None);
    }
}
```
