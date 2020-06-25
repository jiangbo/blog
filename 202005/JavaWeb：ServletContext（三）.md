# JavaWeb：ServletContext（三）

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

### getInitParameter

根据给定的名称，获取 Web 应用的初始化参数。

```java
public String getInitParameter(String name);
```

### getInitParameterNames

获取 Web 应用的所有初始化参数的名称。

```java
public Enumeration<String> getInitParameterNames();
```

### getAttributeNames

获得 ServletContext 中的所有属性的名称。

```java
public Enumeration<String> getAttributeNames();
```

### getAttribute

根据给定的名称，获得 Servlet 容器中得值。

```java
public Object getAttribute(String name);
```

### setAttribute

设值到 ServletContext 中。

```java
public void setAttribute(String name, Object object);
```

### removeAttribute

根据名称删除 ServletContext 中的值。

```java
public void removeAttribute(String name);
```

## ServletContext 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>06java-web-servlet-context</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Contxt 的示例（三）</description>

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
import java.util.Enumeration;

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

        Enumeration<String> names = servletcontext.getInitParameterNames();
        while (names.hasMoreElements()) {
            String name = names.nextElement();
            System.out.println(name + ":" + servletcontext.getInitParameter(name));
        }

        System.out.println("------------------------------------");
        names = servletcontext.getAttributeNames();
        while (names.hasMoreElements()) {
            String name = names.nextElement();
            System.out.println(name + ":" + servletcontext.getAttribute(name));
        }

        System.out.println("------------------------------------");
        servletcontext.setAttribute("u", "jiangbo");
        System.out.println(servletcontext.getAttribute("u"));
        servletcontext.removeAttribute("u");
        System.out.println(servletcontext.getAttribute("u"));
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

    <context-param>
        <param-name>username</param-name>
        <param-value>jiangbo</param-value>
    </context-param>
    <context-param>
        <param-name>password</param-name>
        <param-value>123456</param-value>
    </context-param>

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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/06java-web-servlet-context/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
init...
password:123456
username:jiangbo
------------------------------------
javax.servlet.context.tempdir:C:\work\workspace\sts\.metadata\.plugins\org.eclipse.wst.server.core\tmp0\work\Catalina\localhost\06java-web-servlet-context
org.apache.catalina.resources:org.apache.catalina.webresources.StandardRoot@419db7e4
org.apache.catalina.webappVersion:
org.apache.tomcat.InstanceManager:...jar
javax.websocket.server.ServerContainer:org.apache.tomcat.websocket.server.WsServerContainer@621276ec
org.apache.jasper.compiler.TldCache:org.apache.jasper.compiler.TldCache@2310cb73
org.apache.tomcat.JarScanner:org.apache.tomcat.util.scan.StandardJarScanner@f1e7a62
------------------------------------
jiangbo
null
service...
destroy...
```