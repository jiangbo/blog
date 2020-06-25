# 【JavaScript】开始编写 JavaScript

> 以下内容为学习记录，可以参考 [MDN][2] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 示例

以下将实现在浏览器上显示一个按钮，点击按钮，可以输入内容来改变浏览器按钮上的值。

### 新建 html 模板

在 vscode 中新建一个 index.html 文件，然后输入 html5 自动生成 html 的模板代码。

```html
<!DOCTYPE html>
<html lang="zh">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>JavaScript Demo</title>
</head>

<body>

</body>

</html>
```

其中的 lang 和 title 手动修改一下。

### 网页内容

```html
<p>玩家1：小明</p>
```

### 加入 css

```css
p {
  font-family: sans-serif;
  letter-spacing: 1px;
  text-transform: uppercase;
  text-align: center;
  border: 2px solid rgba(0, 0, 200, 0.6);
  background: rgba(0, 0, 200, 0.3);
  color: rgba(0, 0, 200, 0.6);
  box-shadow: 1px 1px 2px rgba(0, 0, 200, 0.4);
  border-radius: 10px;
  padding: 3px 10px;
  display: inline-block;
  cursor: pointer;
}
```

### 加入 JavaScript

js 代码需要放到 p 标签的后面，因为浏览器是顺序执行，必须要有了 p 标签，才能找到它。

```js
<script>

  const para = document.querySelector('p');
  para.addEventListener('click', updateName);

  function updateName() {
    const name = prompt('输入一个新的名字：');
    para.textContent = '玩家1：' + name;
  }
</script>
```

### 浏览器打开

使用浏览器打开刚刚新建的 index.html，可以看到一个按钮。
然后点击按钮，输入文字，可以看到网页上内容的变化。

![01javascript-what][1]

## 解释

### document

document 表示任何在浏览器中载入的网页，并作为网页内容的入口，也就是 DOM 树。
DOM 树包含了像 <body> 、<table> 这样的元素，以及大量其他元素。
它向网页文档本身提供了全局操作功能，能解决如何获取页面的 URL ，如何在文档中创建一个新的元素这样的问题。

> 就是我们看见的整个网页，通过它可以获取网页上的内容，或者在网页上新增删除修改等。

### querySelector

querySelector 方法返回文档中与指定选择器或选择器组匹配的第一个 html 元素。
如果找不到匹配项，则返回 null（使用的深度优先遍历）。

querySelector 方法接受一个选择器参数，该参数必须是有效的 CSS 选择器，这里不提选择器知识。
其中传递的参数 p，表示从当前的网页中，选择第一个 p 标签。即：

`<p>玩家1：小明</p>`

### addEventListener

通过上一步选择出来的 p 标签，调用 addEventListener 方法，表示需要在 p 标签上增加一个事件。
click 表示事件的类型，即点击事件。updateName 是定义好的一个函数，如果 p 标签被点击，
就会执行这个函数，通常我们将这种不是手动调用的函数为回调函数。

### prompt

显示一个对话框，对话框中包含一条文字信息，用来提示用户输入文字。

`const name = prompt('输入一个新的名字：');`

其中的 “输入一个新的名字：”是提示内容，name 可以接受输入的值，如果取消没有输入，返回 null。

### textContent

textContent 表示一个节点及其后代的文本内容。在这里，将输入的值填到了 p 标签内，所以内容进行了变化。

## 附录

### 说明

如果对其中的一些地方不了解，可以忽略，这只是简单体验一下 js 能做的工作。

### 源码

```html
<!DOCTYPE html>
<html lang="zh">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>JavaScript Demo</title>
  <style>
    p {
      font-family: sans-serif;
      letter-spacing: 1px;
      text-transform: uppercase;
      text-align: center;
      border: 2px solid rgba(0, 0, 200, 0.6);
      background: rgba(0, 0, 200, 0.3);
      color: rgba(0, 0, 200, 0.6);
      box-shadow: 1px 1px 2px rgba(0, 0, 200, 0.4);
      border-radius: 10px;
      padding: 3px 10px;
      display: inline-block;
      cursor: pointer;
    }
  </style>
</head>

<body>
  <p>玩家1：小明</p>
  <script>

    const para = document.querySelector('p');
    para.addEventListener('click', updateName);

    function updateName() {
      const name = prompt('输入一个新的名字：');
      para.textContent = '玩家1：' + name;
    }
  </script>
</body>

</html>
```

[1]: ./images/01javascript-what.png
[2]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/First_steps/A_first_splash