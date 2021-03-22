# Spring cloud：服务注册 eureka 单机版

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0

## 背景

实现一个服务注册组件 eureka 单机版。

## 搭建步骤

### 增加依赖

在 pom.xml 中，增加 eureka 服务端的依赖。

```xml
<dependency>
    <groupId>org.springframework.cloud</groupId>
    <artifactId>spring-cloud-starter-netflix-eureka-server</artifactId>
</dependency>
```

### 增加配置文件

增加 spring 的配置文件：application.yml。

```yml
server:
  port: 8761

spring:
  application:
    name: eureka8761

eureka:
  instance:
    hostname: localhost
  client:
    registerWithEureka: false
    fetchRegistry: false
    serviceUrl:
      defaultZone: http://${eureka.instance.hostname}:${server.port}/eureka/
```

### 启动项目

直接使用 main 方法启动项目。

```java
package jiangbo.springcloud;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class JiangBoApplication {

    public static void main(String[] args) {

        SpringApplication.run(JiangBoApplication.class, args);
    }
}
```

### 验证结果

访问 localhost:8761 看看能否看到 eureka 的界面。

![spring-cloud-eureka][1]

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
    <artifactId>01spring-cloud-eureka</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>

    <properties>
        <java.version>1.7</java.version>
    </properties>

    <dependencies>

        <dependency>
            <groupId>org.springframework.cloud</groupId>
            <artifactId>spring-cloud-starter-netflix-eureka-server</artifactId>
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


[1]:../../images/spring-cloud-eureka01.png