# Spring cloud：环境搭建

## 环境

1. spring cloud Edgware.SR6
2. jdk 7

## 背景

在之前，使用 spring boot 的时候，可以使用 jdk6，但是如果要使用 spring cloud，则 jdk 要求 7 以上。

## 搭建步骤

### 使用 jdk6

如果使用 jdk6 ，那么在启动 spring cloud 的时候，会得到一个错误：

```text
Caused by: java.lang.UnsupportedClassVersionError: org/springframework/cloud/bootstrap/BootstrapApplicationListener : Unsupported major.minor version 51.0
    at java.lang.ClassLoader.defineClass1(Native Method)
    at java.lang.ClassLoader.defineClassCond(ClassLoader.java:631)
    at java.lang.ClassLoader.defineClass(ClassLoader.java:615)
    at java.security.SecureClassLoader.defineClass(SecureClassLoader.java:141)
    at java.net.URLClassLoader.defineClass(URLClassLoader.java:283)
    at java.net.URLClassLoader.access$000(URLClassLoader.java:58)
    at java.net.URLClassLoader$1.run(URLClassLoader.java:197)
    at java.security.AccessController.doPrivileged(Native Method)
    at java.net.URLClassLoader.findClass(URLClassLoader.java:190)
    at org.springframework.boot.loader.LaunchedURLClassLoader.doLoadClass(LaunchedURLClassLoader.java:163)
    at org.springframework.boot.loader.LaunchedURLClassLoader.loadClass(LaunchedURLClassLoader.java:136)
    at java.lang.ClassLoader.loadClass(ClassLoader.java:247)
    at org.springframework.util.ClassUtils.forName(ClassUtils.java:249)
    at org.springframework.boot.SpringApplication.getSpringFactoriesInstances(SpringApplication.java:374)
    ... 12 more
```

所以最低的 jdk 要求是 7，更多的详细信息，可以参考：

https://stackoverflow.com/questions/30878637/which-java-version-is-supported-by-spring-cloud 和

https://github.com/spring-cloud/spring-cloud-commons/blob/master/README.adoc

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
    <artifactId>demo</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>

    <properties>
        <java.version>1.7</java.version>
    </properties>

    <dependencies>

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

之后开发 spring cloud 的组件，就在这个基础上导入依赖。

### 启动类

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

启动类，现在还不能直接启动，因为没有相应的依赖和配置。