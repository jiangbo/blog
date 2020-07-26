# 【JavaScript】async function

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

async function 用来定义一个返回 AsyncFunction 对象的异步函数。异步函数是指通过事件循环异步执行的函数，它会通过一个隐式的 Promise 返回其结果。如果你在代码中使用了异步函数，就会发现它的语法和结构会更像是标准的同步函数。

> 记住 await 关键字只在异步函数内有效。如果你在 async 异步函数外使用它，会抛出语法错误。

## 异步函数示例

```js
function resolveAfter2Seconds() {
  return new Promise(resolve => {
    setTimeout(() => {
      resolve('resolved');
    }, 2000);
  });
}

async function asyncCall() {
  console.log('calling');
  const result = await resolveAfter2Seconds();
  console.log(result);
  // expected output: "resolved"
}

asyncCall();
```

## 异步函数表达式

异步函数表达式与异步函数语句非常相似，语法也基本相同。它们之间的主要区别在于异步函数表达式可以省略函数名称来创建一个匿名函数。另外，异步函数表达式还可以用在 IIFE (立即执行函数表达式，Immediately Invoked Function Expression)。

```js
function resolveAfter2Seconds() {
  return new Promise(resolve => {
    setTimeout(() => {
      resolve('resolved');
    }, 2000);
  });
}

let asyncCall = async function() {
  console.log('calling');
  const result = await resolveAfter2Seconds();
  console.log(result);
  // expected output: "resolved"
}
```

## Promise 与 async

Promise

```js
function getProcessedData(url) {
  return downloadData(url) // 返回一个 promise 对象
    .catch(e => {
      return downloadFallbackData(url)  // 返回一个 promise 对象
    })
    .then(v => {
      return processDataInWorker(v); // 返回一个 promise 对象
    });
}
```

async

```js
async function getProcessedData(url) {
  let v;
  try {
    v = await downloadData(url); 
  } catch (e) {
    v = await downloadFallbackData(url);
  }
  return processDataInWorker(v);
}
```

注意，在上述示例中，return 语句中没有 await 操作符，因为 async function 的返回值将被隐式地传递给 Promise.resolve。

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/async_function
