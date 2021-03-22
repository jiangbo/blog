# 【JavaScript】数组

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- vscode 1.46
- Microsoft Edge 83

## 是什么

数组通常被描述为“像列表一样的对象”; 简单来说，数组是一个包含了多个值的对象。
数组对象可以存储在变量中，并且能用和其他任何类型的值完全相同的方式处理，
区别在于我们可以单独访问列表中的每个值，并使用列表执行一些有用和高效的操作，
如循环：它对数组中的每个元素都执行相同的操作。

也许我们有一系列产品和价格存储在一个数组中，我们想循环遍历所有这些产品，
并将它们打印在发票上，同时将所有产品的价格统计在一起，然后将总价格打印在底部。

### 创建数组

数组由方括号构成，其中包含用逗号分隔的元素列表。

```js
let shopping = ['bread', 'milk', 'cheese', 'hummus', 'noodles'];
console.log(shopping);
let random = ['tree', 795, [0, 1, 2]];
```

## 访问数组

可以使用括号表示法访问数组中的元素。

```js
let shopping = ['bread', 'milk', 'cheese', 'hummus', 'noodles'];
console.log(shopping[0],shopping[3]);
```

## 修改数组元素

字符串方法 toLowerCase 和 toUpperCase 字符串并将所有字符分别转换为小写或大写。

```js
let shopping = ['bread', 'milk', 'cheese', 'hummus', 'noodles'];
shopping[0] = 'jiangbo';
console.log(shopping[0]);
```

## 获取数组长度

和字符串的 length 属性一样，数组也可以通过它来获取数组的长度。

```js
let shopping = ['bread', 'milk', 'cheese', 'hummus', 'noodles'];
console.log(shopping.length);
```

## 字符串转数组

可以使用字符串的 spilt 方法将字符串转为数组。这个方法可以接受一个参数，表示按什么分割字符串。

```js
let myData = 'Manchester,London,Liverpool,Birmingham,Leeds,Carlisle';
console.log(myData.spilt(','));
```

## 数组转字符串

可以使用 join 和 toString 方法将数组转为字符串。
toString 有限制，直接输出固定格式，而 join 可以接受一个参数，表示按什么连接数组的元素。

```js
let dogNames = ["Rocket","Flash","Bella","Slugger"];
dogNames.toString(); //Rocket,Flash,Bella,Slugger
dogName.join('|');
```

## 添加数组元素

可以从数组头或者尾部添加数组元素，从头添加为 unshift，从尾部添加为 push。

```js
let name = ['jiangbo','bo'];
name.push('age');
name.unshift('name');
console.log(name);
```

## 删除数组元素

shift 从数组的头部开始删除元素，pop 从数组的尾部删除元素。

```js
let name = ['name','jiangbo','bo','age'];
name.shift();
name.pop();
console.log(name);
```

[1]: https://developer.mozilla.org/zh-CN/docs/Learn/JavaScript/First_steps/Arrays

