# 【JavaScript】Error

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

通过 Error 的构造器可以创建一个错误对象。当运行时错误产生时，Error 的实例对象会被抛出。Error 对象也可用于用户自定义的异常的基础对象。

## 构造函数

当像函数一样使用 Error 时，如果没有 new，它将返回一个 Error 对象。所以， 仅仅调用 Error 产生的结果与通过 new 关键字构造 Error 对象生成的结果相同。

```js
// this:
const x = Error('I was created using a function call!');
​​​​// has the same functionality as this:
const y = new Error('I was constructed via the "new" keyword!');
```

## 实例属性 name

异常的名称

```js
try {
  throw new Error("Whoops!");
} catch (e) {
  console.log(e.name + ": " + e.message);
}
// Error: Whoops!
```

## 实例属性 message

异常信息

```js
try {
  throw new Error("Whoops!");
} catch (e) {
  console.log(e.name + ": " + e.message);
}
// Error: Whoops!
```

## valueOf

valueOf() 方法返回当前 symbol 对象所包含的 symbol 原始值。覆盖 Object.prototype.valueOf() 方法。

```js
Object(Symbol("foo")) + "bar";
// TypeError: can't convert symbol object to primitive
// 无法隐式的调用 valueOf() 方法

Object(Symbol("foo")).valueOf() + "bar";
// TypeError:  can't convert symbol to string
// 手动调用 valueOf() 方法，虽然转换成了原始值，但 symbol 原始值不能转换为字符串

Object(Symbol("foo")).toString() + "bar";
// "Symbol(foo)bar"，需要手动调用 toString() 方法才行
```

## 错误类型

除了通用的Error构造函数外，JavaScript还有其他类型的错误构造函数。

###  EvalError

创建一个 error 实例，表示错误的原因：与 eval() 有关。

### InternalError（已过时） 

创建一个代表Javascript引擎内部错误的异常抛出的实例。 如: "递归太多"。

### RangeError

创建一个error实例，表示错误的原因：数值变量或参数超出其有效范围。

### ReferenceError

创建一个error实例，表示错误的原因：无效引用。

### SyntaxError

创建一个error实例，表示错误的原因：eval()在解析代码的过程中发生的语法错误。

### TypeError

创建一个error实例，表示错误的原因：变量或参数不属于有效类型。

### URIError

创建一个error实例，表示错误的原因：给 encodeURI()或  decodeURl()传递的参数无效。

### AggregateError

当多个错误​​需要包装在一个错误中时，AggregateError 对象表示一个错误。

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Error

