# JavaWeb：ServletResponse

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## ServletResponse 说明

ServletResponse 可以让 Servlet 向客户端发送响应。
ServletResponse 由 Servlet 容器创建，并将其传递给 Servlet 的 service 方法。

为了向客户端发送二进制的数据，可以使用 getOutputStream 方法返回的 ServletOutputStream 输出流。
为了发送字符数据，可以使用 getWriter 返回的 PrintWriter 对象。
想要同时发送两种数据，需要使用 ServletOutputStream ，并且手动处理字符数据。

可以使用 setCharacterEncoding（java.lang.String） 和 setContentType（java.lang.String） 方法
显式指定响应的编码，或者使用 setLocale（java.util.Locale）方法隐式指定，显式指定优先于隐式指定。
如果未指定编码，则将使用 ISO-8859-1 。必须在 getWriter 之前调用 setCharacterEncoding ，setContentType 或 setLocale 方法。

### 编码系列方法

可以通过 下面的方法获得和设置响应的编码，优先级分别为：setCharacterEncoding、setContentType、setLocale，
并且设置编码应该在 getWriter 方法调用之前。

```java
public String getCharacterEncoding();
public String getContentType();
public void setCharacterEncoding(String charset);
public void setContentType(String type);
public void setLocale(Locale loc);
public Locale getLocale();
```

### 流系列方法

分别获得字节和字符输出流，不能同时使用。

```java
public ServletOutputStream getOutputStream() throws IOException;
public PrintWriter getWriter() throws IOException;
```

### 缓冲系列方法

和输出流的缓冲相关的方法。

```java
public void setBufferSize(int size);
public int getBufferSize();
public void flushBuffer() throws IOException;
public void resetBuffer();
```

### setContentLength

设置响应头中的 ContentLength 。

```java
public void setContentLength(int len);
```

### reset

清空返回的数据，同时也重置输出流。

```java
public void reset();
```

### isCommitted

判断响应是否提交。

```java
public boolean isCommitted();
```


## ServletResponse 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>12java-web-servlet-response</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Response 的示例</description>

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
        System.out.println(resp.getCharacterEncoding());
        System.out.println(resp.getContentType());
        resp.setCharacterEncoding("UTF-8");
        System.out.println(resp.getCharacterEncoding());
        System.out.println(resp.getContentType());
        System.out.println(resp.getLocale());
        resp.setContentLength(44);

        System.out.println("---------------------------");
        resp.getOutputStream();
        resp.reset();
        System.out.println(resp.getCharacterEncoding());
        resp.setContentType("text/html;charset=UTF-8");
        System.out.println(resp.getCharacterEncoding());
        PrintWriter writer = resp.getWriter();

        System.out.println("---------------------------");
        System.out.println(resp.getBufferSize());
        writer.println("没有了的内容");
        resp.resetBuffer();
        writer.print("存在的内容");
        writer.flush();

        System.out.println("---------------------------");
        System.out.println(resp.isCommitted());

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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/12java-web-servlet-response/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
---------------------------
ISO-8859-1
null
UTF-8
null
zh_CN
---------------------------
ISO-8859-1
UTF-8
---------------------------
8192
---------------------------
true
```

### 浏览器显示

```text
存在的内容
```