# 【JavaScript】Array 静态方法和实例属性

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

JavaScript 的 Array 对象是用于构造数组的全局对象，数组是类似于列表的高阶对象。

## 构造函数

使用 Array() 可以创建一个数组。

```js
var fruits = ['Apple', 'Banana'];

console.log(fruits.length);
// 2
```

## 访问数组元素

JavaScript 数组的索引是从 0 开始的，第一个元素的索引为 0，最后一个元素的索引等于该数组的长度减 1。如果指定的索引是一个无效值，JavaScript 数组并不会报错，而是会返回 undefined。

```js
var arr = ['this is the first element', 'this is the second element', 'this is the last element'];
console.log(arr[0]);              // 打印 'this is the first element'
console.log(arr[1]);              // 打印 'this is the second element'
console.log(arr[arr.length - 1]); // 打印 'this is the last element'
```

## 静态方法

### from

Array.from() 方法从一个类似数组或可迭代对象创建一个新的，浅拷贝的数组实例。

```js
console.log(Array.from('foo'));
// expected output: Array ["f", "o", "o"]

console.log(Array.from([1, 2, 3], x => x + x));
// expected output: Array [2, 4, 6]
```

### isArray

Array.isArray() 用于确定传递的值是否是一个 Array。

```js
Array.isArray([1, 2, 3]);  
// true
Array.isArray({foo: 123}); 
// false
Array.isArray("foobar");   
// false
Array.isArray(undefined);  
// false
```

## of

Array.of() 方法创建一个具有可变数量参数的新数组实例，而不考虑参数的数量或类型。

```js
Array.of(7);       // [7] 
Array.of(1, 2, 3); // [1, 2, 3]

Array(7);          // [ , , , , , , ]
Array(1, 2, 3);    // [1, 2, 3]
```

## 实例属性 length

length 表示了数组中元素的个数。

```js
const clothing = ['shoes', 'shirts', 'socks', 'sweaters'];

console.log(clothing.length);
// expected output: 4
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Array
