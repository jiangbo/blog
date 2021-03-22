# Spring cloud：熔断器-暴露监控数据

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

在订单微服务的客户端暴露 hystrix 的监控数据接口。

## 搭建步骤

### pom.xml

需要增加 actuator 监控数据接口的依赖。

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.cloud</groupId>
        <artifactId>spring-cloud-starter-parent</artifactId>
        <version>Edgware.SR6</version>
    </parent>

    <groupId>jiangbo.springcloud</groupId>
    <artifactId>15spring-cloud-order</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>
    <description>暴露 hystrix 监控数据接口</description>

    <properties>
        <java.version>1.7</java.version>
    </properties>

    <dependencies>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-actuator</artifactId>
        </dependency>

        <dependency>
            <groupId>org.springframework.cloud</groupId>
            <artifactId>spring-cloud-starter-netflix-hystrix</artifactId>
        </dependency>

        <dependency>
            <groupId>org.springframework.cloud</groupId>
            <artifactId>spring-cloud-starter-openfeign</artifactId>
        </dependency>

        <dependency>
            <groupId>org.springframework.cloud</groupId>
            <artifactId>spring-cloud-starter-netflix-eureka-client</artifactId>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-devtools</artifactId>
            <scope>provided</scope>
        </dependency>

    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
            </plugin>
        </plugins>
    </build>
</project>
```

### 控制层

```java
package jiangbo.springcloud.controller;

import java.util.List;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import jiangbo.springcloud.dao.dto.PaymentInfo;
import jiangbo.springcloud.service.OrderInfoService;

@RestController
@RequestMapping("/order")
public class OrderInfoController {

    private final OrderInfoService orderInfoService;

    public OrderInfoController(OrderInfoService orderInfoService) {

        this.orderInfoService = orderInfoService;
    }

    @GetMapping("/payment")
    public List<PaymentInfo> payments() {

        return orderInfoService.payments();
    }
}
```

### 服务层

```java
package jiangbo.springcloud.service;

import java.util.List;

import jiangbo.springcloud.dao.dto.PaymentInfo;

public interface OrderInfoService {

    List<PaymentInfo> payments();
}
```

```java
package jiangbo.springcloud.service.impl;

import java.util.List;

import org.springframework.stereotype.Service;

import jiangbo.springcloud.dao.PaymentDao;
import jiangbo.springcloud.dao.dto.PaymentInfo;
import jiangbo.springcloud.service.OrderInfoService;

@Service
public class OrderInfoServiceImpl implements OrderInfoService {

    private final PaymentDao paymentDao;

    public OrderInfoServiceImpl(PaymentDao paymentDao) {

        this.paymentDao = paymentDao;
    }

    @Override
    public List<PaymentInfo> payments() {

        return paymentDao.payments();
    }
}
```

### 数据访问层

```java
package jiangbo.springcloud.dao;

import java.util.List;

import org.springframework.cloud.netflix.feign.FeignClient;
import org.springframework.web.bind.annotation.GetMapping;

import jiangbo.springcloud.dao.dto.PaymentInfo;
import jiangbo.springcloud.dao.http.PaymentDaoImpl;

@FeignClient(name = "payment", fallback = PaymentDaoImpl.class)
public interface PaymentDao {

    @GetMapping("/payment")
    List<PaymentInfo> payments();
}
```

```java
package jiangbo.springcloud.dao.http;

import java.util.Collections;
import java.util.List;

import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;

import jiangbo.springcloud.dao.PaymentDao;
import jiangbo.springcloud.dao.dto.PaymentInfo;

@Component
public class PaymentDaoImpl implements PaymentDao {

    @Override
    public List<PaymentInfo> payments() {

        LoggerFactory.getLogger(getClass()).info("客户端降级...");
        return Collections.emptyList();
    }
}
```

### PaymentInfo

省略了 get/set 方法。

```java
package jiangbo.springcloud.dao.dto;

import java.util.Date;

public class PaymentInfo {

    private long id;

    private long orderId;

    private String amount;

    private String status;

    private Date createTime;
}
```

### 配置

```yml
server:
  port: 4410

spring:
  application:
    name: order

eureka:
  client:
    serviceUrl:
      defaultZone: http://localhost:8761/eureka/

feign:
  hystrix:
    enabled: true
```

### 启动类

```java
package jiangbo.springcloud;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.client.circuitbreaker.EnableCircuitBreaker;
import org.springframework.cloud.netflix.eureka.EnableEurekaClient;
import org.springframework.cloud.netflix.feign.EnableFeignClients;

@SpringBootApplication
@EnableEurekaClient
@EnableFeignClients
@EnableCircuitBreaker
public class JiangBoApplication {

    public static void main(String[] args) {

        SpringApplication.run(JiangBoApplication.class, args);
    }
}
```

## 验证

### 无数据

在启动该服务前，需要有一台 eureka 服务器。

启动服务，浏览器访问 http://localhost:4410/hystrix.stream ，发现没有数据，只有一堆的 ping，如下：

```text
ping: 

ping: 

ping: 

ping: 

ping: 
```

这是因为 spring boot 1.5 只有熔断或者降低执行了，才能有数据。详细信息可以看 [这里][1]。


### 有数据

浏览器访问 http://localhost:4410/order/payment ， 触发一下降级，发现返回了一个空数组，并且控制台打印了如下的内容：

```text
2020-04-25 23:10:22.919  INFO 14148 --- [strix-payment-2] j.springcloud.dao.http.PaymentDaoImpl    : 客户端降级...
```

这时，访问 http://localhost:4410/hystrix.stream ，能看到如下的数据：

![spring-cloud-circuit-breaker][2]

暴露监控数据的环境搭建成功。

[1]:https://stackoverflow.com/questions/40447916/unable-to-get-hystrix-stream-in-spring-cloud
[2]:../../images/spring-cloud-circuit-breaker.png
