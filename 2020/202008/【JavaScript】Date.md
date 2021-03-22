# 【JavaScript】Date

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

创建一个 JavaScript Date 实例，该实例呈现时间中的某个时刻。Date 对象则基于 Unix Time Stamp，即自 1970 年 1 月 1 日（UTC）起经过的毫秒数。

## 构造函数

创建一个新Date对象的唯一方法是通过new 操作符，例如：let now = new Date();
若将它作为常规函数调用（即不加 new 操作符），将返回一个字符串，而非 Date 对象。

```js
const date1 = new Date('December 17, 1995 03:24:00');
// Sun Dec 17 1995 03:24:00 GMT...

const date2 = new Date('1995-12-17T03:24:00');
// Sun Dec 17 1995 03:24:00 GMT...

console.log(date1 === date2);
// expected output: false;

console.log(date1 - date2);
// expected output: 0
```

## 静态方法

### now

返回自 1970-1-1 00:00:00  UTC（世界标准时间）至今所经过的毫秒数。

```js
// this example takes 2 seconds to run
const start = Date.now();

console.log('starting timer...');
// expected output: starting timer...

setTimeout(() => {
  const millis = Date.now() - start;

  console.log(`seconds elapsed = ${Math.floor(millis / 1000)}`);
  // expected output : seconds elapsed = 2
}, 2000);
```

### parse

解析一个表示日期的字符串，并返回从 1970-1-1 00:00:00 所经过的毫秒数。
注意: 由于浏览器差异和不一致，强烈建议不要使用 Date.parse 解析字符串。

### UTC

接受和构造函数最长形式的参数相同的参数（从2到7），并返回从 1970-01-01 00:00:00 UTC 开始所经过的毫秒数。

```js
const utcDate1 = new Date(Date.UTC(96, 1, 2, 3, 4, 5));
const utcDate2 = new Date(Date.UTC(0, 0, 0, 0, 0, 0));

console.log(utcDate1.toUTCString());
// expected output: Fri, 02 Feb 1996 03:04:05 GMT

console.log(utcDate2.toUTCString());
// expected output: Sun, 31 Dec 1899 00:00:00 GMT
```

## 实例方法

### Getter

* Date.prototype.getDate()：根据本地时间返回指定日期对象的月份中的第几天（1-31）。
* Date.prototype.getDay()：根据本地时间返回指定日期对象的星期中的第几天（0-6）。
* Date.prototype.getFullYear()：根据本地时间返回指定日期对象的年份（四位数年份时返回四位数字）。
* Date.prototype.getHours()：根据本地时间返回指定日期对象的小时（0-23）。
* Date.prototype.getMilliseconds()：根据本地时间返回指定日期对象的毫秒（0-999）。
* Date.prototype.getMinutes()：根据本地时间返回指定日期对象的分钟（0-59）。
* Date.prototype.getMonth()：根据本地时间返回指定日期对象的月份（0-11）。
* Date.prototype.getSeconds()：根据本地时间返回指定日期对象的秒数（0-59）。
* Date.prototype.getTime()：返回从1970-1-1 00:00:00 UTC（协调世界时）到该日期经过的毫秒数，对于1970-1-1 00:00:00 UTC之前的时间返回负值。
* Date.prototype.getTimezoneOffset()：返回当前时区的时区偏移。
* Date.prototype.getUTCDate()：根据世界时返回特定日期对象一个月的第几天（1-31）.
* Date.prototype.getUTCDay()：根据世界时返回特定日期对象一个星期的第几天（0-6）.
* Date.prototype.getUTCFullYear()：根据世界时返回特定日期对象所在的年份（4位数）.
* Date.prototype.getUTCHours()：根据世界时返回特定日期对象当前的小时（0-23）.
* Date.prototype.getUTCMilliseconds()：根据世界时返回特定日期对象的毫秒数（0-999）.
* Date.prototype.getUTCMinutes()：根据世界时返回特定日期对象的分钟数（0-59）.
* Date.prototype.getUTCMonth()：根据世界时返回特定日期对象的月份（0-11）.
* Date.prototype.getUTCSeconds()：根据世界时返回特定日期对象的秒数（0-59）.
* Date.prototype.getYear()：根据特定日期返回年份 (通常 2-3 位数). 使用 getFullYear() .

