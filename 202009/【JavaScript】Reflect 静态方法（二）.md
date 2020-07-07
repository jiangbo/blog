# 【JavaScript】Reflect 静态方法（二）

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Reflect 是一个内置的对象，它提供拦截 JavaScript 操作的方法。这些方法与 proxy handlers 的方法相同。Reflect 不是一个函数对象，因此它是不可构造的。

Reflect 上含有一些静态方法，这些方法在 Object 上也有，同时还有一些操作符方法。

## has

静态方法 Reflect.has() 作用与 in 操作符相同。

```js
const object1 = {
  property1: 42
};

console.log(Reflect.has(object1, 'property1'));
// expected output: true

console.log(Reflect.has(object1, 'property2'));
// expected output: false

console.log(Reflect.has(object1, 'toString'));
// expected output: true
```

## isExtensible

静态方法 Reflect.isExtensible() 判断一个对象是否可扩展（即是否能够添加新的属性）。与它 Object.isExtensible() 方法相似。

```js
const object1 = {};

console.log(Reflect.isExtensible(object1));
// expected output: true

Reflect.preventExtensions(object1);

console.log(Reflect.isExtensible(object1));
// expected output: false

const object2 = Object.seal({});

console.log(Reflect.isExtensible(object2));
// expected output: false
```

## ownKeys

静态方法 Reflect.ownKeys() 返回一个由目标对象自身的属性键组成的数组。

```js
const object1 = {
  property1: 42,
  property2: 13
};

const array1 = [];

console.log(Reflect.ownKeys(object1));
// expected output: Array ["property1", "property2"]

console.log(Reflect.ownKeys(array1));
// expected output: Array ["length"]
```

## preventExtensions

静态方法 Reflect.preventExtensions() 方法阻止新属性添加到对象 (例如：防止将来对对象的扩展被添加到对象中)。该方法与 Object.preventExtensions() 相似。

```js
const object1 = {};

console.log(Reflect.isExtensible(object1));
// expected output: true

Reflect.preventExtensions(object1);

console.log(Reflect.isExtensible(object1));
// expected output: false
```

## set

静态方法 Reflect.set() 工作方式就像在一个对象上设置一个属性。

```js
const object1 = {};
Reflect.set(object1, 'property1', 42);

console.log(object1.property1);
// expected output: 42

const array1 = ['duck', 'duck', 'duck'];
Reflect.set(array1, 2, 'goose');

console.log(array1[2]);
// expected output: "goose"
```

### setPrototypeOf

除了返回类型以外，静态方法 Reflect.setPrototypeOf() 与 Object.setPrototypeOf() 方法是一样的。它可设置对象的原型（即内部的 [[Prototype]] 属性）为另一个对象或 null，如果操作成功返回 true，否则返回 false。

```js
const object1 = {};

console.log(Reflect.setPrototypeOf(object1, Object.prototype));
// expected output: true

console.log(Reflect.setPrototypeOf(object1, null));
// expected output: true

const object2 = {};

console.log(Reflect.setPrototypeOf(Object.freeze(object2), null));
// expected output: false
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Reflect
