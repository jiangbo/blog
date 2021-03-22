# 【JavaScript】排查错误

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 排错

之前创建一个猜数字的游戏，如果还没有运行起来的话，接下来的排查错误可以帮忙到你。
如果没有调试出来也没有关系，记着可以打开 F12 进行调试就可以了，以后会用到的。

### 错误类型

一般来说，代码错误主要分为两种：

* 语法错误：代码中存在拼写错误，将导致程序完全或部分不能运行，通常你会收到一些出错信息。只要熟悉语言并了解出错信息的含义，你就能够顺利修复它们。

* 逻辑错误：有些代码语法虽正确，但执行结果和预期相悖，这里便存在着逻辑错误。这意味着程序虽能运行，但会给出错误的结果。由于一般你不会收到来自这些错误的提示，它们通常比语法错误更难修复。

### 包含错误的代码

```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <title>猜数字游戏</title>
  <style>
    html {
      font-family: sans-serif;
    }

    body {
      width: 50%;
      max-width: 800px;
      min-width: 480px;
      margin: 0 auto;
    }

    .lastResult {
      color: white;
      padding: 3px;
    }
  </style>
</head>

<body>
  <h1>猜数字游戏</h1>

  <p>我刚才随机选定了一个100以内的自然数。看你能否在 10 次以内猜中它。每次我都会告诉你所猜的结果是高了还是低了。</p>

  <div class="form">
    <label for="guessField">Enter a guess: </label><input type="text" id="guessField" class="guessField">
    <input type="submit" value="Submit guess" class="guessSubmit">
  </div>

  <div class="resultParas">
    <p class="guesses"></p>
    <p class="lastResult"></p>
    <p class="lowOrHi"></p>
  </div>

</body>

<script>
  let randomNumber = Math.floor(Math.random()) + 1;

  const guesses = document.querySelector('.guesses');
  const lastResult = document.querySelector('.lastResult');
  const lowOrHi = document.querySelector('lowOrHi');
  const guessSubmit = document.querySelector('.guessSubmit');
  const guessField = document.querySelector('.guessField');

  let guessCount = 1;
  let resetButton;

  function checkGuess() {

    let userGuess = Number(guessField.value);
    if (guessCount === 1) {
      guesses.textContent = 'Previous guesses: ';
    }
    guesses.textContent += userGuess + ' ';

    if (userGuess === randomNumber) {
      lastResult.textContent = 'Congratulations! You got it right!';
      lastResult.style.backgroundColor = 'green';
      lowOrHi.textContent = '';
      setGameOver();
    } else if (guessCount === 10) {
      lastResult.textContent = '!!!GAME OVER!!!';
      setGameOver();
    } else {
      lastResult.textContent = 'Wrong!';
      lastResult.style.backgroundColor = 'red';
      if (userGuess < randomNumber) {
        lowOrHi.textContent = 'Last guess was too low!';
      } else if (userGuess > randomNumber) {
        lowOrHi.textContent = 'Last guess was too high!';
      }
    }

    guessCount++;
    guessField.value = '';
    guessField.focus();
  }
  guessSubmit.addeventListener('click', checkGuess);

  function setGameOver() {
    guessField.disabled = true;
    guessSubmit.disabled = true;
    resetButton = document.createElement('button');
    resetButton.textContent = 'Start new game';
    document.body.appendChild(resetButton);
    resetButton.addeventListener('click', resetGame);
  }

  function resetGame() {
    guessCount = 1;

    const resetParas = document.querySelectorAll('.resultParas p');
    for (let i = 0; i < resetParas.length; i++) {
      resetParas[i].textContent = '';
    }
    resetButton.parentNode.removeChild(resetButton);

    guessField.disabled = false;
    guessSubmit.disabled = false;
    guessField.value = '';
    guessField.focus();

    lastResult.style.backgroundColor = 'white';

    randomNumber = Math.floor(Math.random()) + 1;
  }
</script>

</html>
```

浏览器运行打开上面的程序，可以看到页面正常打开，但是点击按钮却没有响应。

### 修复语法错误

切换到猜数字的网页，按 F12 键进入开发者控制台，可以看到控制台中输入了一个红色的错误：

```text
Uncaught TypeError: guessSubmit.addeventListener is not a function
    at index.html:86
```

* 控制台的红色表示这是一个错误。
* 接下来是具体的错误描述信息：“TypeError：guessSubmit.addeventListener is not a function”（类型错误：guessSubmit.addeventListener 不是函数）
* JavaScript 文件名，点击将跳转到开发者工具的“调试器”标签页。
* 如果你按照这个链接，你会看到错误突出显示的确切行。

