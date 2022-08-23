# 0078-Go-if else 条件判断

## 环境

- Time 2022-08-23
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/if-else>

### 目标

使用 Go 语言的 if/else 条件判断。

## 条件判断

条件判断的小括号可以省略，但是后面的大括号不可以省略。

```go
package main

import "fmt"

func main() {

    if 7%2 == 0 {
        fmt.Println("7 is even")
    } else {
        fmt.Println("7 is odd")
    }
}
```

## 只有 if

```go
package main

import "fmt"

func main() {

    if 8%4 == 0 {
        fmt.Println("8 is divisible by 4")
    }
}
```

## 条件中声明变量

```go
package main

import "fmt"

func main() {
    // 条件之前，申明了一个变量，在 if/else 中都可以使用。
    if num := 9; num < 0 {
        fmt.Println(num, "is negative")
    } else if num < 10 {
        fmt.Println(num, "has 1 digit")
    } else {
        fmt.Println(num, "has multiple digits")
    }
}
```

## 总结

使用 Go 语言的 if/else 条件判断。

## 附录
