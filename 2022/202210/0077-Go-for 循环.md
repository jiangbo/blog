# 0077-Go-for 循环

## 环境

- Time 2022-08-23
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/for>

### 目标

使用 Go 语言的 for 循环。

## 单条件循环

类似其它语言中的 while 循环。

```go
package main

import "fmt"

func main() {
    i := 1
    for i <= 3 {
        fmt.Println(i)
        i = i + 1
    }
}
```

## 经典 for 循环

```go
package main

import "fmt"

func main() {
    for j := 7; j <= 9; j++ {
        fmt.Println(j)
    }
}
```

## 无限循环

无限循环可以使用 break 跳出循环。

```go
package main

import "fmt"

func main() {

    for {
        fmt.Println("loop")
        break
    }
}
```

## continue 跳过单次循环

```go
package main

import "fmt"

func main() {

    for n := 0; n <= 5; n++ {
        if n%2 == 0 {
            continue
        }
        fmt.Println(n)
    }
}
```

## 总结

使用 Go 语言的 for 循环。

## 附录
