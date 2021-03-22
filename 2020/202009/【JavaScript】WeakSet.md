# 【JavaScript】WeakSet

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

WeakSet 对象允许你将弱保持对象存储在一个集合中。

## 构造函数

使用 WeakSet 构造函数，您可以创建 WeakSet 对象，这些对象将弱保存的对象存储在集合中。

```js
var ws = new WeakSet();
var foo = {};
var bar = {};

ws.add(foo);
ws.add(bar);

ws.has(foo);    // true
ws.has(bar);    // true

ws.delete(foo); // removes foo from the set
ws.has(foo);    // false, foo has been removed
ws.has(bar);    // true, bar is retained
```

## 实例方法

### add

add() 方法在 WeakSet 对象的最后一个元素后添加新的对象。

```js
const weakset1 = new WeakSet();
const object1 = {};

weakset1.add(object1);
console.log(weakset1.has(object1));
// expected output: true

try {
  weakset1.add(1);
} catch (error) {
  console.log(error);
  // expected output: "Error: Invalid value used in weak set" in Chrome
  // expected output: "TypeError: WeakSet value must be an object, got the number 1" in Firefox
}
```

### delete

delete() 方法从 WeakSet 对象中移除指定的元素。

```js
const weakset1 = new WeakSet();
const object1 = {};

weakset1.add(object1);

console.log(weakset1.has(object1));
// expected output: true

weakset1.delete(object1);

console.log(weakset1.has(object1));
// expected output: false
```

### has

has() 方法根据 WeakSet 是否存在相应对象返回布尔值。

```js
const weakset1 = new WeakSet();
const object1 = {};
const object2 = {};

weakset1.add(object1);

console.log(weakset1.has(object1));
// expected output: true

console.log(weakset1.has(object2));
// expected output: false
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/WeakSet
