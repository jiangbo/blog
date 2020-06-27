# 【JavaScript】对象原型

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 概念

到目前为止我们已经了解了一些关于原型链的实现方式以及成员变量是如何通过它来实现继承，
但是之前涉及到的大部分都是浏览器内置函数（比如 String、Date、Number 和 Array）。
那么我们如何创建一个继承自另一对象的JavaScript对象呢？

> ES2015 在实现继承的方式简单了许多，下面基于 ES2015 实现。

## 定义 Person

```js
function Person(first, last, age, gender, interests) {
  this.name = {
    first,
    last
  };
  this.age = age;
  this.gender = gender;
  this.interests = interests;
};
```

## 定义 Teacher

```js
class Teacher extends Person {
  constructor(first, last, age, gender, interests, subject, grade) {
    super(first, last, age, gender, interests);

    // subject and grade are specific to Teacher
    this.subject = subject;
    this.grade = grade;
  }
}
```

## getter 和 setter

getter 和 setter 在赋值和取值的时候，可以做一些额外的操作。

```js
class Teacher extends Person {
  constructor(first, last, age, gender, interests, subject, grade) {
    super(first, last, age, gender, interests);
    // subject and grade are specific to Teacher
    this._subject = subject;
    this.grade = grade;
  }

  get subject() {
    return this._subject;
  }

  set subject(newSubject) {
    this._subject = newSubject;
  }
}
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Objects/Inheritance