# 【JavaScript】箭头函数

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

箭头函数表达式的语法比函数表达式更简洁，并且没有自己的 this，arguments，super 或 new.target。箭头函数表达式更适用于那些本来需要匿名函数的地方，并且它不能用作构造函数。

## 示例

```js
const materials = [
  'Hydrogen',
  'Helium',
  'Lithium',
  'Beryllium'
];

console.log(materials.map(material => material.length));
// expected output: Array [8, 6, 7, 9]
```

## 演示

```js
// 空的箭头函数返回 undefined
let empty = () => {};

(() => 'foobar')(); 
// Returns "foobar"
// (这是一个立即执行函数表达式,可参阅 'IIFE'术语表) 


var simple = a => a > 15 ? 15 : a; 
simple(16); // 15
simple(10); // 10

let max = (a, b) => a > b ? a : b;

// Easy array filtering, mapping, ...

var arr = [5, 6, 13, 0, 1, 18, 23];

var sum = arr.reduce((a, b) => a + b);  
// 66

var even = arr.filter(v => v % 2 == 0); 
// [6, 0, 18]

var double = arr.map(v => v * 2);       
// [10, 12, 26, 0, 2, 36, 46]

// 更简明的promise链
promise.then(a => {
  // ...
}).then(b => {
  // ...
});

// 无参数箭头函数在视觉上容易分析
setTimeout( () => {
  console.log('I happen sooner');
  setTimeout( () => {
    // deeper code
    console.log('I happen later');
  }, 1);
}, 1);
```


[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Functions/Arrow_functions
