# 【JavaScript】Symbol 静态方法

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

定义在 Symbol 上的方法。

## for

Symbol.for(key) 方法会根据给定的键 key，来从运行时的 symbol 注册表中找到对应的 symbol，如果找到了，则返回它，否则，新建一个与该键关联的 symbol，并放入全局 symbol 注册表中。

```js
console.log(Symbol.for('bar') === Symbol.for('bar'));
// expected output: true

console.log(Symbol('bar') === Symbol('bar'));
// expected output: false

const symbol1 = Symbol.for('foo');

console.log(symbol1.toString());
// expected output: "Symbol(foo)"
```

## keyFor

Symbol.keyFor(sym) 方法用来获取 symbol 注册表中与某个 symbol 关联的键。

```js
const globalSym = Symbol.for('foo'); // global symbol

console.log(Symbol.keyFor(globalSym));
// expected output: "foo"

const localSym = Symbol(); // local symbol

console.log(Symbol.keyFor(localSym));
// expected output: undefined

console.log(Symbol.keyFor(Symbol.iterator));
// expected output: undefined
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Symbol

