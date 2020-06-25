# 【JavaScript】笑话生成器

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 新建 html 文档

```html
<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1">
  <meta name="viewport" content="width=device-width">

  <title>笑话机</title>

  <link href="style.css" rel="stylesheet">
</head>

<body>
  <div>
    <label for="customname">输入自定义名字：</label>
    <input id="customname" type="text" placeholder="李雷">
  </div>
  <div>
    <button class="randomize">生成随机笑话</button>
  </div>
  <!-- 鸣谢：Willy Aguirre 提供的测试代码 -->
  <p class="story"></p>
  <script src="main.js"></script>
</body>

</html>
```

### 定义样式表

```css
body {
  font-family: sans-serif;
  width: 350px;
}

label {
  font-weight: bold;  
}

div {
  padding-bottom: 20px;
}

input[type="text"] {
  padding: 5px;
  width: 150px;
}

p {
  background: #FFC125;
  color: #5E2612;
  padding: 10px;
  visibility: hidden;
}
```

## 生成笑话

新建一个 main.js 文件，需要在 html 文档中引入。

### 选择页面元素

```js
const customName = document.getElementById("customname");
const randomize = document.querySelector(".randomize");
const story = document.querySelector(".story");
```

## 获取数组随机元素

```js
function randomValueFromArray(array) {
  return array[Math.floor(Math.random() * array.length)];
}
```

## 定义基础变量

```js
const storyText = "今天气温 34 摄氏度，:inserta:出去遛弯。当走到:insertb:门前时，突然就:insertc:。" +
  "人们都惊呆了，李雷全程目睹但并没有慌，因为:inserta:是一个 130 公斤的胖子，天气又辣么热。";

const insertX = ["怪兽威利", "大老爹", "圣诞老人"];
const insertY = ["肯德基", "迪士尼乐园", "白宫"];
const insertZ = ["自燃了", "在人行道化成了一坨泥", "变成一条鼻涕虫爬走了"];
```

## 绑定点击事件

```js
randomize.addEventListener("click", result);
```

## 实现笑话生成

```js
function result() {
  const xItem = randomValueFromArray(insertX);
  const yItem = randomValueFromArray(insertY);
  const zItem = randomValueFromArray(insertZ);

  let newStory = storyText.replace(":inserta:", xItem).replace(":inserta:", xItem)
    .replace(":insertb:", yItem).replace(":insertc:", zItem);

  if (customName.value !== "") {
    const name = customName.value;
    newStory = newStory.replace("李雷", name);
  }

  story.textContent = newStory;
  story.style.visibility = "visible";
}
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/First_steps/Silly_story_generator