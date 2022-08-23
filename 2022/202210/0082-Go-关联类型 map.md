# 0082-Go-关联类型 map

## 环境

- Time 2022-08-23
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/maps>

### 目标

使用 Go 语言的关联类型 map。

## 新建 map

```go
package main

import "fmt"

func main() {

    m := make(map[string]int)

    m["k1"] = 7
    m["k2"] = 13

    fmt.Println("map:", m)
}
```

## 修改和获取元素

```go
package main

import "fmt"

func main() {

    m := make(map[string]int)

    m["k1"] = 7
    m["k2"] = 13

    fmt.Println("map:", m)

    m["k1"] = 14
    v1 := m["k1"]
    fmt.Println("v1: ", v1)
}
```

## 获取长度

```go
package main

import "fmt"

func main() {

    m := make(map[string]int)
    m["k1"] = 7
    m["k2"] = 13
    fmt.Println("len:", len(m))
}
```

## 删除元素

```go
package main

import "fmt"

func main() {

    m := make(map[string]int)

    m["k1"] = 7
    m["k2"] = 13
    delete(m, "k2")
    fmt.Println("map:", m)
}
```

## 是否存在值

```go
package main

import "fmt"

func main() {

    m := make(map[string]int)

    m["k1"] = 7
    m["k2"] = 13
    delete(m, "k2")
    _, prs := m["k2"]
    fmt.Println("prs:", prs)
}
```

## 新建并初始化

```go
package main

import "fmt"

func main() {
    n := map[string]int{"foo": 1, "bar": 2}
    fmt.Println("map:", n)
}
```

## 总结

使用 Go 语言的关联类型。

## 附录
