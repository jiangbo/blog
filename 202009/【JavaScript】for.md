# 【JavaScript】for

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

for 语句用于创建一个循环，它包含了三个可选的表达式，这三个表达式被包围在圆括号之中，使用分号分隔，后跟一个用于在循环中执行的语句（通常是一个块语句）。

## 示例

```js
let str = '';

for (let i = 0; i < 9; i++) {
  str = str + i;
}

console.log(str);
// expected output: "012345678"
```

## 可选的 for 表达式

for 语句头部圆括号中的所有三个表达式都是可选的。

例如，初始化块中的表达式没有被指定：

```js
var i = 0;
for (; i < 9; i++) {
    console.log(i);
    // more statements
}
```

像初始化块一样，条件块也是可选的。如果省略此表达式，则必须确保在循环体内跳出，以防创建死循环。

```js
for (var i = 0;; i++) {
   console.log(i);
   if (i > 3) break;
   // more statements
}
```

当然可以忽略所有的表达式。同样的，确保使用了 break 语句来跳出循环并且还要修改（增加）一个变量，使得 break 语句的条件在某个时候是真的。

```js
var i = 0;

for (;;) {
  if (i > 3) break;
  console.log(i);
  i++;
}
```


[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/for
