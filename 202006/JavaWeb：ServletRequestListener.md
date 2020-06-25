# JavaWeb：ServletRequestListener

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## 概述

域对象监听器，可以监听对象的创建和销毁。
一共有三个，这里以 ServletRequestListener 为例学习。

- ServletRequestListener
- HttpSessionListener
- ServletContextListener

## ServletRequestListener 说明

当 ServletRequestListener 监听器被注册后，有事件发生后，
会返回一个 ServletRequestEvent 事件对象。

### requestInitialized

Request 对象被创建的时候调用。

```java
public void requestInitialized(ServletRequestEvent sre);
```

### requestDestroyed

Request 对象被销毁的时候调用。

```java
public void requestDestroyed(ServletRequestEvent sre);
```

## ServletRequestListener 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>18java-web-servlet-request-listener</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Request Listener 的示例</description>

    <properties>
        <maven.compiler.source>1.8</maven.compiler.source>
        <maven.compiler.target>1.8</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

    <dependencies>

        <dependency>
            <groupId>javax.servlet</groupId>
            <artifactId>javax.servlet-api</artifactId>
            <version>3.1.0</version>
            <scope>provided</scope>
        </dependency>
    </dependencies>
</project>
```

### Servlet01

```java
package jiangbo.javaweb;

import java.io.IOException;

import javax.servlet.ServletException;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

@SuppressWarnings("serial")
public class Servlet01 extends HttpServlet {

    @Override
    protected void doGet(HttpServletRequest req, HttpServletResponse resp) throws ServletException, IOException {

        System.out.println("service...");
        System.out.println("name: " + req.getParameter("name"));
    }
}
```

### Listener01

```java
package jiangbo.javaweb;

import javax.servlet.ServletRequestEvent;
import javax.servlet.ServletRequestListener;

public class Listener01 implements ServletRequestListener {

    @Override
    public void requestDestroyed(ServletRequestEvent sre) {

        System.out.println(sre.getServletRequest() + " 被销毁");
    }

    @Override
}
```

### 配置 web.xml

```xml
<web-app xmlns="http://xmlns.jcp.org/xml/ns/javaee" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://xmlns.jcp.org/xml/ns/javaee http://xmlns.jcp.org/xml/ns/javaee/web-app_3_1.xsd"
    version="3.1">

    <listener>
        <listener-class>jiangbo.javaweb.Listener01</listener-class>
    </listener>

    <servlet>
        <servlet-name>servlet01</servlet-name>
        <servlet-class>jiangbo.javaweb.Servlet01</servlet-class>
    </servlet>

    <servlet-mapping>
        <servlet-name>servlet01</servlet-name>
        <url-pattern>/servlet01</url-pattern>
    </servlet-mapping>

</web-app>
```

### 浏览器访问

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/18java-web-servlet-request-listener/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
org.apache.catalina.connector.RequestFacade@4b771028 被创建
service...
org.apache.catalina.connector.RequestFacade@4b771028 被销毁
```