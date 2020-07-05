# 【JavaScript】Set

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Set 对象是值的集合，你可以按照插入的顺序迭代它的元素。Set 中的元素只会出现一次，即 Set 中的元素是唯一的。

## 构造函数

Set 构造函数使您可以创建 Set 对象，该对象存储任何类型的唯一值，无论是原始值还是对象引用。

```js
const set1 = new Set([1, 2, 3, 4, 5]);

console.log(set1.has(1));
// expected output: true

console.log(set1.has(5));
// expected output: true

console.log(set1.has(6));
// expected output: false
```

## 实例属性 size

size 属性将会返回 Set 对象中元素的个数。

```js
const set1 = new Set();
const object1 = {};

set1.add(42);
set1.add('forty two');
set1.add('forty two');
set1.add(object1);

console.log(set1.size);
// expected output: 3
```

## 实例方法

### add

add() 方法用来向一个 Set 对象的末尾添加一个指定的值。

```js
const set1 = new Set();

set1.add(42);
set1.add(42);
set1.add(13);

for (let item of set1) {
  console.log(item);
  // expected output: 42
  // expected output: 13
}
```

### clear

clear() 方法用来清空一个 Set 对象中的所有元素。

```js
const set1 = new Set();
set1.add(1);
set1.add('foo');

console.log(set1.size);
// expected output: 2

set1.clear();

console.log(set1.size);
// expected output: 0
```

### delete

delete() 方法可以从一个 Set 对象中删除指定的元素。

```js
const set1 = new Set();
set1.add({ x: 10, y: 20 }).add({ x: 20, y: 30 });

// Delete any point with `x > 10`.
set1.forEach((point) => {
  if (point.x > 10) {
    set1.delete(point);
  }
});

console.log(set1.size);
// expected output: 1
```

### entries

entries() 方法返回一个新的迭代器对象，这个对象的元素是类似 [value, value] 形式的数组，value 是集合对象中的每个元素，迭代器对象元素的顺序即集合对象中元素插入的顺序。由于集合对象不像 Map 对象那样拥有 key，然而，为了与 Map 对象的 API 形式保持一致，故使得每一个 entry 的 key 和 value 都拥有相同的值，因而最终返回一个 [value, value] 形式的数组。

```js
const set1 = new Set();
set1.add(42);
set1.add('forty two');

const iterator1 = set1.entries();

for (let entry of iterator1) {
  console.log(entry);
  // expected output: [42, 42]
  // expected output: ["forty two", "forty two"]
}
```

### forEach

forEach 方法会根据集合中元素的插入顺序，依次执行提供的回调函数。

```js
function logSetElements(value1, value2, set) {
  console.log(`s[${value1}] = ${value2}`);
}

new Set(['foo', 'bar', undefined]).forEach(logSetElements);

// expected output: "s[foo] = foo"
// expected output: "s[bar] = bar"
// expected output: "s[undefined] = undefined"
```

### has

has() 方法返回一个布尔值来指示对应的值 value 是否存在 Set 对象中。

```js
const set1 = new Set([1, 2, 3, 4, 5]);

console.log(set1.has(1));
// expected output: true

console.log(set1.has(5));
// expected output: true

console.log(set1.has(6));
// expected output: false
```

### keys

keys() 方法返回一个新的 Iterator 对象，该对象按插入顺序包含 Set 对象中每个元素的值。

```js
const set1 = new Set();
set1.add(42);
set1.add('forty two');

const iterator1 = set1.values();

console.log(iterator1.next().value);
// expected output: 42

console.log(iterator1.next().value);
// expected output: "forty two"
```

### values

values() 方法返回一个 Iterator 对象，该对象按照原 Set 对象元素的插入顺序返回其所有元素。

```js
const set1 = new Set();
set1.add(42);
set1.add('forty two');

const iterator1 = set1.values();

console.log(iterator1.next().value);
// expected output: 42

console.log(iterator1.next().value);
// expected output: "forty two"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Set
