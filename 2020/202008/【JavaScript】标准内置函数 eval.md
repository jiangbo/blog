# 【JavaScript】标准内置函数 eval

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

eval() 是全局对象的一个函数属性。

eval() 的参数是一个字符串。如果字符串表示的是表达式，eval() 会对表达式进行求值。
如果参数表示一个或多个 JavaScript 语句，那么eval() 就会执行这些语句。
不需要用 eval() 来执行一个算术表达式：因为 JavaScript 可以自动为算术表达式求值。

如果你以字符串的形式构造了算术表达式，那么可以在后面用 eval() 对它求值。
例如，假设你有一个变量 x，您可以通过将表达式的字符串值（例如 3 * x + 2）赋值给一个变量，
然后在你的代码后面的其他地方调用 eval()，来推迟涉及 x 的表达式的求值。

如果 eval() 的参数不是字符串， eval() 会将参数原封不动地返回。
在下面的例子中，String 构造器被指定，而 eval() 返回了 String 对象而不是执行字符串。

```js
eval(new String("2 + 2")); // 返回了包含"2 + 2"的字符串对象
eval("2 + 2");             // returns 4
```

你可以使用一些通用的方法来绕过这个限制，例如使用 toString()。

```js
var expression = new String("2 + 2");
eval(expression.toString());
```

如果你间接的使用 eval()，比如通过一个引用来调用它，而不是直接的调用 eval。
从 ECMAScript 5 起，它工作在全局作用域下，而不是局部作用域中。这就意味着，例如，下面的代码的作用声明创建一个全局函数，并且 eval 中的这些代码在执行期间不能在被调用的作用域中访问局部变量。

```js
function test() {
  var x = 2, y = 4;
  console.log(eval('x + y'));  // 直接调用，使用本地作用域，结果是 6
  var geval = eval; // 等价于在全局作用域调用
  console.log(geval('x + y')); // 间接调用，使用全局作用域，throws ReferenceError 因为`x`未定义
  (0, eval)('x + y'); // 另一个间接调用的例子
​}
```
> 永远不要使用 eval！

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/eval

