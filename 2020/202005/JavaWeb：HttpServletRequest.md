# JavaWeb：HttpServletRequest

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## HttpServletRequest 说明

HttpServletRequest 中的 isRequestedSessionIdFromUrl 方法被 isRequestedSessionIdFromURL 代替，不建议使用。

### 认证

```java
public static final String BASIC_AUTH = "BASIC";
public static final String FORM_AUTH = "FORM";
public static final String CLIENT_CERT_AUTH = "CLIENT_CERT";
public static final String DIGEST_AUTH = "DIGEST";
public String getAuthType();
public String getRemoteUser();
public java.security.Principal getUserPrincipal();
public boolean isUserInRole(String role);
```

### Header 系列方法

获取请求头信息的系列方法。

```java
public long getDateHeader(String name);
public String getHeader(String name);
public Enumeration<String> getHeaders(String name);
public Enumeration<String> getHeaderNames();
public int getIntHeader(String name);
```

### 请求系列方法

获取请求路径相关方法。

```java
public String getContextPath();
public String getMethod();
public String getPathInfo();
public String getPathTranslated();
public String getQueryString();
public String getRequestURI();
public StringBuffer getRequestURL();
public String getServletPath();
```

### 会话系列方法

```java
public Cookie[] getCookies();
public String getRequestedSessionId();
public HttpSession getSession();
public HttpSession getSession(boolean create);
public boolean isRequestedSessionIdValid();
public boolean isRequestedSessionIdFromCookie();
public boolean isRequestedSessionIdFromURL();
```

## HttpServletRequest 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>11java-web-http-servlet-request</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Http Servlet Request 的示例</description>

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
import java.util.Arrays;
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

        System.out.println("---------------------------");
        System.out.println(req.getAuthType());
        System.out.println(req.getRemoteUser());
        System.out.println(req.getUserPrincipal());

        System.out.println("---------------------------");
        Enumeration<String> names = req.getHeaderNames();
        while (names.hasMoreElements()) {
            String name = names.nextElement();
            System.out.println(name + ":" + req.getHeader(name));
        }
        System.out.println("---------------------------");
        System.out.println(req.getContextPath());
        System.out.println(req.getMethod());
        System.out.println(req.getPathInfo());
        System.out.println(req.getPathTranslated());
        System.out.println(req.getQueryString());
        System.out.println(req.getRequestURI());
        System.out.println(req.getRequestURL());
        System.out.println(req.getServletPath());

        System.out.println("---------------------------");
        System.out.println(Arrays.toString(req.getCookies()));
        System.out.println(req.getRequestedSessionId());

        // 如果没有 session 则创建一个
        System.out.println(req.getSession());

        // 如果没有 session 则返回 null
        System.out.println(req.getSession(false));

        System.out.println(req.isRequestedSessionIdValid());
        System.out.println(req.isRequestedSessionIdFromCookie());
        System.out.println(req.isRequestedSessionIdFromURL());
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/11java-web-http-servlet-request/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
---------------------------
null
null
null
---------------------------
host:localhost:8080
connection:keep-alive
upgrade-insecure-requests:1
user-agent:Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.61 Safari/537.36 Edg/83.0.478.37
accept:text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
sec-fetch-site:none
sec-fetch-mode:navigate
sec-fetch-user:?1
sec-fetch-dest:document
accept-encoding:gzip, deflate, br
accept-language:zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6
---------------------------
/11java-web-http-servlet-request
GET
null
null
null
/11java-web-http-servlet-request/servlet01
http://localhost:8080/11java-web-http-servlet-request/servlet01
/servlet01
---------------------------
null
null
org.apache.catalina.session.StandardSessionFacade@4f4391e1
org.apache.catalina.session.StandardSessionFacade@4f4391e1
false
false
false
```