# 0146-Go-HTTP 客户端

## 环境

- Time 2022-08-25
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/http-clients>

### 目标

使用 Go 语言的 HTTP 客户端。

## 示例

```go
package main

import (
    "bufio"
    "fmt"
    "net/http"
)

func main() {

    resp, err := http.Get("https://gobyexample.com")
    if err != nil {
        panic(err)
    }
    defer resp.Body.Close()

    fmt.Println("Response status:", resp.Status)

    scanner := bufio.NewScanner(resp.Body)
    for i := 0; scanner.Scan() && i < 5; i++ {
        fmt.Println(scanner.Text())
    }

    if err := scanner.Err(); err != nil {
        panic(err)
    }
}
```

## 总结

使用 Go 语言的 HTTP 客户端。

## 附录
