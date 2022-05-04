# 0011-Rust-内部可变 Cell

## 环境

- Time 2022-05-04
- Rust 1.60.0

## 前言

### 说明

默认情况下，Rust 只能有一个可变引用和多个不可变引号，而 `Cell` 可以实现内部可变。

### 目标

内部可变 `Cell` 的方法练习。

## new

```rust
fn main() {
    let cell = Cell::new(44);
    println!("{:?}", cell);
}
```

## set

```rust
fn main() {
    let cell = Cell::new(44);
    cell.set(88);
    println!("{:?}", cell);
}
```

## get

```rust
fn main() {
    let cell = Cell::new(44);
    println!("{:?}", cell.get());
}
```

## swap

```rust
fn main() {
    let cell1 = Cell::new(44);
    let cell2 = Cell::new(88);
    cell1.swap(&cell2);
    println!("{cell1:?}, {cell2:?}");
}
```

## replace

```rust
fn main() {
    let cell = Cell::new(44);
    cell.replace(88);
    println!("{cell:?}");
}
```

## into_inner

```rust
fn main() {
    let cell = Cell::new(44);
    println!("{:?}", cell.into_inner());
}
```

## as_ptr

```rust
fn main() {
    let cell = Cell::new(44);
    println!("{:?}", cell.as_ptr());
}
```

## get_mut

```rust
fn main() {
    let mut cell = Cell::new(44);
    *cell.get_mut() += 44;
    println!("{:?}", cell);
}
```

## from_mut

```rust
fn main() {
    let mut value = 44;
    let cell = Cell::from_mut(&mut value);
    println!("{:?}", cell);
}
```

## take

```rust
fn main() {
    let cell = Cell::new(44);
    let _ = cell.take();
    println!("{:?}", cell);
}
```

## borrow

```rust
fn main() {
    let cell = Cell::new(44);
    println!("{:?}", cell.borrow());
}
```

## borrow_mut

```rust
fn main() {
    let mut cell = Cell::new(44);
    println!("{:?}", cell.borrow_mut());
}
```

## 总结

内部可变 `Cell` 的方法练习。

## 附录
