# 【JavaScript】函数—可重用的代码块

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 展示信息的方法

```js
function displayMessage() {
 
}
```

## html 模板

```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <title>Function stage 4</title>
</head>

<body>
  <button>Display message box</button>

  <script>

  </script>
</body>

</html>
```

## css 样式

```css
  <style>
    .msgBox {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 242px;
      border-radius: 10px;
      background-color: #eee;
      background-image: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.1));
    }

    .msgBox p {
      line-height: 1.5;
      padding: 10px 20px;
      color: #333;
      padding-left: 82px;
      background-position: 25px center;
      background-repeat: no-repeat;
    }

    .msgBox button {
      background: none;
      border: none;
      position: absolute;
      top: 0;
      right: 0;
      font-size: 1.1rem;
      color: #aaa;
    }
  </style>
```

## js 逻辑

```js
    const btn = document.querySelector('button');

    btn.onclick = function () {
      displayMessage('Brian: Hi there, how are you today?', 'chat');
    };

    function displayMessage(msgText, msgType) {
      const html = document.querySelector('html');

      const panel = document.createElement('div');
      panel.setAttribute('class', 'msgBox');
      html.appendChild(panel);

      const msg = document.createElement('p');
      msg.textContent = msgText;
      panel.appendChild(msg);

      const closeBtn = document.createElement('button');
      closeBtn.textContent = 'x';
      panel.appendChild(closeBtn);

      closeBtn.onclick = function () {
        panel.parentNode.removeChild(panel);
      }

      if (msgType === 'warning') {
        msg.style.backgroundImage = 'url(icons/warning.png)';
        panel.style.backgroundColor = 'red';
      } else if (msgType === 'chat') {
        msg.style.backgroundImage = 'url(icons/chat.png)';
        panel.style.backgroundColor = 'aqua';
      } else {
        msg.style.paddingLeft = '20px';
      }
    }
```

如果想获取图片和源码，可以点击[这里][2]。

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Building_blocks/Build_your_own_function
[2]: https://github.com/mdn/learning-area/blob/master/javascript/building-blocks/functions/function-stage-4.html