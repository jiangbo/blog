# 【JavaScript】WeakMap

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

WeakSet 对象允许你将弱保持对象存储在一个集合中。

## 构造函数

WeakMap() 构造函数创建 WeakMap 对象，这些对象是键/值对的集合，在其中键被弱引用。键必须是对象，并且值可以是任意值。

```js
const wm1 = new WeakMap(),
      wm2 = new WeakMap(),
      wm3 = new WeakMap();
const o1 = {},
      o2 = function() {},
      o3 = window;

wm1.set(o1, 37);
wm1.set(o2, 'azerty');
wm2.set(o1, o2); // a value can be anything, including an object or a function
wm2.set(o3, undefined);
wm2.set(wm1, wm2); // keys and values can be any objects. Even WeakMaps!

wm1.get(o2); // "azerty"
wm2.get(o2); // undefined, because there is no key for o2 on wm2
wm2.get(o3); // undefined, because that is the set value

wm1.has(o2); // true
wm2.has(o2); // false
wm2.has(o3); // true (even if the value itself is 'undefined')

wm3.set(o1, 37);
wm3.get(o1); // 37

wm1.has(o1); // true
wm1.delete(o1);
wm1.has(o1); // false
```

## 实例方法

### delete

delete() 方法可以从一个 WeakMap 对象中删除指定的元素。

```js
const weakmap1 = new WeakMap();
const object1 = {};

weakmap1.set(object1, 42);

console.log(weakmap1.delete(object1));
// expected output: true

console.log(weakmap1.has(object1));
// expected output: false
```

### get

get() 方法返回 WeakMap 指定的元素。

```js
const weakmap1 = new WeakMap();
const object1 = {};
const object2 = {};

weakmap1.set(object1, 42);

console.log(weakmap1.get(object1));
// expected output: 42

console.log(weakmap1.get(object2));
// expected output: undefined
```

### has

has() 方法根据 WeakMap 对象的元素中是否存在 key 键返回一个 boolean 值。

```js
const weakmap1 = new WeakMap();
const object1 = {};
const object2 = {};

weakmap1.set(object1, 'foo');

console.log(weakmap1.has(object1));
// expected output: true

console.log(weakmap1.has(object2));
// expected output: false
```

### set

set() 方法根据指定的 key 和 value 在 WeakMap 对象中添加新/更新元素。

```js
const weakmap1 = new WeakMap();
const object1 = {};
const object2 = {};

weakmap1.set(object1, 'foo');
weakmap1.set(object2, 'bar');

console.log(weakmap1.get(object1));
//expected output: "foo"

console.log(weakmap1.get(object2));
//expected output: "bar"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/WeakSet
