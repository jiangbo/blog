# 【JavaScript】标准内置变量 Infinity

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

全局属性 Infinity 是一个数值，表示无穷大。

Infinity 的初始值是 Number.POSITIVE_INFINITY。
Infinity（正无穷大）大于任何值。该值和数学意义上的无穷大很像，
例如任何正值乘以 Infinity 为 Infinity, 任何数值除以 Infinity 为 0。

## 示例

```js
const maxNumber = Math.pow(10, 1000); // max positive number

console.log(maxNumber);

if (maxNumber === Infinity) {
  console.log("Let's call it Infinity!");
  // expected output: "Let's call it Infinity!"
}

console.log(Number.POSITIVE_INFINITY);


console.log(1 / maxNumber);
// expected output: 0

console.log(Infinity); /* Infinity */
console.log(Infinity + 1); /* Infinity */
console.log(Math.pow(10, 1000)); /* Infinity */
console.log(Math.log(0)); /* -Infinity */
console.log(1 / Infinity); /* 0 */
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Infinity

