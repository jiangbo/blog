# 【JavaScript】标准内置变量 undefined

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

全局属性 undefined 表示原始值 undefined。它是一个 JavaScript 的原始数据类型。

在现代浏览器（JavaScript 1.8.5/Firefox 4+），自ECMAscript5标准以来 undefined 是一个不能被配置（non-configurable），不能被重写（non-writable）的属性。即便事实并非如此，也要避免去重写它。

一个没有被赋值的变量的类型是 undefined。如果方法或者是语句中操作的变量没有被赋值，则会返回 undefined。

```js
function test(a){
    console.log(typeof a);    // undefined
    return a;
}

test();                       // 返回"undefined"
```

一个函数如果没有使用 return 语句指定返回值，就会返回一个 undefined 值。

## 示例

### 严格相等和 undefined

你可以使用 undefined 和严格相等或不相等操作符来决定一个变量是否拥有值。
在下面的代码中，变量 x 是未定义的，if 语句的求值结果将是 true

```js
var x;

if (x === undefined) {
// 执行这些语句
} else {
// 这些语句不会被执行
}
```

### Typeof 操作符和 undefined

```js
var x;
if(typeof x === 'undefined') {
    // 执行这些语句
}
```

使用 typeof的原因是它不会在一个变量没有被声明的时候抛出一个错误。

```js
// 这里没有声明y
if(typeof y === 'undefined') {       // 没有错误，执行结果为true
   console.log("y is " + typeof y )  // y is undefined
}

if(y === undefined) {                // ReferenceError: y is not defined

}
```

### Void 操作符和 undefined

```js
var x;
if(x === void 0) {
    // 执行这些语句
}

// 没有声明y
if(y === void 0) {
    // 抛出一个RenferenceError错误(与`typeof`相比)
}
```

## 练习

```js
function test(t) {
  if (t === undefined) {
    return 'Undefined value!';
  }
  return t;
}

let x;

console.log(test(x));
// expected output: "Undefined value!"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/undefined

