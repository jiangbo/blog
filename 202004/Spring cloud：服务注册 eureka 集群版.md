# Spring cloud：服务注册 eureka 集群版

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0

## 背景

实现一个服务注册组件 eureka 集群版。由于在一台电脑上启动，所以使用不同的端口，端口分配为 8761，8762 和 8763。

## 搭建步骤

pom.xml 不需要新增依赖。

### hosts 增加映射

由于使用域名进行访问，所以在 hosts 中进行配置，增加如下的内容：

```text
127.0.0.1 eureka1
127.0.0.1 eureka2
127.0.0.1 eureka3
``` 

### 第一个节点

application.yml：

```yml
server:
  port: 8761

spring:
  application:
    name: eureka

eureka:
  instance:
    hostname: eureka1
  client:
    serviceUrl:
      defaultZone: http://eureka1:8761/eureka/,http://eureka2:8762/eureka/,http://eureka3:8763/eureka/
```

### 第二个节点

application.yml：

```yml
server:
  port: 8762

spring:
  application:
    name: eureka

eureka:
  instance:
    hostname: eureka2
  client:
    serviceUrl:
      defaultZone: http://eureka1:8761/eureka/,http://eureka2:8762/eureka/,http://eureka3:8763/eureka/
```

### 第三个节点

application.yml：

```yml
server:
  port: 8763

spring:
  application:
    name: eureka

eureka:
  instance:
    hostname: eureka3
  client:
    serviceUrl:
      defaultZone: http://eureka1:8761/eureka/,http://eureka2:8762/eureka/,http://eureka3:8763/eureka/
```

### 启动项目

直接使用 main 方法启动三个项目。

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

分别访问 localhost:8761 localhost:8762 localhost:8762 看看能否看到 eureka 的界面，并且能看到集群信息。

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


[1]:../../images/spring-cloud-eureka02.png