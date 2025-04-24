# 0889-wasm-ArrayBuffer

## 目标

JS 可以使用各种 TypedArray 来操作底层的 ArrayBuffer。

## 环境

- Time 2025-04-24

## 参考

1. <https://jimmysong.io/book/wasm-definitive-guide/>

## 想法

修改和改变视图的时候，底层的数据没有修改，所以这里没有进行复制。

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
    <script type="text/javascript">

        const uint32Array = new Uint32Array(4);
        uint32Array[0] = 257;
        console.log(uint32Array);

        console.log("u32 array length:" + uint32Array.length);

        const uint8Array = new Uint8Array(uint32Array.buffer);
        console.log(uint8Array);
        uint8Array[0] = 2;
        console.log(uint8Array);
        console.log(uint32Array);

        console.log("u8 array length:" + uint8Array.length);



        WebAssembly.instantiateStreaming(fetch("demo.wasm"), {
            env: {
                print: (num) => console.log(num)
            }
        }).then(result => {
            const sum = result.instance.exports.add(1, 1);
            console.log('js sum: ', sum);
        })
    </script>
</body>

</html>
```

## 效果

![ArrayBuffer][1]

[1]: images/wasm04.png

## 附录
