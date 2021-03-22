# 【JavaScript】Generator

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

生成器对象是由一个 generator function 返回的，并且它符合可迭代协议和迭代器协议。

## 语法

```js
function* gen() { 
  yield 1;
  yield 2;
  yield 3;
}

let g = gen(); 
// "Generator { }"
```

## 方法

### next

next() 方法返回一个包含属性 done 和 value 的对象。该方法也可以通过接受一个参数用以向生成器传值。

```js
function* gen() { 
  yield 1;
  yield 2;
  yield 3;
}

const g = gen(); // "Generator { }"
g.next();      // "Object { value: 1, done: false }"
g.next();      // "Object { value: 2, done: false }"
g.next();      // "Object { value: 3, done: false }"
g.next();      // "Object { value: undefined, done: true }"
```

### return

return() 方法返回给定的值并结束生成器。

```js
function* gen() { 
  yield 1;
  yield 2;
  yield 3;
}

var g = gen();

g.next();        // { value: 1, done: false }
g.return("foo"); // { value: "foo", done: true }
g.next();        // { value: undefined, done: true }
```

### throw

throw() 方法用来向生成器抛出异常，并恢复生成器的执行，返回带有 done 及 value 两个属性的对象。

```js
function* gen() {
  while(true) {
    try {
       yield 42;
    } catch(e) {
      console.log("Error caught!");
    }
  }
}

var g = gen();
g.next(); // { value: 42, done: false }
g.throw(new Error("Something went wrong")); // "Error caught!"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Generator
