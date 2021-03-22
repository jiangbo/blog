# 【JavaScript】对象原型

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 概念

JavaScript 常被描述为一种基于原型的语言 (prototype-based language)，每个对象拥有一个原型对象，
对象以其原型为模板、从原型继承方法和属性。原型对象也可能拥有原型，并从中继承方法和属性，
一层一层、以此类推。这种关系常被称为原型链 (prototype chain)，
它解释了为何一个对象会拥有定义在其他对象中的属性和方法。

> 可以使用 Object.getPrototypeOf 获得对象的原型。

## 使用Javascript中的原型

在javascript中，函数可以有属性。每个函数都有一个特殊的属性叫作原型（prototype）。

```js
function doSomething(){}
console.log( doSomething.prototype );
// It does not matter how you declare the function, a
//  function in javascript will always have a default
//  prototype property.
var doSomething = function(){}; 
console.log( doSomething.prototype );
```

## 理解原型对象

```js
function Person(first, last, age, gender, interests) {
  
  // 属性与方法定义
  
};

var person1 = new Person('Bob', 'Smith', 32, 'male', ['music', 'skiing']);
person1.valueOf();
```

这个方法仅仅返回了被调用对象的值。在这个例子中发生了如下过程：

* 浏览器首先检查，person1 对象是否具有可用的 valueOf() 方法。
* 如果没有，则浏览器检查 person1 对象的原型对象（即 Person 构造函数的 prototype 属性所指向的对象）是否具有可用的 valueof() 方法。
* 如果也没有，则浏览器检查 Person() 构造函数的 prototype 属性所指向的对象的原型对象（即 Object 构造函数的 prototype 属性所指向的对象）是否具有可用的 valueOf() 方法。这里有这个方法，于是该方法被调用。

## prototype 属性

继承的属性和方法是定义在 prototype 属性之上的（你可以称之为子命名空间 (sub namespace)）。
那些以 Object.prototype. 开头的属性，而非仅仅以 Object. 开头的属性。
prototype 属性的值是一个对象，我们希望被原型链下游的对象继承的属性和方法，都被储存在其中。

### constructor 属性

每个实例对象都从原型中继承了一个 constructor 属性，该属性指向了用于构造此实例对象的构造函数。

```js
person2.constructor // Person()
```

## 修改原型

```js
function Person(first, last, age, gender, interests) {

  // 属性与方法定义

};

var person1 = new Person('Tammi', 'Smith', 32, 'neutral', ['music', 'skiing', 'kickboxing']);

Person.prototype.farewell = function() {
  alert(this.name.first + ' has left the building. Bye for now!');
}
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Objects/Object_prototypes