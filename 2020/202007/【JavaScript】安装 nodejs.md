# 【JavaScript】安装 nodejs

## 环境

- windows 10 2004 版本

## 下载

可以到官方网站 https://nodejs.org/en/ 下在 nodejs，下载完成后，双击安装。
安装完成后，在命令行窗口输入 `node -v` 可以查看 node 的版本号。

```powershell
PS C:\Users\JiangBo> node -v
v12.18.1
```

## 包管理器

下载的 node 中自带了 JavaScript 的包管理器 npm，可以输入 `npm -v` 查看版本信息。

```powershell
PS C:\Users\JiangBo> npm -v
6.14.5
```

## 切换仓库

因为默认的仓库是在国外，下载很慢，所以可以切换到国内的仓库上。

首先可以安装 nrm ：

```powershell
npm i -g nrm --registry https://registry.npm.taobao.org
```

因为全局安装了 nrm，所以可以使用 nrm 命令，使用 nrm use 切换使用的仓库，如：

`nrm use taobao`

```powershell
PS C:\Users\JiangBo> nrm ls

* npm -------- https://registry.npmjs.org/
  yarn ------- https://registry.yarnpkg.com/
  cnpm ------- http://r.cnpmjs.org/
  taobao ----- https://registry.npm.taobao.org/
  nj --------- https://registry.nodejitsu.com/
  npmMirror -- https://skimdb.npmjs.com/registry/
  edunpm ----- http://registry.enpmjs.org/

PS C:\Users\JiangBo> nrm use taobao


   Registry has been set to: https://registry.npm.taobao.org/
```

## node 运行 js

之前的 js 程序都是在浏览器中运行了，使用 node 也可以运行 js 代码。

新建 test.js 文件：

```js
console.log("hello node");
```

使用 node 运行： `node test.js`。
