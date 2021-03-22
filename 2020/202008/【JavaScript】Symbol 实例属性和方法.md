# 【JavaScript】Symbol 实例属性和方法

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

定义在 Symbol 原型上的属性和方法。

## description

description 是一个只读属性，它会返回 Symbol 对象的可选描述的字符串。

```js
console.log(Symbol('desc').description);
// expected output: "desc"

console.log(Symbol.iterator.description);
// expected output: "Symbol.iterator"

console.log(Symbol.for('foo').description);
// expected output: "foo"

console.log(`${Symbol('foo').description}bar`);
// expected output: "foobar"
```

## toString

toString() 方法返回当前 symbol 对象的字符串表示。覆盖Object.prototype.toString() 方法。

```js
console.log(Symbol('desc').toString());
// expected output: "Symbol(desc)"

console.log(Symbol.iterator.toString());
// expected output: "Symbol(Symbol.iterator)

console.log(Symbol.for('foo').toString());
// expected output: "Symbol(foo)"

// console.log(Symbol('foo') + 'bar');
// expected output: Error: Can't convert symbol to string
```

## valueOf

valueOf() 方法返回当前 symbol 对象所包含的 symbol 原始值。覆盖 Object.prototype.valueOf() 方法。

```js
Object(Symbol("foo")) + "bar";
// TypeError: can't convert symbol object to primitive
// 无法隐式的调用 valueOf() 方法

Object(Symbol("foo")).valueOf() + "bar";
// TypeError:  can't convert symbol to string
// 手动调用 valueOf() 方法，虽然转换成了原始值，但 symbol 原始值不能转换为字符串

Object(Symbol("foo")).toString() + "bar";
// "Symbol(foo)bar"，需要手动调用 toString() 方法才行
```

## [@@toPrimitive]

[@@toPrimitive]() 方法可将 Symbol 对象转换为原始值。

```js
const sym = Symbol("example");
sym === sym[Symbol.toPrimitive](); // true
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Symbol

