# 【JavaScript】throw 和 try...catch

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

throw 语句用来抛出一个用户自定义的异常。当前函数的执行将被停止（throw 之后的语句将不会执行），并且控制将被传递到调用堆栈中的第一个 catch 块。如果调用者函数中没有 catch 块，程序将会终止。

## throw

和 Java 语言不同，throw 可以抛出各种对象，而不仅仅是异常对象。

```js
throw "Error2"; // 抛出了一个值为字符串的异常
throw 42;       // 抛出了一个值为整数42的异常
throw true;     // 抛出了一个值为true的异常
```

## catch

catch 可以捕获 throw 抛出来的对象。

```js
try {
    throw "myException"; // generates an exception
}
catch (e) {
    // statements to handle any exceptions
    console.log(e);
}
```

## finally

finally 块包含的语句在 try 块和 catch 之后，try..catch..finally 块后的语句之前执行。请注意，无论是否抛出异常 finally 子句都会执行。此外，如果抛出异常，即使没有 catch 子句处理异常，finally 子句中的语句也会执行。

```js
openMyFile()
try {
   // tie up a resource
   writeMyFile(theData);
}
finally {
   closeMyFile(); // always close the resource
}
```

如果从 finally 块中返回一个值，那么这个值将会成为整个 try-catch-finally 的返回值，无论是否有 return 语句在 try 和 catch 中。这包括在 catch 块里抛出的异常。

```js
try {
  try {
    throw new Error("oops");
  }
  catch (ex) {
    console.error("inner", ex.message);
    throw ex;
  }
  finally {
    console.log("finally");
    return;
  }
}
catch (ex) {
  console.error("outer", ex.message);
}

// 注: 此 try catch 语句需要在 function 中运行才能作为函数的返回值, 否则直接运行会报语法错误
// Output:
// "inner" "oops"
// "finally"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/throw
