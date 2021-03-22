# 【JavaScript】Object 静态方法（三）

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

定义在 Object 构造函数之上的方法，称为静态方法（static method）。

## getOwnPropertyDescriptor

Object.getOwnPropertyDescriptor() 方法返回指定对象上一个自有属性对应的属性描述符。（自有属性指的是直接赋予该对象的属性，不需要从原型链上进行查找的属性）

Object.getOwnPropertyDescriptors() 方法返回指定对象所有自有属性对应的属性描述符。

```js
var o, d;

o = { get foo() { return 17; } };
d = Object.getOwnPropertyDescriptor(o, "foo");
// d {
//   configurable: true,
//   enumerable: true,
//   get: /*the getter function*/,
//   set: undefined
// }

o = { bar: 42 };
d = Object.getOwnPropertyDescriptor(o, "bar");
// d {
//   configurable: true,
//   enumerable: true,
//   value: 42,
//   writable: true
// }

o = {};
Object.defineProperty(o, "baz", {
  value: 8675309,
  writable: false,
  enumerable: false
});
d = Object.getOwnPropertyDescriptor(o, "baz");
// d {
//   value: 8675309,
//   writable: false,
//   enumerable: false,
//   configurable: false
// }
```

## getOwnPropertyNames

Object.getOwnPropertyNames()方法返回一个由指定对象的所有自身属性的属性名（包括不可枚举属性但不包括Symbol值作为名称的属性）组成的数组。

Object.getOwnPropertySymbols() 方法返回一个给定对象自身的所有 Symbol 属性的数组。

```js
var arr = ["a", "b", "c"];
console.log(Object.getOwnPropertyNames(arr).sort()); // ["0", "1", "2", "length"]

// 类数组对象
var obj = { 0: "a", 1: "b", 2: "c"};
console.log(Object.getOwnPropertyNames(obj).sort()); // ["0", "1", "2"]

// 使用Array.forEach输出属性名和属性值
Object.getOwnPropertyNames(obj).forEach(function(val, idx, array) {
  console.log(val + " -> " + obj[val]);
});
// 输出
// 0 -> a
// 1 -> b
// 2 -> c

//不可枚举属性
var my_obj = Object.create({}, {
  getFoo: {
    value: function() { return this.foo; },
    enumerable: false
  }
});
my_obj.foo = 1;

console.log(Object.getOwnPropertyNames(my_obj).sort()); // ["foo", "getFoo"]
```

## getPrototypeOf

Object.getPrototypeOf() 方法返回指定对象的原型（内部 `[[Prototype]]` 属性的值）。

```js
/* eslint-disable no-new-object */
// JavaScript中的 Object 是构造函数（创建对象的包装器）。
// 一般用法是：
const obj = new Object();

// 所以：
Object.getPrototypeOf(Object); // ƒ () { [native code] }
Object.getPrototypeOf(Function); // ƒ () { [native code] }

Object.getPrototypeOf(Object) === Function.prototype; // true

// Object.getPrototypeOf(Object)是把Object这一构造函数看作对象，
// 返回的当然是函数对象的原型，也就是 Function.prototype。

// 正确的方法是，Object.prototype是构造出来的对象的原型。
Object.prototype === Object.getPrototypeOf(obj); // true

Object.prototype === Object.getPrototypeOf({}); // true
```

### setPrototypeOf

Object.setPrototypeOf() 方法设置一个指定的对象的原型 ( 即, 内部 `[[Prototype]]` 属性）到另一个对象或 null。

> 警告: 由于现代 JavaScript 引擎优化属性访问所带来的特性的关系，更改对象的 [[Prototype]] 在各个浏览器和 JavaScript 引擎上都是一个很慢的操作。其在更改继承的性能上的影响是微妙而又广泛的，这不仅仅限于 obj.__proto__ = ... 语句上的时间花费，而且可能会延伸到任何代码，那些可以访问任何 [[Prototype]] 已被更改的对象的代码。如果你关心性能，你应该避免设置一个对象的 [[Prototype]]。相反，你应该使用 Object.create() 来创建带有你想要的 [[Prototype]] 的新对象。

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Object

