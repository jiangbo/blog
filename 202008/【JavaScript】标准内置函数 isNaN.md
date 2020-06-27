# 【JavaScript】标准内置函数 isNaN

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

isNaN() 函数用来确定一个值是否为 NaN。注：isNaN函数内包含一些非常有趣的规则；
你也可以使用 ECMAScript 2015 中定义的 Number.isNaN() 来判断。
如果给定值为 NaN 则返回值为 true；否则为 false。

## isNaN 函数的必要性

与 JavaScript 中其他的值不同，NaN不能通过相等操作符（== 和 ===）来判断，
因为 NaN == NaN 和 NaN === NaN 都会返回 false。因此，isNaN 就很有必要了。

## NaN 值的产生

当算术运算返回一个未定义的或无法表示的值时，NaN 就产生了。
但是，NaN 并不一定用于表示某些值超出表示范围的情况。
将某些不能强制转换为数值的非数值转换为数值的时候，也会得到 NaN。

ECMAScript (ES2015)包含 Number.isNaN() 函数。通过 Number.isNaN(x) 来检测变量 x 是否是一个 NaN 将会是一种可靠的做法。然而，在缺少 Number.isNaN 函数的情况下, 通过表达式(x != x) 来检测变量 x 是否是 NaN 会更加可靠。

一个 isNaN 的 polyfill 可以理解为（这个 polyfill 利用了 NaN 自身永不相等于自身这一特征 ）：

```js
var isNaN = function(value) {
    var n = Number(value);
    return n !== n;
};
```

## 示例

```js
isNaN(NaN);       // true
isNaN(undefined); // true
isNaN({});        // true

isNaN(true);      // false
isNaN(null);      // false
isNaN(37);        // false

// strings
isNaN("37");      // false: 可以被转换成数值37
isNaN("37.37");   // false: 可以被转换成数值37.37
isNaN("37,5");    // true
isNaN('123ABC');  // true:  parseInt("123ABC")的结果是 123, 但是Number("123ABC")结果是 NaN
isNaN("");        // false: 空字符串被转换成0
isNaN(" ");       // false: 包含空格的字符串被转换成0

// dates
isNaN(new Date());                // false
isNaN(new Date().toString());     // true

isNaN("blabla")   // true: "blabla"不能转换成数值
                  // 转换成数值失败， 返回NaN
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/isNaN

