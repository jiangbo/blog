# Markdown：基础操作

## 前言

本文介绍 Markdown 最基本的语法和最简单的使用。其中包括标题、字体、引用、代码段、链接和列表等。本文所有的内容参考：

1. [basic-writing-and-formatting-syntax][1]。

## 标题

为了创建一个标题，可以在标题文字前添加一个到六个 `#` 符号， `#` 的个数决定了标题的大小。

```Markdown
# 一级标题
## 二级标题
### 三级标题
#### 四级标题
##### 五级标题
###### 六级标题
```

## 字体

### 加粗

加粗字体可以使用 `** **`或者 `__ __`。

```markdown
This is **bold** text
```

This is **bold** text

### 斜体

斜体字可以使用 `* *`或者 `_ _`。

```markdown
This is *text*
```

This is *text*

### 中划线

斜体字可以使用 `~~ ~~`。

```markdown
~~This was mistaken text~~
```

~~This was mistaken text~~

### 加粗+斜体

斜体字可以使用 `** **`和 `_ _`。

```markdown
**This text is _extremely_ important**
```

**This text is _extremely_ important**

## 引用

### 文字

```markdown
这是一句名言：
>学而不思则罔，思而不学则殆。
```

这是一句名言：
>学而不思则罔，思而不学则殆。

### 代码

#### 命令

```markdown
使用 `javac` 编译代码。
```

使用 `javac` 编译代码。

#### 代码行

\```java  
public static void main(String[] args) {  
 &nbsp;&nbsp;&nbsp;&nbsp;System.out.println("Hello world!");  
}  
\```

```java
public static void main(String[] args) {
  System.out.println("Hello world!");
}
```

## 链接

### 内部

```markdown
这是[我的主页](https://jiangbo920827.github.io/ "主页")
```

这是[我的主页](https://jiangbo920827.github.io/ "主页")

### 外部

```markdown
这是[我的主页][2]

[2]:https://jiangbo920827.github.io/ "主页"
```

这是[我的主页][2]

[2]:https://jiangbo920827.github.io/ "主页"

## 列表

### 无序

可以使用 `-` 或者 `*` 来创建一个无序列表。

```markdown
- C++
- Python
- Java
```

- C++
- Python
- Java

### 有序

```markdown
1. C++
2. Python
3. Java
```

1. C++
2. Python
3. Java

### 多级

在列表的前面，加上两个空格，可以实现多级列表。

```markdown
1. C++
2. Python
3. Java
   - Maven
   - Spring
   - jvm
```

1. C++
2. Python
3. Java
    - Maven
    - Spring
    - jvm

## 分段

可以使用一个空行来进行分段。

## 忽略

可以使用 `\` 来让 GitHub 忽略 Markdown 格式的解析。

```markdown
\*这不是斜体\*
```

\*这不是斜体\*

## 表格

### 居左

```markdown
|序号|语言|
|:---|:---|
|1|java|
|2|c|
```

|序号|语言|
|:---|:---|
|1|java|
|2|c|

### 居中

```markdown
|序号|语言|
|:---:|:---:|
|1|java|
|2|c|
```

|序号|语言|
|:---:|:---:|
|1|java|
|2|c|

### 居右

```markdown
|序号|语言|
|---:|---:|
|1|java|
|2|c|
```

|序号|语言|
|---:|---:|
|1|java|
|2|c|

[1]:https://help.github.com/articles/basic-writing-and-formatting-syntax/ "GitHub 基础语法"
