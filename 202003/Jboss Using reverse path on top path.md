# Jboss: Using reverse path on top path: /xxx

## 环境

1. jboss 5.2

## 原因

加载资源的协议错误。一般在加载文件的时候，URL 都是以 `file:` 开头，但是在 jboss 上时，由于其虚拟化了路径，导致协议不一致，并且找不到外部的配置文件。

## 分析

通过将项目部署到 jboss 服务器上，打印日志查看其获得的 URL 如下：

`vfsmemory://a653x1c-xfikka-k3i9k2ku-1-k3i9kk9n-2s/`

该目录结构已被虚拟化，不能够获得服务器中的目录结构。

## 外部配置

优化后的外部配置，使用 tomcat 服务器是可以了，由于 jboss 比较特殊，不能够获取服务器真实的相对路径。可以使用 `getClass().getResource("")` ，即使用当前类的相对路径来解决。

```java
// classpath 目录，上级目录的个数和当前类的类名层级相对应。
String path = PathDemo.class.getResource("../../").getPath();
Resource resource = new FileSystemResource(path + EXTERNAL_CONFIG_FILE);

try {

    logger.info("外部配置目录为：{}", resource.getFile().getCanonicalPath());

    if (!resource.exists()) {

        logger.info("外部配置不存在。");
        return;
    }

    ResourcePropertySource source = new ResourcePropertySource(new EncodedResource(resource, "UTF-8"));
    // 外部化配置的优先级最高
    beanFactory.getBean(StandardEnvironment.class).getPropertySources().addFirst(source);

} catch (IOException e) {

    logger.error("加载外部化配置出错。", e);
}
```

## 拓展

如果使用该种方法，发现目录结构完全正确，但始终不能找到配置文件，
则可以参考[这里][1]

[1]:https://www.cnblogs.com/jiangbo44/p/11948981.html

## 附录

### 获取 jboss 目录

```java
logger.info("class / :{}", getClass().getResource("/"));
logger.info("class :{}", getClass().getResource(""));
logger.info("file class / :{}", getClass().getResource("/").getFile());
logger.info("file class :{}", getClass().getResource("").getFile());
logger.info("path class / :{}", getClass().getResource("/").getPath());
logger.info("path class :{}", getClass().getResource("").getPath());
logger.info("loader / :{}", getClass().getClassLoader().getResource("/"));
logger.info("loader:{}", getClass().getClassLoader().getResource(""));
logger.info("file loader / :{}", getClass().getClassLoader().getResource("/").getFile());
logger.info("file loader:{}", getClass().getClassLoader().getResource("").getFile());
logger.info("path loader / :{}", getClass().getClassLoader().getResource("/").getPath());
logger.info("path loader:{}", getClass().getClassLoader().getResource("").getPath());
logger.info("thread /: {}", threadLoader.getResource("/"));
logger.info("thread : {}", threadLoader.getResource(""));
logger.info("file thread /: {}", threadLoader.getResource("/").getFile());
logger.info("file thread : {}", threadLoader.getResource("").getFile());
logger.info("path thread /: {}", threadLoader.getResource("/").getPath());
logger.info("path thread : {}", threadLoader.getResource("").getPath());
```

### 对应输出结果

```text
 class / :vfsmemory://a653x1c-xfikka-k3i9k2ku-1-k3i9kk9n-2s/
 class :vfszip:/DATA/app/jboss/appdeploy/demo.war/WEB-INF/classes/jiangbo/demo/
 file class / :/
 file class :/DATA/app/jboss/appdeploy/demo.war/WEB-INF/classes/jiangbo/demo/
 path class / :/
 path class :/DATA/app/jboss/appdeploy/demo.war/WEB-INF/classes/jiangbo/demo/
 loader / :vfsmemory://a653x1c-xfikka-k3i9k2ku-1-k3i9kk9n-2s/
 loader:vfsmemory://a653x1c-xfikka-k3i9k2ku-1-k3i9kk9n-2s/
 file loader / :/
 file loader:/
 path loader / :/
 path loader:/
 thread /: vfsmemory://a653x1c-xfikka-k3i9k2ku-1-k3i9kk9n-2s/
 thread : vfsmemory://a653x1c-xfikka-k3i9k2ku-1-k3i9kk9n-2s/
 file thread /: /
 file thread : /
 path thread /: /
 path thread : /
```
