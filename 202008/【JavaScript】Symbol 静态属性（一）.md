# 【JavaScript】Symbol 静态属性（一）

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

symbol 是一种基本数据类型 （primitive data type）。Symbol()函数会返回 symbol 类型的值，该类型具有静态属性和静态方法。它的静态属性会暴露几个内建的成员对象；它的静态方法会暴露全局的 symbol 注册，且类似于内建对象类，但作为构造函数来说它并不完整，因为它不支持语法："new Symbol()"。

每个从 Symbol() 返回的 symbol 值都是唯一的。一个 symbol 值能作为对象属性的标识符；这是该数据类型仅有的目的。

```js
const symbol1 = Symbol();
const symbol2 = Symbol(42);
const symbol3 = Symbol('foo');

console.log(typeof symbol1);
// expected output: "symbol"

console.log(symbol2 === 42);
// expected output: false

console.log(symbol3.toString());
// expected output: "Symbol(foo)"

console.log(Symbol('foo') === Symbol('foo'));
// expected output: false
```

## 构造函数

Symbol() 创建一个新的 Symbol 对象。作为构造函数，它不完整，因为它不支持语法 “new Symbol()”。

```js
let sym = new Symbol()  // TypeError
```

## 静态属性

### asyncIterator

Symbol.asyncIterator 符号指定了一个对象的默认异步迭代器。如果一个对象设置了这个属性，它就是异步可迭代对象，可用于for await...of循环。

```js
const myAsyncIterable = new Object();
myAsyncIterable[Symbol.asyncIterator] = async function*() {
    yield "hello";
    yield "async";
    yield "iteration!";
};

(async () => {
    for await (const x of myAsyncIterable) {
        console.log(x);
        // expected output:
        //    "hello"
        //    "async"
        //    "iteration!"
    }
})();
```

### hasInstance

Symbol.hasInstance 用于判断某对象是否为某构造器的实例。因此你可以用它自定义 instanceof 操作符在某个类上的行为。

```js
class MyArray {  
  static [Symbol.hasInstance](instance) {
    return Array.isArray(instance);
  }
}
console.log([] instanceof MyArray); // true
```

### isConcatSpreadable

内置的 Symbol.isConcatSpreadable 符号用于配置某对象作为 Array.prototype.concat() 方法的参数时是否展开其数组元素。


```js
const alpha = ['a', 'b', 'c'];
const numeric = [1, 2, 3];
let alphaNumeric = alpha.concat(numeric);

console.log(alphaNumeric);
// expected output: Array ["a", "b", "c", 1, 2, 3]

numeric[Symbol.isConcatSpreadable] = false;
alphaNumeric = alpha.concat(numeric);

console.log(alphaNumeric);
// expected output: Array ["a", "b", "c", Array [1, 2, 3]]
```

### iterator

Symbol.iterator 为每一个对象定义了默认的迭代器。该迭代器可以被 for...of 循环使用。

```js
var myIterable = {}
myIterable[Symbol.iterator] = function* () {
    yield 1;
    yield 2;
    yield 3;
};
console.log([...myIterable]); // [1, 2, 3]

for (const i of myIterable) {
  console.log(i);
}
```

### match

Symbol.match 指定了匹配的是正则表达式而不是字符串。String.prototype.match() 方法会调用此函数。
Symbol.matchAll 返回一个迭代器，该迭代器针对字符串生成正则表达式的匹配项。此函数由 String.prototype.matchAll() 方法调用。

```js
const regexp1 = /foo/;
// console.log('/foo/'.startsWith(regexp1));
// expected output: (Chrome) Error: First argument to String.prototype.startsWith must not be a regular expression
// expected output: (Firefox) Error: Invalid type: first can't be a Regular Expression

regexp1[Symbol.match] = false;

console.log('/foo/'.startsWith(regexp1));
// expected output: true

console.log('/baz/'.endsWith(regexp1));
// expected output: false
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Symbol

