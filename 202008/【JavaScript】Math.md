# 【JavaScript】Math

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Math 是一个内置对象，它拥有一些数学常数属性和数学函数方法。Math 不是一个函数对象。
Math 用于 Number 类型。它不支持 BigInt。

## 构造函数

与其他全局对象不同的是，Math 不是一个构造器。Math 的所有属性与方法都是静态的。引用圆周率的写法是 Math.PI，调用正余弦函数的写法是 Math.sin(x)，x 是要传入的参数。Math 的常量是使用 JavaScript 中的全精度浮点数来定义的。

## 静态属性

* Math.E：欧拉常数，也是自然对数的底数，约等于 2.718。
* Math.LN2：2 的自然对数，约等于 0.693。
* Math.LN10：10 的自然对数，约等于 2.303。
* Math.LOG2E：以 2 为底的 E 的对数，约等于 1.443。
* Math.LOG10E：以 10 为底的 E 的对数，约等于 0.434。
* Math.PI：圆周率，一个圆的周长和直径之比，约等于 3.14159。
* Math.SQRT1_2：二分之一 ½ 的平方根，约等于 0.707。
* Math.SQRT2：2 的平方根，约等于 1.414。

## 静态方法

Math 有很多的和数学相关的方法，下面只列出常用的方法。

### abs

Math.abs(x) 函数返回指定数字 “x“ 的绝对值。

```js
function difference(a, b) {
  return Math.abs(a - b);
}

console.log(difference(3, 5));
// expected output: 2

console.log(difference(5, 3));
// expected output: 2

console.log(difference(1.23456, 7.89012));
// expected output: 6.6555599999999995
```

### floor

Math.floor() 返回小于或等于一个给定数字的最大整数，向下取整。

```js
console.log(Math.floor(5.95));
// expected output: 5

console.log(Math.floor(5.05));
// expected output: 5

console.log(Math.floor(5));
// expected output: 5

console.log(Math.floor(-5.05));
// expected output: -6
```

### max

Math.max() 函数返回一组数中的最大值。

```js
console.log(Math.max(1, 3, 2));
// expected output: 3

console.log(Math.max(-1, -3, -2));
// expected output: -1

const array1 = [1, 3, 2];

console.log(Math.max(...array1));
// expected output: 3
```

### min

Math.min() 返回零个或更多个数值的最小值。

```js
console.log(Math.min(2, 3, 1));
// expected output: 1

console.log(Math.min(-2, -3, -1));
// expected output: -3

const array1 = [2, 3, 1];

console.log(Math.min(...array1));
// expected output: 1
```

### pow

Math.pow() 函数返回基数（base）的指数（exponent）次幂。

```js
console.log(Math.pow(7, 3));
// expected output: 343

console.log(Math.pow(4, 0.5));
// expected output: 2

console.log(Math.pow(7, -2));
// expected output: 0.02040816326530612
//                  (1/49)

console.log(Math.pow(-7, 0.5));
// expected output: NaN
```

### random

Math.random() 函数返回一个浮点，伪随机数在范围从 0 到小于 1，然后您可以缩放到所需的范围。

```js
function getRandomInt(max) {
  return Math.floor(Math.random() * Math.floor(max));
}

console.log(getRandomInt(3));
// expected output: 0, 1 or 2

console.log(getRandomInt(1));
// expected output: 0

console.log(Math.random());
// expected output: a number between 0 and 1
```

### round

Math.round() 函数返回一个数字四舍五入后最接近的整数。

```js
console.log(Math.round(0.9));
// expected output: 1

console.log(Math.round(5.95), Math.round(5.5), Math.round(5.05));
// expected output: 6 6 5

console.log(Math.round(-5.05), Math.round(-5.5), Math.round(-5.95));
// expected output: -5 -5 -6
```

### ceil

Math.ceil() 函数返回大于或等于一个给定数字的最小整数，向上取整。

```js
console.log(Math.ceil(.95));
// expected output: 1

console.log(Math.ceil(4));
// expected output: 4

console.log(Math.ceil(7.004));
// expected output: 8

console.log(Math.ceil(-7.004));
// expected output: -7
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Math

