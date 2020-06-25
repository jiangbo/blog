# JavaWeb：HttpSession

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## HttpSession 说明

HttpSession 总共有 17 个方法，其中 5 个方法已过期。

### Attribute 系列方法

和 ServletContext、HttpServletRequest 类似，只不过作用域是整个会话。

```java
public Object getAttribute(String name);
public Enumeration<String> getAttributeNames();
public void setAttribute(String name, Object value);
public void removeAttribute(String name);
```

### getCreationTime

获得创建时间。

```java
public long getCreationTime();
```

### getId

获得 Session id。

```java
public String getId();
```

### getLastAccessedTime

获得客户端最后访问时间。

```java
public long getLastAccessedTime();
```

### getServletContext

ServletContext 已经学习过。

```java
public ServletContext getServletContext();
```

### setMaxInactiveInterval

设置 Session 失效的最大非活跃间隔时间，也就是多久不访问就失效，单位是秒，小于零表示永不过期。

```java
public void setMaxInactiveInterval(int interval);
```

### getMaxInactiveInterval

获得 Session 的最大失效时间。

```java
public int getMaxInactiveInterval();
```

### invalidate

让 Session 立即失效。

```java
public void invalidate();
```

### isNew

判断该 Session 是否是新生成的。

```java
public boolean isNew();
```

## HttpSession 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>15java-web-http-session</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Http Session 的示例</description>

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
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.servlet.http.HttpSession;

@SuppressWarnings("serial")
public class Servlet01 extends HttpServlet {

    @Override
    protected void doGet(HttpServletRequest req, HttpServletResponse resp) throws ServletException, IOException {

        System.out.println("service...");
        HttpSession session = req.getSession();

        System.out.println(session.getCreationTime());
        System.out.println(session.getId());
        System.out.println(session.getLastAccessedTime());
        System.out.println(session.getServletContext());
        System.out.println(session.getMaxInactiveInterval());
        System.out.println(session.isNew());
        session.invalidate();
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/15java-web-http-session/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
service...
1590826119603
4F3991F35A5304BA4C8F9E4E2FF934C8
1590826119603
org.apache.catalina.core.ApplicationContextFacade@2683b085
1800
true
```