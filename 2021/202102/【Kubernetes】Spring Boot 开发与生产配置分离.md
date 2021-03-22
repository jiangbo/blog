# 【Kubernetes】Spring Boot 开发与生产配置分离

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M2

## 目标

新建 Spring Boot 项目，将开发与生产配置进行分离。在将配置分离后，不用每次打包时都额外修改配置文件。

## 创建 Spring Boot 项目

### pom.xml

```xml
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>2.5.0-M2</version>
        <relativePath /> <!-- lookup parent from repository -->
    </parent>
    <groupId>jiangbo.spring.demo</groupId>
    <artifactId>spring-k8s</artifactId>
    <version>product</version>
    <name>spring-boot-kubernetes-demo</name>
    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-actuator</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-jdbc</artifactId>
        </dependency>
        <dependency>
            <groupId>org.postgresql</groupId>
            <artifactId>postgresql</artifactId>
            <scope>runtime</scope>
        </dependency>

    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
                <executions>
                    <execution>
                        <id>build-info</id>
                        <goals>
                            <goal>build-info</goal>
                        </goals>
                    </execution>
                </executions>
            </plugin>
        </plugins>
    </build>
    <repositories>
        <repository>
            <id>spring-milestones</id>
            <name>Spring Milestones</name>
            <url>https://repo.spring.io/milestone</url>
        </repository>
    </repositories>
    <pluginRepositories>
        <pluginRepository>
            <id>spring-milestones</id>
            <name>Spring Milestones</name>
            <url>https://repo.spring.io/milestone</url>
        </pluginRepository>
    </pluginRepositories>

</project>
```

### application.yaml

```yaml
management:
  endpoint:
    shutdown:
      enabled: true
  endpoints:
    web:
      exposure:
        include: "*"
        
spring:
  datasource:
    url: jdbc:postgresql://192.168.56.103/postgres
    username: postgres
    password: 123456
```

### application-product.yaml

```yaml
spring:
  datasource:
    url: jdbc:postgresql://svc-pg/postgres
```

### 代码

```java
package jiangbo.spring.docker;

import java.net.InetAddress;
import java.net.UnknownHostException;
import java.util.List;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.info.BuildProperties;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class SpringDemoApplication {

    @Autowired
    private JdbcTemplate jdbcTemplate;

    private final BuildProperties buildProperties;

    public SpringDemoApplication(BuildProperties buildProperties) {

        this.buildProperties = buildProperties;
    }

    private static String hostname;

    public static void main(String[] args) throws UnknownHostException, InterruptedException {

        hostname = InetAddress.getLocalHost().getHostName();
        SpringApplication.run(SpringDemoApplication.class, args);
    }

    @GetMapping("/hostname")
    public String hello() {

        return hostname + " " + buildProperties.getVersion();
    }

    private static final String SQL = "SELECT * FROM public.user";

    @GetMapping("/users")
    public List<Person> users() {

        return jdbcTemplate.query(SQL, Person.ROW_MAPPER);
    }
}
```

### 推送镜像

```
mvn clean package

docker build -t jiangbo920827/spring-k8s:product .

docker push jiangbo920827/spring-k8s:product
```

## 环境准备

将 Kubernetes 环境的数据库访问 Service 准备好，如果已经新建了，可以忽略下面的步骤。

### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-pg
spec:
  ports:
    - port: 5432
```

### Endpoints

使用外部的数据库。

```yaml
apiVersion: v1
kind: Endpoints
metadata:
  name: svc-pg
subsets:
  - addresses:
      - ip: 192.168.56.103
    ports:
      - port: 5432
```

## 总结

新建了一个 Spring Boot 项目，并且将开发的配置与生产的分离，两个写到不同的配置文件。

## 附录
