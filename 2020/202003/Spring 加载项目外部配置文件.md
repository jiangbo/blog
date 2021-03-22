# Spring 加载项目外部配置文件

## 背景

在项目的部署过程中，一般是打成 war 或者 jar 包，这样一般存在两种问题：

1. 即使是配置文件修改，也还需要整个项目重新打包和部署。
2. 整个项目只有一套环境，不能切换。

针对上面的问题，可以使用外部化配置来解决。

## 需求

由于服务器上的应用服务器存放路径未知，只知应用服务器的目录结构。所以需要通过文件的相对路径，实现外部化配置。

## 环境

   1. JDK6
   2. Spring 3.2.5

## 方案

### File

不建议使用 File 的相对路径来获取，因为以下两点：

1. File 的相对目录会随着启动的目录变化而变化。
2. File 不能够获取 jar 包中的配置文件。

### Class

可以使用 `getClass().getResource` 或者 `getClass().getResourceAsStream` 来获取，可以获取到 jar 包中的文件。

1. 如果文件路径以“/”开头，表示获取classpath路径下的文件。
2. 不以“/”开头，获取和当前类同一目录下的文件。

### ClassLoader

ClassLoader 和 Class 类似，区别是 ClassLoader 以“/”开头返回null，并且相对路径是获取 classpath 下的文件。

## 实现

最终选择使用 ClassLoader 来实现较为简单，部分实现。

```java
InputStream is = getClass().getClassLoader().getResourceAsStream(EXTERNAL_CONFIG_FILE);

if (is == null) {
    logger.info("外部配置不存在。");
}
try {

    ResourcePropertySource source = new ResourcePropertySource(new InputStreamResource(is));
    // 外部配置优先级最高
    beanFactory.getBean(StandardEnvironment.class).getPropertySources().addFirst(source);

} catch (IOException e) {

    logger.error("获取外部配置失败。", e);
}
```

## 拓展

如果使用该种方法，出现错误：
`xxx has been normalized to [null] which is not valid`
则可以参考[这里][1]

[1]:https://www.cnblogs.com/jiangbo44/p/11948981.html
