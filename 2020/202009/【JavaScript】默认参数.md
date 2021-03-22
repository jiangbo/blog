# 【JavaScript】默认参数

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

函数默认参数允许在没有值或 undefined 被传入时使用默认形参。

## 示例

```js
function multiply(a, b = 1) {
  return a * b;
}

console.log(multiply(5, 2));
// expected output: 10

console.log(multiply(5));
// expected output: 5
```

## 传入 undefined vs 其他假值

```js
function test(num = 1) {
  console.log(typeof num);
}

test();          // 'number' (num is set to 1)
test(undefined); // 'number' (num is set to 1 too)

// test with other falsy values:
test('');        // 'string' (num is set to '')
test(null);      // 'object' (num is set to null)
```

## 调用时解析

```js
function append(value, array = []) {
  array.push(value);
  return array;
}

append(1); //[1]
append(2); //[2], not [1, 2]
```

## 默认参数可用于后面的默认参数

```js
function greet(name, greeting, message = greeting + ' ' + name) {
    return [name, greeting, message];
}

greet('David', 'Hi');  // ["David", "Hi", "Hi David"]
greet('David', 'Hi', 'Happy Birthday!');  // ["David", "Hi", "Happy Birthday!"]
```

## 位于默认参数之后非默认参数

```js
function f(x = 1, y) { 
  return [x, y]; 
}

f(); // [1, undefined]
f(2); // [2, undefined]
```

## 有默认值的解构参数

```js
function f([x, y] = [1, 2], {z: z} = {z: 3}) { 
  return x + y + z; 
}

f(); // 6
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Functions/Default_parameters
