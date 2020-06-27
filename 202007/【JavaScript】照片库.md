# 【JavaScript】照片库

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## html 模板

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">

    <title>照片库</title>

    <link rel="stylesheet" href="style.css">
    
  </head>

  <body>
    <h1>照片库示例</h1>

    <div class="full-img">
      <img class="displayed-img" src="images/pic1.jpg">
      <div class="overlay"></div>
      <button class="dark">变暗</button>
    </div>

    <div class="thumb-bar">


    </div>
    <script src="main.js"></script>
  </body>
</html>
```

## 样式表

```css
h1 {
  font-family: sans-serif;
  text-align: center;
}

body {
  width: 640px;
  margin: 0 auto;
}

.full-img {
  position: relative;
  display: block;
  width: 640px;
  height: 480px;
}

.overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 640px;
  height: 480px;
  background-color: rgba(0,0,0,0);
}

button {
  border: 0;
  background: rgba(150,150,150,0.6);
  text-shadow: 1px 1px 1px white;
  border: 1px solid #999;
  position: absolute;
  cursor: pointer;
  top: 2px;
  left: 2px;
}

.thumb-bar img {
  display: block;
  width: 20%;
  float: left;
  cursor: pointer;
}
```

## 定义变量

```js
const displayedImage = document.querySelector(".displayed-img");
const thumbBar = document.querySelector(".thumb-bar");

const btn = document.querySelector("button");
const overlay = document.querySelector(".overlay");
```

## 图片列表

```js
/* 添加图片循环 */
for (let i = 1; i < 6; i++) {
  const newImage = document.createElement("img");
  newImage.setAttribute("src", "images/pic" + i + ".jpg");
  thumbBar.appendChild(newImage);
}
```

## 切换图片

```js
/* 切换图片 */
thumbBar.addEventListener("click", function(event) {
  const src = event.target.getAttribute("src");
  displayedImage.setAttribute("src", src);
});
```

## 变亮和变暗

addEventListener 和 removeEventListener 可以增加和删除事件。

```js
/* 编写 变暗/变量 按钮功能 */
btn.addEventListener("click", function(event) {
  const color = event.target.getAttribute("class");
  if (color === "dark") {
    btn.setAttribute("class", "light");
    btn.textContent = "变亮";
    overlay.style.backgroundColor = "rgba(0, 0, 0, 0.5)";
  } else {
    btn.setAttribute("class", "dark");
    btn.textContent = "变暗";
    overlay.style.backgroundColor = "rgba(0, 0, 0, 0)";
  }
});
```

[1]: https://developer.mozilla.org/zh-CN/docs/learn/JavaScript/Building_blocks/%E7%9B%B8%E7%89%87%E8%B5%B0%E5%BB%8A