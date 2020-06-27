# 【JavaScript】Object 构造函数和属性

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Object 构造函数为给定值创建一个对象包装器。如果给定值是 null 或 undefined，
将会创建并返回一个空对象，否则，将返回一个与给定值对应类型的对象。

当以非构造函数形式被调用时，Object 等同于 new Object()。

## 构造函数 Object()

Object() 创建一个对象包装器。

```js
const o = new Object();
o.foo = 42;

const obj1 = new Object();
const obj2 = new Object(undefined);
const obj3 = new Object(null);

console.log(Object);
console.log(Object.getPrototypeOf(Object));
console.log(Object.prototype);
console.log(Object.getPrototypeOf(Object.prototype));

console.log(Function.prototype);
console.log(Object.getPrototypeOf(Object) === Function.prototype);
```

## Object 属性

### length

Object.length 的值为 1。

```js
console.log(Object.length); // 1
```

### prototype

Object.prototype 是 Object 的原型对象，可以为所有 Object 类型的对象添加属性。

```js
console.log(Object.prototype);
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Object

