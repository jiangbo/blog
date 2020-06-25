# JavaWeb：GenericServlet

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## GenericServlet 说明

GenericServlet 实现了 Servlet, ServletConfig, java.io.Serializable 这三个接口，其中：
Serializable 和序列化相关，不在这里说明。
关于 Servlet 可以参考 [JavaWeb：Servlet 接口][1]。
关于 ServletConfig 可以参考 [JavaWeb：ServletConfig][2]。

### LSTRING_FILE 属性

值为：javax.servlet.LocalStrings，国际化资源的名称，和国际化提示信息相关。

```java
private static final String LSTRING_FILE = "javax.servlet.LocalStrings";
```

### lStrings 属性

ResourceBundle 类型的变量，和国际化提示信息相关。

```java
private static ResourceBundle lStrings = ResourceBundle.getBundle(LSTRING_FILE);
```

### config 属性

ServletConfig 类型的变量，保存为属性，实现某些方法的时候需要使用它。

```java
private transient ServletConfig config;
```

### init 方法

GenericServlet 新增的 init 方法，如果有初始化逻辑，建议覆盖它而不是 `init(ServletConfig config)` ，如果忘记调用 `super.init(config)` 或者对 config 属性赋值，会产生空指针异常。

> 建议覆盖无参的 init 方法

```java
public void init() throws ServletException {
}
```

## GenericServlet 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>07java-web-generic-servlet</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Generic Servlet 的示例</description>

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

import javax.servlet.GenericServlet;
import javax.servlet.ServletException;
import javax.servlet.ServletRequest;
import javax.servlet.ServletResponse;

@SuppressWarnings("serial")
public class Servlet01 extends GenericServlet {

    @Override
    public void init() throws ServletException {

        System.out.println("generic init...");
    }

    @Override
    public void service(ServletRequest req, ServletResponse res) throws ServletException, IOException {
        System.out.println("service...");
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/07java-web-generic-servlet/servlet01 项目地址，然后关闭 tomcat。

### 控制台输出

```text
generic init...
service...
```

## GenericServlet 源码分析

### ServletConfig 属性

```java
private transient ServletConfig config;

public void init(ServletConfig config) throws ServletException {
    this.config = config;
    this.init();
}

public ServletConfig getServletConfig() {
    return config;
}
```

通过将 ServletConfig 保存为属性，实现了 getServletConfig 方法，可以随时获取 ServletConfig 对象，那么 ServletConfig 接口的方法都可以被调用了。

### 获得 ServletContext

```java
public ServletContext getServletContext() {
    ServletConfig sc = getServletConfig();
    if (sc == null) {
        throw new IllegalStateException(
            lStrings.getString("err.servlet_config_not_initialized"));
    }

    return sc.getServletContext();
}
```

通过获取 ServletConfig 对象，再获取 ServletContext。ServletConfig 接口的其它方法实现思路类似，都是获取 ServletConfig 对象，再调用其方法。Servlet 接口的其它方法，基本都是空实现。


[1]: https://www.cnblogs.com/jiangbo44/p/12879457.html
[2]: https://www.cnblogs.com/jiangbo44/p/12885482.html