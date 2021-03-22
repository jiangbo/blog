# 【Docker】Spring Boot 和 Docker

参考教程：https://spring.io/guides/gs/spring-boot-docker/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 编写 Spring Boot 服务

### 定义 pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>2.2.11.RELEASE</version>
        <relativePath /> <!-- lookup parent from repository -->
    </parent>

    <groupId>jiangbo.spring.docker</groupId>
    <artifactId>spring-docker</artifactId>
    <version>1.0.0</version>

    <properties>
        <java.version>1.8</java.version>
    </properties>
    
    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
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

### 定义接口

```java
package jiangbo.spring.docker;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class SpringDockerApplication {

    @GetMapping("/")
    public String home() {

        return "Hello Docker World";
    }

    public static void main(String[] args) {

        SpringApplication.run(SpringDockerApplication.class, args);
    }
}
```

### 打包并上传到服务器

```sh
[root@master demo1]# ll
total 34604
-rw-r--r--. 1 root root      108 Nov 25 20:25 Dockerfile
-rw-r--r--. 1 root root 17711362 Nov 25 20:25 spring-docker-1.0.0.jar
-rw-r--r--. 1 root root 17711374 Nov 25 21:44 spring-docker-1.0.1.jar
[root@master demo1]# pwd
/root/docker/demo1
[root@master demo1]#
```

## 生成镜像

### 定义 Dockerfile

```Dockerfile
[root@master demo1]# cat Dockerfile
FROM openjdk:8-jdk-alpine
ARG JAR_FILE=*.jar
COPY ${JAR_FILE} app.jar
ENTRYPOINT ["java","-jar","/app.jar"]
```

### 生成 1.0.0 镜像

```sh
[root@master demo1]# docker build --build-arg JAR_FILE=spring-docker-1.0.0.jar -t gs-spring-boot-docker:1.0.0 .
Sending build context to Docker daemon  35.43MB
Step 1/4 : FROM openjdk:8-jdk-alpine
 ---> a3562aa0b991
Step 2/4 : ARG JAR_FILE=*.jar
 ---> Running in d6f5be8caaf5
Removing intermediate container d6f5be8caaf5
 ---> 729cc2317b0b
Step 3/4 : COPY ${JAR_FILE} app.jar
 ---> 4bcb5820c7c2
Step 4/4 : ENTRYPOINT ["java","-jar","/app.jar"]
 ---> Running in a0de243efcdc
Removing intermediate container a0de243efcdc
 ---> 2fea2b509a53
Successfully built 2fea2b509a53
Successfully tagged gs-spring-boot-docker:1.0.0
[root@master demo1]# docker image ls
REPOSITORY                       TAG                 IMAGE ID            CREATED             SIZE
gs-spring-boot-docker            1.0.0               2fea2b509a53        13 seconds ago      123MB
springio/gs-spring-boot-docker   latest              b4368075c1fc        16 minutes ago      122MB
ubuntu                           latest              d70eaf7277ea        4 weeks ago         72.9MB
openjdk                          8-jdk-alpine        a3562aa0b991        18 months ago       105MB
[root@master demo1]#
```

### 生成 1.0.1 镜像

```sh
[root@master demo1]# docker build --build-arg JAR_FILE=spring-docker-1.0.1.jar -t gs-spring-boot-docker:1.0.1 .
Sending build context to Docker daemon  35.43MB
Step 1/4 : FROM openjdk:8-jdk-alpine
 ---> a3562aa0b991
Step 2/4 : ARG JAR_FILE=*.jar
 ---> Using cache
 ---> 528e5285c315
Step 3/4 : COPY ${JAR_FILE} app.jar
 ---> Using cache
 ---> 069384d42672
Step 4/4 : ENTRYPOINT ["java","-jar","/app.jar"]
 ---> Using cache
 ---> df2edec3a41a
Successfully built df2edec3a41a
Successfully tagged gs-spring-boot-docker:1.0.1
[root@master demo1]# docker image ls
REPOSITORY                       TAG                 IMAGE ID            CREATED             SIZE
gs-spring-boot-docker            1.0.1               df2edec3a41a        37 seconds ago      123MB
gs-spring-boot-docker            1.0.0               2fea2b509a53        2 minutes ago       123MB
springio/gs-spring-boot-docker   latest              b4368075c1fc        19 minutes ago      122MB
ubuntu                           latest              d70eaf7277ea        4 weeks ago         72.9MB
openjdk                          8-jdk-alpine        a3562aa0b991        18 months ago       105MB
[root@master demo1]#
```

