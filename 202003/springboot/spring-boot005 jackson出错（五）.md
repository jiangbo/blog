# spring-boot 使用 jackson 出错（五）

## 环境

1. jdk 6
2. tomcat 6.0.53
3. sts 4.4.2
4. maven 3.2.5

## 原因

spring boot 1.5.22.RELEASE 默认使用的 jackson 的版本是 2.8.x，但是 JDK6 只支持 2.7.x 以下的版本。

## 出错详情

```text
Caused by: java.lang.UnsupportedClassVersionError: com/fasterxml/jackson/databind/AnnotationIntrospector : Unsupported major.minor version 51.0 (unable to load class com.fasterxml.jackson.databind.AnnotationIntrospector)
    at org.apache.catalina.loader.WebappClassLoader.findClassInternal(WebappClassLoader.java:2950) ~[catalina.jar:6.0.53]
    at org.apache.catalina.loader.WebappClassLoader.findClass(WebappClassLoader.java:1177) ~[catalina.jar:6.0.53]
    at org.apache.catalina.loader.WebappClassLoader.loadClass(WebappClassLoader.java:1665) ~[catalina.jar:6.0.53]
    at org.apache.catalina.loader.WebappClassLoader.loadClass(WebappClassLoader.java:1544) ~[catalina.jar:6.0.53]
    at org.springframework.boot.actuate.autoconfigure.EndpointAutoConfiguration.configurationPropertiesReportEndpoint(EndpointAutoConfiguration.java:183) ~[spring-boot-actuator-1.5.22.RELEASE.jar:1.5.22.RELEASE]
    at org.springframework.boot.actuate.autoconfigure.EndpointAutoConfiguration$$EnhancerBySpringCGLIB$$3c833ed0.CGLIB$configurationPropertiesReportEndpoint$10(<generated>) ~[spring-boot-actuator-1.5.22.RELEASE.jar:1.5.22.RELEASE]
    at org.springframework.boot.actuate.autoconfigure.EndpointAutoConfiguration$$EnhancerBySpringCGLIB$$3c833ed0$$FastClassBySpringCGLIB$$f590329b.invoke(<generated>) ~[spring-boot-actuator-1.5.22.RELEASE.jar:1.5.22.RELEASE]
    at org.springframework.cglib.proxy.MethodProxy.invokeSuper(MethodProxy.java:228) ~[spring-core-4.3.25.RELEASE.jar:4.3.25.RELEASE]
    at org.springframework.context.annotation.ConfigurationClassEnhancer$BeanMethodInterceptor.intercept(ConfigurationClassEnhancer.java:358) ~[spring-context-4.3.25.RELEASE.jar:4.3.25.RELEASE]
    at org.springframework.boot.actuate.autoconfigure.EndpointAutoConfiguration$$EnhancerBySpringCGLIB$$3c833ed0.configurationPropertiesReportEndpoint(<generated>) ~[spring-boot-actuator-1.5.22.RELEASE.jar:1.5.22.RELEASE]
    at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method) ~[na:1.6.0_45]
    at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:39) ~[na:1.6.0_45]
    at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:25) ~[na:1.6.0_45]
    at java.lang.reflect.Method.invoke(Method.java:597) ~[na:1.6.0_45]
    at org.springframework.beans.factory.support.SimpleInstantiationStrategy.instantiate(SimpleInstantiationStrategy.java:162) ~[spring-beans-4.3.25.RELEASE.jar:4.3.25.RELEASE]
    ... 33 common frames omitted
```

## 解决方式

覆盖 spring boot 指定的默认版本，降低 jackson 的版本到 2.6.x。

```xml
<jackson.version>2.6.7.3</jackson.version>
```
