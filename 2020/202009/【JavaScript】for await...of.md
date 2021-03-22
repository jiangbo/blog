# 【JavaScript】for await...of

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

for await...of 语句会在异步或者同步可迭代对象上创建一个迭代循环，包括 String，Array，Array-like 对象（比如arguments 或者NodeList)，TypedArray，Map， Set和自定义的异步或者同步可迭代对象。其会调用自定义迭代钩子，并为每个不同属性的值执行语句。这个语句只能在 async function 内使用。

## 示例

```js
const asyncIterable = {
  [Symbol.asyncIterator]() {
    return {
      i: 0,
      next() {
        if (this.i < 3) {
          return Promise.resolve({ value: this.i++, done: false });
        }

        return Promise.resolve({ done: true });
      }
    };
  }
};

(async function() {
   for await (let num of asyncIterable) {
     console.log(num);
   }
})();

// 0
// 1
// 2
```

## 迭代异步生成器

```js
async function* asyncGenerator() {
  var i = 0;
  while (i < 3) {
    yield i++;
  }
}

(async function() {
  for await (num of asyncGenerator()) {
    console.log(num);
  }
})();
// 0
// 1
// 2
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/for-await...of
