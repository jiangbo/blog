# 【Rust】trait-删除

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/trait/drop.html>  

## 示例

Drop trait 可以实现删除或者说释放资源的目的。

### main.rs

```rust
struct Droppable {
    name: &'static str,
}

// 实现 Drop
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");
    // 可以手动调用来提前删除
    drop(_a);
    println!("end of the main function");
}
```

## 总结

了解了 Rust 中变量的删除时机，也可以手动调用 drop 函数还提前删除。

## 附录