### Setter

* Date.prototype.setDate()：根据本地时间为指定的日期对象设置月份中的第几天。
* Date.prototype.setFullYear()：根据本地时间为指定日期对象设置完整年份（四位数年份是四个数字）。
* Date.prototype.setHours()：根据本地时间为指定日期对象设置小时数。
* Date.prototype.setMilliseconds()：根据本地时间为指定日期对象设置毫秒数。
* Date.prototype.setMinutes()：根据本地时间为指定日期对象设置分钟数。
* Date.prototype.setMonth()：根据本地时间为指定日期对象设置月份。
* Date.prototype.setSeconds()：根据本地时间为指定日期对象设置秒数。
* Date.prototype.setTime()：通过指定从 1970-1-1 00:00:00 UTC 开始经过的毫秒数来设置日期对象的时间，对于早于 1970-1-1 00:00:00 UTC的时间可使用负值。
* Date.prototype.setUTCDate()：根据世界时设置 Date 对象中月份的一天 (1 ~ 31)。
* Date.prototype.setUTCFullYear()：根据世界时设置 Date 对象中的年份（四位数字）。
* Date.prototype.setUTCHours()：根据世界时设置 Date 对象中的小时 (0 ~ 23)。
* Date.prototype.setUTCMilliseconds()：根据世界时设置 Date 对象中的毫秒 (0 ~ 999)。
* Date.prototype.setUTCMinutes()：根据世界时设置 Date 对象中的分钟 (0 ~ 59)。
* Date.prototype.setUTCMonth()：根据世界时设置 Date 对象中的月份 (0 ~ 11)。
* Date.prototype.setUTCSeconds()：根据世界时设置 Date 对象中的秒钟 (0 ~ 59)。
* Date.prototype.setYear()：setYear() 方法用于设置年份。请使用 setFullYear() 方法代替。

### Conversion getter

* Date.prototype.toDateString()：以人类易读（human-readable）的形式返回该日期对象日期部分的字符串。
* Date.prototype.toISOString()：把一个日期转换为符合 ISO 8601 扩展格式的字符串。
* Date.prototype.toJSON()：使用 toISOString() 返回一个表示该日期的字符串。为了在 JSON.stringify() 方法中使用。
* Date.prototype.toGMTString() ：返回一个基于 GMT (UT) 时区的字符串来表示该日期。请使用 toUTCString() 方法代替。
* Date.prototype.toLocaleDateString()：返回一个表示该日期对象日期部分的字符串，该字符串格式与系统设置的地区关联（locality sensitive）。
* Date.prototype.toLocaleFormat() ：使用格式字符串将日期转换为字符串。
* Date.prototype.toLocaleString()：返回一个表示该日期对象的字符串，该字符串与系统设置的地区关联（locality sensitive）。覆盖了 Object.prototype.toLocaleString() 方法。
* Date.prototype.toLocaleTimeString()：返回一个表示该日期对象时间部分的字符串，该字符串格式与系统设置的地区关联（locality sensitive）。
* Date.prototype.toSource()：返回一个与Date等价的原始字符串对象，你可以使用这个值去生成一个新的对象。重写了 Object.prototype.toSource() 这个方法。
* Date.prototype.toString()：返回一个表示该日期对象的字符串。覆盖了Object.prototype.toString() 方法。
* Date.prototype.toTimeString()：以人类易读格式返回日期对象时间部分的字符串。
* Date.prototype.toUTCString()：把一个日期对象转换为一个以UTC时区计时的字符串。
* Date.prototype.valueOf()：返回一个日期对象的原始值。覆盖了 Object.prototype.valueOf() 方法。

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Date
