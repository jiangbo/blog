# 【JavaScript】对象构建实践

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 概述

本文通过编写一个弹球 demo 来展示 JavaScript 中对象的重要性。
我们的小球会在屏幕上弹跳，当它们碰到彼此时会变色。

## 定义 html

```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <title>弹跳彩球</title>
  <link rel="stylesheet" href="style.css">
</head>

<body>
  <h1>弹跳彩球</h1>
  <canvas></canvas>

  <script src="main.js"></script>
</body>

</html>
```

## 定义样式表

```css
html, body {
  margin: 0;
}

html {
  font-family: sans-serif;
  height: 100%;
}

body {
  overflow: hidden;
  height: inherit;
}

h1 {
  font-size: 2rem;
  letter-spacing: -1px;
  position: absolute;
  margin: 0;
  top: -4px;
  right: 5px;

  color: transparent;
  text-shadow: 0 0 4px white;
}
```

## 定义画布

```js
// 设置画布

const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d");

const width = canvas.width = window.innerWidth;
const height = canvas.height = window.innerHeight;
```

## 随机值函数

```js
/**
 * 生成随机数的函数
 * @param {number} min 最小值
 * @param {number} max 最大值
 * @return {number} 随机值
 */
function random(min, max) {
  const num = Math.floor(Math.random() * (max - min)) + min;
  return num;
}
```

## 定义小球模型

```js
function Ball(x, y, velX, velY, color, size) {
  this.x = x;
  this.y = y;
  this.velX = velX;
  this.velY = velY;
  this.color = color;
  this.size = size;
}
```

## 画小球

```js
Ball.prototype.draw = function() {
  ctx.beginPath();
  ctx.fillStyle = this.color;
  ctx.arc(this.x, this.y, this.size, 0, 2 * Math.PI);
  ctx.fill();
}
```

## 更新小球位置

```js
Ball.prototype.update = function() {
  if ((this.x + this.size) >= width) {
    this.velX = -(this.velX);
  }

  if ((this.x - this.size) <= 0) {
    this.velX = -(this.velX);
  }

  if ((this.y + this.size) >= height) {
    this.velY = -(this.velY);
  }

  if ((this.y - this.size) <= 0) {
    this.velY = -(this.velY);
  }

  this.x += this.velX;
  this.y += this.velY;
}
```

## 生成多个小球

```js
let balls = [];

while (balls.length < 25) {
    let size = random(10, 20);
    let ball = new Ball(
      // 为避免绘制错误，球至少离画布边缘球本身一倍宽度的距离
      random(0 + size, width - size),
      random(0 + size, height - size),
      random(-7, 7),
      random(-7, 7),
      randomColor(),
      size
    );
    balls.push(ball);
  }
```

## 填充画布

```js
function loop() {
  ctx.fillStyle = 'rgba(0, 0, 0, 0.25)';
  ctx.fillRect(0, 0, width, height);

  for (let i = 0; i < balls.length; i++) {
    balls[i].draw();
    balls[i].update();
  }

  requestAnimationFrame(loop);
}
```

## 最后

```js
loop();

Ball.prototype.collisionDetect = function() {
  for (let j = 0; j < balls.length; j++) {
    if (this !== balls[j]) {
      const dx = this.x - balls[j].x;
      const dy = this.y - balls[j].y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      if (distance < this.size + balls[j].size) {
        balls[j].color = this.color = randomColor();
      }
    }
  }
}

balls[i].collisionDetect();
```


[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Objects/Object_building_practice