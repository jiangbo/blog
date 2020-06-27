# 【JavaScript】String 构造函数和静态方法

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

String 全局对象是一个用于字符串或一个字符序列的构造函数。
使用 “\” 可以转义字符。

```js
const string1 = "A string primitive";
const string2 = 'Also a string primitive';
const string3 = `Yet another string primitive`; // 模板字符串
const string4 = new String("A String object");
```

从 ECMAScript 2015 开始，字符串字面量也可以称为模板字面量：

```js
`hello world` `hello! world!` `hello ${who}` escape `<a>${who}</a>`
```

## 构造函数

String() 创建一个新的 String 对象。 当作为函数而不是构造函数调用时，它执行类型转换，这通常更有用。

```js
typeof String('Hello world'); // string
typeof new String('Hello world'); // object
```

## 获取字符

有两种方式可以获取到字符串中的字符。

```js
return 'cat'.charAt(1) // returns "a"
return 'cat'[1] // returns "a"
```

## 比较字符串

```js
let a = 'a'
let b = 'b'
if (a < b) { // true
  console.log(a + ' is less than ' + b)
} else if (a > b) {
  console.log(a + ' is greater than ' + b)
} else {
  console.log(a + ' and ' + b + ' are equal.')
}
```

## String 基础类型和对象

```js
let s_prim = 'foo'
let s_obj = new String(s_prim)

console.log(typeof s_prim) // Logs "string"
console.log(typeof s_obj)  // Logs "object"
```

## 长字符串

有两种方式定义长字符串。

```js
let longString = "This is a very long string which needs " +
                 "to wrap across multiple lines because " +
                 "otherwise my code is unreadable."

let longString = "This is a very long string which needs \
to wrap across multiple lines because \
otherwise my code is unreadable."
```

## 静态方法

### fromCharCode

String.fromCharCode() 方法返回由指定的 UTF-16 代码单元序列创建的字符串。

```js
console.log(String.fromCharCode(189, 43, 190, 61));
// expected output: "½+¾="

```

### fromCodePoint

String.fromCodePoint() 静态方法返回使用指定的代码点序列创建的字符串。

```js
console.log(String.fromCodePoint(9731, 9733, 9842, 0x2F804));
// expected output: "☃★♲你"
```

### raw

String.raw() 是一个模板字符串的标签函数，是用来获取一个模板字符串的原始字符串的，比如说，占位符（例如 ${foo}）会被处理为它所代表的其他字符串，而转义字符（例如 \n）不会。

```js

// Create a variable that uses a Windows
// path without escaping the backslashes:
const filePath = String.raw`C:\Development\profile\aboutme.html`;

console.log(`The file was uploaded from: ${filePath}`);
// expected output: "The file was uploaded from: C:\Development\profile\aboutme.html"
```

## 实例属性

除了 prototype，字符串有一个实例属性 length，它代表了字符串的长度。

```js
var x = "Mozilla";
var empty = "";

console.log("Mozilla is " + x.length + " code units long");
/* "Mozilla is 7 code units long" */

console.log("The empty string is has a length of " + empty.length);
/* "The empty string is has a length of 0" */
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/String
