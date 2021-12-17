# 【Rust】泛型函数

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/gen_fn.html>  

## 示例

### main.rs

```rust
struct A;
struct SGen<T>(T);
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // 参数是泛型，并且在定义的时候就确定了
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    // 在调用的时候，才确定参数泛型类型
    generic::<char>(SGen('a'));
    generic(SGen('c'));
}
```

## 总结

了解了 Rust 中怎么定义一个泛型函数。

## 附录
