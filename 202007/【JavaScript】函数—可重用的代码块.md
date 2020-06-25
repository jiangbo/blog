# 【JavaScript】函数—可重用的代码块

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 概念

在JavaScript中另一个基本概念是函数, 它允许你在一个代码块中存储一段用于处理单任务的代码，
然后在任何你需要的时候用一个简短的命令来调用，而不是把相同的代码写很多次。

## 浏览器内置函数

avaScript有许多内置的函数，可以让您做很多有用的事情，而无需自己编写所有的代码。
事实上,许多调用（运行或者执行的专业词语）浏览器内置函数时调用的代码并不是使用JavaScript来编写，
大多数调用浏览器后台的函数的代码，是使用像C++这样更低级的系统语言编写的，而不是像JavaScript这样的web编程语言。

请记住，这些内置浏览器函数不是核心JavaScript语言的一部分——被定义为浏览器API的一部分，
它建立在默认语言之上，以提供更多的功能（请参阅本课程的早期部分以获得更多的描述）。
我们将在以后的模块中更详细地使用浏览器API。

## 函数与方法

程序员把函数称为对象方法（method）的一部分，你还不必了解JavaScript中已建构的对象在更深层次上是如何运作的。
在我们继续前进之前，我们需要澄清一些有关方法和函数概念之间可能存在的误会，
当你在网络上浏览相关信息的时候，你很可能会碰上这两个术语。

到目前为止我们所使用的内置代码同属于这两种形式：函数和方法。
你可以在 [这里][2] 查看内置函数，内置对象以及其相关方法的完整列表。

严格说来，内置浏览器函数并不是函数——它们是方法。
这听起来有点可怕和令人困惑，但不要担心,函数和方法在很大程度上是可互换的，至少在我们的学习阶段是这样的。

二者区别在于方法是在对象内定义的函数。
浏览器内置函数（方法）和变量（称为属性）存储在结构化对象内，以使代码更加高效，易于处理。

## 自定义函数

```js
function draw() {
  ctx.clearRect(0,0,WIDTH,HEIGHT);
  for (var i = 0; i < 100; i++) {
    ctx.beginPath();
    ctx.fillStyle = 'rgba(255,0,0,0.5)';
    ctx.arc(random(WIDTH), random(HEIGHT), random(50), 0, 2 * Math.PI);
    ctx.fill();
  }
}
```

## 调用函数

使用括号来调用函数。

```js
function myFunction() {
  alert('hello');
}

myFunction()
// calls the function once
```

## 匿名函数

```js
var myButton = document.querySelector('button');

myButton.onclick = function() {
  alert('hello');
}
```

## 函数参数

一些函数需要在调用它们时指定参数 ——这些参数值需要放在函数括号内，才能正确地完成其工作。

```js
var myArray = ['I', 'love', 'chocolate', 'frogs'];
var madeAString = myArray.join(' ');
// returns 'I love chocolate frogs'
var madeAString = myArray.join();
// returns 'I,love,chocolate,frogs'
```

## 函数作用域和冲突

我们来谈一谈 scope 即作用域，这是处理函数时一个非常重要的概念。
当你创建一个函数时，函数内定义的变量和其他东西都在它们自己的单独的范围内, 
意味着它们被锁在自己独立的隔间中, 不能被函数外的代码访问。

所有函数的最外层被称为全局作用域。在全局作用域内定义的值可以在任意地方访问。

JavaScript 由于各种原因而建立，但主要是由于安全性和组织性。
有时您不希望变量可以在代码中的任何地方访问，您从其他地方调用的外部脚本可能会开始搞乱您的代码并导致问题，
因为它们恰好与代码的其他部分使用了相同的变量名称，造成冲突。这可能是恶意的，或者是偶然的。

```js
<!-- Excerpt from my HTML -->
<script src="first.js"></script>
<script src="second.js"></script>
<script>
  greeting();
</script>
// first.js
var name = 'Chris';
function greeting() {
  alert('Hello ' + name + ': welcome to our company.');
}
// second.js
var name = 'Zaptec';
function greeting() {
  alert('Our company is called ' + name + '.');
}
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Building_blocks/Functions
[2]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects