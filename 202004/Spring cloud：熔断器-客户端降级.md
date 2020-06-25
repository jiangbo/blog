# Spring cloud：熔断器-服务端降级

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

在服务端发生超时或者错误时，客户端进行服务降级处理。

## 搭建步骤

### 增加依赖

```xml
<dependency>
    <groupId>org.springframework.cloud</groupId>
    <artifactId>spring-cloud-starter-netflix-hystrix</artifactId>
</dependency>
```

### 控制层

```java
package jiangbo.springcloud.controller;

import java.util.List;

import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import jiangbo.springcloud.dao.dto.PaymentInfo;
import jiangbo.springcloud.entity.OrderInfo;
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
public interface OrderInfoService {

    List<PaymentInfo> payments();
}
```

```java
package jiangbo.springcloud.service.impl;

import java.util.List;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import jiangbo.springcloud.dao.OrderInfoDao;
import jiangbo.springcloud.dao.PaymentDao;
import jiangbo.springcloud.dao.dto.PaymentInfo;
import jiangbo.springcloud.dao.dto.PaymentRequest;
import jiangbo.springcloud.dao.dto.PaymentResponse;
import jiangbo.springcloud.entity.OrderInfo;
import jiangbo.springcloud.entity.OrderStatusEnum;
import jiangbo.springcloud.service.OrderInfoService;

@Service
public class OrderInfoServiceImpl implements OrderInfoService {

    private final OrderInfoDao orderInfoDao;

    private final PaymentDao paymentDao;

    public OrderInfoServiceImpl(OrderInfoDao orderInfoDao, PaymentDao paymentDao) {

        this.orderInfoDao = orderInfoDao;
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
import org.springframework.web.bind.annotation.PostMapping;

import jiangbo.springcloud.dao.dto.PaymentInfo;
import jiangbo.springcloud.dao.dto.PaymentRequest;
import jiangbo.springcloud.dao.dto.PaymentResponse;
import jiangbo.springcloud.dao.http.PaymentDaoImpl;

@FeignClient(name = "payment", fallback = PaymentDaoImpl.class)
public interface PaymentDao {

    @PostMapping("/payment")
    PaymentResponse payment(PaymentRequest paymentRequest);

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
import jiangbo.springcloud.dao.dto.PaymentRequest;
import jiangbo.springcloud.dao.dto.PaymentResponse;

@Component
public class PaymentDaoImpl implements PaymentDao {

    @Override
    public PaymentResponse payment(PaymentRequest paymentRequest) {
        return null;
    }

    @Override
    public List<PaymentInfo> payments() {

        LoggerFactory.getLogger(getClass()).info("客户端降级...");
        return Collections.emptyList();
    }
}
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

只启动 eureka 和订单微服务，浏览器访问 http://localhost:4410/order/payment ，发现返回了一个空数组，并且控制台打印了如下的内容：

```text
2020-04-25 23:10:22.919  INFO 14148 --- [strix-payment-2] j.springcloud.dao.http.PaymentDaoImpl    : 客户端降级...
```

则表示客户端服务降级配置成功。

## 附录

### pom.xml

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
    <artifactId>14spring-cloud-order</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>

    <properties>
        <java.version>1.7</java.version>
    </properties>

    <dependencies>

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
            <artifactId>spring-boot-starter-jdbc</artifactId>
        </dependency>

        <dependency>
            <groupId>mysql</groupId>
            <artifactId>mysql-connector-java</artifactId>
            <scope>runtime</scope>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-test</artifactId>
            <scope>test</scope>
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