# 【JavaScript】标准内置变量 globalThis

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

在以前，从不同的 JavaScript 环境中获取全局对象需要不同的语句。
在 Web 中，可以通过 window、self 或者 frames 取到全局对象，但是在 Web Workers 中，只有 self 可以。
在 Node.js 中，它们都无法获取，必须使用 global。

在松散模式下，可以在函数中返回 this 来获取全局对象，但是在严格模式和模块环境下，this 会返回 undefined。
globalThis 提供了一个标准的方式来获取不同环境下的全局 this 对象（也就是全局对象自身）。不
像 window 或者 self 这些属性，它确保可以在有无窗口的各种环境下正常工作。
所以，你可以安心的使用 globalThis，不必担心它的运行环境。
为便于记忆，你只需要记住，全局作用域中的 this 就是 globalThis。

## 示例

在 globalThis 之前，获取某个全局对象的唯一方式就是 Function('return this')()，
但是这在某些情况下会违反 CSP 规则，所以，es6-shim 使用了类似如下的方式：

```js
var getGlobal = function () { 
  if (typeof self !== 'undefined') { return self; } 
  if (typeof window !== 'undefined') { return window; } 
  if (typeof global !== 'undefined') { return global; } 
  throw new Error('unable to locate global object'); 
}; 

var globals = getGlobal(); 

if (typeof globals.setTimeout !== 'function') { 
  // 此环境中没有 setTimeout 方法！
}
```

但是有了 globalThis 之后，只需要：

```js
if (typeof globalThis.setTimeout !== 'function') {
  //  此环境中没有 setTimeout 方法！
}
```

## 练习

```js
function canMakeHTTPRequest() {
  return typeof globalThis.XMLHttpRequest === 'function';
}

console.log(canMakeHTTPRequest());
// expected output (in a browser): true
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/globalThis

