# 【JavaScript】字符串常用方法

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 字符串长度

```js
let str = "This is 张三";
console.log(str.length); // 10
console.log(String.length); // 1
console.log("".length); // 0
```

## 按索引获取字符

按索引获取字符串可以获取某个索引上的字符串，索引从 0 开始。

```js
let str = "This is 张三";
console.log(str[0]); // T
console.log(str[str.length - 1]); // 三
```

## 查找子字符串

有时候你会想要找出一个较小的字符串是否存在于一个较大的字符串中，这可以使用 indexOf 方法来完成，
该方法需要一个 parameter，即你想要搜索的子字符串，如果没有则返回 -1;
如果包含子字符串，则返回第一次出现的位置，从 0 开始计数。

```js
let str = "This is 张三";
console.log(str.indexOf("张三")); // 8
console.log(str.indexOf("si")); // -1
```

## 转换大小写

字符串方法 toLowerCase 和 toUpperCase 字符串并将所有字符分别转换为小写或大写。

```js
let str = "This is 张三";
console.log(str.toUpperCase()); // THIS IS 张三
console.log(str.toLowerCase()); // this is 张三
```

## 替换

可以使用 replace 方法将字符串中的一个子字符串替换为另一个子字符串。
它需要两个参数：要被替换下的字符串和要被替换上的字符串。

```js
let str = "This is 张三";
console.log(str.replace("张三", "李四")); // This is 李四
```

## 截取

slice 方法可以根据位置来截取字符串，包含两个参数，一个是开始位置，一个结束位置。
截取时，包含开始位置，而不包含结束位置。

```js
let str = "This is 张三";
console.log(str.slice());
console.log(str.slice(3));
console.log(str.slice(3, 9));
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/First_steps/Useful_string_methods