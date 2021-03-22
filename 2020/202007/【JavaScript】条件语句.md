# 【JavaScript】条件语句

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 条件判断语句

人类（以及其他的动物）无时无刻不在做决定，这些决定都影响着他们的生活，从小事（“我应该吃一片还是两片饼干”）到重要的大事（“我应该留在我的祖国，在我父亲的农场工作；还是应该去美国学习天体物理学”）。

### if 语句

```js
let a = 3;
if(a === 3){
  console.log("相等");
}
```

### if...else 语句

```js
let a = 3;
if(a === 3){
  console.log("相等");
}else{
  console.log("不相等");
}
```

### if...esle if 语句

```js
let a = 3;
if(a === 3){
  console.log("等于三");
}else if(a === 4){
  console.log("等于四");
}
```

## 逻辑运算符

逻辑与（&&）、逻辑或（||）、非（!），逻辑与表示两个都为真才为真，逻辑或有一个为真就为真，非是取反。

```js
let a = 3;
if(a === 3 || a === 4){
  console.log("等于三或者等于四");
}
```

## switch 语句

if...else 语句能够很好地实现条件代码，但是它们不是没有缺点，它们主要适用于您只有几个选择的情况。
对于只想将变量设置一系列为特定值的选项或根据条件打印特定语句的情况，
语法可能会很麻烦，特别是如果您有大量选择。

```js
let choice = 'snowing';
switch (choice) {
  case 'sunny':
    para.textContent = 'It is nice and sunny outside today. Wear shorts! Go to the beach, or the park, and get an ice cream.';
    break;
  case 'rainy':
    para.textContent = 'Rain is falling outside; take a rain coat and a brolly, and don\'t stay out for too long.';
    break;
  case 'snowing':
    para.textContent = 'The snow is coming down — it is freezing! Best to stay in with a cup of hot chocolate, or go build a snowman.';
    break;
  case 'overcast':
    para.textContent = 'It isn\'t raining, but the sky is grey and gloomy; it could turn any minute, so take a rain coat just in case.';
    break;
  default:
    para.textContent = '';
}
```

## 三元运算符

```js
let a = 3;
let result = a === 3? "相等":"不相等";
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Building_blocks/conditionals