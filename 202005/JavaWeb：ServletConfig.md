# JavaWeb：ServletConfig

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## ServletConfig 说明

ServletConfig 包含四个需要被实现的方法。

### getServletName

获取 Servlet 的名称。

``` java
public String getServletName();
```

### getServletContext

通过该方法，可以获取到 ServletContext 接口的一个对象。

```java
public ServletContext getServletContext();
```

### getInitParameter

通过提供的名称，获得该 Servlet 的初始化参数的值。

```java
public String getInitParameter(String name);
```

### getInitParameterNames

获取该 Servlet 所有的初始化参数的名称。

```java
public Enumeration<String> getInitParameterNames();
```

## ServletConfig 示例

下面的示例基于 tomcat，所以 ServeltConfig 的实现是 tomcat 的实现。

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>03java-web-servlet-config</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet Config 的示例</description>

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

### 调用 ServletConfig 的方法

```java
ppackage jiangbo.javaweb;

import java.io.IOException;
import java.util.Enumeration;

import javax.servlet.Servlet;
import javax.servlet.ServletConfig;
import javax.servlet.ServletException;
import javax.servlet.ServletRequest;
import javax.servlet.ServletResponse;

public class Servlet01 implements Servlet {

    private ServletConfig servletConfig;

    @Override
    public void init(ServletConfig config) throws ServletException {

        System.out.println("init...");

        System.out.println(config.getServletName());
        System.out.println(config.getServletContext());
        System.out.println(config.getInitParameter("username"));
        System.out.println(config.getInitParameter("password"));

        for (Enumeration<String> names = config.getInitParameterNames(); names.hasMoreElements();) {
            String element = names.nextElement();
            System.out.println(element + ":" + config.getInitParameter(element));
        }

        this.servletConfig = config;
    }

    @Override
    public ServletConfig getServletConfig() {

        return servletConfig;
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

        <init-param>
            <param-name>username</param-name>
            <param-value>jiangbo</param-value>
        </init-param>

        <init-param>
            <param-name>password</param-name>
            <param-value>123456</param-value>
        </init-param>
    </servlet>

    <servlet-mapping>
        <servlet-name>servlet01</servlet-name>
        <url-pattern>/servlet01</url-pattern>
    </servlet-mapping>

</web-app>
```

### 浏览器访问

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/03java-web-servlet-config/servlet01 项目地址，连续访问三次，然后关闭 tomcat。

### 控制台输出

```text
init...
servlet01
org.apache.catalina.core.ApplicationContextFacade@498bec65
jiangbo
123456
password:123456
username:jiangbo
service...
service...
service...
destroy...
```