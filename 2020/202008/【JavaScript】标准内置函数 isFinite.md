# 【JavaScript】标准内置函数 isFinite

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

该全局 isFinite() 函数用来判断被传入的参数值是否为一个有限数值（finite number）。
在必要情况下，参数会首先转为一个数值。

## 示例

你可以用这个方法来判定一个数字是否是有限数字。isFinite 方法检测它参数的数值。
如果参数是 NaN，正无穷大或者负无穷大，会返回 false，其他返回 true。

```js
isFinite(Infinity);  // false
isFinite(NaN);       // false
isFinite(-Infinity); // false

isFinite(0);         // true
isFinite(2e64);      // true, 在更强壮的Number.isFinite(null)中将会得到false


isFinite("0");       // true, 在更强壮的Number.isFinite('0')中将会得到false
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/isFinite

