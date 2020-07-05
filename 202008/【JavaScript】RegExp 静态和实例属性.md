# 【JavaScript】RegExp 静态和实例属性

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

RegExp 对象用于将文本与一个模式匹配。

## 构造函数

有两种方法可以创建一个 RegExp 对象：一种是字面量，另一种是构造函数。

字面量
* 由斜杠包围而不是引号包围。

构造函数的字符串参数
* 由引号而不是斜杠包围。

以下三种表达式都会创建相同的正则表达式：

```js
/ab+c/i;
new RegExp('ab+c', 'i');
new RegExp(/ab+c/, 'i');
```
当表达式被赋值时，字面量形式提供正则表达式的编译（compilation）状态，当正则表达式保持为常量时使用字面量。例如当你在循环中使用字面量构造一个正则表达式时，正则表达式不会在每一次迭代中都被重新编译（recompiled）。

而正则表达式对象的构造函数，如 new RegExp('ab+c') 提供了正则表达式运行时编译（runtime compilation）。如果你知道正则表达式模式将会改变，或者你事先不知道什么模式，而是从另一个来源获取，如用户输入，这些情况都可以使用构造函数。

```js
const regex1 = /\w+/;
const regex2 = new RegExp('\\w+');

console.log(regex1);
// expected output: /\w+/

console.log(regex2);
// expected output: /\w+/

console.log(regex1 === regex2);
// expected output: false
```

## 静态属性

### RegExp[@@species]

RegExp[@@species] 访问器属性返回 RegExp 的构造器。

```js
class MyRegExp extends RegExp {
  // Overwrite MyRegExp species to the parent RegExp constructor
  static get [Symbol.species]() {
    return RegExp;
  }
}

const regex1 = new MyRegExp('foo', 'g');

console.log(regex1.test('football'));
// expected output: true
```

### lastIndex

lastIndex 是正则表达式的一个可读可写的整型属性，用来指定下一次匹配的起始索引。

```js
const regex1 = new RegExp( 'foo', 'g' );
const str1 = 'table football, foosball';

regex1.test(str1);

console.log(regex1.lastIndex);
// expected output: 9

regex1.test(str1);

console.log(regex1.lastIndex);
// expected output: 19
```

## 实例属性

### flags

flags 属性返回一个字符串，由当前正则表达式对象的标志组成。

```js
// outputs RegExp flags in alphabetical order

console.log(/foo/ig.flags);
// expected output: "gi"

console.log(/bar/myu.flags);
// expected output: "muy"
```

### dotAll

dotAll 属性表明是否在正则表达式中一起使用"s"修饰符（引入/s修饰符，使得.可以匹配任意单个字符）。dotAll 是一个只读的属性，属于单个正则表达式实例。

```js
const regex1 = new RegExp('foo', 's');

console.log(regex1.dotAll);
// expected output: true

const regex2 = new RegExp('bar');

console.log(regex2.dotAll);
// expected output: false
```

### global

global 属性表明正则表达式是否使用了 "g" 标志。global 是一个正则表达式实例的只读属性。

```js
const regex1 = new RegExp('foo', 'g');

console.log(regex1.global);
// expected output: true

const regex2 = new RegExp('bar', 'i');

console.log(regex2.global);
// expected output: false
```

### ignoreCase

ignoreCase 属性表明正则表达式是否使用了 "i" 标志。ignoreCase 是正则表达式实例的只读属性。

```js
const regex1 = new RegExp('foo');
const regex2 = new RegExp('foo', 'i');

console.log(regex1.test('Football'));
// expected output: false

console.log(regex2.ignoreCase);
// expected output: true

console.log(regex2.test('Football'));
// expected output: true
```

### multiline

multiline 属性表明正则表达式是否使用了 "m" 标志。multiline 是正则表达式实例的一个只读属性。

```js
const regex1 = new RegExp('^football');
const regex2 = new RegExp('^football', 'm');

console.log(regex1.multiline);
// expected output: false

console.log(regex2.multiline);
// expected output: true

console.log(regex1.test('rugby\nfootball'));
// expected output: false

console.log(regex2.test('rugby\nfootball'));
// expected output: true
```

### source

source 属性返回一个值为当前正则表达式对象的模式文本的字符串，该字符串不会包含正则字面量两边的斜杠以及任何的标志字符。

```js
var regex = /fooBar/ig;

console.log(regex.source); // "fooBar"，不包含 /.../ 和 "ig"。
```

### sticky

sticky 属性反映了搜索是否具有粘性（仅从正则表达式的 lastIndex 属性表示的索引处搜索）。sticky 是正则表达式对象的只读属性。

```js
const str1 = 'table football';
const regex1 = new RegExp('foo', 'y');

regex1.lastIndex = 6;

console.log(regex1.sticky);
// expected output: true

console.log(regex1.test(str1));
// expected output: true

console.log(regex1.test(str1));
// expected output: false
```

### unicode

unicode 属性表明正则表达式带有"u" 标志。 unicode 是正则表达式独立实例的只读属性。

```js
const regex1 = new RegExp('\u{61}');
const regex2 = new RegExp('\u{61}', 'u');

console.log(regex1.unicode);
// expected output: false

console.log(regex2.unicode);
// expected output: true

console.log(regex1.source);
// expected output: "a"

console.log(regex2.source);
// expected output: "a"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/RegExp
