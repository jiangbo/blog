# 0001-Rust-B-树

## 环境

- Time 2022-04-26
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。
B-树是一种多路搜索树，在标准库中已有相应的实现。
一般编程语言会使用二叉搜索树（BST）来实现有序 Map 和 Set，而 Rust 选择了 B-树。

### 目标

了解使用 B-树实现有序 Map 的原因，简单使用 BTreeMap 的方法。

## BTreeMap 介绍

An ordered map based on a B-Tree.

`BTreeMap` 是基于 B-树的有序 Map。

B-Trees represent a fundamental compromise between cache-efficiency and actually minimizing the amount of work performed in a search. In theory, a binary search tree (BST) is the optimal choice for a sorted map, as a perfectly balanced BST performs the theoretical minimum amount of comparisons necessary to find an element (log~2~n). However, in practice the way this is done is very inefficient for modern computer architectures. In particular, every element is stored in its own individually heap-allocated node. This means that every single insertion triggers a heap-allocation, and every single comparison should be a cache-miss. Since these are both notably expensive things to do in practice, we are forced to at very least reconsider the BST strategy.

B-树代表了缓存效率和实际最小化搜索中执行的工作量之间的基本折衷。理论上，二叉搜索树 (BST) 是排序 Map 的最佳选择，因为完美平衡的 BST 执行查找元素所需理论最小比较量 (log~2~n)。然而，在实践中，这样做的方式对于现代计算机体系结构来说是非常低效的。特别是，每个元素都存储在其自己的单独堆分配节点中。这意味着每次插入都会触发堆分配，并且每次比较都是缓存未命中。由于在实践中这些都是非常昂贵的事情，我们不得不至少重新考虑 BST 策略。

A B-Tree instead makes each node contain B-1 to 2B-1 elements in a contiguous array. By doing this, we reduce the number of allocations by a factor of B, and improve cache efficiency in searches. However, this does mean that searches will have to do more comparisons on average. The precise number of comparisons depends on the node search strategy used. For optimal cache efficiency, one could search the nodes linearly. For optimal comparisons, one could search the node using binary search.

相反，B-Tree 使每个节点在连续数组中包含 B-1 到 2B-1 个元素。通过这样做，我们将分配次数减少了 B 倍，并提高了搜索中的缓存效率。但是，这确实意味着在平均情况下，搜索将不得不进行更多的比较。比较的精确数量取决于使用的节点搜索策略。为了获得最佳缓存效率，可以线性搜索节点。为了进行最佳比较，可以使用二分搜索来搜索节点。

Currently, our implementation simply performs naive linear search. This provides excellent performance on small nodes of elements which are cheap to compare. However in the future we would like to further explore choosing the optimal search strategy based on the choice of B, and possibly other factors. Using linear search, searching for a random element is expected to take B * log(n) comparisons, which is generally worse than a BST. In practice, however, performance is excellent.

目前，我们的实现只是简单地执行线性搜索。这在比较较少的元素节点上提供了出色的性能。然而，在未来，我们将进一步探索基于 B 的选择以及可能的其他因素来选择最优搜索策略。使用线性搜索，搜索随机元素预计需要 B * log(n) 比较，这通常比 BST 差。然而，在实践中，性能非常出色。

It is a logic error for a key to be modified in such a way that the key’s ordering relative to any other key, as determined by the Ord trait, changes while it is in the map. This is normally only possible through Cell, RefCell, global state, I/O, or unsafe code. The behavior resulting from such a logic error is not specified (it could include panics, incorrect results, aborts, memory leaks, or non-termination) but will not be undefined behavior.

以这样一种方式修改键是一个逻辑错误，即修改已经插入到 map 中元素的键相对于任何其他键的顺序（由 Ord 特征确定）。这通常只能通过 Cell、RefCell、全局状态、I/O 或 unsafe 代码实现。由此类逻辑错误导致的行为未指定（它可能包括恐慌、不正确的结果、中止、内存泄漏或未终止），但不会是未定义的行为。

## new

```rust
fn main() {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
}
```

## insert

```rust
use std::collections::BTreeMap;
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
}
```

## is_empty

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    println!("{}", map.is_empty())
}
```

## clear

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    map.clear();
    println!("{}", map.is_empty())
}
```

## get

可变版本：get_mut。

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    println!("name:{:?}", map.get("name"));
    println!("name1:{:?}", map.get("name1"))
}
```

## get_key_value

```rust
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name", "JiangBo");
    println!("name:{:?}", map.get_key_value("name"));
}
```

## 总结

了解顺序 map 为什么采用 B-树实现，以及 `BTreeMap` 中包含的方法。

## 附录
