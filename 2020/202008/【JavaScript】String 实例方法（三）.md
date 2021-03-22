# 【JavaScript】String 实例方法（三）

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## toLowerCase

toLowerCase() 会将调用该方法的字符串值转为小写形式，并返回，基本上和 toLocaleLowerCase() 一致。

```js
const sentence = 'The quick brown fox jumps over the lazy dog.';

console.log(sentence.toLowerCase());
// expected output: "the quick brown fox jumps over the lazy dog."
```

## toUpperCase

toUpperCase() 方法将调用该方法的字符串转为大写形式并返回），基本上和 toLocaleUpperCase() 一致。

```js
const sentence = 'The quick brown fox jumps over the lazy dog.';

console.log(sentence.toUpperCase());
// expected output: "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG."
```

## toString

toString() 方法返回指定对象的字符串形式，覆盖了 Object.prototype.toString() 的方法。

```js
const stringObj = new String('foo');

console.log(stringObj);
// expected output: String { "foo" }

console.log(stringObj.toString());
// expected output: "foo"
```

## trim

trim() 方法会从一个字符串的两端删除空白字符。在这个上下文中的空白字符是所有的空白字符 (space, tab, no-break space 等) 以及所有行终止符字符（如 LF，CR 等）。

```js
const greeting = '   Hello world!   ';

console.log(greeting);
// expected output: "   Hello world!   ";

console.log(greeting.trim());
// expected output: "Hello world!";
```

## trimStart

trimStart() 方法从字符串的开头删除空格。

```js
const greeting = '   Hello world!   ';

console.log(greeting);
// expected output: "   Hello world!   ";

console.log(greeting.trimStart());
// expected output: "Hello world!   ";
```

## trimEnd

trimEnd() 方法从一个字符串的末端移除空白字符

```js
const greeting = '   Hello world!   ';

console.log(greeting);
// expected output: "   Hello world!   ";

console.log(greeting.trimEnd());
// expected output: "   Hello world!";
```

## valueOf

valueOf() 返回基础类型。

```js
const stringObj = new String('foo');

console.log(stringObj);
// expected output: String { "foo" }

console.log(stringObj.valueOf());
// expected output: "foo"
```



[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/String
