# 【Kubernetes】构建 Spring boot 分层镜像

## 环境

1. docker 19.03
2. kubernetes 1.20.2
3. Spring Boot 2.5.0-M1

## 目标

创建一个 Spring Boot 项目，可以返回主机名和当前项目的版本，采用分层方式打包 jar 并推送到远程仓库。
主要还是之前打包镜像的方式，需要下载文件，很容器失败。

## 创建 Spring Boot 项目

### pom.xml

build-info 是可以添加构建信息，获取项目的版本号使用的。

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
    <groupId>jiangbo.spring.docker</groupId>
    <artifactId>spring-docker</artifactId>
    <version>1.0.1</version>
    <name>spring-boot-docker</name>
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

### 主机名

```java
package jiangbo.spring.docker;

import java.net.InetAddress;
import java.net.UnknownHostException;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.info.BuildProperties;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class SpringDockerApplication {

    private final BuildProperties buildProperties;

    public SpringDockerApplication(BuildProperties buildProperties) {

        this.buildProperties = buildProperties;
    }

    private static String hostname;

    public static void main(String[] args) throws UnknownHostException {

        hostname = InetAddress.getLocalHost().getHostName();
        SpringApplication.run(SpringDockerApplication.class, args);
    }

    @GetMapping("/hostname")
    public String hello() {

        return hostname + " " + buildProperties.getVersion();
    }

}
```

### Dockerfile

Dockerfile 文件放到项目的根目录。

```Dockerfile
FROM openjdk:8-jre-alpine as builder
ARG JAR_FILE=target/*.jar
COPY ${JAR_FILE} application.jar
RUN java -Djarmode=layertools -jar application.jar extract

FROM openjdk:8-jre-alpine
COPY --from=builder dependencies/ ./
COPY --from=builder snapshot-dependencies/ ./
COPY --from=builder spring-boot-loader/ ./
COPY --from=builder application/ ./
ENTRYPOINT ["java", "org.springframework.boot.loader.JarLauncher"]
```

## 打包镜像

### 本地 Docker 环境

```powershell
PS D:\workspace\sts\spring-docker> docker version -f "{{.Client.Version}}"
20.10.2
```

如果没有安装 Docker，可以参考：[Docker：在 wsl2 上安装 docker][1]
安装完成后，最好安装一个桌面使用的 Docker Desktop。

### 构建项目

```powershell
PS D:\workspace\sts\spring-docker> mvn clean package
[INFO] Scanning for projects...
[INFO]
[INFO] ----------------< jiangbo.spring.docker:spring-docker >-----------------
[INFO] Building spring-boot-docker 1.0.1
[INFO] --------------------------------[ jar ]---------------------------------
[INFO]
[INFO] --- maven-clean-plugin:3.1.0:clean (default-clean) @ spring-docker ---
[INFO] Deleting D:\workspace\sts\spring-docker\target
[INFO]
[INFO] --- spring-boot-maven-plugin:2.5.0-M1:build-info (build-info) @ spring-docker ---
[INFO]
[INFO] --- maven-resources-plugin:3.2.0:resources (default-resources) @ spring-docker ---
[INFO] Using 'UTF-8' encoding to copy filtered resources.
[INFO] Using 'UTF-8' encoding to copy filtered properties files.
[INFO] Copying 1 resource
[INFO] Copying 0 resource
[INFO]
[INFO] --- maven-compiler-plugin:3.8.1:compile (default-compile) @ spring-docker ---
[INFO] Changes detected - recompiling the module!
[INFO] Compiling 1 source file to D:\workspace\sts\spring-docker\target\classes
[INFO]
[INFO] --- maven-resources-plugin:3.2.0:testResources (default-testResources) @ spring-docker ---
[INFO] Using 'UTF-8' encoding to copy filtered resources.
[INFO] Using 'UTF-8' encoding to copy filtered properties files.
[INFO] skip non existing resourceDirectory D:\workspace\sts\spring-docker\src\test\resources
[INFO]
[INFO] --- maven-compiler-plugin:3.8.1:testCompile (default-testCompile) @ spring-docker ---
[INFO] Changes detected - recompiling the module!
[INFO]
[INFO] --- maven-surefire-plugin:2.22.2:test (default-test) @ spring-docker ---
[INFO]
[INFO] --- maven-jar-plugin:3.2.0:jar (default-jar) @ spring-docker ---
[INFO] Building jar: D:\workspace\sts\spring-docker\target\spring-docker-1.0.1.jar
[INFO]
[INFO] --- spring-boot-maven-plugin:2.5.0-M1:repackage (repackage) @ spring-docker ---
[INFO] Replacing main artifact with repackaged archive
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time:  2.723 s
[INFO] Finished at: 2021-01-24T10:33:39+08:00
[INFO] ------------------------------------------------------------------------
```

### 打包镜像

