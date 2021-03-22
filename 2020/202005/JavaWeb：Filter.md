# JavaWeb：Filter

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## Filter 说明

Filter 和 Servlet 类似，需要在 web.xml 中配置，其中执行顺序和配置顺序相关。过滤器的主要作用是：

- 认证
- 日志
- 图片转换
- 压缩
- 加密

### init

和 Servlet 的 init 方法类似，filterConfig 和 ServletConfig 类似。

```java
public void init(FilterConfig filterConfig) throws ServletException;
```

### doFilter

doFilter 可以拦截请求和响应。FilterChain 是过滤链，是一个接口，只有一个 doFilter 方法。
拦截方式可以选择 url 或者 某个 Servlet 。拦截的方式可以选择：

- REQUEST 直接请求，默认
- FORWARD 转发
- INCLUDE 包含
- ERROR 出错
- ASYNC Servlet 3.0 之后的

```java
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException;
```

### destroy

Filter 销毁时调用。

```java
public void destroy();
```

## Filter 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>16java-web-filter</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Filter 的示例</description>

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

public class Filter01 implements Filter {

    @Override
    public void init(FilterConfig filterConfig) throws ServletException {

        System.out.println("filter01 init...");
    }

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain)
            throws IOException, ServletException {

        System.out.println("filter01 before");
        chain.doFilter(request, response);
        System.out.println("filter01 after");
    }

    @Override
    public void destroy() {

        System.out.println("filter01 destroy...");
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/16java-web-filter/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
filter01 init...
filter01 before
service...
filter01 after
filter01 destroy...
```