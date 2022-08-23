# 0073-Go-hello world

## 环境

- Time 2022-08-23
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/hello-world>

### 目标

使用 Go 语言打印 hello world。

## 初始化项目

`go mod init jiangbo/go`

## 打印 hello world

```go
package main

func main() {
    println("hello world")
}
```

## 直接运行

```text
PS C:\Users\JiangBo\work\workspace\vscode\go> go run main.go
hello world
```

## 编译运行

```text
PS C:\Users\JiangBo\work\workspace\vscode\go> go build main.go
PS C:\Users\JiangBo\work\workspace\vscode\go> .\main.exe
hello world
```

## 总结

使用 Go 语言打印 hello world。

## 附录
