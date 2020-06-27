# 【JavaScript】标准内置函数 parseInt

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

parseInt(string, radix) 将一个字符串 string 转换为 radix 进制的整数， radix 为介于 2-36 之间的数。
parseInt 函数将其第一个参数转换为一个字符串，对该字符串进行解析，然后返回一个整数或 NaN。

如果不是 NaN，返回值将是第一个参数的整数，即第一个参数作为指定的 radix 的数字。(例如，radix 为 10，就是可以转换为十进制数，为 8 可以转换为八进制数，16 可以转换为十六进制数，以此类推)。

对于 radix 为 10 以上的，英文字母表示大于 9 的数字。例如，对于十六进制数（基数16），则使用 A 到 F 。

如果 parseInt 遇到的字符不是指定 radix 参数中的数字，它将忽略该字符以及所有后续字符，并返回到该点为止已解析的整数值。 parseInt 将数字截断为整数值。允许前导和尾随空格。

由于某些数字在其字符串表示形式中使用 e 字符（例如 6.022×23 表示 6.022e23 ），因此当对非常大或非常小的数字使用数字时，使用 parseInt 截断数字将产生意外结果。 parseInt 不应替代 Math.floor()。


parseInt 可以理解两个符号。+ 表示正数，- 表示负数（从ECMAScript 1开始）。它是在去掉空格后作为解析的初始步骤进行的。如果没有找到符号，算法将进入下一步；否则，它将删除符号，并对字符串的其余部分进行数字解析。

如果 radix 是 undefined、0 或未指定的，JavaScript 会假定以下情况：

1. 如果输入的 string 以 "0x" 或 "0x"（一个0，后面是小写或大写的X）开头，那么 radix 被假定为 16，字符串的其余部分被解析为十六进制数。
2. 如果输入的 string 以 "0"（0）开头， radix 被假定为 8（八进制）或 10（十进制）。具体选择哪一个 radix 取决于实现。ECMAScript 5 澄清了应该使用 10 (十进制)，但不是所有的浏览器都支持。因此，在使用 parseInt 时，一定要指定一个 radix。
3. 如果输入的 string 以任何其他值开头， radix 是 10 (十进制)。
4. 如果第一个字符不能转换为数字，parseInt会返回 NaN。

为了算术的目的，NaN 值不能作为任何 radix 的数字。你可以调用 isNaN 函数来确定 parseInt 的结果是否为 NaN。如果将 NaN 传递给算术运算，则运算结果也将是 NaN。

要将一个数字转换为特定的 radix 中的字符串字段，请使用 thatNumber.toString(radix) 函数。

## 使用 parseInt

```js
parseInt("0xF", 16);
parseInt("F", 16);
parseInt("17", 8);
parseInt(021, 8);
parseInt("015", 10);   // parseInt(015, 10); 返回 13
parseInt(15.99, 10);
parseInt("15,123", 10);
parseInt("FXX123", 16);
parseInt("1111", 2);
parseInt("15 * 3", 10);
parseInt("15e2", 10);
parseInt("15px", 10);
parseInt("12", 13);
```

## parseInt 返回 NaN

```js
parseInt("Hello", 8); // 根本就不是数值
parseInt("546", 2);   // 除了“0、1”外，其它数字都不是有效二进制数字
```

## 没有指定 radix 参数时的八进制解析

尽管 ECMAScript 3 已经不赞成这种做法，且 ECMAScript 5 已经禁止了这种做法，但是仍然有很多实现环境仍然把以 0 开头的数值字符串（numeric string）解释为一个八进制数。下面的例子可能返回八进制的结果，也可能返回十进制的结果。总是指定一个基数（radix）可以避免这种不可靠的行为。

```js
parseInt("0e0"); 
// 0

parseInt("08"); 
// 0, '8' 不是八进制数字.
```

## ECMAScript 5 移除了八进制解析

ECMAScript 5 规范不再允许 parseInt 函数的实现环境把以 0 字符开始的字符串作为八进制数值。ECMAScript 5 陈述如下：

根据给定 radix，parseInt 函数产生一个由字符串参数内容解析过来的整数值。字符串中开头的空白会被忽略。如果 radix 没有指定或者为 0，参数会被假定以 10 为基数来解析，如果数值以字符对 0x 或 0X 开头，会假定以 16 为基数来解析。

这与 ECMAScript 3 有所不同，ECMAScript 3 仅仅是不提倡这种做法但并没有禁止这种做法。

直至 2013 年，很多实现环境并没有采取新的规范所规定的做法, 而且由于必须兼容旧版的浏览器，所以永远都要明确给出 radix 参数的值.

## 一个更严格的解析函数

```js
filterInt = function (value) {
  if(/^(\-|\+)?([0-9]+|Infinity)$/.test(value))
    return Number(value);
  return NaN;
}

console.log(filterInt('421'));               // 421
console.log(filterInt('-421'));              // -421
console.log(filterInt('+421'));              // 421
console.log(filterInt('Infinity'));          // Infinity
console.log(filterInt('421e+0'));            // NaN
console.log(filterInt('421hop'));            // NaN
console.log(filterInt('hop1.61803398875'));  // NaN
console.log(filterInt('1.61803398875'));     // NaN
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/parseInt

