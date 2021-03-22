# JavaWeb：ServletRequestAttributeListener

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## 概述

域对象中属性的监听器，可以监听属性的设置、改变和移除。
一共有三个，这里以 ServletRequestAttributeListener 为例学习。

- ServletRequestAttributeListener
- HttpSessionAttributeListener
- ServletContextAttributeListener

## ServletRequestAttributeListener 说明

当 ServletRequestAttributeListener 监听器被注册后，有事件发生后，
会返回一个 ServletRequestAttributeEvent 事件对象。

### attributeAdded

当有属性被添加的时候被调用。

```java
public void attributeAdded(ServletRequestAttributeEvent srae);
```

### attributeRemoved

当有属性被移除的时候被调用。

```java
public void attributeRemoved(ServletRequestAttributeEvent srae);
```

### attributeReplaced

当有属性被替换的时候被调用。

```java
public void attributeReplaced(ServletRequestAttributeEvent srae);
```

## ServletRequestAttributeListener 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>19java-web-servlet-request-attribute-listener</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Request Attribute Listener 的示例</description>

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
        req.setAttribute("username", "将波");
        req.setAttribute("username", "波将");
        req.removeAttribute("username");
    }
}
```

### Listener01

```java
package jiangbo.javaweb;

import javax.servlet.ServletRequestAttributeEvent;
import javax.servlet.ServletRequestAttributeListener;

public class Listener01 implements ServletRequestAttributeListener {

    @Override
    public void attributeAdded(ServletRequestAttributeEvent srae) {

        System.out.println("新增了属性 " + srae.getName() + ":" + srae.getValue());
    }

    @Override
    public void attributeRemoved(ServletRequestAttributeEvent srae) {

        System.out.println("删除了属性 " + srae.getName() + ":" + srae.getValue());
    }

    @Override
    public void attributeReplaced(ServletRequestAttributeEvent srae) {

        System.out.println("替换了属性 " + srae.getName() + ":" + srae.getValue() + "--->"
                + srae.getServletRequest().getAttribute(srae.getName()));
    }
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/19java-web-servlet-request-attribute-listener/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
替换了属性 org.apache.catalina.ASYNC_SUPPORTED:true--->false
service...
新增了属性 username:将波
替换了属性 username:将波--->波将
删除了属性 username:波将
```