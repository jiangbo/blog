# 0079-Go-switch 分支

## 环境

- Time 2022-08-23
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/switch>

### 目标

使用 Go 语言的 switch 分支语句。

## 整数分支

```go
package main

import "fmt"

func main() {

	i := 2
	fmt.Print("write ", i, " as ")
	switch i {
	case 1:
		fmt.Println("one")
	case 2:
		fmt.Println("two")
	case 3:
		fmt.Println("three")
	}
}
```

## 多个条件

```go
package main

import (
	"fmt"
	"time"
)

func main() {

	switch time.Now().Weekday() {
	// 逗号分隔多个条件
	case time.Saturday, time.Sunday:
		fmt.Println("It's the weekend")
	default:
		fmt.Println("It's a weekday")
	}
}
```

## 表达式

```go
package main

import (
	"fmt"
	"time"
)

func main() {

	t := time.Now()
	switch {
    // 可以接表达式，和 if/else 功能相同
	case t.Hour() < 12:
		fmt.Println("It's before noon")
	default:
		fmt.Println("It's after noon")
	}
}
```

## 类型选择

```go
package main

import "fmt"

func main() {

	whatAmI(true)
	whatAmI(1)
	whatAmI("hey")
}

func whatAmI(a any) {
	// 类型选择 switch
	switch t := a.(type) {
	case bool:
		fmt.Println("I'm a bool")
	case int:
		fmt.Println("I'm an int")
	default:
		fmt.Printf("Don't know type %T\n", t)
	}
}
```

## 总结

使用 Go 语言的 switch 分支语句。

## 附录
