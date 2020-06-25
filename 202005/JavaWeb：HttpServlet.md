# JavaWeb：HttpServlet

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## HttpServlet 说明

HttpServlet 继承于 GenericServlet，关于 GenericServlet 的说明参考：[JavaWeb：GenericServlet][1]

### METHOD 属性

在 HttpServlet 中定义了七种 http 请求的方法，分别是：
1. OPTIONS
2. GET
3. HEAD
4. POST
5. PUT
6. DELETE
7. TRACE

我们最常用的是 GET 和 POST。

```java
private static final String METHOD_DELETE = "DELETE";
private static final String METHOD_HEAD = "HEAD";
private static final String METHOD_GET = "GET";
private static final String METHOD_OPTIONS = "OPTIONS";
private static final String METHOD_POST = "POST";
private static final String METHOD_PUT = "PUT";
private static final String METHOD_TRACE = "TRACE";
```

### HEADER 属性

网页过期和缓存相关的两个消息头。

```java
private static final String HEADER_IFMODSINCE = "If-Modified-Since";
private static final String HEADER_LASTMOD = "Last-Modified";
```

### 国际化属性

还有两个和 GenericServlet 类似的国际化属性。

```java
private static final String LSTRING_FILE = "javax.servlet.http.LocalStrings";
private static ResourceBundle lStrings = ResourceBundle.getBundle(LSTRING_FILE);
```

### doXXX 系列方法

根据不同的请求类型，会调用对应的方法。

> 如果请求的方式没有实现，则会给客户端返回一个错误页面。

### service

除了之前的 service 方法，新增了一个 service(HttpServletRequest req, HttpServletResponse resp) 方法，
可以避免对 req 和 resp 的转换，HttpServletRequest、HttpServletResponse、ServletResponse、ServletRequest 之后学习。

### getLastModified

页面缓存相关，获取最后的修改时间，-1 表示时间未知，不缓存。

```Java
protected long getLastModified(HttpServletRequest req) {
    return -1;
}
```

## HttpServlet 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>08java-web-http-servlet</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Http Servlet 的示例</description>

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

### 方法示例

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
        resp.getWriter().write("Hello World!");
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

</web-app>
```

### 浏览器访问

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/08java-web-http-servlet/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
Hello World!
```

## HttpServlet 源码分析

### 请求的转化

HttpServlet 将 ServletRequest 转为了 HttpServletRequest，将 ServletResponse 转为了 HttpServletResponse。

```Java
@Override
public void service(ServletRequest req, ServletResponse res) throws ServletException, IOException {

    HttpServletRequest  request;
    HttpServletResponse response;

    if (!(req instanceof HttpServletRequest && res instanceof HttpServletResponse)) {
        throw new ServletException("non-HTTP request or response");
    }

    request = (HttpServletRequest) req;
    response = (HttpServletResponse) res;

    service(request, response);
}
```

### 请求类型的分发

HttpServlet 将不同的请求类型分发到不同的方法上。可以看到，用的只是普通的 if-else 判断

```java
protected void service(HttpServletRequest req, HttpServletResponse resp) throws ServletException, IOException {

    String method = req.getMethod();

    if (method.equals(METHOD_GET)) {
        long lastModified = getLastModified(req);
        if (lastModified == -1) {
            // servlet doesn't support if-modified-since, no reason
            // to go through further expensive logic
            doGet(req, resp);
        } else {
            long ifModifiedSince = req.getDateHeader(HEADER_IFMODSINCE);
            if (ifModifiedSince < lastModified) {
                // If the servlet mod time is later, call doGet()
                // Round down to the nearest second for a proper compare
                // A ifModifiedSince of -1 will always be less
                maybeSetLastModified(resp, lastModified);
                doGet(req, resp);
            } else {
                resp.setStatus(HttpServletResponse.SC_NOT_MODIFIED);
            }
        }

    } else if (method.equals(METHOD_HEAD)) {
        long lastModified = getLastModified(req);
        maybeSetLastModified(resp, lastModified);
        doHead(req, resp);

    } else if (method.equals(METHOD_POST)) {
        doPost(req, resp);

    } else if (method.equals(METHOD_PUT)) {
        doPut(req, resp);

    } else if (method.equals(METHOD_DELETE)) {
        doDelete(req, resp);

    } else if (method.equals(METHOD_OPTIONS)) {
        doOptions(req,resp);

    } else if (method.equals(METHOD_TRACE)) {
        doTrace(req,resp);

    } else {
        //
        // Note that this means NO servlet supports whatever
        // method was requested, anywhere on this server.
        //

        String errMsg = lStrings.getString("http.method_not_implemented");
        Object[] errArgs = new Object[1];
        errArgs[0] = method;
        errMsg = MessageFormat.format(errMsg, errArgs);

        resp.sendError(HttpServletResponse.SC_NOT_IMPLEMENTED, errMsg);
    }
}
```

[1]: https://www.cnblogs.com/jiangbo44/p/12934340.html