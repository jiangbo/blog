# 【JavaScript】Reflect 静态方法（一）

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Reflect 是一个内置的对象，它提供拦截 JavaScript 操作的方法。这些方法与 proxy handlers 的方法相同。Reflect 不是一个函数对象，因此它是不可构造的。

Reflect 上含有一些静态方法，这些方法在 Object 上也有，同时还有一些操作符方法。

## apply

静态方法 Reflect.apply() 通过指定的参数列表发起对目标（target）函数的调用。

```js
console.log(Reflect.apply(Math.floor, undefined, [1.75]));
// expected output: 1

console.log(Reflect.apply(String.fromCharCode, undefined, [104, 101, 108, 108, 111]));
// expected output: "hello"

console.log(Reflect.apply(RegExp.prototype.exec, /ab/, ['confabulation']).index);
// expected output: 4

console.log(Reflect.apply(''.charAt, 'ponies', [3]));
// expected output: "i"
```

## construct

Reflect.construct() 方法的行为有点像 new 操作符构造函数，相当于运行 new target(...args)。

```js
function func1(a, b, c) {
  this.sum = a + b + c;
}

const args = [1, 2, 3];
const object1 = new func1(...args);
const object2 = Reflect.construct(func1, args);

console.log(object2.sum);
// expected output: 6

console.log(object1.sum);
// expected output: 6
```

## defineProperty

静态方法 Reflect.defineProperty() 基本等同于 Object.defineProperty() 方法，唯一不同是返回 Boolean 值。

```js
const object1 = {};

if (Reflect.defineProperty(object1, 'property1', { value: 42 })) {
  console.log('property1 created!');
  // expected output: "property1 created!"
} else {
  console.log('problem creating property1');
}

console.log(object1.property1);
// expected output: 42
```

## deleteProperty

静态方法 Reflect.deleteProperty() 允许用于删除属性。它很像 delete operator，但它是一个函数。

```js
const object1 = {
  property1: 42
};

Reflect.deleteProperty(object1, 'property1');

console.log(object1.property1);
// expected output: undefined

const array1 = [1, 2, 3, 4, 5];
Reflect.deleteProperty(array1, '3');

console.log(array1);
// expected output: Array [1, 2, 3, undefined, 5]
```

## get

Reflect.get() 方法与从对象 (target[propertyKey]) 中读取属性类似，但它是通过一个函数执行来操作的。

```js
const object1 = {
  x: 1,
  y: 2
};

console.log(Reflect.get(object1, 'x'));
// expected output: 1

const array1 = ['zero', 'one'];

console.log(Reflect.get(array1, 1));
// expected output: "one"
```

## getOwnPropertyDescriptor

静态方法 Reflect.getOwnPropertyDescriptor() 与 Object.getOwnPropertyDescriptor() 方法相似。如果在对象中存在，则返回给定的属性的属性描述符。否则返回 undefined。 

```js
const object1 = {
  property1: 42
};

console.log(Reflect.getOwnPropertyDescriptor(object1, 'property1').value);
// expected output: 42

console.log(Reflect.getOwnPropertyDescriptor(object1, 'property2'));
// expected output: undefined

console.log(Reflect.getOwnPropertyDescriptor(object1, 'property1').writable);
// expected output: true
```

## getPrototypeOf

静态方法 Reflect.getPrototypeOf() 与 Object.getPrototypeOf() 方法几乎是一样的。都是返回指定对象的原型（即内部的 [[Prototype]] 属性的值）。

```js
const object1 = {
  property1: 42
};

const proto1 = Reflect.getPrototypeOf(object1);

console.log(proto1);
// expected output: [object Object]

console.log(Reflect.getPrototypeOf(proto1));
// expected output: null
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Reflect
