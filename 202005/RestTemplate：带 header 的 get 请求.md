# RestTemplate：带 header 的 get 请求

## 环境

1. jdk 7
2. 4.3.24.RELEASE

## 示例

```java
package jiangbo.springweb.rest;

import java.util.Collections;

import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.client.RestTemplate;

public class RestTemplateDemo02 {

    public static void main(String[] args) {

        RestTemplate template = new RestTemplate();

        String url = "http://www.baidu.com";

        HttpHeaders headers = new HttpHeaders();
        headers.add(HttpHeaders.AUTHORIZATION, "Bearer 123456");
        headers.setAccept(Collections.singletonList(MediaType.TEXT_HTML));

        HttpEntity<String> entity = new HttpEntity<>(headers);
        ResponseEntity<String> response = template.exchange(url, HttpMethod.GET, entity, String.class);
        System.out.println(response.getBody());
    }
}
```