# 【JavaScript】对象基础

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 构造函数和对象

有些人认为 JavaScript 不是真正的面向对象的语言，比如它没有像许多面向对象的语言一样有用于创建class类的声明。
JavaScript 用一种称为构建函数的特殊函数来定义对象和它们的特征。构建函数非常有用，
因为很多情况下您不知道实际需要多少个对象（实例）。构建函数提供了创建您所需对象（实例）的有效方法，
将对象的数据和特征函数按需联结至相应对象。

不像“经典”的面向对象的语言，从构建函数创建的新实例的特征并非全盘复制，
而是通过一个叫做原形链的参考链链接过去的。所以这并非真正的实例，
严格的讲，JavaScript 在对象间使用和其它语言的共享机制不同。

## 定义对象

```js
    function Person(first, last, age, gender, interests) {
      this.name = {
        'first': first,
        'last': last
      };
      this.age = age;
      this.gender = gender;
      this.interests = interests;
      this.bio = function() {
        alert(this.name.first + ' ' + this.name.last + ' is ' + this.age + ' years old. He likes ' +
          this.interests[0] + ' and ' + this.interests[1] + '.');
      };
      this.greeting = function() {
        alert('Hi! I\'m ' + this.name.first + '.');
      };
    };
```

## 使用对象

```js
    var person = {};
    var person1 = new Person('Bob', 'Smith', 32, 'male', ['music', 'skiing']);

    console.log(person1);
    console.log(person1.interests);
```

## 创建对象的其他方式

到现在为止，我们了解到了两种不同的创建对象的方式：声明一个对象的语法，与使用构造函数。

### Object()构造函数

```js
var person1 = new Object();
```

## 使用create()方法

另外一种访问属性的方式是使用括号表示法(bracket notation)。

```js
var person2 = Object.create(person1);
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Objects/Object-oriented_JS