出错信息显示“guessSubmit.addeventListener 不是一个函数”，说明这里可能存在拼写错误。
如果你不确定某语法的拼写是否正确，可以到 MDN 上去查找，这里错误显然是我们把函数名写错造成的。
请记住，JavaScript 区分大小写，所以任何轻微的不同或大小写问题都会导致出错。
将 addeventListener 改为 addEventListener 便可解决。

### 再次修复语法错误

`Uncaught TypeError: Cannot set property 'textContent' of null  index.html:78`

可以看到提示 78 行的取值 textContent 的属性为 null，原因是：
`const lowOrHi = document.querySelector('lowOrHi');` 缺少一个.，这里应该是一个 class 选择器。
修改为: `const lowOrHi = document.querySelector('.lowOrHi');`


### 最后修复语法错误

在游戏结束时，再次得到一个错误：`Uncaught TypeError: resetButton.addeventListener is not a function`
还是第一次遇到的问题，函数名称写错了。

### 修复逻辑错误

玩的次数多了，发现每次正确的数字总是 1，这里面包含一个逻辑错误。分析如下：

Math.random 生成一个在 0 和 1 之间的十进制随机数，
Math.floor 会舍弃小数部分返回与之最接近的整数，所以最终得到的值
Math.floor(Math.random()) + 1 总是 0 和 1，并且 1 占绝大多数。
修复如下：
`Math.floor(Math.random() * 100) + 1;`

## 附录

### 源码

```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <title>猜数字游戏</title>
  <style>
    html {
      font-family: sans-serif;
    }

    body {
      width: 50%;
      max-width: 800px;
      min-width: 480px;
      margin: 0 auto;
    }

    .lastResult {
      color: white;
      padding: 3px;
    }
  </style>
</head>

<body>
  <h1>猜数字游戏</h1>

  <p>我刚才随机选定了一个100以内的自然数。看你能否在 10 次以内猜中它。每次我都会告诉你所猜的结果是高了还是低了。</p>

  <div class="form">
    <label for="guessField">输入一个数字: </label><input type="text" id="guessField" class="guessField">
    <input type="submit" value="猜测" class="guessSubmit">
  </div>

  <div class="resultParas">
    <p class="guesses"></p>
    <p class="lastResult"></p>
    <p class="lowOrHi"></p>
  </div>

</body>

<script>
  let randomNumber = Math.floor(Math.random() * 100) + 1;

  const guesses = document.querySelector('.guesses');
  const lastResult = document.querySelector('.lastResult');
  const lowOrHi = document.querySelector('.lowOrHi');
  const guessSubmit = document.querySelector('.guessSubmit');
  const guessField = document.querySelector('.guessField');

  let guessCount = 1;
  let resetButton;

  function checkGuess() {

    let userGuess = Number(guessField.value);
    if (guessCount === 1) {
      guesses.textContent = 'Previous guesses: ';
    }
    guesses.textContent += userGuess + ' ';

    if (userGuess === randomNumber) {
      lastResult.textContent = 'Congratulations! You got it right!';
      lastResult.style.backgroundColor = 'green';
      lowOrHi.textContent = '';
      setGameOver();
    } else if (guessCount === 10) {
      lastResult.textContent = '!!!GAME OVER!!!';
      setGameOver();
    } else {
      lastResult.textContent = 'Wrong!';
      lastResult.style.backgroundColor = 'red';
      if (userGuess < randomNumber) {
        lowOrHi.textContent = 'Last guess was too low!';
      } else if (userGuess > randomNumber) {
        lowOrHi.textContent = 'Last guess was too high!';
      }
    }

    guessCount++;
    guessField.value = '';
    guessField.focus();
  }
  guessSubmit.addEventListener('click', checkGuess);

  function setGameOver() {
    guessField.disabled = true;
    guessSubmit.disabled = true;
    resetButton = document.createElement('button');
    resetButton.textContent = 'Start new game';
    document.body.appendChild(resetButton);
    resetButton.addEventListener('click', resetGame);
  }

  function resetGame() {
    guessCount = 1;

    const resetParas = document.querySelectorAll('.resultParas p');
    for (let i = 0; i < resetParas.length; i++) {
      resetParas[i].textContent = '';
    }
    resetButton.parentNode.removeChild(resetButton);

    guessField.disabled = false;
    guessSubmit.disabled = false;
    guessField.value = '';
    guessField.focus();

    lastResult.style.backgroundColor = 'white';

    randomNumber = Math.floor(Math.random()) + 1;
  }
</script>

</html>
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/First_steps/What_went_wrong