# 【JavaScript】基本类型

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

基本类型（基本数值、基本数据类型）是一种既非对象也无方法的数据。
在 JavaScript 中，共有7种基本类型：
* string
* number
* bigint
* boolean
* null
* undefined
* symbol (ECMAScript 2016新增)。

多数情况下，基本类型直接代表了最底层的语言实现。

所有基本类型的值都是不可改变的。但需要注意的是，基本类型本身和一个赋值为基本类型的变量的区别。
变量会被赋予一个新值，而原值不能像数组、对象以及函数那样被改变。

## 示例1

这个示例会帮助你了解基本类型不可改变的事实。

```js
// 使用字符串方法不会改变一个字符串
var bar = "baz";
console.log(bar);               // baz
bar.toUpperCase();
console.log(bar);               // baz

// 使用数组方法可以改变一个数组
var foo = [];
console.log(foo);               // []
foo.push("plugh");
console.log(foo);               // ["plugh"]

// 赋值行为可以给基本类型一个新值，而不是改变它
bar = bar.toUpperCase();       // BAZ
```

## 示例2

下面的示例将让你体会到JavaScript是如何处理基本类型的。

```js
// 基本类型
let foo = 5;

// 定义一个貌似可以改变基本类型值的函数
function addTwo(num) {
   num += 2;
}
// 和前面的函数一样
function addTwo_v2(foo) {
   foo += 2;
}

// 调用第一个函数，并传入基本类型值作为参数
addTwo(foo);
// Getting the current Primitive value
console.log(foo);   // 5

// 尝试调用第二个函数...
addTwo_v2(foo);
console.log(foo);   // 5
```

## 包装对象

除了 null 和 undefined 之外，所有基本类型都有其对应的包装对象：

* String 为字符串基本类型。
* Number 为数值基本类型。
* BigInt 为大整数基本类型。
* Boolean 为布尔基本类型。
* Symbol 为字面量基本类型。

这个包裹对象的 valueOf() 方法返回基本类型值。

[1]: https://developer.mozilla.org/zh-CN/docs/Glossary/Primitive

