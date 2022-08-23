# 0081-Go-切片 slice

## 环境

- Time 2022-08-23
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/slices>

### 目标

使用 Go 语言的切片类型。

## 新建切片类型

```go
package main

import "fmt"

func main() {

    slice := make([]string, 3)
    fmt.Println("emp:", slice)
}
```

## 修改和获取元素

```go
package main

import "fmt"

func main() {

    slice := make([]string, 3)
    fmt.Println("emp:", slice)

    slice[0] = "a"
    slice[1] = "b"
    slice[2] = "c"
    fmt.Println("set:", slice)
    fmt.Println("get:", slice[2])
}
```

## 获取长度

```go
package main

import "fmt"

func main() {

    slice := make([]string, 3)
    fmt.Println("emp:", slice)

    slice[0] = "a"
    slice[1] = "b"
    slice[2] = "c"

    fmt.Println("len:", len(slice))
}
```

## 追加元素

```go
package main

import "fmt"

func main() {

    slice := make([]string, 3)
    fmt.Println("emp:", slice)

    slice[0] = "a"
    slice[1] = "b"
    slice[2] = "c"

    fmt.Println("len:", len(slice))

    slice = append(slice, "d")
    slice = append(slice, "e", "f")
    fmt.Println("apd:", slice)
}
```

## 切片拷贝

```go
package main

import "fmt"

func main() {

    slice := make([]string, 3)

    slice[0] = "a"
    slice[1] = "b"
    slice[2] = "c"

    slice = append(slice, "d")
    slice = append(slice, "e", "f")

    dst := make([]string, len(slice))
    copy(dst, slice)
    fmt.Println("cpy:", dst)
}
```

## 截取

```go
package main

import "fmt"

func main() {

    slice := make([]string, 3)

    slice[0] = "a"
    slice[1] = "b"
    slice[2] = "c"

    slice = append(slice, "d")
    slice = append(slice, "e", "f")

    l := slice[2:5]
    fmt.Println("sl1:", l)
    // 从头开始
    l = slice[:5]
    fmt.Println("sl2:", l)
    // 直到末尾
    l = slice[2:]
    fmt.Println("sl3:", l)
}
```

## 新建并初始化切片

```go
package main

import "fmt"

func main() {

    t := []string{"g", "h", "i"}
    fmt.Println("dcl:", t)
}
```

## 二维切片

```go
package main

import "fmt"

func main() {

    twoD := make([][]int, 3)
    for i := 0; i < 3; i++ {
        innerLen := i + 1
        twoD[i] = make([]int, innerLen)
        for j := 0; j < innerLen; j++ {
            twoD[i][j] = i + j
        }
    }
    fmt.Println("2d: ", twoD)
}
```

## 总结

使用 Go 语言的切片类型。

## 附录