```powershell
PS D:\workspace\sts\spring-docker> set DOCKER_BUILDKIT=1 && docker build -t jiangbo920827/spring-docker:1.0.1 .
Sending build context to Docker daemon  17.27MB
Step 1/10 : FROM openjdk:8-jre-alpine as builder
 ---> f7a292bbb70c
Step 2/10 : ARG JAR_FILE=target/*.jar
 ---> Using cache
 ---> 061b93704007
Step 3/10 : COPY ${JAR_FILE} application.jar
 ---> 513a1b35a087
Step 4/10 : RUN java -Djarmode=layertools -jar application.jar extract
 ---> Running in e858a0b32c11
Removing intermediate container e858a0b32c11
 ---> cbe7ec568f03
Step 5/10 : FROM openjdk:8-jre-alpine
 ---> f7a292bbb70c
Step 6/10 : COPY --from=builder dependencies/ ./
 ---> Using cache
 ---> 711998faf56e
Step 7/10 : COPY --from=builder snapshot-dependencies/ ./
 ---> Using cache
 ---> 4b4747144e36
Step 8/10 : COPY --from=builder spring-boot-loader/ ./
 ---> Using cache
 ---> 2bac136552de
Step 9/10 : COPY --from=builder application/ ./
 ---> 1e167e9c75fb
Step 10/10 : ENTRYPOINT ["java", "org.springframework.boot.loader.JarLauncher"]
 ---> Running in 9ee0626dde02
Removing intermediate container 9ee0626dde02
 ---> 45cf9fb94923
Successfully built 45cf9fb94923
Successfully tagged jiangbo920827/spring-docker:1.0.1
SECURITY WARNING: You are building a Docker image from Windows against a non-Windows Docker host. 
All files and directories added to build context will have '-rwxr-xr-x' permissions. 
It is recommended to double check and reset permissions for sensitive files and directories.
```

### 查看镜像

```powershell
PS D:\workspace\sts\spring-docker> docker image ls jiangbo920827/spring-docker
REPOSITORY                    TAG       IMAGE ID       CREATED          SIZE
jiangbo920827/spring-docker   1.0.1     45cf9fb94923   49 seconds ago   102MB
jiangbo920827/spring-docker   1.0.0     ba6729253f7d   41 years ago     244MB
```

### 查看构建历史

```powershell
PS D:\workspace\sts\spring-docker> docker image history jiangbo920827/spring-docker:1.0.1
IMAGE          CREATED              CREATED BY                                      SIZE      COMMENT
45cf9fb94923   About a minute ago   /bin/sh -c #(nop)  ENTRYPOINT ["java" "org.s…   0B
1e167e9c75fb   About a minute ago   /bin/sh -c #(nop) COPY dir:0f25ac2bcafd4948d…   5.68kB
2bac136552de   22 minutes ago       /bin/sh -c #(nop) COPY dir:412e548e4ebc6bb74…   243kB
4b4747144e36   22 minutes ago       /bin/sh -c #(nop) COPY dir:3875f37b8a0ed7494…   0B
711998faf56e   22 minutes ago       /bin/sh -c #(nop) COPY dir:efc10934d3d2bd7a3…   17MB
f7a292bbb70c   20 months ago        /bin/sh -c set -x  && apk add --no-cache   o…   79.4MB
<missing>      20 months ago        /bin/sh -c #(nop)  ENV JAVA_ALPINE_VERSION=8…   0B
<missing>      20 months ago        /bin/sh -c #(nop)  ENV JAVA_VERSION=8u212       0B
<missing>      20 months ago        /bin/sh -c #(nop)  ENV PATH=/usr/local/sbin:…   0B
<missing>      20 months ago        /bin/sh -c #(nop)  ENV JAVA_HOME=/usr/lib/jv…   0B
<missing>      20 months ago        /bin/sh -c {   echo '#!/bin/sh';   echo 'set…   87B
<missing>      20 months ago        /bin/sh -c #(nop)  ENV LANG=C.UTF-8             0B
<missing>      20 months ago        /bin/sh -c #(nop)  CMD ["/bin/sh"]              0B
<missing>      20 months ago        /bin/sh -c #(nop) ADD file:a86aea1f3a7d68f6a…   5.53MB
```

### 上传到远程仓库

```powershell
PS D:\workspace\sts\spring-docker> docker push jiangbo920827/spring-docker:1.0.1
The push refers to repository [docker.io/jiangbo920827/spring-docker]
c061f84a1560: Layer already exists
92ea1308db64: Pushed
f79eda5bcd70: Layer already exists
edd61588d126: Layer already exists
9b9b7f3d56a0: Layer already exists
f1b5933fe4b5: Layer already exists
1.0.1: digest: sha256:c4a13b0e139e230dc65f69093930b4d999bbc51490993c3c3b43f06c0437ae6c size: 1576
```

## 总结

介绍了编写 Spring Boot 项目，返回主机名和当前项目版本，并使用分层 jar 的方式打包成镜像，推送到远程仓库。

[1]: https://www.cnblogs.com/jiangbo44/p/12637389.html

## 附录