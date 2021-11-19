# 【Rust】单元测试

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html>  

## 示例

### 测试相等

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}

fn main() {}
```

### 测试恐慌

```rust
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}

fn main() {}
```

### 运行一个测试

```text
C:\Users\jiangbo\work\rust>cargo test test_any_panic
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests (target\debug\deps\rust-ee9e808a3f21b94a.exe)

running 1 test
test tests::test_any_panic - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

### 运行多个测试

```text
C:\Users\jiangbo\work\rust>cargo test panic
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests (target\debug\deps\rust-ee9e808a3f21b94a.exe)

running 2 tests
test tests::test_any_panic - should panic ... ok
test tests::test_specific_panic - should panic ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

### 忽略测试

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}

fn main() {}
```

## 总结

了解了 Rust 中外部函数接口的使用。

## 附录
