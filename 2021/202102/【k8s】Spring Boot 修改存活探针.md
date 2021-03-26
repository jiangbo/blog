# 【k8s】Spring Boot 修改存活探针

## 环境

1. kubernetes 1.20.4
2. Spring Boot 2.5.0-M3

## 目标

新建 Spring Boot 项目，提供一个 GET 请求来修改项目的存活探针。

## 示例

### pom.xml

```xml
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>2.5.0-M3</version>
        <relativePath /> <!-- lookup parent from repository -->
    </parent>
    <groupId>jiangbo.spring.demo</groupId>
    <artifactId>spring-k8s</artifactId>
    <version>liveness</version>

    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>

        <dependency>
            <groupId>org.projectlombok</groupId>
            <artifactId>lombok</artifactId>
            <scope>provided</scope>
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

### 接口定义

```java
package jiangbo.spring.docker;

import java.net.InetAddress;
import java.util.List;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.availability.AvailabilityChangeEvent;
import org.springframework.boot.availability.LivenessState;
import org.springframework.context.ApplicationContext;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import lombok.AllArgsConstructor;

@AllArgsConstructor
@SpringBootApplication
@RestController
public class SpringDemoApplication {

    private JdbcTemplate jdbcTemplate;

    private ApplicationContext context;

    private static String hostname;

    public static void main(String[] args) throws Exception {

        hostname = InetAddress.getLocalHost().getHostName();
        SpringApplication.run(SpringDemoApplication.class, args);
    }

    @GetMapping("/hostname")
    public String hello() {

        return hostname;
    }

    private static final String SQL = "SELECT * FROM public.user";

    @GetMapping("/users")
    public List<Person> users() {

        return jdbcTemplate.query(SQL, Person.ROW_MAPPER);
    }

    @GetMapping(value = "/liveness")
    public String liveness(String name) {

        LivenessState state = LivenessState.valueOf(name);
        AvailabilityChangeEvent.publish(context, state);
        return state.toString();
    }
}
```

### 配置文件

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

logging:
  level:
    web: debug
  pattern:
    console: "%d{yyyy-MM-dd HH:mm:ss} - %msg%n"
```

### 打包并生成镜像

```
PS D:\workspace\sts\spring-demo> docker build -t jiangbo920827/spring-k8s:liveness .
[+] Building 2.2s (12/12) FINISHED
 => [internal] load build definition from Dockerfile                                         0.0s
 => => transferring dockerfile: 32B                                                          0.0s
 => [internal] load .dockerignore                                                            0.0s
 => => transferring context: 2B                                                              0.0s
 => [internal] load metadata for docker.io/library/openjdk:8-jre-alpine                      0.0s
 => [internal] load build context                                                            0.3s
 => => transferring context: 23.26MB                                                         0.3s
 => CACHED [stage-1 1/5] FROM docker.io/library/openjdk:8-jre-alpine                         0.0s
 => [builder 2/3] COPY target/*.jar application.jar                                          0.3s
 => [builder 3/3] RUN java -Djarmode=layertools -jar application.jar extract                 1.2s
 => CACHED [stage-1 2/5] COPY --from=builder dependencies/ ./                                0.0s
 => CACHED [stage-1 3/5] COPY --from=builder snapshot-dependencies/ ./                       0.0s
 => CACHED [stage-1 4/5] COPY --from=builder spring-boot-loader/ ./                          0.0s
 => [stage-1 5/5] COPY --from=builder application/ ./                                        0.0s
 => exporting to image                                                                       0.0s
 => => exporting layers                                                                      0.0s
 => => writing image sha256:27e1956a7558e66cc463d09c86bcda059fd6534d520a9ab68fb8567048f786f2 0.0s
 => => naming to docker.io/jiangbo920827/spring-k8s:liveness                                 0.0s
PS D:\workspace\sts\spring-demo> docker push jiangbo920827/spring-k8s:liveness
The push refers to repository [docker.io/jiangbo920827/spring-k8s]
31e294c1279b: Pushed
e323b9ec2845: Layer already exists
5f70bf18a086: Layer already exists
6d24f25c4501: Layer already exists
edd61588d126: Layer already exists
9b9b7f3d56a0: Layer already exists
f1b5933fe4b5: Layer already exists
liveness: digest: sha256:14e2cc6a35655774f0134d27cc947fce7bea64e9ce2f18fbfe32f3690200e7c4 size: 1782
```

## 总结

新建一个可以修改存活探针的 Spring Boot 项目，并且生成镜像传送到 Docker 仓库。

## 附录
