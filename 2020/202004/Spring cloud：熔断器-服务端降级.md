# Spring cloud：熔断器-服务端降级

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

在服务端发生超时或者错误时，服务端进行服务降级处理。

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
@HystrixCommand(fallbackMethod = "fallbackAllPayments")
@GetMapping
public List<PaymentInfo> allPayments() {

    LoggerFactory.getLogger(getClass()).info("invoke payment port: {}", port);
    int i = 1 / 0;
    return paymentService.queryAllPayments();
}

public List<PaymentInfo> fallbackAllPayments() {

    LoggerFactory.getLogger(getClass()).info("invoke payment error");
    return Collections.emptyList();
}
```

### 启动类

```java
package jiangbo.springcloud;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.client.circuitbreaker.EnableCircuitBreaker;
import org.springframework.cloud.netflix.eureka.EnableEurekaClient;

@SpringBootApplication
@EnableEurekaClient
@EnableCircuitBreaker
public class JiangBoApplication {

    public static void main(String[] args) {

        SpringApplication.run(JiangBoApplication.class, args);
    }
}
```

## 验证

浏览器访问 http://localhost:4420/payment ，发现返回了一个空数组，并且控制台打印了如下的内容：

```text
2020-04-25 17:52:58.153  INFO 3976 --- [mentContrller-1] j.s.controller.PaymentContrller          : invoke payment port: 4420
2020-04-25 17:52:58.191  INFO 3976 --- [mentContrller-1] j.s.controller.PaymentContrller          : invoke payment error
```

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
    <artifactId>13spring-cloud-payment</artifactId>
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