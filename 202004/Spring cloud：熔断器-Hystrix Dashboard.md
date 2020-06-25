# Spring cloud：熔断器-Hystrix Dashboard

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

搭建 Hystrix Dashboard，监控微服务的状态。

## 搭建步骤

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
    <artifactId>16spring-cloud-hystrix-dashboard</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>
    <description>搭建 hystrix 的监控面板</description>

    <properties>
        <java.version>1.7</java.version>
    </properties>

    <dependencies>

        <dependency>
            <groupId>org.springframework.cloud</groupId>
            <artifactId>spring-cloud-starter-netflix-hystrix-dashboard</artifactId>
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
  port: 4500
```

### 启动类

```java
package jiangbo.springcloud;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.netflix.hystrix.dashboard.EnableHystrixDashboard;

@SpringBootApplication
@EnableHystrixDashboard
public class JiangBoApplication {

    public static void main(String[] args) {

        SpringApplication.run(JiangBoApplication.class, args);
    }
}
```

## 验证

### 打开主页

访问 http://localhost:4500/hystrix ，打开 Hystrix Dashboard。

![spring-cloud-hystrix-dashboard][1]


### 填写信息

第一行填写之前暴露的监控数据接口：http://localhost:4410/hystrix.stream

delay 可以默认，title可以自定义，这里填写 order，点击 Monitor Stream ，就可以开始监控了，多次访问 http://localhost:4410/order/payment 看看监控面板的变化。

![spring-cloud-hystrix-dashboard2][2]

[1]:../../images/spring-cloud-hystrix-dashboard.png
[2]:../../images/spring-cloud-hystrix-dashboard2.png
