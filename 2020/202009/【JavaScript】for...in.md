# 【JavaScript】for...in

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

for...in 语句以任意顺序遍历一个对象的除 Symbol 以外的可枚举属性。

> 提示：for...in不应该用于迭代一个 Array，其中索引顺序很重要。

## 示例

```js
const object = { a: 1, b: 2, c: 3 };

for (const property in object) {
  console.log(`${property}: ${object[property]}`);
}

// expected output:
// "a: 1"
// "b: 2"
// "c: 3"
```

## 仅迭代自身的属性

如果你只要考虑对象本身的属性，而不是它的原型，那么使用 getOwnPropertyNames() 或执行 hasOwnProperty() 来确定某属性是否是对象本身的属性（也能使用 propertyIsEnumerable）。或者，如果你知道不会有任何外部代码干扰，您可以使用检查方法扩展内置原型。


## 为什么用for...in

for... in是为遍历对象属性而构建的，不建议与数组一起使用，数组可以用 Array.prototype.forEach() 和 for...of，那么 for...in 的到底有什么用呢？

它最常用的地方应该是用于调试，可以更方便的去检查对象属性（通过输出到控制台或其他方式）。尽管对于处理存储数据，数组更实用些，但是你在处理有 key-value 数据（比如属性用作“键”），需要检查其中的任何键是否为某值的情况时，还是推荐用 for...in。

```js
var triangle = {a: 1, b: 2, c: 3};

function ColoredTriangle() {
  this.color = 'red';
}

ColoredTriangle.prototype = triangle;

var obj = new ColoredTriangle();

for (var prop in obj) {
  if (obj.hasOwnProperty(prop)) {
    console.log(`obj.${prop} = ${obj[prop]}`);
  } 
}

// Output:
// "obj.color = red"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/for...in
