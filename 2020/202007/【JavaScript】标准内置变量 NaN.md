# 【JavaScript】标准内置变量 NaN

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

全局属性 NaN 的值表示不是一个数字（Not-A-Number）。

NaN 属性的初始值就是 NaN，和 Number.NaN 的值一样。
在现代浏览器中（ ES5 中）， NaN 属性是一个不可配置（non-configurable），不可写（non-writable）的属性。
但在 ES3 中，这个属性的值是可以被更改的，但是也应该避免覆盖。

编码中很少直接使用到 NaN。通常都是在计算失败时，作为 Math 的某个方法的返回值出现的
（例如：Math.sqrt(-1)）或者尝试将一个字符串解析成数字但失败了的时候（例如：parseInt("blabla")）。

## 判断一个值是否是NaN

NaN 如果通过 == 、 != 、 === 、以及 !== 与其他任何值比较都将不相等，包括与其他 NAN值进行比较。
必须使用 Number.isNaN() 或 isNaN() 函数。在执行自比较之中：NaN，也只有NaN，比较之中不等于它自己。

```js
NaN === NaN;        // false
Number.NaN === NaN; // false
isNaN(NaN);         // true
isNaN(Number.NaN);  // true

function valueIsNaN(v) { return v !== v; }
valueIsNaN(1);          // false
valueIsNaN(NaN);        // true
valueIsNaN(Number.NaN); // true
```

但是，请注意isNaN（）和Number.isNaN（）之间的区别：
如果当前值是NaN，或者将其强制转换为数字后将是NaN，则前者将返回true。而后者仅当值当前为NaN时才为true：

```js
isNaN('hello world');        // true
Number.isNaN('hello world'); // false
```

## 示例

```js
function sanitise(x) {
  if (isNaN(x)) {
    return NaN;
  }
  return x;
}

console.log(sanitise('1'));
// expected output: "1"

console.log(sanitise('NotANumber'));
// expected output: NaN
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/NaN

