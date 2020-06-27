# 【JavaScript】Function

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Function 构造函数创建一个新的 Function 对象。直接调用此构造函数可用动态创建函数，但会遇到和 eval 类似的的安全问题和(相对较小的)性能问题。然而，与 eval 不同的是，Function 创建的函数只能在全局作用域中运行。

## 静态属性

其中 arguments、caller 和 displayName 已过时，这里不介绍。

### length

Function.length 返回 1。

```js
console.log(Function.length);
```

### name

Function.name 返回 Function。

```js
console.log(Function.name);
```

## 实例方法

### apply

apply() 方法调用一个具有给定 this 值的函数，以及作为一个数组（或类似数组对象）提供的参数。

```js
const numbers = [5, 6, 2, 3, 7];

const max = Math.max.apply(null, numbers);

console.log(max);
// expected output: 7

const min = Math.min.apply(null, numbers);

console.log(min);
// expected output: 2
```

### bind

bind() 方法创建一个新的函数，在 bind() 被调用时，这个新函数的 this 被指定为 bind() 的第一个参数，而其余参数将作为新函数的参数，供调用时使用。不会立即调用函数。

```js
const module = {
  x: 42,
  getX: function() {
    return this.x;
  }
};

const unboundGetX = module.getX;
console.log(unboundGetX()); // The function gets invoked at the global scope
// expected output: undefined

const boundGetX = unboundGetX.bind(module);
console.log(boundGetX());
// expected output: 42
```

### call

call() 方法使用一个指定的 this 值和单独给出的一个或多个参数来调用一个函数。

> 该方法的语法和作用与 apply() 方法类似，只有一个区别，就是 call() 方法接受的是一个参数列表，而 apply() 方法接受的是一个包含多个参数的数组。

```js
function Product(name, price) {
  this.name = name;
  this.price = price;
}

function Food(name, price) {
  Product.call(this, name, price);
  this.category = 'food';
}

console.log(new Food('cheese', 5).name);
// expected output: "cheese"
```

### toString

toString() 方法返回一个表示当前函数源代码的字符串，覆盖了 Object.prototype.toString 方法。


```js
function sum(a, b) {
  return a + b;
}

console.log(sum.toString());
// expected output: "function sum(a, b) {
//                     return a + b;
//                   }"

console.log(Math.abs.toString());
// expected output: "function abs() { [native code] }"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Function

