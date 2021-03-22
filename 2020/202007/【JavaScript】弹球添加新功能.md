# 【JavaScript】弹球添加新功能

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 概述

我们的弹球 demo 很有趣, 但是现在我们想让它更具有互动性，我们为它添加一个由玩家控制的“恶魔圈”，
如果恶魔圈抓到弹球会把它会吃掉。我们还想测验你面向对象的水平，首先创建一个通用 Shape() 对象，
然后由它派生出弹球和恶魔圈。最后，我们为 demo 添加一个计分器来记录剩下的球数。

## 定义 html

```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <title>Bouncing balls</title>
  <link rel="stylesheet" href="style.css">
</head>

<body>
  <h1>bouncing balls</h1>
  <p>Ball count: </p>
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
  font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
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

p {
  position: absolute;
  margin: 0;
  top: 35px;
  right: 5px;
  color: #aaa;
}
```

## 定义画布

```js
const para = document.querySelector("p");
let count = 0;

// setup canvas

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

## 定义形状模型

```js
// define shape constructor

function Shape(x, y, velX, velY, exists) {
  this.x = x;
  this.y = y;
  this.velX = velX;
  this.velY = velY;
  this.exists = exists;
}
```

## 定义小球模型

```js
// define Ball constructor, inheriting from Shape

function Ball(x, y, velX, velY, exists, color, size) {
  Shape.call(this, x, y, velX, velY, exists);

  this.color = color;
  this.size = size;
}

Ball.prototype = Object.create(Shape.prototype);
Ball.prototype.constructor = Ball;
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

## 小球碰撞检测

```js
// define ball collision detection

Ball.prototype.collisionDetect = function() {
  for (let j = 0; j < balls.length; j++) {
    if (!(this === balls[j])) {
      const dx = this.x - balls[j].x;
      const dy = this.y - balls[j].y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      if (distance < this.size + balls[j].size && balls[j].exists) {
        balls[j].color = this.color = "rgb(" + random(0, 255) + "," +
          random(0, 255) + "," + random(0, 255) + ")";
      }
    }
  }
};
```

## 定义恶魔圈

```js
// define EvilCircle constructor, inheriting from Shape

function EvilCircle(x, y, exists) {
  Shape.call(this, x, y, 20, 20, exists);

  this.color = "white";
  this.size = 10;
}

EvilCircle.prototype = Object.create(Shape.prototype);
EvilCircle.prototype.constructor = EvilCircle;
```

## 画恶魔圈

```js
// define EvilCircle draw method

EvilCircle.prototype.draw = function() {
  ctx.beginPath();
  ctx.strokeStyle = this.color;
  ctx.lineWidth = 3;
  ctx.arc(this.x, this.y, this.size, 0, 2 * Math.PI);
  ctx.stroke();
};
```

## 恶魔圈边界

```js
// define EvilCircle checkBounds method

EvilCircle.prototype.checkBounds = function() {
  if ((this.x + this.size) >= width) {
    this.x -= this.size;
  }

  if ((this.x - this.size) <= 0) {
    this.x += this.size;
  }

  if ((this.y + this.size) >= height) {
    this.y -= this.size;
  }

  if ((this.y - this.size) <= 0) {
    this.y += this.size;
  }
};
```

## 控制恶魔圈

```js
EvilCircle.prototype.setControls = function() {
  const _this = this;
  window.onkeydown = function(e) {
    if (e.key === "a") {
      _this.x -= _this.velX;
    } else if (e.key === "d") {
      _this.x += _this.velX;
    } else if (e.key === "w") {
      _this.y -= _this.velY;
    } else if (e.key === "s") {
      _this.y += _this.velY;
    }
  };
};
```

## 恶魔圈碰撞检测

```js
// define EvilCircle collision detection

EvilCircle.prototype.collisionDetect = function() {
  for (let j = 0; j < balls.length; j++) {
    if (balls[j].exists) {
      const dx = this.x - balls[j].x;
      const dy = this.y - balls[j].y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      if (distance < this.size + balls[j].size) {
        balls[j].exists = false;
        count--;
        para.textContent = "Ball count: " + count;
      }
    }
  }
};
```

## 生成多个小球

```js
// define array to store balls and populate it

const balls = [];

while (balls.length < 25) {
  const size = random(10, 20);
  const ball = new Ball(
    // ball position always drawn at least one ball width
    // away from the adge of the canvas, to avoid drawing errors
    random(0 + size, width - size),
    random(0 + size, height - size),
    random(-7, 7),
    random(-7, 7),
    true,
    "rgb(" + random(0, 255) + "," + random(0, 255) + "," + random(0, 255) + ")",
    size
  );
  balls.push(ball);
  count++;
  para.textContent = "Ball count: " + count;
}
```

## 最后

```js
// define loop that keeps drawing the scene constantly

const evil = new EvilCircle(random(0, width), random(0, height), true);
evil.setControls();

function loop() {
  ctx.fillStyle = "rgba(0,0,0,0.25)";
  ctx.fillRect(0, 0, width, height);

  for (let i = 0; i < balls.length; i++) {
    if (balls[i].exists) {
      balls[i].draw();
      balls[i].update();
      balls[i].collisionDetect();
    }
  }

  evil.draw();
  evil.checkBounds();
  evil.collisionDetect();

  requestAnimationFrame(loop);
}


loop();
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Objects/%E5%90%91%E2%80%9C%E5%BC%B9%E8%B7%B3%E7%90%83%E2%80%9D%E6%BC%94%E7%A4%BA%E7%A8%8B%E5%BA%8F%E6%B7%BB%E5%8A%A0%E6%96%B0%E5%8A%9F%E8%83%BD