# spring-boot 搭建环境（一）

## 环境

1. jdk 8
2. tomcat 8.5
3. sts 4.4.2
4. maven 3.6.1

## 新建 maven 项目

首先创建一个普通的 maven 项目。

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>jiangbo.demo</groupId>
    <artifactId>demo</artifactId>
    <version>1.0.0</version>

</project>
```

## 定义 parent

spring-boot-dependencies 定义很多的 jar 的版本信息。通过引入它，在使用 jar 的时候，可以不指定版本，默认由 spring-boot-dependencies 来管理。当然，也是可以覆盖的。而 spring-boot-starter-parent 继承于 spring-boot-dependencies 所以也具有版本管理的功能。除此之外，还定义了一些插件，来完成一些特别的功能。在 pom.xml 中引入  spring-boot-starter-parent。

```xml
<parent>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-parent</artifactId>
    <version>1.5.22.RELEASE</version>
</parent>
```

## 引入打包插件

spring-boot-maven-plugin 这个插件可以将项目打成一个可以执行的 jar，如果现在还不想打包，可以先不配置。

```xml
<build>
    <plugins>
        <plugin>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-maven-plugin</artifactId>
        </plugin>
    </plugins>
</build>
```

## 定义 jdk 版本

默认情况下，spring-boot 使用的 jdk 版本是 1.6。由于我们先使用内置的 tomcat 跑一下，所以必须将 jdk 升级到 1.8。

```xml
<properties>
    <java.version>1.8</java.version>
</properties>
```

## spring-boot-starter-web

我们使用 spring mvc 来编写代码，所以将 spring-boot-starter-web 引入进来，就可以开发 spring mvc 程序了。

>starter: spring boot 把一个完整的功能需要的所有的 jar 引到一起，定义成的一系列 jar 的集合和配置。

```xml
<dependencies>
    <dependency>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-web</artifactId>
    </dependency>
