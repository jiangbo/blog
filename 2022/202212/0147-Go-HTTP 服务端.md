# 0147-Go-HTTP 服务端

## 环境

- Time 2022-08-25
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/http-servers>

### 目标

使用 Go 语言 HTTP 服务端。

## 示例

```go
package main

import (
    "fmt"
    "net/http"
)

func hello(w http.ResponseWriter, req *http.Request) {

    fmt.Fprintf(w, "hello\n")
}

func headers(w http.ResponseWriter, req *http.Request) {

    for name, headers := range req.Header {
        for _, h := range headers {
            fmt.Fprintf(w, "%v: %v\n", name, h)
        }
    }
}

func main() {

    http.HandleFunc("/hello", hello)
    http.HandleFunc("/headers", headers)

    http.ListenAndServe(":8090", nil)
}
```

## 总结

使用 Go 语言 HTTP 服务端。

## 附录
