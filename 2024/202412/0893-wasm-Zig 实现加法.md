# 0893-wasm-Zig 实现加法

## 目标

使用 Zig 来编写一个加法的函数，然后编译到 wasm，最后浏览器进行调用输出。

## 环境

- Time 2025-04-25

## 参考

1. <https://blog.mjgrzymek.com/blog/zigwasm>

## 想法

Zig 对这 wasm 这块的支持还不成熟，缺少很多工具，可以简单试用一下。

## build.zig.zon

```zig
.{
    .name = .wasm,
    .version = "0.0.0",
    .fingerprint = 0xa9a8d1fef8755172,
    .minimum_zig_version = "0.14.0",
    .dependencies = .{},
    .paths = .{""},
}
```

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
    exe.root_module.export_symbol_names = &[_][]const u8{"add"};

    b.installArtifact(exe);
}
```

## main.zig

```zig
extern fn print(a: i32) void;

export fn add(a: i32, b: i32) i32 {
    print(a + b);
    return a + b;
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
        <div>计算加法 2 + 2 = <span id="result"></span></div>
    </div>

    <script type="text/javascript">

        WebAssembly.instantiateStreaming(fetch("demo.wasm"), {
            env: {
                print: (x) => console.log(x)
            }
        }).then(result => {
            const add = result.instance.exports.add;
            document.getElementById('result').innerText = add(2, 2);
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

导出了一块内存，大小是 1M，原因可以看这里：

1. <https://ziggit.dev/t/using-zig-with-webassembly/3478>
2. <https://github.com/WebAssembly/tool-conventions/blob/4dd47d204df0c789c23d246bc4496631b5c199c4/DynamicLinking.md?plain=1#L136>

```pwsh
PS C:\workspace\wasm> wasm2wat .\zig-out\bin\demo.wasm
(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (import "env" "print" (func (;0;) (type 0)))
  (func (;1;) (type 1) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add
    local.tee 0
    call 0
    local.get 0)
  (memory (;0;) 16)
  (global (;0;) (mut i32) (i32.const 1048576))
  (export "memory" (memory 0))
  (export "add" (func 1)))
```
