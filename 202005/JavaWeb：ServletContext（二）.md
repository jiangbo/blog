# JavaWeb：ServletContext（二）

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

### getResourcePaths

获得给定路径下的目录和文件，只显示一级，其中目录以 / 结尾。给定的参数必须以 / 开头，获得的路径是相对于 Web 应用的根目录。从 Servlet 3.0 开始，会寻找在 /WEB-INF/lib 目录中 jar 里面的 META-INF/resources 目录下的文件和目录。

```java
public Set<String> getResourcePaths(String path);
```

### getResource

寻找路径的方法和上面的方法类似，不过是返回一个 URL 而不是 String 类型的地址。

```java
public URL getResource(String path) throws MalformedURLException;
```

### getResourceAsStream

寻找路径的方法和上面的方法类似，不过返回的是一个输入流。

```java
public InputStream getResourceAsStream(String path);
```

### getRequestDispatcher

根据路径获取 RequestDispatcher，RequestDispatcher 接口之后再学习。

```java
public RequestDispatcher getRequestDispatcher(String path);
```

### getNamedDispatcher

根据名称获取 RequestDispatcher，RequestDispatcher 接口之后再学习。

```java
public RequestDispatcher getNamedDispatcher(String name);
```

### getRealPath

获得给定的路径在磁盘上的绝对路径。

```java
public String getRealPath(String path);
```

## ServletContext 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>05java-web-servlet-context</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Contxt 的示例（二）</description>

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
        System.out.println(servletcontext.getResourcePaths("/"));
        System.out.println(servletcontext.getRequestDispatcher("/servlet01"));
        System.out.println(servletcontext.getNamedDispatcher("servlet01"));
        System.out.println(servletcontext.getRealPath("/"));
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/05java-web-servlet-context/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
init...
[/META-INF/, /WEB-INF/]
org.apache.catalina.core.ApplicationDispatcher@79b6fe94
org.apache.catalina.core.ApplicationDispatcher@5292cb4a
C:\work\workspace\sts\.metadata\.plugins\org.eclipse.wst.server.core\tmp0\wtpwebapps\05java-web-servlet-context\
service...
destroy...
```