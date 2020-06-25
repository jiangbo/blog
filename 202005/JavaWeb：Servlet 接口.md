# JavaWeb：Servlet 接口

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## Servlet 说明

Java web 的核心接口 Servlet，很多的功能都围绕它展开。
作为顶级接口，我们可以选择直接实现它。它包含五个需要被实现的方法，下面将依次介绍。
Servlet 的生命周期方法调用：

1. 在 Servlet 创建后，调用 init 方法。
2. 所有客户端的请求都会由 service 方法来处理。
3. Servlet 被销毁时，调用 destroy 方法，然后被 GC 回收。

### init

init 方法会在 Servlet 被实例化后被调用，且只被调用一次。

``` java
public void init(ServletConfig config) throws ServletException;
```

### getServletConfig

通过该方法，可以获取到 ServletConfig 接口的一个对象。ServletConfig 之后介绍。

```java
public ServletConfig getServletConfig();
```

### service

客户端通过配置的路径可以访问到该方法，每被访问一次，该方法就被调用一次。

```java
public void service(ServletRequest req, ServletResponse res) throws ServletException, IOException
```

### getServletInfo

获取 Servlet 的信息。

```java
public String getServletInfo();
```

### destroy

该方法在 Servlet 被销毁时调用。

```java
public void destroy();
```

## Servlet 示例

### 提供 Servlet-api

Servlet 的实现和接口在 Servlet 容器中都有，不需要自己提供。
所以我们只需要 Servlet-api 来保证我们编写代码不出现编译错误即可。

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>02java-web-servlet</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Servlet 的示例</description>

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

### 实现 Servlet 接口

```java
package jiangbo.javaweb;

import java.io.IOException;

import javax.servlet.Servlet;
import javax.servlet.ServletConfig;
import javax.servlet.ServletException;
import javax.servlet.ServletRequest;
import javax.servlet.ServletResponse;

public class Servlet01 implements Servlet {

    @Override
    public void init(ServletConfig config) throws ServletException {
        System.out.println("init...");
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
    xsi:schemaLocation="http://xmlns.jcp.org/xml/ns/javaee
                      http://xmlns.jcp.org/xml/ns/javaee/web-app_3_1.xsd" version="3.1">

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

其中的 servlet-name 必须一样，它们才能关联起来。其中的逻辑是：

1. 访问 /servlet01 路径时，找到其 servlet-name 为 servelt01。
2. 根据 servlet01 找到对应的 servlet，得到 class 为：jiangbo.javaweb.Servlet01。
3. 调用 jiangbo.javaweb.Servlet01 类的 service 方法。

映射路径的注意事项：

1. 可以使用 * 作为通配符，但精确匹配优先。
2. 必须以 / 或者 * 开头。
3. 模糊匹配可以是 /test/* 或者 *.do，但不能是 /test/*.do。
4. 可以将多个路径映射到同一个 Servlet 上，但是不能同一个路径映射多个 Servlet 。

### 浏览器访问

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/02java-web-servlet/servlet01 项目地址，连续访问三次，然后关闭 tomcat。

### 控制台输出

查看控制台输出, init 方法执行了一次，而 service 访问被调用了三次。

```text
init...
service...
service...
service...
destroy...
```