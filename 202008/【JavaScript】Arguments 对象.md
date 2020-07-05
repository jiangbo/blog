# 【JavaScript】Arguments 对象

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

arguments 是一个对应于传递给函数的参数的类数组对象。

arguments对象是所有（非箭头）函数中都可用的局部变量。
你可以使用arguments对象在函数中引用函数的参数。
此对象包含传递给函数的每个参数，第一个参数在索引0处。

```js
arguments[0]
arguments[1]
arguments[2]
```

arguments对象不是一个 Array 。
它类似于Array，但除了length属性和索引元素之外没有任何Array属性。
例如，它没有 pop 方法。但是它可以被转换为一个真正的Array：

```js
var args = Array.prototype.slice.call(arguments);
var args = [].slice.call(arguments);

// ES2015
const args = Array.from(arguments);
const args = [...arguments];
```

如果调用的参数多于正式声明接受的参数，则可以使用arguments对象。
这种技术对于可以传递可变数量的参数的函数很有用。使用 arguments.length来确定传递给函数参数的个数，
然后使用arguments对象来处理每个参数。要确定函数签名中（输入）参数的数量，请使用Function.length属性。

## 对 arguments 使用 typeof

typeof参数返回 'object'。

```js
console.log(typeof arguments);    // 'object'
// arguments 对象只能在函数内使用
function test(a){
    console.log(a,Object.prototype.toString.call(arguments));
    console.log(arguments[0],arguments[1]);
    console.log(typeof arguments[0]);
}
test(1);
/*
1 "[object Arguments]"
1 undefined
number
*/
```

可以使用索引确定单个参数的类型。

```js
console.log(typeof arguments[0]); //this will return the typeof individual arguments.
```

## 属性

* arguments.callee 指向当前执行的函数。
* arguments.caller 指向调用当前函数的函数。
* arguments.length 指向传递给当前函数的参数数量。
* arguments[@@iterator] 返回一个新的 Array 迭代器对象，该对象包含参数中每个索引的值。

注意:现在在严格模式下，arguments 对象已与过往不同。
arguments[@@iterator] 不再与函数的实际形参之间共享，同时 caller 属性也被移除。

### 遍历参数求和

```js
function add() {
    var sum =0,
        len = arguments.length;
    for(var i=0; i<len; i++){
        sum += arguments[i];
    }
    return sum;
}
add()                           // 0
add(1)                          // 1
add(1,2,3,4);                   // 10
```

### 定义连接字符串的函数

```js
function myConcat(separator) {
  var args = Array.prototype.slice.call(arguments, 1);
  return args.join(separator);
}
// returns "red, orange, blue"
myConcat(", ", "red", "orange", "blue");

// returns "elephant; giraffe; lion; cheetah"
myConcat("; ", "elephant", "giraffe", "lion", "cheetah");

// returns "sage. basil. oregano. pepper. parsley"
myConcat(". ", "sage", "basil", "oregano", "pepper", "parsley");
```

### 定义创建HTML列表的方法

```js
function list(type) {
  var result = "<" + type + "l><li>";
  var args = Array.prototype.slice.call(arguments, 1);
  result += args.join("</li><li>");
  result += "</li></" + type + "l>"; // end list

  return result;
}
var listHTML = list("u", "One", "Two", "Three");

/* listHTML is:

"<ul><li>One</li><li>Two</li><li>Three</li></ul>"

*/
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Functions/arguments

