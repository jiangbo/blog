# JavaWeb：HttpServletRequestWrapper

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## 概述

在 Servlet API 中有四个包装类，它们的作用类似，这里以 HttpServletRequestWrapper 为例学习。
四个包装类分别是：

- ServletRequestWrapper
- HttpServletRequestWrapper
- ServletResponseWrapper
- HttpServletResponseWrapper

## HttpServletRequestWrapper 说明

HttpServletRequestWrapper 主要是 HttpServletRequest 的包装，
它提供了 HttpServletRequest 的所有方法，主要作用是改变 HttpServletRequest 的行为。

## HttpServletRequestWrapper 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>17java-web-http-servlet-request-wrapper</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Http Servlet Request Wrapper 的示例</description>

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

### Filter01

```java
package jiangbo.javaweb;

import java.io.IOException;

import javax.servlet.Filter;
import javax.servlet.FilterChain;
import javax.servlet.FilterConfig;
import javax.servlet.ServletException;
import javax.servlet.ServletRequest;
import javax.servlet.ServletResponse;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletRequestWrapper;

public class Filter01 implements Filter {

    @Override
    public void init(FilterConfig filterConfig) throws ServletException {
    }

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain)
            throws IOException, ServletException {
        chain.doFilter(new KeyWordWrapper(request), response);
    }

    @Override
    public void destroy() {
    }

    static class KeyWordWrapper extends HttpServletRequestWrapper {

        private static final String KEY_WORD = "java";

        public KeyWordWrapper(ServletRequest request) {
            super((HttpServletRequest) request);
        }

        @Override
        public String getParameter(String name) {
            return super.getParameter(name).replaceAll(KEY_WORD, "****");
        }
    }
}
```

### 配置 web.xml

```xml
<web-app xmlns="http://xmlns.jcp.org/xml/ns/javaee" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://xmlns.jcp.org/xml/ns/javaee http://xmlns.jcp.org/xml/ns/javaee/web-app_3_1.xsd"
    version="3.1">

    <servlet>
        <servlet-name>servlet01</servlet-name>
        <servlet-class>jiangbo.javaweb.Servlet01</servlet-class>
    </servlet>

    <servlet-mapping>
        <servlet-name>servlet01</servlet-name>
        <url-pattern>/servlet01</url-pattern>
    </servlet-mapping>
    
    <filter>
        <filter-name>filter01</filter-name>
        <filter-class>jiangbo.javaweb.Filter01</filter-class>
    </filter>
    <filter-mapping>
        <filter-name>filter01</filter-name>
        <servlet-name>servlet01</servlet-name>
</filter-mapping>
    
</web-app>
```

### 浏览器访问

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/17java-web-http-servlet-request-wrapper/servlet01?name=javascript 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
name: ****script
```