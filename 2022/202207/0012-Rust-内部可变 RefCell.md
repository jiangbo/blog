# 0012-Rust-内部可变 RefCell

## 环境

- Time 2022-05-04
- Rust 1.60.0

## 前言

### 说明

默认情况下，Rust 只能有一个可变引用和多个不可变引号，而 `RefCell` 可以实现内部可变。

### 目标

内部可变 `RefCell` 的方法练习。

## new

```rust
fn main() {
    let cell = RefCell::new(44);
    println!("{:?}", cell);
    println!("{:?}", cell.into_inner());
}
```

## replace

```rust
fn main() {
    let cell = RefCell::new(44);
    cell.replace(88);
    println!("{:?}", cell);
}
```

## replace_with

```rust
fn main() {
    let cell = RefCell::new(44);
    cell.replace_with(|old| *old + 44);
    println!("{:?}", cell);
}
```

## swap

```rust
fn main() {
    let cell1 = RefCell::new(44);
    let cell2 = RefCell::new(88);
    cell1.swap(&cell2);
    println!("{:?}", cell1);
}
```

## try_borrow

```rust
fn main() {
    let cell = RefCell::new(44);
    println!("{:?}", cell.try_borrow());
}
```

## try_borrow_mut

```rust
fn main() {
    let cell = RefCell::new(44);
    println!("{:?}", cell.try_borrow_mut());
}
```

## get_mut

```rust
fn main() {
    let mut cell = RefCell::new(44);
    *cell.get_mut() += 44;
    println!("{:?}", cell);
}
```

## 总结

内部可变 `RefCell` 的方法练习。

## 附录
