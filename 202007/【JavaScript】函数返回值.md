# 【JavaScript】函数返回值

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 概念

返回值意如其名，是指函数执行完毕后返回的值。
有些函数没有返回值就像（返回值在这种情况下被列出为空值 void 或未定义值 undefined）。

```js
var newString = myText.replace('string', 'sausage');
```

## 返回值

```js
function randomNumber(number) {
  return Math.floor(Math.random()*number);
}
```

## 示例

### html 模板

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Function library example</title>
  <style>
    input {
      font-size: 2em;
      margin: 10px 1px 0;
    }
  </style>
</head>
<body>

  <input class="numberInput" type="text">
  <p></p>

  <script>
    const input = document.querySelector('.numberInput');
    const para = document.querySelector('p');

  </script>
</body>
</html>
```

### 定义函数

```js
function squared(num) {
  return num * num;
}

function cubed(num) {
  return num * num * num;
}

function factorial(num) {
  var x = num;
  while (x > 1) {
    num *= x-1;
    x--;
  }
  return num;
}
```

### 定义事件

```js
input.onchange = function() {
  var num = input.value;
  if (isNaN(num)) {
    para.textContent = 'You need to enter a number!';
  } else {
    para.textContent = num + ' squared is ' + squared(num) + '. ' +
                       num + ' cubed is ' + cubed(num) + '. ' +
                       num + ' factorial is ' + factorial(num) + '.';
  }
}
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Building_blocks/Return_values