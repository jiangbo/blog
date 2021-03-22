# 【JavaScript】剩余参数

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

剩余参数语法允许我们将一个不定数量的参数表示为一个数组。

## 示例

```js
function sum(...theArgs) {
  return theArgs.reduce((previous, current) => {
    return previous + current;
  });
}

console.log(sum(1, 2, 3));
// expected output: 6

console.log(sum(1, 2, 3, 4));
// expected output: 10
```

## 剩余参数和 arguments对象的区别

剩余参数和 arguments 对象之间的区别主要有三个：

1. 剩余参数只包含那些没有对应形参的实参，而 arguments 对象包含了传给函数的所有实参。
2. arguments 对象不是一个真正的数组，而剩余参数是真正的 Array 实例。
3. arguments 对象还有一些附加的属性（如 callee 属性）。

## 解构剩余参数

剩余参数可以被解构，这意味着他们的数据可以被解包到不同的变量中。

```js
function f(...[a, b, c]) {
  return a + b + c;
}

f(1)          // NaN (b and c are undefined)
f(1, 2, 3)    // 6
f(1, 2, 3, 4) // 6 (the fourth parameter is not destructured)
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Functions/rest_parameters
