# 【JavaScript】debugger

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

debugger 语句调用任何可用的调试功能，例如设置断点。 如果没有调试功能可用，则此语句不起作用。

## 示例

下面的例子演示了一个包含 debugger 语句的函数，当函数被调用时，会尝试调用一个可用的调试器进行调试。
当 debugger 被调用时, 执行暂停在 debugger 语句的位置。就像在脚本源代码中的断点一样。

```js
function potentiallyBuggyCode() {
    debugger;
    // do potentially buggy stuff to examine, step through, etc.
}
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/debugger
