# Spring cloud：unknown property 'feign hystrix'

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

在配置客户端服务降级时，服务降级始终不成功，需要配置一个属性，但是 ide 提示 unknown property 'feign hystrix'。

## 分析

### 过程
在服务降级配置的过程中，一直不生效。根据官方文档，需要配置：

```yml
feign:
  hystrix:
    enabled: true
```

但是 ide 提示 unknown property 'feign hystrix' 存在误导，导致没有配置，让服务降级的配置不成功。

### 解决

忽略 ide 的提示，配置上这个属性，启动微服务试试看。

