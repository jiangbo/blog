# JavaWeb：HttpServletResponse

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## HttpServletResponse 说明

### 状态码系列属性

HttpServletResponse 中定义了很多以 SC_ 开头的状态码属性，比如我们经常看见的 404 、500 等。

### Header 系列方法

和响应头相关的方法，有几个是 Servlet 3.0 之后的方法。

```java
public boolean containsHeader(String name);
public void setDateHeader(String name, long date);
public void addDateHeader(String name, long date);
public void setHeader(String name, String value);
public void addHeader(String name, String value);
public void setIntHeader(String name, int value);
public void addIntHeader(String name, int value);
public String getHeader(String name);
public Collection<String> getHeaders(String name);
public Collection<String> getHeaderNames();
```

### addCookie

增加一个 Cookie 对象到响应中。

```java
public void addCookie(Cookie cookie);
```

### 状态码

获得和设置状态码，getStatus 是 3.0 之后的方法。

```java
public void setStatus(int sc);
public int getStatus();
```

### 路径编码

在不支持 Cookie 的浏览器上，可以将 Sessionid 编码到路径中。

```java
public String encodeURL(String url);
public String encodeRedirectURL(String url);
```

### sendError

跳转到错误页面。

```java
public void sendError(int sc) throws IOException;
public void sendError(int sc, String msg) throws IOException;
```

### sendRedirect

重定向，客户端跳转，路径：

- 不加 / 表示相对于当前的请求路径。
- 加 / 表示 Servlet 容器的根目录。
- 加 // 表示是一个网络地址。

```java
public void sendRedirect(String location) throws IOException;
```

## HttpServletResponse 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>13java-web-servlet-response</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Http Servlet Response 的示例</description>

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
import java.io.PrintWriter;
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.Collection;

import javax.servlet.ServletException;
import javax.servlet.http.Cookie;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

@SuppressWarnings("serial")
public class Servlet01 extends HttpServlet {

    @Override
    protected void doGet(HttpServletRequest req, HttpServletResponse resp) throws ServletException, IOException {

        System.out.println("service...");

        System.out.println("---------------------------");
        resp.setIntHeader("Expires", -1);
        Collection<String> names = resp.getHeaderNames();
        System.out.println(names.size());
        for (String name : names) {
            System.out.println(name + ":" + resp.getHeader(name));
        }

        System.out.println("---------------------------");
        resp.addCookie(new Cookie("cname", "将波"));
        System.out.println(resp.getStatus());

        String name = req.getParameter("name");
        if (name == null) {

            String url = URLEncoder.encode("将波", StandardCharsets.UTF_8);
            resp.sendRedirect("servlet01?name=" + url);
        } else {
            System.out.println("name: " + name);
            resp.setContentType("text/html;charset=UTF-8");
            PrintWriter writer = resp.getWriter();
            writer.write(name);
            writer.flush();
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

</web-app>
```

### 浏览器访问

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/13java-web-http-servlet-response/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
---------------------------
1
Expires:-1
---------------------------
200
service...
---------------------------
1
Expires:-1
---------------------------
200
name: 将波
```

### 浏览器

由于进行了重定向，所以浏览器地址变化成了 http://localhost:8080/13java-web-http-servlet-response/servlet01?name=将波 ，并且页面上显示如下内容：

```text
将波
```