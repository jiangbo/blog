# 【JavaScript】标准内置函数 encodeURIComponent

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

encodeURIComponent() 是对统一资源标识符（URI）的组成部分进行编码的方法。它使用一到四个转义序列来表示字符串中的每个字符的 UTF-8 编码（只有由两个 Unicode 代理区字符组成的字符才用四个转义字符编码）。
ecodeURIComponent() 方法用于解码由 encodeURIComponent 方法或者其它类似方法编码的部分统一资源标识符（URI）。

## 描述

encodeURIComponent 转义除了以下字符外的所有字符：
不转义的字符：
  `A-Z a-z 0-9 - _ . ! ~ * ' ( )`

encodeURIComponent() 和 encodeURI 有以下几个不同点：

```js
var set1 = ";,/?:@&=+$";  // 保留字符
var set2 = "-_.!~*'()";   // 不转义字符
var set3 = "#";           // 数字标志
var set4 = "ABC abc 123"; // 字母数字字符和空格

console.log(encodeURI(set1)); // ;,/?:@&=+$
console.log(encodeURI(set2)); // -_.!~*'()
console.log(encodeURI(set3)); // #
console.log(encodeURI(set4)); // ABC%20abc%20123 (the space gets encoded as %20)

console.log(encodeURIComponent(set1)); // %3B%2C%2F%3F%3A%40%26%3D%2B%24
console.log(encodeURIComponent(set2)); // -_.!~*'()
console.log(encodeURIComponent(set3)); // %23
console.log(encodeURIComponent(set4)); // ABC%20abc%20123 (the space gets encoded as %20)
```

## 示例

下面这个例子提供了 UTF-8 下 Content-Disposition 和 Link 的服务器响应头信息的参数：

```js
var fileName = 'my file(2).txt';
var header = "Content-Disposition: attachment; filename*=UTF-8''" 
             + encodeRFC5987ValueChars(fileName);

console.log(header); 
// 输出 "Content-Disposition: attachment; filename*=UTF-8''my%20file%282%29.txt"


function encodeRFC5987ValueChars (str) {
    return encodeURIComponent(str).
        // 注意，仅管 RFC3986 保留 "!"，但 RFC5987 并没有
        // 所以我们并不需要过滤它
        replace(/['()]/g, escape). // i.e., %27 %28 %29
        replace(/\*/g, '%2A').
            // 下面的并不是 RFC5987 中 URI 编码必须的
            // 所以对于 |`^ 这3个字符我们可以稍稍提高一点可读性
            replace(/%(?:7C|60|5E)/g, unescape);
}
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent

