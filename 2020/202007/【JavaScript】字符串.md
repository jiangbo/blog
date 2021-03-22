# 【JavaScript】字符串

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 创建字符串

字符串，即我们通常所说的文本。

`let string = 'The revolution will not be televised.';`

字符串可以使用单引号或者双引号包裹起来，但是必须成对。

### 转义字符

在字符串中，可以使用 \ 来转义字符。

`let bigmouth = 'I\'ve got no right to take my place...';`

## 连接字符串

在 JavaScript 中连接字符串使用加号(+)操作符

```js
let one = 'Hello, ';
let two = 'how are you?';
let joined = one + two;
console.log(joined);
```

输出内容为：Hello, how are you?

## 数字与字符

1. 数字和字符串拼接，浏览器会将数字转成字符串，然后拼接在一起。
2. 两个至包含数字的字符串相加，还是会进行字符串拼接，而不是数学运算。
3. 可以使用 Number(str) 转换成相应的数字。
4. 没有数字有一个 toString 方法，可以将数字转换成字符串。

```js
let myDate = '19' + '67';
console.log(myDate);
console.log(typeof myDate);
console.log(Number(myDate));
console.log(typeof Number(myDate));
console.log(typeof Number(myDate).toString());
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/First_steps/Strings
