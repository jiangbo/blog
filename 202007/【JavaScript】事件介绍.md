# 【JavaScript】事件介绍

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 概念

事件是您在编程时系统内发生的动作或者发生的事情，系统响应事件后，如果需要，您可以某种方式对事件做出回应。
例如：如果用户在网页上单击一个按钮，您可能想通过显示一个信息框来响应这个动作。

## 一系列事件

就像上面提到的, 事件是您在编程时系统内发生的动作或者发生的事情——系统会在事件出现时产生或触发某种信号，
并且会提供一个自动加载某种动作（列如：运行一些代码）的机制，比如在一个机场，当跑道清理完成，
飞机可以起飞时，飞行员会收到一个信号，因此他们开始起飞。

在Web中, 事件在浏览器窗口中被触发并且通常被绑定到窗口内部的特定部分，
可能是一个元素、一系列元素、被加载到这个窗口的 HTML 代码或者是整个浏览器窗口。
举几个可能发生的不同事件：

* 用户在某个元素上点击鼠标或悬停光标。
* 用户在键盘中按下某个按键。
* 用户调整浏览器的大小或者关闭浏览器窗口。
* 一个网页停止加载。
* 提交表单。
* 播放、暂停、关闭视频。
* 发生错误。

每个可用的事件都会有一个事件处理器，也就是事件触发时会运行的代码块。
当我们定义了一个用来回应事件被激发的代码块的时候，我们说我们注册了一个事件处理器。
注意事件处理器有时候被叫做事件监听器，从我们的用意来看这两个名字是相同的，
尽管严格地来说这块代码既监听也处理事件。监听器留意事件是否发生，然后处理器就是对事件发生做出的回应。

## 事件类型

btn.onfocus 及 btn.onblur：颜色将于按钮被置于焦点或解除焦点时改变（尝试使用Tab移动至按钮上，然后再移开）。
这些通常用于显示有关如何在置于焦点时填写表单字段的信息，或者如果表单字段刚刚填入不正确的值，则显示错误消息。

btn.ondblclick：颜色将仅于按钮被双击时改变。

window.onkeypress, window.onkeydown, window.onkeyup：当按钮被按下时颜色会发生改变。 
keypress 指的是通俗意义上的按下按钮 (按下并松开), 而 keydown 和 keyup 指的是按键动作的一部分,
分别指按下和松开。注意如果你将事件处理器添加到按钮本身，它将不会工作，
我们只能将它添加到代表整个浏览器窗口的 window对象中。

btn.onmouseover 和 btn.onmouseout：颜色将会在鼠标移入按钮上方时发生改变, 或者当它从按钮移出时。

一些事件非常通用，几乎在任何地方都可以用（比如 onclick 几乎可以用在几乎每一个元素上），
然而另一些元素就只能在特定场景下使用，比如我们只能在 video 元素上使用 onplay。

## 行内事件处理器（请勿使用）

```html
<button onclick="bgChange()">Press me</button>
<script>
function bgChange() {
  var rndCol = 'rgb(' + random(255) + ',' + random(255) + ',' + random(255) + ')';
  document.body.style.backgroundColor = rndCol;
}
</script>
```

## onxxx 事件处理器

```js
var buttons = document.querySelectorAll('button');

for (var i = 0; i < buttons.length; i++) {
  buttons[i].onclick = bgChange;
}
```

## EventListener 事件处理器

addEventListener 和 removeEventListener 可以增加和删除事件。

```js
var btn = document.querySelector('button');

function bgChange() {
  var rndCol = 'rgb(' + random(255) + ',' + random(255) + ',' + random(255) + ')';
  document.body.style.backgroundColor = rndCol;
}   

btn.addEventListener('click', bgChange);
```

使用 removeEventListener 可以移除事件：

```js
btn.removeEventListener('click', bgChange);
```

在这个简单的、小型的项目中可能不是很有用，但是在大型的、复杂的项目中就非常有用了，
可以非常高效地清除不用的事件处理器，另外在其他的一些场景中也非常有效，
比如您需要在不同环境下运行不同的事件处理器，您只需要恰当地删除或者添加事件处理器即可。

> 在浏览器兼容的情况下，使用 EventListener，需要兼容旧浏览器则使用 onxxx 的方式。

## 其他事件概念

### 事件对象

有时候在事件处理函数内部，您可能会看到一个固定指定名称的参数，例如event，evt或简单的e。
这被称为事件对象，它被自动传递给事件处理函数，以提供额外的功能和信息。

```js
function bgChange(e) {
  var rndCol = 'rgb(' + random(255) + ',' + random(255) + ',' + random(255) + ')';
  e.target.style.backgroundColor = rndCol;
  console.log(e);
}  

btn.addEventListener('click', bgChange);
```
> 事件对象 e 的 target 属性始终是事件刚刚发生的元素的引用。

### 阻止默认行为

有时，你会遇到一些情况，你希望事件不执行它的默认行为。最常见的例子是Web表单，例如自定义注册表单。
当你填写详细信息并按提交按钮时，自然行为是将数据提交到服务器上的指定页面进行处理，
并将浏览器重定向到某种“成功消息”页面（或 相同的页面，如果另一个没有指定。）

当用户没有正确提交数据时，麻烦就来了。作为开发人员，你希望停止提交信息给服务器，
并给他们一个错误提示，告诉他们什么做错了，以及需要做些什么来修正错误。
一些浏览器支持自动的表单数据验证功能，但由于许多浏览器不支持，
因此建议你不要依赖这些功能，并实现自己的验证检查。
我们来看一个简单的例子。

```html
<form>
  <div>
    <label for="fname">First name: </label>
    <input id="fname" type="text">
  </div>
  <div>
    <label for="lname">Last name: </label>
    <input id="lname" type="text">
  </div>
  <div>
     <input id="submit" type="submit">
  </div>
</form>
<p></p>
```

```js
var form = document.querySelector('form');
var fname = document.getElementById('fname');
var lname = document.getElementById('lname');
var submit = document.getElementById('submit');
var para = document.querySelector('p');

form.onsubmit = function(e) {
  if (fname.value === '' || lname.value === '') {
    e.preventDefault();
    para.textContent = 'You need to fill in both names!';
  }
}
```

### 事件冒泡及捕获

当一个事件发生在具有父元素的元素上时，现代浏览器运行两个不同的阶段：捕获阶段和冒泡阶段。

在捕获阶段：
浏览器检查元素的最外层祖先 <html>，是否在捕获阶段中注册了一个 onclick 事件处理程序，如果是，则运行它。
然后，它移动到<html>中单击元素的下一个祖先元素，并执行相同的操作，然后是单击元素再下一个祖先元素，
依此类推，直到到达实际点击的元素。

在冒泡阶段，恰恰相反:
浏览器检查实际点击的元素是否在冒泡阶段中注册了一个onclick事件处理程序，如果是，则运行它。
然后它移动到下一个直接的祖先元素，并做同样的事情，然后是下一个，等等，直到它到达<html>元素。

如果要阻止事件冒泡，则使用 stopPropagation 方法。

### 事件委托

冒泡还允许我们利用事件委托——这个概念依赖于这样一个事实，如果你想要在大量子元素中单击任何一个都可以运行一段代码，
您可以将事件监听器设置在其父节点上，并让子节点上发生的事件冒泡到父节点上，而不是每个子节点单独设置事件监听器。

一个很好的例子是一系列列表项，如果你想让每个列表点击时弹出一条信息，您可以将click单击事件监听器设置在父元素<ul>上，它将会冒泡到列表项上。


[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Building_blocks/Events