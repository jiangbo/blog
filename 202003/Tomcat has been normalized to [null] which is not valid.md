# Tomcat:has been normalized to [null] which is not valid

## 环境

1. tomcat 8.5

## 原因

在使用相对路径加载配置文件时，如果相对路径超出了 tomcat 容器的根目录，那么 tomcat 会提示 `xxx has been normalized to [null] which is not valid` 。

## 分析

下面从 tomcat 的源码来进行分析。

### validate

在 StandardRoot 中，有一个 validate 方法，该方法的注释如下：

```text
/**
    * Ensures that this object is in a valid state to serve resources, checks
    * that the path is a String that starts with '/' and checks that the path
    * can be normalized without stepping outside of the root.
    *
    * @param path
    * @return  the normalized path
    */
```

方法的最后一句说明了出错的原因，不能够超出容器的根目录。

### normalize

该方法可以解析路径参数，其中超出根目录的检查逻辑就在这里。在解析父级目录时，如果发现超出了根目录，则直接返回 null。

```java
while (true) {
    int index = normalized.indexOf("/../");
    if (index < 0) {
        break;
    }
    if (index == 0) {
        return null;  // Trying to go outside our context
    }
    int index2 = normalized.lastIndexOf('/', index - 1);
    normalized = normalized.substring(0, index2) + normalized.substring(index + 3);
}
```

## 外部配置

在之前的外部配置中，由于使用了相对路径，并且超出了容器的根目录，如果出现这个错误，则需要进行修改，可以使用 URL 的方式来实现。

```java
URL url = Thread.currentThread().getContextClassLoader().getResource("");

try {

    Resource resource = new UrlResource(url).createRelative(EXTERNAL_CONFIG_FILE);

    if (!resource.exists()) {

        logger.info("外化配置不存在。");
        return;
    }

    logger.info("外部配置存在。");
    ResourcePropertySource source = new ResourcePropertySource(new EncodedResource(resource, "UTF-8"));
    // 外部配置的优先级最高
    beanFactory.getBean(StandardEnvironment.class).getPropertySources().addFirst(source);

} catch (IOException e) {

    logger.error("加载外部化配置出错。", e);
}
```

## 拓展

如果使用该种方法，出现错误：
`java.lang.RuntimeException: java.io.IOException: Using reverse path on top path: /xxx`
则可以参考[这里][1]

[1]:https://www.cnblogs.com/jiangbo44/p/11948981.html

## 附录

### validate方法

```java
/**
 * Ensures that this object is in a valid state to serve resources, checks
 * that the path is a String that starts with '/' and checks that the path
 * can be normalized without stepping outside of the root.
 *
 * @param path
 * @return  the normalized path
 */
private String validate(String path) {
    if (!getState().isAvailable()) {
        throw new IllegalStateException(
                sm.getString("standardRoot.checkStateNotStarted"));
    }

    if (path == null || path.length() == 0 || !path.startsWith("/")) {
        throw new IllegalArgumentException(
                sm.getString("standardRoot.invalidPath", path));
    }

    String result;
    if (File.separatorChar == '\\') {
        // On Windows '\\' is a separator so in case a Windows style
        // separator has managed to make it into the path, replace it.
        result = RequestUtil.normalize(path, true);
    } else {
        // On UNIX and similar systems, '\\' is a valid file name so do not
        // convert it to '/'
        result = RequestUtil.normalize(path, false);
    }

    // 检查到超出根目录，在这里抛出了异常。
    if (result == null || result.length() == 0 || !result.startsWith("/")) {
        throw new IllegalArgumentException(
                sm.getString("standardRoot.invalidPathNormal", path, result));
    }

    return result;
}
```

### normalize方法

```java
/**
 * Normalize a relative URI path that may have relative values ("/./",
 * "/../", and so on ) it it.  <strong>WARNING</strong> - This method is
 * useful only for normalizing application-generated paths.  It does not
 * try to perform security checks for malicious input.
 *
 * @param path Relative path to be normalized
 * @param replaceBackSlash Should '\\' be replaced with '/'
 *
 * @return The normalized path or <code>null</code> if the path cannot be
 *         normalized
 */
public static String normalize(String path, boolean replaceBackSlash) {

    if (path == null) {
        return null;
    }

    // Create a place for the normalized path
    String normalized = path;

    if (replaceBackSlash && normalized.indexOf('\\') >= 0)
        normalized = normalized.replace('\\', '/');

    // Add a leading "/" if necessary
    if (!normalized.startsWith("/"))
        normalized = "/" + normalized;

    boolean addedTrailingSlash = false;
    if (normalized.endsWith("/.") || normalized.endsWith("/..")) {
        normalized = normalized + "/";
        addedTrailingSlash = true;
    }

    // Resolve occurrences of "//" in the normalized path
    while (true) {
        int index = normalized.indexOf("//");
        if (index < 0) {
            break;
        }
        normalized = normalized.substring(0, index) + normalized.substring(index + 1);
    }

    // Resolve occurrences of "/./" in the normalized path
    while (true) {
        int index = normalized.indexOf("/./");
        if (index < 0) {
            break;
        }
        normalized = normalized.substring(0, index) + normalized.substring(index + 2);
    }

    // 在这里检查，超出根目录返回null。
    // Resolve occurrences of "/../" in the normalized path
    while (true) {
        int index = normalized.indexOf("/../");
        if (index < 0) {
            break;
        }
        if (index == 0) {
            return null;  // Trying to go outside our context
        }
        int index2 = normalized.lastIndexOf('/', index - 1);
        normalized = normalized.substring(0, index2) + normalized.substring(index + 3);
    }

    if (normalized.length() > 1 && addedTrailingSlash) {
        // Remove the trailing '/' we added to that input and output are
        // consistent w.r.t. to the presence of the trailing '/'.
        normalized = normalized.substring(0, normalized.length() - 1);
    }

    // Return the normalized path that we have completed
    return normalized;
}

```