## 镜像的大小

### 分别导出镜像

```sh
[root@master demo1]# docker save gs-spring-boot-docker:1.0.0 -o gs-spring-boot-docker:1.0.0.tar
[root@master demo1]# docker save gs-spring-boot-docker:1.0.1 -o gs-spring-boot-docker:1.0.1.tar
[root@master demo1]# mkdir images
[root@master demo1]# mv gs-spring-boot-docker\:1.0.* images/
[root@master demo1]# tar -zcvf images.tar.gz images/
images/
images/gs-spring-boot-docker:1.0.0.tar
images/gs-spring-boot-docker:1.0.1.tar
[root@master demo1]# ll -h
total 203M
-rw-r--r--. 1 root root  108 Nov 25 20:25 Dockerfile
drwxr-xr-x. 2 root root   84 Nov 25 21:55 images
-rw-r--r--. 1 root root 170M Nov 25 21:55 images.tar.gz
-rw-r--r--. 1 root root  17M Nov 25 20:25 spring-docker-1.0.0.jar
-rw-r--r--. 1 root root  17M Nov 25 21:44 spring-docker-1.0.1.jar
```

### 一起导出镜像

```sh
[root@master demo1]# docker save gs-spring-boot-docker:1.0.0 gs-spring-boot-docker:1.0.1 | gzip > images2.tar.gz
[root@master demo1]# ll -h
total 303M
-rw-r--r--. 1 root root  108 Nov 25 20:25 Dockerfile
drwxr-xr-x. 2 root root   84 Nov 25 21:55 images
-rw-r--r--. 1 root root 100M Nov 25 21:57 images2.tar.gz
-rw-r--r--. 1 root root 170M Nov 25 21:55 images.tar.gz
-rw-r--r--. 1 root root  17M Nov 25 20:25 spring-docker-1.0.0.jar
-rw-r--r--. 1 root root  17M Nov 25 21:44 spring-docker-1.0.1.jar
```

> 镜像的体积明显降低了，如果镜像数和共用的层越多，则减少的体积越明显。因为它们共用了基础的 openjdk 镜像。

## 分层构建镜像

### 定义构建文件

```Dockerfile
FROM openjdk:8-jdk-alpine
RUN addgroup -S spring && adduser -S spring -G spring
USER spring:spring
ARG DEPENDENCY=target/dependency
COPY ${DEPENDENCY}/BOOT-INF/lib /app/lib
COPY ${DEPENDENCY}/META-INF /app/META-INF
COPY ${DEPENDENCY}/BOOT-INF/classes /app
ENTRYPOINT ["java","-cp","app:app/lib/*","jiangbo.spring.docker.SpringDockerApplication"]
```

### 使用分层构建

```sh
[root@master demo3]# docker build -t gs-spring-boot-docker:1.0.0 .
Sending build context to Docker daemon   35.6MB
Step 1/8 : FROM openjdk:8-jdk-alpine
 ---> a3562aa0b991
Step 2/8 : RUN addgroup -S spring && adduser -S spring -G spring
 ---> Running in 7de3e020b8ee
Removing intermediate container 7de3e020b8ee
 ---> 0b5e90edf7db
Step 3/8 : USER spring:spring
 ---> Running in d6eedeb3d48b
Removing intermediate container d6eedeb3d48b
 ---> c834729e9d31
Step 4/8 : ARG DEPENDENCY=target/dependency
 ---> Running in 05cdec87fea5
Removing intermediate container 05cdec87fea5
 ---> c9623e256de4
Step 5/8 : COPY ${DEPENDENCY}/BOOT-INF/lib /app/lib
 ---> 69a7896fce2f
Step 6/8 : COPY ${DEPENDENCY}/META-INF /app/META-INF
 ---> 34f10a99aaa2
Step 7/8 : COPY ${DEPENDENCY}/BOOT-INF/classes /app
 ---> 5f3ea897413d
Step 8/8 : ENTRYPOINT ["java","-cp","app:app/lib/*","jiangbo.spring.docker.SpringDockerApplication"]
 ---> Running in b0e2c78ebd9b
Removing intermediate container b0e2c78ebd9b
 ---> 87fc89fd35cb
Successfully built 87fc89fd35cb
Successfully tagged gs-spring-boot-docker:1.0.0
[root@master demo3]# docker image ls
REPOSITORY              TAG                 IMAGE ID            CREATED             SIZE
gs-spring-boot-docker   1.0.0               87fc89fd35cb        7 seconds ago       122MB
ubuntu                  latest              d70eaf7277ea        4 weeks ago         72.9MB
openjdk                 8-jdk-alpine        a3562aa0b991        18 months ago       105MB
[root@master demo3]#
```

