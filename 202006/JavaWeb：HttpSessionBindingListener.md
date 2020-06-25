# JavaWeb：HttpSessionBindingListener

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Servlet 3.1
- Tomcat 8.5
- Maven 3.6.3

## HttpSessionBindingListener 说明

HttpSessionBindingListener 可以监听实现了该接口的对象在 Session 中被添加或者移除的事件。HttpSessionActivationListener 可以监听 Session 被钝化或者激活的事件。

### valueBound

值被绑定到 Session 的时候调用。

```java
public void valueBound(HttpSessionBindingEvent event);
```

### attributeRemoved

值从 Session 解绑的时候调用。

```java
public void valueUnbound(HttpSessionBindingEvent event);
```

## HttpSessionActivationListener 说明

### sessionWillPassivate

Session 被钝化时调用。

```java
public void sessionWillPassivate(HttpSessionEvent se);
```

### sessionDidActivate

Session 被激活时调用。

```java
public void sessionDidActivate(HttpSessionEvent se);
```

## HttpSessionBindingListener 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.javaweb</groupId>
    <artifactId>20java-web-http-session-binding-listener</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>Http Session Binding Listener 的示例</description>

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

### Servlet01

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
        session.setAttribute("listener", new Listener01());
        session.removeAttribute("listener");
        session.setAttribute("listener", new Listener01());
    }
}
```

### Listener01

```java
package jiangbo.javaweb;

import java.io.Serializable;

import javax.servlet.http.HttpSessionActivationListener;
import javax.servlet.http.HttpSessionBindingEvent;
import javax.servlet.http.HttpSessionBindingListener;
import javax.servlet.http.HttpSessionEvent;

public class Listener01 implements HttpSessionBindingListener, HttpSessionActivationListener, Serializable {

    private static final long serialVersionUID = 1L;

    @Override
    public void sessionWillPassivate(HttpSessionEvent se) {
        System.out.println("session 被钝化：" + se.getSession());
    }

    @Override
    public void sessionDidActivate(HttpSessionEvent se) {
        System.out.println("session 激活了：" + se.getSession());
    }

    @Override
    public void valueBound(HttpSessionBindingEvent event) {
        System.out.println("session 值绑定 " + event.getName() + ":" + event.getValue());
    }

    @Override
    public void valueUnbound(HttpSessionBindingEvent event) {
        System.out.println("session 值解绑 " + event.getName() + ":" + event.getValue());
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

将该项目部署到服务器并启动，在客户端访问 http://localhost:8080/20java-web-http-session-binding-listener/servlet01 项目地址，然后关闭 tomcat，再次启动 tomcat，看控制台输出。

### 控制台输出

```text
service...
session 值绑定 listener:jiangbo.javaweb.Listener01@5a190a11
session 值解绑 listener:null
session 值解绑 listener:jiangbo.javaweb.Listener01@5a190a11
session 值绑定 listener:jiangbo.javaweb.Listener01@66f3bb03
session 被钝化：org.apache.catalina.session.StandardSessionFacade@4959d1c6
session 激活了：org.apache.catalina.session.StandardSessionFacade@4f4ec254
```