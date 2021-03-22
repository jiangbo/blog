# spring-boot 使用jdk6（三）

## 环境

1. jdk 6
2. tomcat 7.0.59
3. sts 4.4.2
4. maven 3.2.5

## 背景

由于环境限制，还在使用 JDK6，所以需要将 spring boot 进行配置，支持JDK6。
以下所有的操作建立在之前的项目基础上。

### 修改 JDK 版本

之前，我们修改了 spring boot 默认的 JDK 从 6  升级到了 8，现在将修改的地方删除。
删除以下内容：

```xml
<java.version>1.8</java.version>
```

这样，spring boot 默认的 JDK 版本又回到了 6。

## 降低内置 tomcat 版本

如下这个时候，使用 main 方法启动项目，发现提示如下信息：

`java.lang.UnsupportedClassVersionError: javax/annotation/ManagedBean : Unsupported major.minor version 51.0`

该错误信息是说，javax/annotation/ManagedBean 这个类使用的 JDK7 编译的，不支持 JDK6，需要降低 tomcat 的版本。

tomcat8 以上的版本都是比 JDK6 的版本要求高，所以降低 tomcat 到 7.0.59 版本。

```xml
<tomcat.version>7.0.59</tomcat.version>
```

## 解决日志报错

将 tomcat 降级到 7 之后，使用 main 方法运行项目，结果提示日志类找不到。

```text
Caused by: java.lang.ClassNotFoundException: org.apache.juli.logging.LogFactory
    at java.net.URLClassLoader$1.run(URLClassLoader.java:202) ~[na:1.6.0_45]
    at java.security.AccessController.doPrivileged(Native Method) ~[na:1.6.0_45]
    at java.net.URLClassLoader.findClass(URLClassLoader.java:190) ~[na:1.6.0_45]
    at java.lang.ClassLoader.loadClass(ClassLoader.java:306) ~[na:1.6.0_45]
    at sun.misc.Launcher$AppClassLoader.loadClass(Launcher.java:301) ~[na:1.6.0_45]
    at java.lang.ClassLoader.loadClass(ClassLoader.java:247) ~[na:1.6.0_45]
    ... 12 common frames omitted
```

为了修复该问题，在 pom.xml 中增加如下配置（如果是打成 jar，需要将依赖范围 provided 删除，这里演示的是 war 包部署）：

```xml
<dependency>
    <groupId>org.apache.tomcat</groupId>
    <artifactId>tomcat-juli</artifactId>
    <version>${tomcat.version}</version>
    <scope>provided</scope>
</dependency>
```

## war 包部署

spring boot 最少需要 servlet 3.0 的 api，所以 tomcat7 是最低限制（下一节讲解 servlet 2.5 的部署）。
由于 JDK 使用的 6，导致 maven 打包出现错误，降低 maven 的版本到 3.2.5，这是支持 JDK6 的最后一个版本。

## 运行并访问

直接运行 main 方法或者部署到 tomcat7 中，可以看到项目正常启动，访问 localhost:8080（/demo，如果是使用外部服务器启动的话），可以正常显示 `hello world!`。

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
    <packaging>war</packaging>

    <properties>
        <tomcat.version>7.0.59</tomcat.version>
    </properties>

    <dependencies>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-tomcat</artifactId>
            <scope>provided</scope>
        </dependency>

        <dependency>
            <groupId>org.apache.tomcat</groupId>
            <artifactId>tomcat-juli</artifactId>
            <version>${tomcat.version}</version>
            <scope>provided</scope>
        </dependency>

    </dependencies>

</project>
```
