# 【JavaScript】Map

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Map 对象保存键值对，并且能够记住键的原始插入顺序。任何值(对象或者原始值) 都可以作为一个键或一个值。

## 构造函数

Map() 构造函数创建 Map 对象。

```js
let myMap = new Map([
  [1, 'one'],
  [2, 'two'],
  [3, 'three'],
])
```

## 实例属性 size

size 是可访问属性，用于返回一个 Map 对象的成员数量。

```js
const map1 = new Map();

map1.set('a', 'alpha');
map1.set('b', 'beta');
map1.set('g', 'gamma');

console.log(map1.size);
// expected output: 3
```

## 实例方法

### clear

clear() 方法会移除 Map 对象中的所有元素。

```js
const map1 = new Map();

map1.set('bar', 'baz');
map1.set(1, 'foo');

console.log(map1.size);
// expected output: 2

map1.clear();

console.log(map1.size);
// expected output: 0
```

### delete

delete() 方法用于移除 Map 对象中指定的元素。

```js
const map1 = new Map();
map1.set('bar', 'foo');

console.log(map1.delete('bar'));
// expected result: true
// (true indicates successful removal)

console.log(map1.has('bar'));
// expected result: false
```

### entries

entries() 方法返回一个新的包含 [key, value] 对的 Iterator 对象，返回的迭代器的迭代顺序与 Map 对象的插入顺序相同。

```js
const map1 = new Map();

map1.set('0', 'foo');
map1.set(1, 'bar');

const iterator1 = map1.entries();

console.log(iterator1.next().value);
// expected output: ["0", "foo"]

console.log(iterator1.next().value);
// expected output: [1, "bar"]
```

### forEach

forEach() 方法将会以插入顺序对 Map 对象中的每一个键值对执行一次参数中提供的回调函数。

```js
function logMapElements(value, key, map) {
  console.log(`m[${key}] = ${value}`);
}

new Map([['foo', 3], ['bar', {}], ['baz', undefined]])
  .forEach(logMapElements);

// expected output: "m[foo] = 3"
// expected output: "m[bar] = [object Object]"
// expected output: "m[baz] = undefined"
```

### get

get() 方法返回某个 Map 对象中的一个指定元素。

```js
const map1 = new Map();
map1.set('bar', 'foo');

console.log(map1.get('bar'));
// expected output: "foo"

console.log(map1.get('baz'));
// expected output: undefined
```

### has

方法 has() 返回一个 boolean 值，用来表明 map 中是否存在指定元素.

```js
const map1 = new Map();
map1.set('bar', 'foo');

console.log(map1.has('bar'));
// expected output: true

console.log(map1.has('baz'));
// expected output: false
```

### keys

keys() 返回一个引用的 Iterator 对象。它包含按照顺序插入 Map 对象中每个元素的key值。

```js
const map1 = new Map();

map1.set('0', 'foo');
map1.set(1, 'bar');

const iterator1 = map1.keys();

console.log(iterator1.next().value);
// expected output: "0"

console.log(iterator1.next().value);
// expected output: 1
```

### set

set() 方法为 Map 对象添加或更新一个指定了键（key）和值（value）的（新）键值对。

```js
const map1 = new Map();
map1.set('bar', 'foo');

console.log(map1.get('bar'));
// expected output: "foo"

console.log(map1.get('baz'));
// expected output: undefined
```

### values

values() 方法返回一个新的 Iterator 对象。它包含按顺序插入 Map 对象中每个元素的 value 值。

```js
const map1 = new Map();

map1.set('0', 'foo');
map1.set(1, 'bar');

const iterator1 = map1.values();

console.log(iterator1.next().value);
// expected output: "foo"

console.log(iterator1.next().value);
// expected output: "bar"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Map
