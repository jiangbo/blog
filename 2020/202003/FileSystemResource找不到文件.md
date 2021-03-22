# FileSystemResource 找不到文件

## 环境

1. Spring 3.2.5.RELEASE

## 原因

使用 FileSystemResource 加载文件的过程中，发现一个奇怪的现象，路径完全正确，但是找不到文件的情况。可能的原因是文件的路径上有压缩文件，比如 war 或者 jar，相对路径也不行。

## 分析

File 只能和文件系统对应，不能获取到压缩包中的文件，所以即使相对路径计算后，还是不能获取到相应的文件。

## 外部配置

先使用相对路径计算出文件的绝对路径，再新建一个 FileSystemResource。

```java
// classpath 目录，上级目录的个数和当前类的类名层级相对应。
String path = PathDemo.class.getResource("../../").getPath();

try {

    // 不能直接通过文件或者文件路径创建，需要使用解析后的文件路径创建。
    String canonicalPath = new File(path + EXTERNAL_CONFIG_FILE).getCanonicalPath();
    logger.info("外部配置文件路径为：{}", canonicalPath);
    Resource resource = new FileSystemResource(canonicalPath);

    if (!resource.exists()) {

        logger.info("外部配置不存在。");
        return;
    }

    ResourcePropertySource source = new ResourcePropertySource(new EncodedResource(resource, "UTF-8"));
    // 外部配置的优先级最高
    beanFactory.getBean(StandardEnvironment.class).getPropertySources().addFirst(source);

} catch (IOException e) {

    logger.error("加载外部化配置出错。", e);
}
```
