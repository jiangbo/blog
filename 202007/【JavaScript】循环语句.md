# 【JavaScript】循环语句

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## for 循环

```js
for (var i = 0; i < 10; i++) {
  console.log(i);
}
```

### break

如果要在所有迭代完成之前退出循环，可以使用 break 语句。

```js
for (var i = 0; i < 10; i++) {
  console.log(i);
  if(i === 5){
    break;
  }
}
```

### continue

continue 可以跳过一次循环。

```js
for (var i = 0; i < 10; i++) {
  console.log(i);
  if(i === 5){
    continue;
  }
}
```

## while 循环

### while 语句

```js
let a = 3;
while(a > 10){

  console.log(a++);
}
```

### do...while 语句

```js
let a = 3;
do{
 console.log(a++);
}while(a > 10);
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/Building_blocks/Looping_code