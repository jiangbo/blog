# 0013-wasm-hello world

## 环境

- Time 2022-05-12
- Rust 1.60.0
- Node 12.22.5
- wasm-pack 0.10.2

## 前言

### 说明

环境配置参考：<https://rustwasm.github.io/docs/book/game-of-life/setup.html>
参考：<https://rustwasm.github.io/docs/book/game-of-life/hello-world.html>

### 目标

实现第一个 wasm 程序，使用 wasm 弹出一个 alert 框。

## Cargo.toml

```toml
[package]
edition = "2021"
name = "game"
version = "0.1.0"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["console_error_panic_hook"]

[dependencies]
console_error_panic_hook = {version = "*", optional = true}
wasm-bindgen = "*"
wee_alloc = {version = "*", optional = true}

[dev-dependencies]
wasm-bindgen-test = "*"

[profile.release]
opt-level = "s"

```

## lib.rs

```rust
mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("hello wasm");
}
```

## utils.rs

```rust
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
```

## 增加构建目标

```sh
rustup target add wasm32-unknown-unknown
```

## wasm-pack

```text
wasm-pack build
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
warning: function is never used: `set_panic_hook`
 --> src\utils.rs:1:8
  |
1 | pub fn set_panic_hook() {
  |        ^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `game` (lib) generated 1 warning
    Finished release [optimized] target(s) in 0.07s
[WARN]: :-) origin crate has no README
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 9.67s
[INFO]: :-) Your wasm pkg is ready to publish at C:\Users\jiangbo\work\game\pkg.
```

## 生成前端

```sh
npm init wasm-app www
```

## 增加依赖

```json
"dependencies": {
    "game": "file:../pkg"
}
```

## 启动

```sh
npm run start
```

## 总结

实现了第一个 wasm 程序，在浏览器中弹出了一个 alert 框。

## 附录
