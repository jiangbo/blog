# 0083-Go-range 遍历

## 环境

- Time 2022-08-23
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/range>

### 目标

使用 Go 语言的 range 遍历。

## 切片求和

```go
package main

import "fmt"

func main() {

    nums := []int{2, 3, 4}
    sum := 0
    for _, num := range nums {
        sum += num
    }
    fmt.Println("sum:", sum)
}

```

## 获取索引

```go
package main

import "fmt"

func main() {

    nums := []int{2, 3, 4}
    for i, num := range nums {
        if num == 3 {
            fmt.Println("index:", i)
        }
    }
}
```

## map 遍历键和值

```go
package main

import "fmt"

func main() {

    kvs := map[string]string{"a": "apple", "b": "banana"}
    for k, v := range kvs {
        fmt.Printf("%s -> %s\n", k, v)
    }
}
```

## map 遍历键

```go
package main

import "fmt"

func main() {

    kvs := map[string]string{"a": "apple", "b": "banana"}
    for k := range kvs {
        fmt.Println("key:", k)
    }
}
```

## 遍历字节

```go
package main

import "fmt"

func main() {
    for i, c := range "go" {
        fmt.Println(i, c)
    }
}
```

## 总结

使用 Go 语言的 range 进行遍历操作。

## 附录
