# 【Rust】标准库-链表

## 环境

- Time 2022-04-02
- Rust 1.59.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构。

### 特点

链表也是一种线性的数据结构，可以方便地插入和删除元素，不过按位置访问需要从头遍历。
由于 Rust 的所有权机制，实现链表比其它语言复杂，不过标准库已内置。

## 示例

### new

```Rust
let _ = LinkedList::<i32>::new();
```

### append

```rust
fn main() {
    let mut list1 = LinkedList::new();
    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');

    list1.append(&mut list2);
    println!("{list1:?}");
}
```

### iter

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(0);
    list.push_back(44);

    list.iter().for_each(|e| println!("{e}"))
}
```

### is_empty

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);

    println!("{:?}", list.is_empty());
}
```

### len

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);

    println!("{:?}", list.len());
}
```

### clear

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);
    list.clear();

    println!("{:?}", list.is_empty());
}
```

### front

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);
    list.push_back(444);

    println!("{:?}", list.front());
}
```

### back

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);
    list.push_back(444);

    println!("{:?}", list.back());
}
```

### push_front

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);
    list.push_front(444);

    println!("{:?}", list.front());
}
```

### pop_front

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);
    list.push_front(444);

    println!("{:?}", list.pop_front());
}
```

### push_back

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);
    list.push_front(444);

    println!("{:?}", list.front());
}
```

### pop_back

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);
    list.push_front(444);

    println!("{:?}", list.pop_back());
}
```

### split_off

```rust
fn main() {
    let mut list = LinkedList::new();
    list.push_back(44);
    list.push_front(444);

    println!("{:?}", list.split_off(1));
}
```

## 总结

使用了标准库中提供的链表。

## 附录
