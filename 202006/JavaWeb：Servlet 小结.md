# Java Web：Servlet 小结

> 以下资料全部来源于[维基百科][1]

servlet 2.5 的 api 基本上都学习完了，这里对 servlet 做一个小结。

## Java Servlet

Servlet（Server Applet），全称Java Servlet，未有中文译文，是用Java编写的服务器端程序。
其主要功能在于交互式地浏览和修改数据，生成动态Web内容。
狭义的Servlet是指Java语言实现的一个接口，广义的Servlet是指任何实现了这个Servlet接口的类，
一般情况下，人们将Servlet理解为后者。

Servlet运行于支持Java的应用服务器中。从实现上讲，Servlet可以响应任何类型的请求，
但绝大多数情况下Servlet只用来扩展基于HTTP协议的Web服务器。

最早支持Servlet标准的是JavaSoft的Java Web Server。
此后，一些其它的基于Java的Web服务器开始支持标准的Servlet。

 ## 历史

以下是从 servlet 2.5 开始的历史 

| Servlet API 版本 | 发布日期 | 平台 | 重要变化 |
| :---: | :---: | :----: | :----: |			
|Servlet 4.0 | 2017年9月 | ava EE 8, Java SE 8 | HTTP/2 |
|Servlet 3.1 | 2013年5月 | Java EE 7, Java SE 7 | Non-blocking I/O, HTTP protocol upgrade mechanism (WebSocket) |
|Servlet 3.0 | 2009年12月 | Java EE 6, Java SE 6 | Pluggability, Ease of development, Async Servlet, Security, File Uploading |
|Servlet 2.5 | 2005年9月 | Java EE 5, Java SE 5 | Requires Java SE 5, supports annotation |
	
## 工作模式

  - 客户端发送请求至服务器
  - 服务器启动并调用Servlet，Servlet根据客户端请求生成响应内容并将其传给服务器
  - 服务器将响应返回客户端
  - 其他

## 通用Servlet

一般来说，通用Servlet由javax.servlet.GenericServlet实现Servlet接口。
程序设计人员可以通过使用或继承这个类来实现通用Servlet应用。

### HttpServlet

javax.servlet.http.HttpServlet实现了专门用于响应HTTP请求的Servlet，
提供了响应对应HTTP标准请求的doGet()、doPost()等方法。

## 生命周期

当servlet被部署在应用服务器中（应用服务器中用于管理Java组件的部分被抽象成为容器）以后，
由容器控制servlet的生命周期。除非特殊指定，否则在容器启动的时候，
servlet是不会被加载的，servlet只会在第一次请求的时候被加载和实例化。
servlet一旦被加载，一般不会从容器中删除，直至应用服务器关闭或重新启动。
但当容器做存储器回收动作时，servlet有可能被删除。
也正是因为这个原因，第一次访问servlet所用的时间要大大多于以后访问所用的时间。

servlet在服务器的运行生命周期为，在第一次请求（或其实体被内存垃圾回收后再被访问）时被加载并执行一次初始化方法，
跟着执行正式运行方法，之后会被常驻并每次被请求时直接执行正式运行方法，
直到服务器关闭或被清理时执行一次销毁方法后实体销毁。

## 与JSP的关系

Java服务器页面（JSP）是HttpServlet的扩展。由于HttpServlet大多是用来响应HTTP请求，
并返回Web页面（例如HTML、XML），所以不可避免地，在编写servlet时会涉及大量的HTML内容，
这给servlet的书写效率和可读性带来很大障碍，JSP便是在这个基础上产生的。
其功能是使用HTML的书写格式，在适当的地方加入Java代码片段，将程序员从复杂的HTML中解放出来，
更专注于servlet本身的内容。

JSP在首次被访问的时候被应用服务器转换为servlet，在以后的运行中，
容器直接调用这个servlet，而不再访问JSP页面。JSP的实质仍然是servlet。

[1]: https://zh.wikipedia.org/wiki/Java_Servlet