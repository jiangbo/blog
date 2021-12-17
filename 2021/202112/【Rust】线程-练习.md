# 【Rust】线程-练习

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std_misc/threads/testcase_mapreduce.html>  

## 示例

### main.rs

```rust
use std::thread;

static N_THREADS: usize = 10;

fn main() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];
    let chunked_data = data.splitn(N_THREADS, '\n');

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, result);
            result
        }));
    }

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();
    println!("Final sum result: {}", final_result);
}
```

## 总结

了解了 Rust 中 Map-Reduce 的使用，通过线程来计算数字的和。

## 附录
