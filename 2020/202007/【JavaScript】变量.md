# 【JavaScript】变量

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 变量是什么

一个变量，就是一个用于存放数值的容器。
这个数值可能是一个用于累加计算的数字，或者是一个句子中的字符串。
变量的独特之处在于它存放的数值是可以改变的。

### 示例

```html
<!DOCTYPE html>
<html lang="zh">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>变量</title>
</head>

<body>
  <button>点我</button>
  <script>

    const button = document.querySelector('button');

    button.onclick = function () {
      let name = prompt('What is your name?');
      alert('Hello ' + name + ', nice to see you!');
    }
  </script>
</body>

</html>
```

在上面的例子中，点击按钮之后，第一行代码会在屏幕上弹出一个对话框，让你输入名字，然后存储输入的名字到一个变量。
第二行代码将会显示包含你名字的欢迎信息，你的名字就是从之前的变量里面读取的。

**我们说，变量是用来存储数值的，那么有一个重要的概念需要区分。变量不是数值本身，
它们仅仅是一个用于存储数值的容器。你可以把变量想象成一个个用来装东西的纸箱子。**

## 声明变量

要想使用变量，你需要做的第一步就是创建它 -- 更准确的说，是声明一个变量。
声明一个变量的语法是在 var 或 let 关键字之后加上这个变量的名字

`let myName;`

因为 myName 并没有定义值，这时候输入 myName 将会得到 undefined。

## 初始化变量

在变量名之后跟上一个“=”就可以对它进行初始化，也就是赋值。

`let myName = 'JiangBo';`

### var 与 let 的区别

申明变量可以使用 var 或者 let，您可能会想：“为什么我们需要两个关键字来定义变量?”。

这是由于历史的原因，最初创建 JavaScript 时，是只有 var 的。 
在大多数情况下，这种方法可以接受， 但有时在工作方式上会有一些问题——它的设计会令人困惑或令人讨厌。
因此，let 是在现代版本中的 JavaScript 创建的一个新的关键字。
IE 11 可以支持 let 和 const。

> 推荐使用 let 而不是 var。

## 更新变量

一旦变量赋值，您可以通过简单地给它一个不同的值来更新它。

```js
let name = "JiangBo";
name = "BoJiang";
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/First_steps/Variables