# JavaWeb：ServletRequest（二）

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## ServletRequest 说明

ServletRequest 中的 getRealPath 该方法被 ServletContext 中同名方法代替，不建议使用。

### Encoding 系列方法

获得或者设置编码。

> 编码设置只对请求体有效。

```java
public String getCharacterEncoding();
public void setCharacterEncoding(String env) throws UnsupportedEncodingException;
```

### Content 系列方法

获取内容的长度和类型。

```java
public int getContentLength();
public String getContentType();
```

### 流系列方法

获得一个 ServletInputStream 和 BufferedReader 输入流。

```java
public ServletInputStream getInputStream() throws IOException;
public BufferedReader getReader() throws IOException;
```

### Parameter 系列方法

获取请求参数。
> getParameterValues 可以获取同一name的多个值，比如复选框。

```Java
public String getParameter(String name);
public Enumeration<String> getParameterNames();
public String[] getParameterValues(String name);
public Map<String, String[]> getParameterMap();
```

### Locale 系列方法

获得地区参数

```Java
public Locale getLocale();
public Enumeration<Locale> getLocales();
```

### getRequestDispatcher

获得 RequestDispatcher 对象，该对象之后学习。
和 ServletContext 的 getRequestDispatcher 的不一样在于，这个可以使用相对路径。

```Java
public RequestDispatcher getRequestDispatcher(String path);
```

## ServletRequest 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>10java-web-servlet-request</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Request 的示例（二）</description>

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
import java.util.Locale;

import javax.servlet.ServletException;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

@SuppressWarnings("serial")
public class Servlet01 extends HttpServlet {

    @Override
    protected void doGet(HttpServletRequest req, HttpServletResponse resp) throws ServletException, IOException {

        System.out.println("service...");

        System.out.println(req.getCharacterEncoding());

        System.out.println(req.getContentLength());
        System.out.println(req.getContentType());

        System.out.println(req.getInputStream());
//      System.out.println(req.getReader());

        System.out.println(req.getLocale());
        Enumeration<Locale> locales = req.getLocales();
        while (locales.hasMoreElements()) {
            System.out.println(locales.nextElement());
        }

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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/10java-web-servlet-request/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
null
-1
null
org.apache.catalina.connector.CoyoteInputStream@2557fe4e
zh_CN
zh_CN
zh
en
en_GB
en_US
false
```