# 【JavaScript】import

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

静态的 import 语句用于导入由另一个模块导出的绑定。无论是否声明了 strict mode，导入的模块都运行在严格模式下。在浏览器中，import 语句只能在声明了 type="module" 的 script 的标签中使用。

此外，还有一个类似函数的动态 import()，它不需要依赖 type="module" 的 script 标签。

在 script 标签中使用 nomodule 属性，可以确保向后兼容。

在您希望按照一定的条件或者按需加载模块的时候，动态 import() 是非常有用的。而静态型的 import 是初始化加载依赖项的最优选择，使用静态 import 更容易从代码静态分析工具和 tree shaking 中受益。

## 导入整个模块的内容

这将myModule插入当前作用域，其中包含来自位于 /modules/my-module.js 文件中导出的所有接口。

```js
import * as myModule from '/modules/my-module.js';
```

## 导入单个接口

给定一个名为 myExport 的对象或值，它已经从模块 my-module 导出（因为整个模块被导出）或显式地导出（使用 export 语句），将 myExport 插入当前作用域。

```js
import {myExport} from '/modules/my-module.js';
```

## 导入多个接口

这将 foo 和 bar 插入当前作用域。

```js
import {foo, bar} from '/modules/my-module.js';
```

## 导入带有别名的接口

你可以在导入时重命名接口。例如，将 shortName 插入当前作用域。

```js
import {reallyReallyLongModuleExportName as shortName}
  from '/modules/my-module.js';
```

## 导入时重命名多个接口

使用别名导入模块的多个接口。

```js
import {
  reallyReallyLongModuleMemberName as shortName, 
  anotherLongModuleName as short
} from '/modules/my-module.js';
```

## 仅为副作用而导入一个模块

整个模块仅为副作用（中性词，无贬义含义）而导入，而不导入模块中的任何内容（接口）。这将运行模块中的全局代码, 但实际上不导入任何值。

```js
import '/modules/my-module.js';
```

## 导入默认值

引入模块可能有一个 default export（无论它是对象，函数，类等）可用。然后可以使用 import 语句来导入这样的默认接口。

最简单的用法是直接导入默认值：

```js
import myDefault from '/modules/my-module.js';
```

也可以同时将 default 语法与上述用法（命名空间导入或命名导入）一起使用。在这种情况下，default 导入必须首先声明。 例如：

```js
import myDefault, * as myModule from '/modules/my-module.js';
// myModule used as a namespace
```

或者

```js
import myDefault, {foo, bar} from '/modules/my-module.js';
// specific, named imports
```

## 动态 import

关键字 import 可以像调用函数一样来动态的导入模块。以这种方式调用，将返回一个 promise。

```js
import('/modules/my-module.js')
  .then((module) => {
    // Do something with the module.
  });
```

这种使用方式也支持 await 关键字。

```js
let module = await import('/modules/my-module.js');
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/import
