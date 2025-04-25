# 0895-wasm-Zig JSON 的传递

## 目标

实现 JS 传递 JSON 给 wasm，wasm 传递 JSON 给 JS。

## 环境

- Time 2025-04-25

## 参考

1. <https://blog.mjgrzymek.com/blog/zigwasm>

## 想法

感觉编译的 wasm 突然变得很大，不清楚是否是因为使用了 JSON 导致的。WASM 的学习就先到这里了，看看其它的。

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
    exe.root_module.export_symbol_names = &[_][]const u8{"computeAge"};

    b.installArtifact(exe);
}
```

## main.zig

```zig
const std = @import("std");
const allocator = std.heap.wasm_allocator;

const Person = struct {
    name: []u8,
    born: u16 = 0,
    age: u8 = 0,
};

export fn computeAge(buffer: [*]u8, len: u32, capacity: usize) i32 {
    const str = buffer[0..len];

    const parsed = std.json.parseFromSlice(Person, allocator, str, .{}) catch unreachable;
    defer parsed.deinit();
    var people = parsed.value;

    people.age = @intCast(2025 - people.born);

    const dst = buffer[0..capacity];
    var output: std.ArrayListUnmanaged(u8) = .initBuffer(dst);

    std.json.stringify(people, .{}, output.fixedWriter()) catch unreachable;
    return @intCast(output.items.len);
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
            const { memory, computeAge } = result.instance.exports;
            computeAge(memory.buffer)

            const person = { "name": "张三", "born": 2000 };
            const json = JSON.stringify(person);
            const encode = new TextEncoder('utf8');
            const { written } = encode.encodeInto(json, new Uint8Array(memory.buffer));

            const len = computeAge(memory.buffer, written, memory.buffer.byteLength);

            const decode = new TextDecoder('utf8');
            var bytes = new Uint8Array(memory.buffer, 0, len);
            const newPerson = JSON.parse(decode.decode(bytes));
            console.log(newPerson);
        });
    </script>
</body>

</html>
```

## 效果

![JSON 传递][1]

[1]: images/wasm10.png

## 附录
