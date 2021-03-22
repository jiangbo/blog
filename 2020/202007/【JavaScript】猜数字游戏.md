# 【JavaScript】猜数字游戏

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 示例

以下内容为了解和熟悉 JavaScript，不需要理解，能够复制粘贴并正确运行即可。

### 新建网页模板

在 vscode 中新建一个 index.html 文件，然后输入以下内容。

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
      <label for="guessField">请猜数: </label>
      <input type="text" id="guessField" class="guessField">
      <input type="submit" value="确定" class="guessSubmit">
    </div>

    <div class="resultParas">
      <p class="guesses"></p>
      <p class="lastResult"></p>
      <p class="lowOrHi"></p>
    </div>

    <script>

      // 开始编写 JavaScript 代码

    </script>
  </body>
</html>
```

### 定义变量

首先将需要使用的变量都定义出来。

```js
let randomNumber = Math.floor(Math.random() * 100) + 1;

const guesses = document.querySelector('.guesses');
const lastResult = document.querySelector('.lastResult');
const lowOrHi = document.querySelector('.lowOrHi');

const guessSubmit = document.querySelector('.guessSubmit');
const guessField = document.querySelector('.guessField');

let guessCount = 1;
let resetButton;
```

### 定义猜数字主逻辑

```js
function checkGuess() {
  let userGuess = Number(guessField.value);
  if (guessCount === 1) {
    guesses.textContent = '上次猜的数：';
  }
  guesses.textContent += userGuess + ' ';
 
  if (userGuess === randomNumber) {
    lastResult.textContent = '恭喜你！猜对了';
    lastResult.style.backgroundColor = 'green';
    lowOrHi.textContent = '';
    setGameOver();
  } else if (guessCount === 10) {
    lastResult.textContent = '!!!GAME OVER!!!';
    setGameOver();
  } else {
    lastResult.textContent = '你猜错了！';
    lastResult.style.backgroundColor = 'red';
    if(userGuess < randomNumber) {
      lowOrHi.textContent = '你猜低了！';
    } else if(userGuess > randomNumber) {
      lowOrHi.textContent = '你猜高了';
    }
  }
 
  guessCount++;
  guessField.value = '';
  guessField.focus();
}
```

### 结束游戏

```js
function setGameOver() {
  guessField.disabled = true;
  guessSubmit.disabled = true;
  resetButton = document.createElement('button');
  resetButton.textContent = '开始新游戏';
  document.body.appendChild(resetButton);
  resetButton.addEventListener('click', resetGame);
}
```

### 重置游戏

```js
function resetGame() {
  guessCount = 1;

  const resetParas = document.querySelectorAll('.resultParas p');
  for (let i = 0 ; i < resetParas.length; i++) {
    resetParas[i].textContent = '';
  }

  resetButton.parentNode.removeChild(resetButton);

  guessField.disabled = false;
  guessSubmit.disabled = false;
  guessField.value = '';
  guessField.focus();

  lastResult.style.backgroundColor = 'white';

  randomNumber = Math.floor(Math.random() * 100) + 1;
}
```

### 浏览器打开

用浏览器打开刚刚编写的程序，试试游戏是否能够正常运行。

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
    <label for="guessField">请猜数: </label>
    <input type="text" id="guessField" class="guessField">
    <input type="submit" value="确定" class="guessSubmit">
  </div>

  <div class="resultParas">
    <p class="guesses"></p>
    <p class="lastResult"></p>
    <p class="lowOrHi"></p>
  </div>

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
        guesses.textContent = '上次猜的数：';
      }
      guesses.textContent += userGuess + ' ';

      if (userGuess === randomNumber) {
        lastResult.textContent = '恭喜你！猜对了';
        lastResult.style.backgroundColor = 'green';
        lowOrHi.textContent = '';
        setGameOver();
      } else if (guessCount === 10) {
        lastResult.textContent = '!!!GAME OVER!!!';
        setGameOver();
      } else {
        lastResult.textContent = '你猜错了！';
        lastResult.style.backgroundColor = 'red';
        if (userGuess < randomNumber) {
          lowOrHi.textContent = '你猜低了！';
        } else if (userGuess > randomNumber) {
          lowOrHi.textContent = '你猜高了';
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
      resetButton.textContent = '开始新游戏';
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

      randomNumber = Math.floor(Math.random() * 100) + 1;
    }

  </script>
</body>

</html>
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/First_steps/Silly_story_generator