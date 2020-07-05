# 【JavaScript】RegExp 实例方法

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## compile

已废弃的 compile() 方法被用于在脚本执行过程中（重新）编译正则表达式。与 RegExp 构造函数基本一样。

## exec

exec() 方法在一个指定字符串中执行一个搜索匹配。返回一个结果数组或 null。

在设置了 global 或 sticky 标志位的情况下（如 /foo/g or /foo/y），JavaScript RegExp 对象是有状态的。他们会将上次成功匹配后的位置记录在 lastIndex 属性中。使用此特性，exec() 可用来对单个字符串中的多次匹配结果进行逐条的遍历（包括捕获到的匹配），而相比之下， String.prototype.match() 只会返回匹配到的结果。

如果你只是为了判断是否匹配（true或 false），可以使用 RegExp.test() 方法，或者 String.search() 方法。

```js
const regex1 = RegExp('foo*', 'g');
const str1 = 'table football, foosball';
let array1;

while ((array1 = regex1.exec(str1)) !== null) {
  console.log(`Found ${array1[0]}. Next starts at ${regex1.lastIndex}.`);
  // expected output: "Found foo. Next starts at 9."
  // expected output: "Found foo. Next starts at 19."
}
```

## test

test() 方法执行一个检索，用来查看正则表达式与指定的字符串是否匹配。返回 true 或 false。

```js
const str = 'table football';

const regex = RegExp('foo*');
const globalRegex = RegExp('foo*', 'g');

console.log(regex.test(str));
// expected output: true

console.log(regex.test(str));
// expected output: true

console.log(globalRegex.lastIndex);
// expected output: 0

console.log(globalRegex.test(str));
// expected output: true

console.log(globalRegex.lastIndex);
// expected output: 9

console.log(globalRegex.test(str));
// expected output: false
```

## [@@match]

对正则表达式匹配字符串时，[@@match]() 方法用于获取匹配结果。

```js
class RegExp1 extends RegExp {
  [Symbol.match](str) {
    const result = RegExp.prototype[Symbol.match].call(this, str);
    if (result) {
      return 'VALID';
    }
    return 'INVALID';
  }
}

console.log('2012-07-02'.match(new RegExp1('([0-9]+)-([0-9]+)-([0-9]+)')));
// expected output: "VALID"
```

## [@@matchAll]

[@@matchAll]() 方法返回对字符串使用正则表达式的所有匹配项。

```js
class MyRegExp extends RegExp {
  [Symbol.matchAll](str) {
    let result = RegExp.prototype[Symbol.matchAll].call(this, str);
    if (!result) {
      return null;
    }
    return Array.from(result);
  }
}

let re = new MyRegExp('-[0-9]+', 'g');
console.log('2016-01-02|2019-03-07'.matchAll(re));
// expected output: Array [Array ["-01"], Array ["-02"], Array ["-03"], Array ["-07"]]
```

## [@@replace]

[@@replace]() 方法会在一个字符串中用给定的替换器，替换所有符合正则模式的匹配项，并返回替换后的新字符串结果。用来替换的参数可以是一个字符串或是一个针对每次匹配的回调函数。

```js
class RegExp1 extends RegExp {
  [Symbol.replace](str) {
    return RegExp.prototype[Symbol.replace].call(this, str, '#!@?');
  }
}

console.log('football'.replace(new RegExp1('foo')));
// expected output: "#!@?tball"
```

## [@@search]

[@@search]() 方法执行了一个在给定字符串中的一个搜索以取得匹配正则模式的项。

```js
class RegExp1 extends RegExp {
  constructor(str) {
    super(str);
    this.pattern = str;
  }
  [Symbol.search](str) {
    return str.indexOf(this.pattern);
  }
}

console.log('table football'.search(new RegExp1('foo')));
// expected output: 6
```

### [@@split]

[@@split]() 方法切割 String 对象为一个其子字符串的数组 。

```js
class RegExp1 extends RegExp {
  [Symbol.split](str, limit) {
    const result = RegExp.prototype[Symbol.split].call(this, str, limit);
    return result.map(x => `(${x})`);
  }
}

console.log('2016-01-02'.split(new RegExp1('-')));
// expected output: Array ["(2016)", "(01)", "(02)"]

console.log('2016-01-02'.split(new RegExp('-')));
// expected output: Array ["2016", "01", "02"]
```

### toString

toString() 返回一个表示该正则表达式的字符串。

```js
console.log(new RegExp('a+b+c'));
// expected output: /a+b+c/

console.log(new RegExp('a+b+c').toString());
// expected output: "/a+b+c/"

console.log(new RegExp('bar', 'g').toString());
// expected output: "/bar/g"

console.log(new RegExp('\n', 'g').toString());
// expected output (if your browser supports escaping): "/\n/g"

console.log(new RegExp('\\n', 'g').toString());
// expected output: "/\n/g"
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/RegExp
