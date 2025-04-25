# 0894-wasm-Zig 填充字符串

## 目标

使用 Zig 来编写一个填充字符串的函数，然后编译到 wasm，最后浏览器进行调用输出。

## 环境

- Time 2025-04-25

## 参考

1. <https://blog.mjgrzymek.com/blog/zigwasm>

## 想法

基于之前的 WAT 写的字符串例子来写的，环境和之前一样。

## build.zig

```zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{
        .name = "demo",
        .root_source_file = b.path("src/main.zig"),
        .target = b.standardTargetOptions(.{}),
        .optimize = b.standardOptimizeOption(.{}),
    });

    exe.entry = .disabled;
    exe.root_module.export_symbol_names = &[_][]const u8{"fillString"};

    b.installArtifact(exe);
}
```

## main.zig

```zig
const text = "纸上得来终觉浅。WebAssembly is designed to be pretty-printed.";

export fn fillString(buffer: [*]u8) i32 {
    const dst = buffer[0..text.len];
    @memcpy(dst, text);
    return text.len;
}
```

## 构建

```pwsh
zig build -Dtarget=wasm32-freestanding --release=small
```

## index.html

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>测试 WASM</title>
</head>

<body>
    <div id="container">
        <div>中文: <span id="chinese"></span></div>
        <div>英文: <span id="english"></span></div>
    </div>

    <script type="text/javascript">

        WebAssembly.instantiateStreaming(fetch("demo.wasm")).then(result => {
            const { memory, fillString } = result.instance.exports;
            fillString(memory.buffer)

            const decode = new TextDecoder('utf8');

            var bytes = new Uint8Array(memory.buffer, 0, 24);
            var string = decode.decode(bytes);
            document.getElementById('chinese').innerText = string;

            bytes = new Uint8Array(memory.buffer, 24, 45);
            string = decode.decode(bytes);
            document.getElementById('english').innerText = string;
        });
    </script>
</body>

</html>
```

## 效果

![字符串][1]

[1]: images/wasm08.png

## 附录

### wasm2wat

```pwsh
PS C:\workspace\wasm> wasm2wat .\zig-out\bin\demo.wasm
(module
  (type (;0;) (func (param i32) (result i32)))
  (func (;0;) (type 0) (param i32) (result i32)
    local.get 0
    i32.const 1048576
    i32.const 69
    memory.copy
    i32.const 69)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (export "memory" (memory 0))
  (export "fillString" (func 0))
  (data (;0;) (i32.const 1048576) "\e7\ba\b8\e4\b8\8a\e5\be\97\e6\9d\a5\e7\bb\88\e8\a7\89\e6\b5\85\e3\80\82WebAssembly is designed to be pretty-printed.\00"))
```
