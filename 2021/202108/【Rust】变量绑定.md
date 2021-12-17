# 【Rust】变量绑定

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html>

Rust 通过静态类型提供类型安全，可以在变量绑定时进行类型注释。  
然而，在大多数情况下，编译器将能够从上下文中推断出变量的类型，大大减轻了注释负担。  
使用 `let` 关键字进行变量绑定。

## 示例

### 变量申明

```rust
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 复制一个变量
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 变量没有使用的话，编译器进行警告，可以以下划线开头抑制编译器警告
    let _unused_variable = 3u32;
}
```

### 可变变量

变量绑定默认是不可变的，可以加上 `mut` 使其可变。

```rust
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误，不可变变量
    // _immutable_binding += 1;
}
```

### 变量作用域

变量绑定有作用域，它被限制在一个代码块中生存，代码块是使用大括号包围的语句的集合。

```rust
fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    // 错误，已超出变量的作用域
    // println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);
}
```

### 变量遮盖

重复申明变量可以使变量被遮盖。

```rust
fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);
        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
```

### 变量提前申明

变量绑定可以先申明，后初始化。不过较少使用，可能会导致变量未初始化。

```rust
fn main() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // 错误，变量未初始化
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
}
```

### 变量冻结

可以将可变的变量赋值给同名的不可变变量，由于变量遮盖，变量将不再可变，这叫变量冻结。

```rust
fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // 错误，变量被冻结，不可变。
        // _mutable_integer = 50;
    }

    // 未被冻结
    _mutable_integer = 3;
}
```

## 总结

了解了 Rust 中的变量绑定，以及变量的可变，遮盖，冻结，提前申明等。

## 附录
