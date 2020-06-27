# 【JavaScript】Number

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

JavaScript 的 Number 对象是经过封装的能让你处理数字值的对象。Number 对象由 Number() 构造器创建。

JavaScript 的 Number 类型为双精度 IEEE 754 64 位浮点类型。

## 构造函数

如果参数无法被转换为数字，则返回 NaN。
在非构造器上下文中 (如：没有 new 操作符)，Number 能被用来执行类型转换。

```js
new Number(value); 
var a = new Number('123'); // a === 123 is false
var b = Number('123'); // b === 123 is true
a instanceof Number; // is true
b instanceof Number; // is false
```

## 静态属性

* Number.EPSILON：两个可表示 (representable) 数之间的最小间隔。
* Number.MAX_SAFE_INTEGER：JavaScript 中最大的安全整数 (2^53 - 1)。
* Number.MAX_VALUE：能表示的最大正数。最小的负数是 -MAX_VALUE。
* Number.MIN_SAFE_INTEGER：JavaScript 中最小的安全整数 (-(253 - 1)).
* Number.MIN_VALUE：能表示的最小正数即最接近 0 的正数 (实际上不会变成 0)。最大的负数是 -MIN_VALUE。
* Number.NaN：特殊的“非数字”值。
* Number.NEGATIVE_INFINITY：特殊的负无穷大值，在溢出时返回该值。
* Number.POSITIVE_INFINITY：特殊的正无穷大值，在溢出时返回该值。


### 静态方法

其中 Number.parseFloat(string) 和 Number.parseInt(string, [radix]) 这两个方法与全局的函数相同，并且处于 ECMAScript 6 规范中（用于全局变量的模块化）。

isNaN 和 isFinite 方法和全局的方法相同，但是不会自行将参数转换成数字，只有在参数是值为 NaN/finite 的数字时，才会返回 true。

### isInteger

Number.isInteger() 方法用来判断给定的参数是否为整数。

```js
function fits(x, y) {
  if (Number.isInteger(y / x)) {
    return 'Fits!';
  }
  return 'Does NOT fit!';
}

console.log(fits(5, 10));
// expected output: "Fits!"

console.log(fits(5, 11));
// expected output: "Does NOT fit!"
```

### isSafeInteger

Number.isSafeInteger() 方法用来判断传入的参数值是否是一个“安全整数”（safe integer）。

一个安全整数是一个符合下面条件的整数：

* 可以准确地表示为一个IEEE-754双精度数字,
* 其IEEE-754表示不能是舍入任何其他整数以适应IEEE-754表示的结果。.

比如，2^53 - 1 是一个安全整数，它能被精确表示，在任何 IEEE-754 舍入模式（rounding mode）下，没有其他整数舍入结果为该整数。作为对比，2^53 就不是一个安全整数，它能够使用 IEEE-754 表示，但是 2^53 + 1 不能使用 IEEE-754 直接表示，在就近舍入（round-to-nearest）和向零舍入中，会被舍入为 253。

安全整数范围为 -(2^53 - 1)到 2^53 - 1 之间的整数，包含 -(2^53 - 1)和 2^53 - 1。

```js
function warn(x) {
  if (Number.isSafeInteger(x)) {
    return 'Precision safe.';
  }
  return 'Precision may be lost!';
}

console.log(warn(Math.pow(2, 53)));
// expected output: "Precision may be lost!"

console.log(warn(Math.pow(2, 53) - 1));
// expected output: "Precision safe."
```

## 实例方法

Number.prototype.toString()：返回一个表示该数值对象的字符串。覆盖了 Object.prototype.toString() 方法。
Number.prototype.valueOf()：返回该数值对象的原始值。覆盖了 Object.prototype.valueOf() 方法。
Number.prototype.toLocaleString()：返回一个与语言相关的该数值对象的字符串表示。覆盖了Object.prototype.toLocaleString() 方法。

### toExponential

toExponential() 方法以指数表示法返回该数值字符串表示形式。

```js
function expo(x, f) {
  return Number.parseFloat(x).toExponential(f);
}

console.log(expo(123456, 2));
// expected output: "1.23e+5"

console.log(expo('123456'));
// expected output: "1.23456e+5"

console.log(expo('oink'));
// expected output: "NaN"
```

### toFixed

toFixed() 方法使用定点表示法来格式化一个数值。保留小数的位数，会进行四舍五入和用 0 占位。

```js
function financial(x) {
  return Number.parseFloat(x).toFixed(2);
}

console.log(financial(123.456));
// expected output: "123.46"

console.log(financial(0.004));
// expected output: "0.00"

console.log(financial('1.23e+5'));
// expected output: "123000.00"
```

###  toPrecision

toPrecision() 方法以指定的精度返回该数值对象的字符串表示。

```js
function precise(x) {
  return Number.parseFloat(x).toPrecision(4);
}

console.log(precise(123.456));
// expected output: "123.5"

console.log(precise(0.004));
// expected output: "0.004000"

console.log(precise('1.23e+5'));
// expected output: "1.230e+5"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Number

