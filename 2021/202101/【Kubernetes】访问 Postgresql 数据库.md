# 【Kubernetes】访问 Postgresql 数据库

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

创建 Spring Boot 项目，访问 PG 数据库，并将数据库表中的数据展示出来。

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
        <version>2.5.0-M1</version>
        <relativePath /> <!-- lookup parent from repository -->
    </parent>
    <groupId>jiangbo.spring.demo</groupId>
    <artifactId>spring-demo</artifactId>
    <version>external</version>
    <name>spring-boot-demo</name>
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

### 配置

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
    url: jdbc:postgresql://192.168.56.103:5432/postgres
    username: postgres
    password: 123456
```

### Person

```java
package jiangbo.spring.docker;

import org.springframework.jdbc.core.BeanPropertyRowMapper;
import org.springframework.jdbc.core.RowMapper;

public class Person {

    public static final RowMapper<Person> ROW_MAPPER = new BeanPropertyRowMapper<>(Person.class);

    private String name;

    private Integer age;

    public String getName() {
        return name;
    }
    public void setName(String name) {
        this.name = name;
    }
    public Integer getAge() {
        return age;
    }
    public void setAge(Integer age) {
        this.age = age;
    }
}
```

### SpringDemoApplication

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

## 新建 PG 数据库

为了简单，直接使用 Docker 创建 PG 数据库，选择在 192.168.56.103 服务器上创建。

### 创建 PG

```
[root@node2 ~]# docker run  -e POSTGRES_PASSWORD=123456 -p 5432:5432 -d postgres:9.6-alpine
3af43ff1af5df8d0f77ad58784dca5887655c1969a99fc3e1614e2fcdeec5f3b
```

### 生成表结构

可以选择数据库图形管理工具新建表，也可以使用 DDL：

```sql
CREATE TABLE public."user" (
	"name" varchar NOT NULL,
	age int4 NULL
);
```

### 插入数据

```sql
INSERT INTO public."user" ("name",age) VALUES ('jiangbo',44);
```

## 访问测试

![external.png][1]

## 总结

介绍了新建 Spring Boot 项目，并访问 PG 数据库，获取数据。

[1]: images/external.png

## 附录
