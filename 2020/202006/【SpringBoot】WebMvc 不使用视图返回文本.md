# 【SpringBoot】WebMvc 不使用视图返回文本

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Spring Boot 1.5.22.RELEASE
- Maven 3.6.3

## 概述

使用 Spring WebMvc 的时候，经常返回的内容包括 json 和 xml 和视图，下面给出直接在 controller 中返回文本的示例。

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>1.5.22.RELEASE</version>
    </parent>

    <groupId>jiangbo.spring.boot</groupId>
    <artifactId>spring-boot-return-text</artifactId>
    <version>1.0.0</version>
    <description>返回文本的示例</description>

    <properties>
        <java.version>1.8</java.version>
    </properties>

    <dependencies>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
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

### JiangBoApplication

```java
package jiangbo.spring.boot;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class JiangBoApplication {

    public static void main(String[] args) {

        SpringApplication.run(JiangBoApplication.class, args);
    }
}
```

### HelloController

```java
package jiangbo.spring.boot.controller;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ResponseBody;

@Controller
public class HelloController {

    @GetMapping("/hello")
    @ResponseBody
    public String hello(String name) {

        return "hello: " + name;
    }
}
```

### 浏览器访问

使用 SpringBoot 的方式启动项目，在浏览器端访问 http://localhost:8080/hello?name=张三 ，查看浏览是否显示：

```text
hello: 张三
```

因为 SpringBoot 帮自动配置了一些类型转换，所以即使是中文也不会出现乱码。