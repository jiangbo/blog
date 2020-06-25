# spring-boot 使用servlet2.5（四）

## 环境

1. jdk 6
2. tomcat 6.0.53
3. sts 4.4.2
4. maven 3.2.5

## 背景

由于环境限制，还在使用 servlet 2.5，所以需要将 spring boot 进行配置，支持 servlet 2.5，只针对外部服务器部署。
以下所有的操作建立在之前的项目基础上。

### 删除 SpringBootServletInitializer

SpringBootServletInitializer 使用的是 servlet 3.0 的特性，由于未达到，所以该种启动方式无效了，直接删除。
修改后的 DemoApplication 如下：

```java
package jiangbo.demo;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class DemoApplication {

    public static void main(String[] args) {

        SpringApplication.run(DemoApplication.class, args);
    }
}
```

## 增加 spring boot 启动入口

现在，将项目部署到 tomcat6 中，发现 tomcat 正常启动，但是 spring boot 却没有启动，这是因为 spring boot 没有启动入口了。
下面 增加 web.xml，配置启动入口。

### 增加历史依赖

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-legacy</artifactId>
    <version>1.1.0.RELEASE</version>
</dependency>
```

该依赖允许在 web.xml 中来定义 spring boot 的启动点。

### spring boot 启动入口

在 web.xml 中定义 spring boot 的启动入口。web.xml 在放在 webapp/WEB-INF 目录下。

```xml
<?xml version="1.0" encoding="UTF-8"?>
<web-app version="2.5" xmlns="http://java.sun.com/xml/ns/javaee"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://java.sun.com/xml/ns/javaee 
    https://java.sun.com/xml/ns/javaee/web-app_2_5.xsd">

    <context-param>
        <param-name>contextConfigLocation</param-name>
        <!-- 这里换成 spring boot 的启动类 -->
        <param-value>jiangbo.demo.DemoApplication</param-value>
    </context-param>

    <listener>
        <listener-class>org.springframework.boot.legacy.context.web.SpringBootContextLoaderListener</listener-class>
    </listener>

</web-app>
```

## 启动 tomcat6

将项目部署到 tomcat6中，进行启动，可以发现，spring boot 已经正常运行起来了。
访问 localhost:8080/demo，得到一个404错误，这是由于还没有注册 spring mvc 的 dispatcherServlet。

## 注册 dispatcherServlet

使用 web.xml 注册 dispatcherServlet 属于 spring 的内容。

```xml
<servlet>
    <servlet-name>dispatcherServlet</servlet-name>
    <servlet-class>org.springframework.web.servlet.DispatcherServlet</servlet-class>
    <init-param>
        <param-name>contextAttribute</param-name>
        <param-value>org.springframework.web.context.WebApplicationContext.ROOT</param-value>
    </init-param>
    <load-on-startup>1</load-on-startup>
</servlet>

<servlet-mapping>
    <servlet-name>dispatcherServlet</servlet-name>
    <url-pattern>/</url-pattern>
</servlet-mapping>
```

其中的 contextAttribute 表示 spring mvc 没有自己的 spring 容器，使用 spring boot 启动起来的 root 容器。

## 运行并访问

再次部署到 tomcat6 中，可以看到项目正常启动，访问 localhost:8080/demo，可以正常显示 `hello world!`。

## 附录

### 完整 web.xml

```xml
<?xml version="1.0" encoding="UTF-8"?>
<web-app version="2.5" xmlns="http://java.sun.com/xml/ns/javaee"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://java.sun.com/xml/ns/javaee 
    https://java.sun.com/xml/ns/javaee/web-app_2_5.xsd">

    <context-param>
        <param-name>contextConfigLocation</param-name>
        <!-- 这里换成 spring boot 的启动类 -->
        <param-value>jiangbo.demo.DemoApplication</param-value>
    </context-param>

    <listener>
        <listener-class>org.springframework.boot.legacy.context.web.SpringBootContextLoaderListener</listener-class>
    </listener>

    <servlet>
        <servlet-name>dispatcherServlet</servlet-name>
        <servlet-class>org.springframework.web.servlet.DispatcherServlet</servlet-class>
        <init-param>
            <param-name>contextAttribute</param-name>
            <param-value>org.springframework.web.context.WebApplicationContext.ROOT</param-value>
        </init-param>
        <load-on-startup>1</load-on-startup>
    </servlet>

    <servlet-mapping>
        <servlet-name>dispatcherServlet</servlet-name>
        <url-pattern>/</url-pattern>
    </servlet-mapping>

</web-app>
```

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
        <tomcat.version>7.0.59</tomcat.version>
    </properties>

    <dependencies>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-legacy</artifactId>
            <version>1.1.0.RELEASE</version>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-tomcat</artifactId>
            <scope>provided</scope>
        </dependency>

        <dependency>
            <groupId>org.apache.tomcat</groupId>
            <artifactId>tomcat-juli</artifactId>
            <version>${tomcat.version}</version>
            <scope>provided</scope>
        </dependency>

    </dependencies>

</project>
```