### 启动并验证功能

```sh
[root@master demo3]# docker run -d -p8080:8080 gs-spring-boot-docker:1.0.0
39a77196d1f9afe30a3d7ed3b06b4a02a29cd01ada110801d237612a2b0a69bd
[root@master demo3]# curl localhost:8080
Hello Docker World[root@master demo3]#
```

## 分层减少体积

### 构建分层 1.0.1 版本

```sh
[root@master demo4]# ll
total 17304
-rw-r--r--. 1 root root      346 Nov 25 22:14 Dockerfile
-rw-r--r--. 1 root root 17711374 Nov 25 21:44 spring-docker-1.0.1.jar
[root@master demo4]# mkdir -p target/dependency && (cd target/dependency; jar -xf ../../*.jar)
[root@master demo4]# docker build -t gs-spring-boot-docker:1.0.1 .
Sending build context to Docker daemon   35.6MB
Step 1/8 : FROM openjdk:8-jdk-alpine
 ---> a3562aa0b991
Step 2/8 : RUN addgroup -S spring && adduser -S spring -G spring
 ---> Using cache
 ---> 0b5e90edf7db
Step 3/8 : USER spring:spring
 ---> Using cache
 ---> c834729e9d31
Step 4/8 : ARG DEPENDENCY=target/dependency
 ---> Using cache
 ---> c9623e256de4
Step 5/8 : COPY ${DEPENDENCY}/BOOT-INF/lib /app/lib
 ---> Using cache
 ---> 69a7896fce2f
Step 6/8 : COPY ${DEPENDENCY}/META-INF /app/META-INF
 ---> f7eeb2f515f6
Step 7/8 : COPY ${DEPENDENCY}/BOOT-INF/classes /app
 ---> 23822d3f9fd8
Step 8/8 : ENTRYPOINT ["java","-cp","app:app/lib/*","jiangbo.spring.docker.SpringDockerApplication"]
 ---> Running in a0001411f73c
Removing intermediate container a0001411f73c
 ---> 6c6f105fb47c
Successfully built 6c6f105fb47c
Successfully tagged gs-spring-boot-docker:1.0.1
[root@master demo4]# docker image ls
REPOSITORY              TAG                 IMAGE ID            CREATED             SIZE
gs-spring-boot-docker   1.0.1               6c6f105fb47c        22 seconds ago      122MB
gs-spring-boot-docker   1.0.0               87fc89fd35cb        4 minutes ago       122MB
ubuntu                  latest              d70eaf7277ea        4 weeks ago         72.9MB
openjdk                 8-jdk-alpine        a3562aa0b991        18 months ago       105MB
[root@master demo4]# docker run -d -p8081:8080 gs-spring-boot-docker:1.0.1
0482f6119e0b72a03b47341c2f5eeac4b89659b5b56a840edfcdb67a2a276fa7
[root@master demo4]# curl localhost:8081
Hello Docker World v1.0.1[root@master demo4]#
```

### 比较分层的大小

```sh
[root@master demo4]# docker save gs-spring-boot-docker:1.0.0 gs-spring-boot-docker:1.0.1 | gzip > images3.tar.gz
[root@master demo4]# ll -h
total 102M
-rw-r--r--. 1 root root 346 Nov 25 22:14 Dockerfile
-rw-r--r--. 1 root root 85M Nov 25 22:17 images3.tar.gz
-rw-r--r--. 1 root root 17M Nov 25 21:44 spring-docker-1.0.1.jar
drwxr-xr-x. 3 root root  24 Nov 25 22:15 target
[root@master demo4]#
```

> 相比较之前的 100M，这次减少了 15M。除了共用的基础 openjdk 镜像，还共用了 spring 的 jar 包层，变化的就只是我们编写的代码层。

## spring 2.3 构建镜像

### 直接构建镜像

可以直接使用 spring-boot:build-image 构建镜像，需要有 Docker 环境，并且会从 github 上下载文件，大概率会失败。

### 生成分层 jar

Spring Boot 2.3 之后，就可以生成分层 jar 了。

```xml
<parent>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-parent</artifactId>
    <version>2.4.0</version>
    <relativePath /> <!-- lookup parent from repository -->
</parent>
```

### 查看分层

