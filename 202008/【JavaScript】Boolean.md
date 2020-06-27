# 【JavaScript】Boolean

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Boolean 对象是一个布尔值的对象包装器。

如果需要，作为第一个参数传递的值将转换为布尔值。如果省略或值 0，-0，null，false，NaN，undefined，或空字符串（""），该对象具有的初始值 false。所有其他值，包括任何对象，空数组（[]）或字符串 "false"，都会创建一个初始值为的对象 true。

注意不要将基本类型中的布尔值 true 和 false 与值为 true 和 false 的 Boolean 对象弄混了。

其值不是 undefined 或 null 的任何对象（包括其值为false的布尔对象）在传递给条件语句时都将计算为 true。

```js
var x = new Boolean(false);
if (x) {
  // 这里的代码会被执行
}
```

## 构造器

Boolean() 创建一个新的 Boolean 对象。


不要用创建 Boolean 对象的方式将一个非布尔值转化成布尔值，直接将 Boolean 当做转换函数来使用即可，或者使用双重非（!!）运算符：

```js
var x = Boolean(expression);     // 推荐
var x = !!(expression);          // 推荐
var x = new Boolean(expression); // 不太好
```

对于任何对象，即使是值为 false 的 Boolean 对象，当将其传给 Boolean 函数时，生成的 Boolean 对象的值都是 true。

```js
var myFalse = new Boolean(false);   // true
var g = new Boolean(myFalse);       // true
var myString = new String("Hello");
var s = new Boolean(myString);      // true
```

最后，不要在应该使用基本类型布尔值的地方使用 Boolean 对象。

## 实例方法

### toString

toString() 方法返回指定的布尔对象的字符串形式。重写 Object.prototype.toString() 方法。

```js
const flag1 = new Boolean(true);

console.log(flag1.toString());
// expected output: "true"

const flag2 = new Boolean(1);

console.log(flag2.toString());
// expected output: "true"
```

### name

valueOf() 方法返回一个Boolean对象的原始值。重写 Object.prototype.valueOf() 方法。

```js
const x = new Boolean();

console.log(x.valueOf());
// expected output: false

const y = new Boolean('Mozilla');

console.log(y.valueOf());
// expected output: true
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Boolean

