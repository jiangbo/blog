# 【JavaScript】标准内置函数 parseFloat

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

parseFloat() 函数解析一个参数（必要时先转换为字符串）并返回一个浮点数。
parseFloat 是个全局函数，不属于任何对象。

* 如果 parseFloat 在解析过程中遇到了正号（+）、负号（- U+002D HYPHEN-MINUS）、数字（0-9）、小数点（.）、或者科学记数法中的指数（e 或 E）以外的字符，则它会忽略该字符以及之后的所有字符，返回当前已经解析到的浮点数。
* 第二个小数点的出现也会使解析停止（在这之前的字符都会被解析）。
* 参数首位和末位的空白符会被忽略。
* 如果参数字符串的第一个字符不能被解析成为数字,则 parseFloat 返回 NaN。
* parseFloat 也可以解析并返回 Infinity。
* parseFloat 解析 BigInt 为 Numbers, 丢失精度。因为末位 n 字符被丢弃。

考虑使用 Number(value) 进行更严谨的解析，只要参数带有无效字符就会被转换为 NaN 。

parseFloat 也可以转换一个已经定义了 toString 或者 valueOf 方法的对象，它返回的值和在调用该方法的结果上调用 parseFloat 值相同。

## parseFloat 返回正常数字

```js
parseFloat(3.14);
parseFloat('3.14');
parseFloat('  3.14  ');
parseFloat('314e-2');
parseFloat('0.0314E+2');
parseFloat('3.14some non-digit characters');
parseFloat({ toString: function() { return "3.14" } });
```

## parseFloat 返回 NaN

```js
parseFloat("FF2");
```

## parseFloat 和 BigInt

以下例子均返回 900719925474099300，当整数太大以至于不能被转换时将失去精度。

```js
parseFloat(900719925474099267n);
parseFloat('900719925474099267n');
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/parseFloat

