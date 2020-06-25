# JavaWeb：Cookie

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## Cookie 说明

Cookie 具有一个名称和一个值，也包括其它的可选属性，例如注释，路径和域限定符，最大生存时间等。
一般来说，浏览器总共支持 300 个 Cookie，每个 Web 应用支持 20 个，每个 Cookie 的大小应该限制在 4KB 以内。
其中在 Servlet 3.0 之后新增了一个 SessionCookieConfig 接口，可以对 web 应用的所有 Cookie 配置，
他们的方法都一致，这里以 Cookie 举例。

### 属性

Cookie 定义了很多的属性，并且提供了 get 和 set 方法。
maxAge 表示 Cookie 的过期时间，单位是秒，负数表示关闭浏览器失效。

```java
private String name; // NAME= ... "$Name" style is reserved
private String value; // value of NAME

//
// Attributes encoded in the header's cookie fields.
//

private String comment; // ;Comment=VALUE ... describes cookie's use
            // ;Discard ... implied by maxAge < 0
private String domain; // ;Domain=VALUE ... domain that sees cookie
private int maxAge = -1; // ;Max-Age=VALUE ... cookies auto-expire
private String path; // ;Path=VALUE ... URLs that see the cookie
private boolean secure; // ;Secure ... e.g. use SSL
private int version = 0; // ;Version=1 ... means RFC 2109++ style
private boolean isHttpOnly = false;
```

### clone

Cookie 实现了 Cloneable 接口。

```java
public Object clone() {
    try {
        return super.clone();
    } catch (CloneNotSupportedException e) {
        throw new RuntimeException(e.getMessage());
    }
}
```

## Cookie 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>14java-web-cookie</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Cookie 的示例</description>

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
import javax.servlet.http.Cookie;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

@SuppressWarnings("serial")
public class Servlet01 extends HttpServlet {

    @Override
    protected void doGet(HttpServletRequest req, HttpServletResponse resp) throws ServletException, IOException {

        System.out.println("service...");
        req.getSession();
        resp.addCookie(new Cookie("cname", "将波"));

        Cookie[] cookies = req.getCookies();
        for (Cookie cookie : cookies) {
            System.out.println("---------------------------");
            System.out.println(cookie.getName());
            System.out.println(cookie.getValue());
            System.out.println(cookie.getComment());
            System.out.println(cookie.getDomain());
            System.out.println(cookie.getMaxAge());
            System.out.println(cookie.getPath());
            System.out.println(cookie.getSecure());
            System.out.println(cookie.getVersion());
            System.out.println(cookie.isHttpOnly());
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/14java-web-cookie/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
---------------------------
JSESSIONID
805DD19DC4752222E13A653E41B23910
null
null
-1
null
false
0
false
---------------------------
cname
将波
null
null
-1
null
false
0
false
```