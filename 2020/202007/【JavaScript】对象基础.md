# 【JavaScript】对象基础

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 概念

对象是一个包含相关数据和方法的集合（通常由一些变量和函数组成，我们称之为对象里面的属性和方法）。

## 创建对象

```js
var person = {};
var person = {
  name : ['Bob', 'Smith'],
  age : 32,
  gender : 'male',
  interests : ['music', 'skiing'],
  bio : function() {
    alert(this.name[0] + ' ' + this.name[1] + ' is ' + this.age + ' years old. He likes ' + this.interests[0] + ' and ' + this.interests[1] + '.');
  },
  greeting: function() {
    alert('Hi! I\'m ' + this.name[0] + '.');
  }
};
```

## 点表示法

可以使用点表示法获取对象的属性和方法。

```js
person.age
person.interests[1]
```

## 子命名空间

可以用一个对象来做另一个对象成员的值。

```js
person.name =  {
  first : 'Bob',
  last : 'Smith'
}
```

## 括号表示法

另外一种访问属性的方式是使用括号表示法(bracket notation)。

```js
person['age']
person['name']['first']
```

这看起来很像访问一个数组的元素，从根本上来说是一回事儿，你使用了关联了值的名字，而不是索引去选择元素。
难怪对象有时被称之为关联数组(associative array)了——对象做了字符串到值的映射，而数组做的是数字到值的映射。

## 设置对象成员

```js
person.age = 45
person['name']['last'] = 'Cratchit'
```

设置成员并不意味着你只能更新已经存在的属性的值，你完全可以创建新的成员：

```js
person['eyes'] = 'hazel'
person.farewell = function() { alert("Bye everybody!") }
```

括号表示法一个有用的地方是它不仅可以动态的去设置对象成员的值，还可以动态的去设置成员的名字。

```js
var myDataName = 'height'
var myDataValue = '1.75m'
person[myDataName] = myDataValue
```

## "this"的含义

关键字"this"指向了当前代码运行时的对象。

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Objects/Basics