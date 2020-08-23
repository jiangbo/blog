# 【JavaScript】BigInt

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

BigInt 是一种内置对象，它提供了一种方法来表示大于 2^53 - 1 的整数。这原本是 Javascript 中可以用 Number 表示的最大数字。BigInt 可以表示任意大的整数。

## 构造函数

可以用在一个整数字面量后面加 n 的方式定义一个 BigInt ，如：10n，或者调用函数 BigInt()。

```js
const theBiggestInt = 9007199254740991n;

const alsoHuge = BigInt(9007199254740991);
// ↪ 9007199254740991n

const hugeString = BigInt("9007199254740991");
// ↪ 9007199254740991n

const hugeHex = BigInt("0x1fffffffffffff");
// ↪ 9007199254740991n

const hugeBin = BigInt("0b11111111111111111111111111111111111111111111111111111");
// ↪ 9007199254740991n
```

## 类型信息

使用 typeof 测试时， BigInt 对象返回 "bigint" ：

```js
typeof 1n === 'bigint'; // true
typeof BigInt('1') === 'bigint'; // true
```

使用 Object 包装后， BigInt 被认为是一个普通 "object" ：

```js
typeof Object(1n) === 'object'; // true
```

## 静态方法

### asIntN

BigInt.asIntN 静态方法将 BigInt 值转换为一个 -2^width-1 与 2^(width-1)-1 之间的有符号整数。

```js
const max = 2n ** (64n - 1n) - 1n;

function check64bit(number) {
  (number > max) ?
    console.log('Number doesn\'t fit in signed 64-bit integer!') :
    console.log(BigInt.asIntN(64, number));
}

check64bit(2n ** 64n);
// expected output: "Number doesn't fit in signed 64-bit integer!"

check64bit(2n ** 32n);
// expected output: 4294967296n
```

### asUintN

BigInt.asUintN 静态方法将 BigInt 转换为一个 0 和 2width-1 之间的无符号整数。

```js
const max = 2n ** 64n - 1n;

function check64bit(number) {
  (number > max) ?
    console.log('Number doesn\'t fit in unsigned 64-bit integer!') :
    console.log(BigInt.asUintN(64, number));
}

check64bit(2n ** 64n);
// expected output: "Number doesn't fit in unsigned 64-bit integer!"

check64bit(2n ** 32n);
// expected output: 4294967296n
```

## 实例方法

BigInt.prototype.toLocaleString()：返回此数字的 language-sensitive 形式的字符串。覆盖 Object.prototype.toLocaleString() 方法。
BigInt.prototype.toString()：返回以指定基数(base)表示指定数字的字符串。覆盖 Object.prototype.toString() 方法。
BigInt.prototype.valueOf()：返回指定对象的基元值。 覆盖 Object.prototype.valueOf() 方法。

## 使用建议

### 转化

由于在 Number 与 BigInt 之间进行转换会损失精度，因而建议仅在值可能大于253 时使用 BigInt 类型，并且不在两种类型之间进行相互转换。

### 密码学

由于对 BigInt 的操作不是常数时间的，因而 BigInt 不适合用于密码学。

### 在 JSON 中使用

对任何 BigInt 值使用 JSON.stringify() 都会引发 TypeError，因为默认情况下 BigInt 值不会在 JSON 中序列化。但是，如果需要，可以实现 toJSON 方法：

```js
BigInt.prototype.toJSON = function() { return this.toString(); }
```

JSON.stringify 现在生成如下字符串，而不是抛出异常:

```js
JSON.stringify(BigInt(1));
// '"1"'
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/BigInt

