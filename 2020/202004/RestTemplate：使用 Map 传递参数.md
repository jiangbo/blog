# RestTemplate：使用 Map 传递参数

## 环境

1. jdk 7
2. spring 4.3.24.RELEASE

## 原因

在使用 RestTemplate 的过程中，使用 java.util.HashMap 传递参数一直不生效。根据 [这里][1] 的说法，需要使用 MultiValueMap 和 其实现类 LinkedMultiValueMap。

## 分析

下面从源码来看，为什么 HashMap 不生效而 LinkedMultiValueMap 可以生效。

### debug 源码

跟着源码一直走下去，可以发现一个关键的类：FormHttpMessageConverter，这个类的 canWrite 方法如下：

```java
public boolean canWrite(Class<?> clazz, MediaType mediaType) {
    if (!MultiValueMap.class.isAssignableFrom(clazz)) {
        return false;
    }
    if (mediaType == null || MediaType.ALL.equals(mediaType)) {
        return true;
    }
    for (MediaType supportedMediaType : getSupportedMediaTypes()) {
        if (supportedMediaType.isCompatibleWith(mediaType)) {
            return true;
        }
    }
    return false;
}
```

在这里检查了请求中传递的类型是否实现了 MultiValueMap，所以不是这个类型的不能被处理。
java.util.HashMap 不是 MultiValueMap 类型，所以不能被处理。

## 示例

```java
package jiangbo.springweb.rest;

import org.springframework.util.LinkedMultiValueMap;
import org.springframework.util.MultiValueMap;
import org.springframework.web.client.RestTemplate;

public class RestTemplateDemo04 {

    public static void main(String[] args) {

        String url = "https://dx.ipyy.net/sms.aspx";

        MultiValueMap<String, String> parameters = new LinkedMultiValueMap<>();

        RestTemplate restTemplate = new RestTemplate();
        String result = restTemplate.postForObject(url, parameters, String.class);

        System.out.println(result);

        parameters.add("action", "send");
        result = restTemplate.postForObject(url, parameters, String.class);

        System.out.println(result);
    }
}
```

[1]:https://stackoverflow.com/questions/49667914/spring-resttemplate-post-using-parameters-in-a-hashmap-throws-400-bad-request

