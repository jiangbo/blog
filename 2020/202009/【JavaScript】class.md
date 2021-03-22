# 【JavaScript】class

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

class 声明创建一个基于原型继承的具有给定名称的新类。

## 示例

```js
class Polygon {
  constructor(height, width) {
    this.area = height * width;
  }
}

console.log(new Polygon(4, 3).area);
// expected output: 12
```

constructor 是构造函数，this 表示 Polygon 的示例。

## 继承

在下面的例子中，我们首先定义一个名为 Polygon 的类，然后继承它来创建一个名为 Square 的类。注意，构造函数中使用的 super() 只能在构造函数中使用，并且必须在使用 this 关键字前调用。

```js
class Polygon {
  constructor(height, width) {
    this.name = 'Polygon';
    this.height = height;
    this.width = width;
  }
}

class Square extends Polygon {
  constructor(length) {
    super(length, length);
    this.name = 'Square';
  }
}
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/class