```sh
[root@master demo5]#  java -Djarmode=layertools -jar spring-docker-1.0.3.jar list
dependencies
spring-boot-loader
snapshot-dependencies
application
[root@master demo5]#
```

### 定义 Dockerfile

```sh
[root@master demo5]# cat Dockerfile
FROM openjdk:8-jdk-alpine as builder
WORKDIR application
ARG JAR_FILE=target/*.jar
COPY ${JAR_FILE} application.jar
RUN java -Djarmode=layertools -jar application.jar extract

FROM openjdk:8-jdk-alpine
WORKDIR application
COPY --from=builder application/dependencies/ ./
COPY --from=builder application/snapshot-dependencies/ ./
COPY --from=builder application/application/ ./
ENTRYPOINT ["java", "org.springframework.boot.loader.JarLauncher"]

[root@master demo5]#
```

### 整包生成镜像

```sh
[root@master demo5]# docker build -t spring-docker:1.0.3 --build-arg JAR_FILE=spring-docker-1.0.3.jar .
Sending build context to Docker daemon  34.68MB
Step 1/11 : FROM openjdk:8-jdk-alpine as builder
 ---> a3562aa0b991
Step 2/11 : WORKDIR application
 ---> Using cache
 ---> a71643e49255
Step 3/11 : ARG JAR_FILE=target/*.jar
 ---> Using cache
 ---> 6662d39ccdf1
Step 4/11 : COPY ${JAR_FILE} application.jar
 ---> Using cache
 ---> 0949a32dea3f
Step 5/11 : RUN java -Djarmode=layertools -jar application.jar extract
 ---> Using cache
 ---> d5bbe3e76dd6
Step 6/11 : FROM openjdk:8-jdk-alpine
 ---> a3562aa0b991
Step 7/11 : WORKDIR application
 ---> Using cache
 ---> a71643e49255
Step 8/11 : COPY --from=builder application/dependencies/ ./
 ---> Using cache
 ---> 01abc9655f2d
Step 9/11 : COPY --from=builder application/snapshot-dependencies/ ./
 ---> Using cache
 ---> 75a072a91176
Step 10/11 : COPY --from=builder application/application/ ./
 ---> c2b6469091f0
Step 11/11 : ENTRYPOINT ["java", "org.springframework.boot.loader.JarLauncher"]
 ---> Running in 7e4922e3f93d
Removing intermediate container 7e4922e3f93d
 ---> 49522222e638
Successfully built 49522222e638
Successfully tagged spring-docker:1.0.3
```

### 查看 2.3 的分层

```sh
[root@master demo5]# docker history 49522222e638
IMAGE               CREATED             CREATED BY                                      SIZE                COMMENT
49522222e638        8 minutes ago       /bin/sh -c #(nop)  ENTRYPOINT ["java" "org.s…   0B
c2b6469091f0        8 minutes ago       /bin/sh -c #(nop) COPY dir:d414716117e4bcaac…   3.95kB
75a072a91176        9 minutes ago       /bin/sh -c #(nop) COPY dir:8b993266a653e9e77…   0B
01abc9655f2d        9 minutes ago       /bin/sh -c #(nop) COPY dir:ad93c3ca68c6d39b5…   16.8MB
a71643e49255        9 minutes ago       /bin/sh -c #(nop) WORKDIR /application          0B
a3562aa0b991        18 months ago       /bin/sh -c set -x  && apk add --no-cache   o…   99.3MB
<missing>           18 months ago       /bin/sh -c #(nop)  ENV JAVA_ALPINE_VERSION=8…   0B
<missing>           18 months ago       /bin/sh -c #(nop)  ENV JAVA_VERSION=8u212       0B
<missing>           18 months ago       /bin/sh -c #(nop)  ENV PATH=/usr/local/sbin:…   0B
<missing>           18 months ago       /bin/sh -c #(nop)  ENV JAVA_HOME=/usr/lib/jv…   0B
<missing>           18 months ago       /bin/sh -c {   echo '#!/bin/sh';   echo 'set…   87B
<missing>           18 months ago       /bin/sh -c #(nop)  ENV LANG=C.UTF-8             0B
<missing>           18 months ago       /bin/sh -c #(nop)  CMD ["/bin/sh"]              0B
<missing>           18 months ago       /bin/sh -c #(nop) ADD file:a86aea1f3a7d68f6a…   5.53MB
```

可以看到，已经进行了分层，并不是整个 jar 包构建。

## 总结

介绍了 Spring Boot 生成镜像的方式，比较多种生成镜像方式后镜像的大小。