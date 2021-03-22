# 【Kubernetes】Spring Boot 通过名称访问

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

前面我们测试过服务名和 IP 是对应的，现在通过 Spring Boot 项目，通过名称访问数据库验证。

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
    <version>svc</version>
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
    url: jdbc:postgresql://svc-pg/postgres
    username: postgres
    password: 123456
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

docker build -t jiangbo920827/spring-demo:svc .

docker push jiangbo920827/spring-demo:svc
```

## 访问数据库

### rc.yaml

```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: rc-demo
spec:
  replicas: 3
  template:
    metadata:
      labels:
        app: myapp
    spec:
      containers:
        - name: pod-demo
          image: jiangbo920827/spring-demo:svc
          ports:
            - containerPort: 8080

```

### 创建 Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: svc-demo
spec:
  selector:
    app: myapp
  ports:
    - port: 80
      targetPort: 8080

```

### 查看 Service

```
[root@master kubernetes]# kubectl describe service svc-demo
Name:              svc-demo
Namespace:         default
Labels:            <none>
Annotations:       <none>
Selector:          app=myapp
Type:              ClusterIP
IP Families:       <none>
IP:                10.105.41.106
IPs:               10.105.41.106
Port:              <unset>  80/TCP
TargetPort:        8080/TCP
Endpoints:         10.244.1.115:8080,10.244.1.116:8080,10.244.1.117:8080
Session Affinity:  None
Events:            <none>
```

## 验证

```
[root@master kubernetes]# curl 10.105.41.106/users | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100    29    0    29    0     0     73      0 --:--:-- --:--:-- --:--:--    73
[
  {
    "name": "jiangbo",
    "age": 44
  }
]
```

## 总结

新建了一个 Spring Boot 项目，通过服务名的方式，访问到数据库中的数据，并展示。

## 附录
