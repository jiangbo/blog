# JavaWeb：简单介绍

## Java EE 简介

Java EE 的全称是 Java Platform, Enterprise Edition，即 Java 企业版。其中相关的概念在[这里][1]找到。
Java EE 包含很多的技术，不过很多的技术我们都没有使用到。其中的技术大概有：

+ Enterprise JavaBeans Technology（EJB）
+ Java Servlet Technology（Servlet）
+ JavaServer Faces Technology（JSF）
+ JavaServer Pages Technology（JSP）
+ JavaServer Pages Standard Tag Library（JSTL）
+ Java Persistence API（JPA）
+ Java Transaction API（JTA）

这里就不一一列举了，查看全部的技术可以点击[这里][2]。

## Java Web

Java Web 使用 java 技术来开发 web 项目，主要使用的技术有：Servlet、JSP 和 JSTL。Web 项目，通俗点说，就是写网页。
其中包含两种：一种是静态网页，就是 html 和 JS 等；另一种是动态网页，使用 JSP/Servlet 可以开发动态的网页。

### Servlet 容器

要开发 Servlet 项目，需要有 Servlet 容器。
经常使用的有 Servlet 容器有 Tomcat 、Jetty 等。
Tomcat 的安装很简单，直接到[官网][3]下载解压即可使用，也可以将其配置到 eclipse 中。

[1]:https://javaee.github.io/tutorial/overview001.html#A1046550
[2]:https://javaee.github.io/tutorial/overview008.html
[3]:http://tomcat.apache.org/