</dependencies>
```

## 定义启动类

spring boot 的运行方式和普通的 java 程序运行一样，直接使用 main 方法运行，所以需要定义一个启动类。

```java
package jiangbo.demo;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class DemoApplication {

    public static void main(String[] args) {

        SpringApplication.run(DemoApplication.class, args);
    }
}
```

注解 @SpringBootApplication 标记这是一个 spring boot 项目，去掉的话启动不了 tomcat，所以要加上。

## 定义 controller

定义一个 http 访问的路径，该部分属于 spring 的知识。

```java
package jiangbo.demo.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class DemoController {

    @GetMapping
    public String home() {

        return "hello world!";
    }
}
```

上面定义了一个根路径的映射，在浏览器上可以访问，需要注意该类必须在 DemoApplication 类的子包或者相同包中，这是因为 spring 的注解扫描的原因。

## 启动应用

直接在 main 方法中开始 run，启动程序。
可以看到类似下面的信息：

```text
  .   ____          _            __ _ _
 /\\ / ___'_ __ _ _(_)_ __  __ _ \ \ \ \
( ( )\___ | '_ | '_| | '_ \/ _` | \ \ \ \
 \\/  ___)| |_)| | | | | || (_| |  ) ) ) )
  '  |____| .__|_| |_|_| |_\__, | / / / /
 =========|_|==============|___/=/_/_/_/
 :: Spring Boot ::       (v1.5.22.RELEASE)

2019-11-30 21:38:08.710  INFO 4268 --- [           main] jiangbo.demo.DemoApplication             : Starting DemoApplication on DESKTOP-056SPR2 with PID 4268 (D:\work\workspace\sts\demo\target\classes started by DengSiSi in D:\work\workspace\sts\demo)
2019-11-30 21:38:08.715  INFO 4268 --- [           main] jiangbo.demo.DemoApplication             : No active profile set, falling back to default profiles: default
2019-11-30 21:38:08.766  INFO 4268 --- [           main] ationConfigEmbeddedWebApplicationContext : Refreshing org.springframework.boot.context.embedded.AnnotationConfigEmbeddedWebApplicationContext@268f106e: startup date [Sat Nov 30 21:38:08 CST 2019]; root of context hierarchy
2019-11-30 21:38:09.640  INFO 4268 --- [           main] s.b.c.e.t.TomcatEmbeddedServletContainer : Tomcat initialized with port(s): 8080 (http)
2019-11-30 21:38:09.661  INFO 4268 --- [           main] o.apache.catalina.core.StandardService   : Starting service [Tomcat]
2019-11-30 21:38:09.662  INFO 4268 --- [           main] org.apache.catalina.core.StandardEngine  : Starting Servlet Engine: Apache Tomcat/8.5.43
2019-11-30 21:38:09.744  INFO 4268 --- [ost-startStop-1] o.a.c.c.C.[Tomcat].[localhost].[/]       : Initializing Spring embedded WebApplicationContext
2019-11-30 21:38:09.744  INFO 4268 --- [ost-startStop-1] o.s.web.context.ContextLoader            : Root WebApplicationContext: initialization completed in 978 ms
2019-11-30 21:38:09.873  INFO 4268 --- [ost-startStop-1] o.s.b.w.servlet.ServletRegistrationBean  : Mapping servlet: 'dispatcherServlet' to [/]
2019-11-30 21:38:09.877  INFO 4268 --- [ost-startStop-1] o.s.b.w.servlet.FilterRegistrationBean   : Mapping filter: 'characterEncodingFilter' to: [/*]
2019-11-30 21:38:09.878  INFO 4268 --- [ost-startStop-1] o.s.b.w.servlet.FilterRegistrationBean   : Mapping filter: 'hiddenHttpMethodFilter' to: [/*]
2019-11-30 21:38:09.878  INFO 4268 --- [ost-startStop-1] o.s.b.w.servlet.FilterRegistrationBean   : Mapping filter: 'httpPutFormContentFilter' to: [/*]
2019-11-30 21:38:09.878  INFO 4268 --- [ost-startStop-1] o.s.b.w.servlet.FilterRegistrationBean   : Mapping filter: 'requestContextFilter' to: [/*]
2019-11-30 21:38:10.166  INFO 4268 --- [           main] s.w.s.m.m.a.RequestMappingHandlerAdapter : Looking for @ControllerAdvice: org.springframework.boot.context.embedded.AnnotationConfigEmbeddedWebApplicationContext@268f106e: startup date [Sat Nov 30 21:38:08 CST 2019]; root of context hierarchy
2019-11-30 21:38:10.212  INFO 4268 --- [           main] s.w.s.m.m.a.RequestMappingHandlerMapping : Mapped "{[],methods=[GET]}" onto public java.lang.String jiangbo.demo.controller.DemoController.home()
2019-11-30 21:38:10.216  INFO 4268 --- [           main] s.w.s.m.m.a.RequestMappingHandlerMapping : Mapped "{[/error]}" onto public org.springframework.http.ResponseEntity<java.util.Map<java.lang.String, java.lang.Object>> org.springframework.boot.autoconfigure.web.BasicErrorController.error(javax.servlet.http.HttpServletRequest)
2019-11-30 21:38:10.216  INFO 4268 --- [           main] s.w.s.m.m.a.RequestMappingHandlerMapping : Mapped "{[/error],produces=[text/html]}" onto public org.springframework.web.servlet.ModelAndView org.springframework.boot.autoconfigure.web.BasicErrorController.errorHtml(javax.servlet.http.HttpServletRequest,javax.servlet.http.HttpServletResponse)
2019-11-30 21:38:10.246  INFO 4268 --- [           main] o.s.w.s.handler.SimpleUrlHandlerMapping  : Mapped URL path [/webjars/**] onto handler of type [class org.springframework.web.servlet.resource.ResourceHttpRequestHandler]
2019-11-30 21:38:10.247  INFO 4268 --- [           main] o.s.w.s.handler.SimpleUrlHandlerMapping  : Mapped URL path [/**] onto handler of type [class org.springframework.web.servlet.resource.ResourceHttpRequestHandler]
2019-11-30 21:38:10.285  INFO 4268 --- [           main] o.s.w.s.handler.SimpleUrlHandlerMapping  : Mapped URL path [/**/favicon.ico] onto handler of type [class org.springframework.web.servlet.resource.ResourceHttpRequestHandler]
2019-11-30 21:38:10.484  INFO 4268 --- [           main] o.s.j.e.a.AnnotationMBeanExporter        : Registering beans for JMX exposure on startup
2019-11-30 21:38:10.526  INFO 4268 --- [           main] s.b.c.e.t.TomcatEmbeddedServletContainer : Tomcat started on port(s): 8080 (http)
2019-11-30 21:38:10.536  INFO 4268 --- [           main] jiangbo.demo.DemoApplication             : Started DemoApplication in 2.065 seconds (JVM running for 2.742)
```

## 浏览器访问

在浏览器输入 localhost:8080,应该可以看到浏览器输出一个 `hello world!`，表示 spring boot 环境搭建成功。

## 附录

### 完整 pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>1.5.22.RELEASE</version>
    </parent>

    <groupId>jiangbo.demo</groupId>
    <artifactId>demo</artifactId>
    <version>1.0.0</version>

    <properties>
        <java.version>1.8</java.version>
    </properties>

    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
            </plugin>
        </plugins>
    </build>

</project>
```
