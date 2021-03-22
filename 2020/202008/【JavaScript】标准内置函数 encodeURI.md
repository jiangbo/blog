# 【JavaScript】标准内置函数 encodeURI

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

encodeURI() 函数通过将特定字符的每个实例替换为一个、两个、三或四转义序列来对统一资源标识符 (URI) 进行编码 (该字符的 UTF-8 编码仅为四转义序列)由两个 "代理" 字符组成)。
decodeURI() 函数能解码由encodeURI 创建或其它流程得到的统一资源标识符（URI）。

## 描述

假定一个 URI 是完整的 URI，那么无需对那些保留的并且在 URI 中有特殊意思的字符进行编码。

`http://username:password@www.example.com:80/path/to/file.php?foo=316&bar=this+has+spaces#anchor`

encodeURI 自身无法产生能适用于 HTTP GET 或 POST 请求的 URI，例如对于 XMLHTTPRequests, 因为 "&", "+", 和 "=" 不会被编码，然而在 GET 和 POST 请求中它们是特殊字符。

## 示例

```js
const uri = "https://mozilla.org/?x=шеллы";
const encoded = encodeURI(uri);
console.log(encoded);
// expected output: "https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"

try {
  console.log(decodeURI(encoded));
  // expected output: "https://mozilla.org/?x=шеллы"
} catch (e) { // catches a malformed URI
  console.error(e);
}

```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/encodeURI

