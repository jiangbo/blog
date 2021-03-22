# JavaWeb：ServletContext（一）

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## ServletContext 说明

ServletContext 接口是 web 服务器帮我们实现的，
我们只需要获取到 servletContext的实现，进行使用就可以了。
ServletContext 是 web 应用的上下文，一个 web 应用只有一个 ServletContext 的实现。

在 Servlet 3.0 之前，ServletContext 包含两个属性和二十四个方法，其中有四个方法已经过时，不再说明，它们分别是：

- `Servlet getServlet(String name)` 根据名称获取 Servlet。
- `Enumeration<String> getServletNames()` 获取所有的 Servlet 名称。
- `Enumeration<Servlet> getServlets()` 获得所有的 Servlet。
- `void log(Exception exception, String msg)` 日志记录异常栈信息。

下面将介绍一部分方法，其中 Servlet 3.0 之后的方法不介绍。

### TEMPDIR 属性

Servlet 3.0 之后才有，值为 javax.servlet.context.tempdir。定义 Servlet 容器的临时工作目录，每个 web 应用独立、不可见。JSP 生成的 java 类和 class 就在里面。tomcat 默认工作目录在 work 目录下面。

### ORDERED_LIBS 属性

Servlet 3.0 之后才有，值为 javax.servlet.context.orderedLibs。它的值（ `java.util.List<java.lang.String>` 类型）包含了 web 应用的 WEB-INF/lib 目录中的 JAR 文件的名字列表。如果没有指定绝对路径或者相对路径的话，返回 null。

### getContextPath

获得上下文路径，以 / 开头，不以 / 结尾。

```java
public String getContextPath();
```

### getContext

通过给定的路径，获取同一台服务器中其它 web 应用的 ServletContext。没有或者在强安全的环境中，会返回 null。tomcat 默认返回 null，想打开需要在 server.xml 中的 Context 节点设置 `crossContext="true"`。

```java
public ServletContext getContext(String uripath);
```

### getMajorVersion

获得该 Servlet 容器支持的 Servlet API 的主版本号。Tomcat 8.5 支持的 Servlet API 的版本是 3.1 。所以主版本号是 3 。

```java
public int getMajorVersion();
```

### getMinorVersion

获得该 Servlet 容器支持的 Servlet API 的次版本号。Tomcat 8.5 支持的 Servlet API 的版本是 3.1 。所以次版本号是 1 。

```java
public int getMinorVersion();
```

### getMimeType

获得指定文件的媒体类型，MIME（Media types）,常见得媒体类型有网页 text/html。想看完整的媒体类型，可以[点击这里][1]。

```java
public String getMimeType(String file);
```

### getServerInfo

获得运行该 Web 应用的服务器信息，格式为：服务器名称/服务器版本号，除了规定的内容外，还可以返回其它额外的信息。

```java
public String getServerInfo();
```

### getServletContextName

获得 ServletContext 的名称，也就是在 web.xml 中 display-name 节点的内容，没有就返回 null。

```java
public String getServletContextName();
```

### log

记录一些信息到日志文件，该日志文件的名称和类型由 Servlet 容器指定。
在 Tomcat 中日志配置在 conf 目录下，`log(String msg)` 的日志级别是 INFO，
`log(String message, Throwable throwable)` 会记录异常堆栈，并且日志的级别是 SEVERE 。
在 Eclipse 中配置的 Tomcat 没有配置日志的话，会直接打印到控制台中，详细信息[点击这里][2]。

```java
public void log(String msg);
public void log(String message, Throwable throwable);
```

## ServletContext 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>04java-web-servlet-context</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Contxt 的示例（一）</description>

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

import javax.servlet.Servlet;
import javax.servlet.ServletConfig;
import javax.servlet.ServletContext;
import javax.servlet.ServletException;
import javax.servlet.ServletRequest;
import javax.servlet.ServletResponse;

public class Servlet01 implements Servlet {

    @Override
    public void init(ServletConfig config) throws ServletException {

        System.out.println("init...");

        ServletContext servletcontext = config.getServletContext();
        System.out.println(servletcontext.getContextPath());
        System.out.println(servletcontext.getMajorVersion());
        System.out.println(servletcontext.getMinorVersion());
        System.out.println(servletcontext.getMimeType("text.html"));
        System.out.println(servletcontext.getServerInfo());
        System.out.println(servletcontext.getServletContextName());
    }

    @Override
    public ServletConfig getServletConfig() {

        return null;
    }

    @Override
    public void service(ServletRequest req, ServletResponse res) throws ServletException, IOException {
        System.out.println("service...");
    }

    @Override
    public String getServletInfo() {
        return null;
    }

    @Override
    public void destroy() {
        System.out.println("destroy...");
    }
}
```

### 配置 web.xml

```xml
<web-app xmlns="http://xmlns.jcp.org/xml/ns/javaee" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://xmlns.jcp.org/xml/ns/javaee http://xmlns.jcp.org/xml/ns/javaee/web-app_3_1.xsd"
    version="3.1">

    <display-name>jiangbo web</display-name>

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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/04java-web-servlet-context/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
init...
/04java-web-servlet-context
3
1
text/html
Apache Tomcat/8.5.53
jiangbo web
service...
destroy...
```