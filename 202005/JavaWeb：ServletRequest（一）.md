# JavaWeb：ServletRequest（一）

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## ServletRequest 说明

ServletRequest 中的 getRealPath 该方法被 ServletContext 中同名方法代替，不建议使用。

### Attribute 系列方法

获得属性、添加属性、删除属性、获得全部属性名，和之前学过的 ServletContext 中的方法一样，只不过 ServletRequest 是将属性设置到 Request 中。

```java
public Object getAttribute(String name);
public Enumeration<String> getAttributeNames();
public void setAttribute(String name, Object o);
public void removeAttribute(String name);
```

### Local 系列方法

获取 web 服务器的信息，主机名、主机地址和主机端口。

```java
public String getLocalName();
public String getLocalAddr();
public int getLocalPort();
public String getServerName();
public int getServerPort();
```

### Remote 系列方法

获取客户端的信息，客户端的名称、地址和端口。

```java
public String getRemoteHost();
public String getRemoteAddr();
public int getRemotePort();
```

### 协议系列方法

```java
public String getProtocol();
public String getScheme();
```

### isSecure

是否安全，比如是否是 https 等。

```Java
public boolean isSecure();
```

## ServletRequest 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>09java-web-servlet-request</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Request 的示例（一）</description>

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

import javax.servlet.ServletException;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

@SuppressWarnings("serial")
public class Servlet01 extends HttpServlet {

    @Override
    protected void doGet(HttpServletRequest req, HttpServletResponse resp) throws ServletException, IOException {

        System.out.println("service...");

        req.setAttribute("user", "jiangbo");
        Enumeration<String> names = req.getAttributeNames();
        while (names.hasMoreElements()) {
            String name = names.nextElement();
            System.out.println(name + ":" + req.getAttribute(name));
        }

        System.out.println("--------------------------------------");

        System.out.println(req.getLocalName());
        System.out.println(req.getLocalAddr());
        System.out.println(req.getLocalPort());
        System.out.println(req.getServerName());
        System.out.println(req.getServerPort());

        System.out.println("--------------------------------------");

        System.out.println(req.getRemoteHost());
        System.out.println(req.getRemoteAddr());
        System.out.println(req.getRemotePort());

        System.out.println("--------------------------------------");

        System.out.println(req.getProtocol());
        System.out.println(req.getScheme());
        System.out.println(req.isSecure());
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/09java-web-servlet-request/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
user:jiangbo
--------------------------------------
0:0:0:0:0:0:0:1
0:0:0:0:0:0:0:1
8080
localhost
8080
--------------------------------------
0:0:0:0:0:0:0:1
0:0:0:0:0:0:0:1
53620
--------------------------------------
HTTP/1.1
http
false
```