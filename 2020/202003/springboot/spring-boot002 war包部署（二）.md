# spring-boot war包部署（二）

## 环境

1. jdk 8
2. tomcat 8.5
3. sts 4.4.2
4. maven 3.6.1

## 背景

有时候，服务器已经有了，我们必须要使用 war 包进行部署，所以需要 spring boot 支持打包和部署成 war。
本节内容在上一节的基础上进行操作。

### 修改 pom.xml

由于需要将打包方式从 jar 修改为 war，所以需要修改 maven 默认的打包方式。

```xml
<packaging>war</packaging>
```

## 去除打包插件

因为我们直接部署到外部的服务器，不需要 spring boot 帮我们打包成 jar 来运行，所以去掉打包插件。
删除如下的内容：

```xml
<build>
    <plugins>
        <plugin>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-maven-plugin</artifactId>
        </plugin>
    </plugins>
</build>
```

## 继承 SpringBootServletInitializer

ServletContainerInitializer 接口，是 servlet 3.0 提供的一个接口，可以不使用 web.xml 配置的方式启动 spring。
SpringBootServletInitializer 类就是应用了这个特性，所以 spring boot 默认不支持 servlet 3.0 以下的版本。

```java
package jiangbo.demo;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.builder.SpringApplicationBuilder;
import org.springframework.boot.web.support.SpringBootServletInitializer;

@SpringBootApplication
public class DemoApplication extends SpringBootServletInitializer {

    @Override
    protected SpringApplicationBuilder configure(SpringApplicationBuilder builder) {

        return builder.sources(DemoApplication.class);
    }

    public static void main(String[] args) {

        SpringApplication.run(DemoApplication.class, args);
    }
}
```

可以使用 configure 方法来启动 spring boot 应用。

## 排除内置 tomcat

因为提供了自己的应用服务器，所以内置的 tomcat 就可以去掉了。排除掉后，出现了一个找不到 servlet api 的错误，加上就好了。

```xml
<dependency>
    <groupId>javax.servlet</groupId>
    <artifactId>javax.servlet-api</artifactId>
    <scope>provided</scope>
</dependency>

<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-web</artifactId>
    <exclusions>
        <exclusion>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-tomcat</artifactId>
        </exclusion>
    </exclusions>
</dependency>
```

## 运行并访问

将修改后的项目部署到自己的 tomcat 中，启动并访问 localhost:8080/demo，得到和之前一样的效果。
由于是部署到自己的 tomcat 中，所以不要忘记加上 context path。

## 问题

这样修改后，可以部署到外面的服务器上了，但是运行 main 方法的时候，发现运行出错。
这是由于把内置的 tomcat 排除了造成的。那么，可以同时满足使用 main 方法运行，又部署到外面的服务器上吗？

## 解决

将之前排除的 tomcat 撤销，将增加的 servlet api 也撤销，再进行如下的修改：

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-tomcat</artifactId>
    <scope>provided</scope>
</dependency>
```

这样，现在的项目既支持从 main 方法启动，又支持部署到外部的服务器。

## 附录

### 完整 pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>1.5.22.RELEASE</version>
    </parent>

    <groupId>jiangbo.demo</groupId>
    <artifactId>demo</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>

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
            <artifactId>spring-boot-starter-tomcat</artifactId>
            <scope>provided</scope>
        </dependency>

    </dependencies>
</project>
```
