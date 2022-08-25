# 0124-Go-JSON 转换

## 环境

- Time 2022-08-25
- Go 1.19

## 前言

### 说明

参考：<https://gobyexample.com/json>

### 目标

使用 Go 语言的 JSON。

## 简单值

```go
package main

import (
    "encoding/json"
    "fmt"
)

func main() {

    bolB, _ := json.Marshal(true)
    fmt.Println(string(bolB))

    intB, _ := json.Marshal(1)
    fmt.Println(string(intB))

    fltB, _ := json.Marshal(2.34)
    fmt.Println(string(fltB))

    strB, _ := json.Marshal("gopher")
    fmt.Println(string(strB))
}
```

## 引用值

```go
package main

import (
    "encoding/json"
    "fmt"
)

func main() {

    slcD := []string{"apple", "peach", "pear"}
    slcB, _ := json.Marshal(slcD)
    fmt.Println(string(slcB))

    mapD := map[string]int{"apple": 5, "lettuce": 7}
    mapB, _ := json.Marshal(mapD)
    fmt.Println(string(mapB))
}
```

## 结构体

```go
package main

import (
    "encoding/json"
    "fmt"
)

type response1 struct {
    Page   int
    Fruits []string
}

type response2 struct {
    Page   int      `json:"page"`
    Fruits []string `json:"fruits"`
}

func main() {
    res1D := &response1{
        Page:   1,
        Fruits: []string{"apple", "peach", "pear"}}
    res1B, _ := json.Marshal(res1D)
    fmt.Println(string(res1B))

    res2D := &response2{
        Page:   1,
        Fruits: []string{"apple", "peach", "pear"}}
    res2B, _ := json.Marshal(res2D)
    fmt.Println(string(res2B))
}
```

## 反序列化

```go
package main

import (
    "encoding/json"
    "fmt"
    "os"
)

type response1 struct {
    Page   int
    Fruits []string
}

type response2 struct {
    Page   int      `json:"page"`
    Fruits []string `json:"fruits"`
}

func main() {
    byt := []byte(`{"num":6.13,"strs":["a","b"]}`)
    var dat map[string]any
    if err := json.Unmarshal(byt, &dat); err != nil {
        panic(err)
    }
    fmt.Println(dat)

    num := dat["num"].(float64)
    fmt.Println(num)

    strs := dat["strs"].([]any)
    str1 := strs[0].(string)
    fmt.Println(str1)

    str := `{"page": 1, "fruits": ["apple", "peach"]}`
    res := response2{}
    json.Unmarshal([]byte(str), &res)
    fmt.Println(res)
    fmt.Println(res.Fruits[0])

    enc := json.NewEncoder(os.Stdout)
    d := map[string]int{"apple": 5, "lettuce": 7}
    enc.Encode(d)
}
```

## 总结

使用 Go 语言的 JSON。

## 附录
