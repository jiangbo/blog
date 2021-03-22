# Spring cloud：网关-Zuul路由

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

搭建 Zuul 网关，提供统一的访问入口。

## 搭建步骤

### 准备

该部分基于前面已经搭建好的订单和支付微服务，需要保证有微服务可用。

### pom.xml

需要增加 spring-cloud-starter-netflix-zuul 的依赖。

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
    <artifactId>17spring-cloud-zuul</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>
    <description>搭建 Zuul 网关环境</description>

    <properties>
        <java.version>1.7</java.version>
    </properties>

    <dependencies>

        <dependency>
            <groupId>org.springframework.cloud</groupId>
            <artifactId>spring-cloud-starter-netflix-zuul</artifactId>
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

### 配置

```yml
server:
  port: 80

spring:
  application:
    name: zuul

eureka:
  client:
    serviceUrl:
      defaultZone: http://localhost:8761/eureka/
```

### 启动类

```java
package jiangbo.springcloud;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.netflix.zuul.EnableZuulProxy;

@SpringBootApplication
@EnableZuulProxy
public class JiangBoApplication {

    public static void main(String[] args) {

        SpringApplication.run(JiangBoApplication.class, args);
    }
}
```

## 验证

### 打开主页

访问 http://localhost/order/order ，能成功看到订单的信息；
访问 http://localhost/payment/payment ，能成功看到支付的信息；

### 路径说明

http://localhost/order/order 这个访问路径的说明：

其中 localhost 是访问网关的地址，由于网关 80 端口可以省略不写，所以就可以只写 localhost。
第一个 order 是微服务 order 的名称，在配置文件 spring.application.name 中定义的，在 eureka 上也可以看到。
最后一个 order 是订单微服务的路径映射。