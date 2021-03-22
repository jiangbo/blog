# 【Java】JavaScript 加密 Java 解密

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Maven 3.6.3
- JSEncrypt 2.3.1

## 概述

前端 JavaScript 使用 RSA 的公钥加密，后端 Java 使用私钥进行解密。

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.springmvc</groupId>
    <artifactId>spring-mvc-rsa</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>
    <description>RSA：javascript 加密，java 解密</description>

    <properties>
        <maven.compiler.source>1.8</maven.compiler.source>
        <maven.compiler.target>1.8</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

    <dependencyManagement>
        <dependencies>
            <dependency>
                <groupId>org.springframework</groupId>
                <artifactId>spring-framework-bom</artifactId>
                <version>4.3.24.RELEASE</version>
                <type>pom</type>
                <scope>import</scope>
            </dependency>
        </dependencies>
    </dependencyManagement>

    <dependencies>

        <dependency>
            <groupId>javax.servlet</groupId>
            <artifactId>javax.servlet-api</artifactId>
            <version>3.1.0</version>
            <scope>provided</scope>
        </dependency>

        <dependency>
            <groupId>org.springframework</groupId>
            <artifactId>spring-webmvc</artifactId>
        </dependency>

    </dependencies>


    <build>
        <plugins>
            <plugin>
                <artifactId>maven-war-plugin</artifactId>
                <version>3.1.0</version>
            </plugin>
        </plugins>
    </build>

</project>
```

### RootConfiguration

```java
package jiangbo.springmvc;

import org.springframework.context.annotation.ComponentScan;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.EnableWebMvc;
import org.springframework.web.servlet.config.annotation.ResourceHandlerRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurerAdapter;

@Configuration
@ComponentScan
@EnableWebMvc
public class RootConfiguration extends WebMvcConfigurerAdapter {

    @Override
    public void addResourceHandlers(ResourceHandlerRegistry registry) {

        registry.addResourceHandler("/**").addResourceLocations("/");
    }
}
```

### JiangBoDispatcherServletInitializer

```java
package jiangbo.springmvc;

import org.springframework.web.servlet.support.AbstractAnnotationConfigDispatcherServletInitializer;

public class JiangBoDispatcherServletInitializer extends AbstractAnnotationConfigDispatcherServletInitializer {

    @Override
    protected Class<?>[] getRootConfigClasses() {
        return null;
    }

    @Override
    protected Class<?>[] getServletConfigClasses() {
        return new Class<?>[] { RootConfiguration.class };
    }

    @Override
    protected String[] getServletMappings() {
        return new String[] { "/" };
    }
}
```

### WelcomeController

```java
package jiangbo.springmvc.controller;

import java.security.GeneralSecurityException;
import java.security.KeyFactory;
import java.security.spec.PKCS8EncodedKeySpec;
import java.util.Base64;

import javax.crypto.Cipher;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.ResponseBody;

@Controller
public class WelcomeController {

    private static final String PRIVATE_KEY = "MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC59t6pNlV7cM6CY9jvrNCxgP4cuJrJfglCwdvQwei/zmH2Q4hzdYsSTlgjCOGbm18BKuzZi+wKtWVg9CqnwHJa1q7HKB7nNv9kwoOE/nLB5c370AFC+wtkgdvmbCqkIquKR/Yzyo5Q2a2+JTCOO64Vi+wArPUUCc86XWohApV+Kg1Syti0dTQ6V76R/cNmqGORKf3yv+es6iwUPcYSvfB83SK5iHCUrbKOv7rE5CoFYTiW6X53TRz3GSfIW22NdhQgm9vKA5FjilXP0cgYlRUEI9Zob6unLhIynhxiU1fEEPhVkn2nwTnA+m9Tv7cL2oZIgKp48eTCCeviWFCUg947AgMBAAECggEBAJD0CSNEouURTSb9xT8tKY7yySCCFMUyZO2QRlM2ksHPkTRv95Bn8vtV7gWBCrpMJYW3e98m3dqLAnbMW+NecsSzypCaiVwY1oInzHSCEdlp02GRBSq8nGi/gDTf420FSUkERrigDsaepnssaei0REGvS0pLhs0TyF/qKKuKZBRuRpIMzHEgL2rOn4gdv5blGp0q0tsUgXrWQmpcR9u7LVKdUD9N/4EmgCEIZcELrzqe8xwjDosV9gQ5RmJ4gwqSg3KcYHuPopH6gD/iQVuC4uJBekhyVLnqjEC6STXfETYnn2+fYe1YaG3C8vHOfV/nmRHzyQMWXs8+wuiqvX7HiOECgYEA4d3E3qDjSaJ3QeAzdaR3/lInPSvHwqpuEQOmEkPkf0PaRjiPmVGrmEPgSmWdIpxOmv+3geaWJgmyOWnpVXg9qsH49WeIj/W6qlEy6eZNBU5rlvdRX0WxV6UcfC1/T0uKGvYW6ryzkFlefmdsYqcuDuiB+eyzpcHwJj8/qV+YtXcCgYEA0sZKZkB9FiMP4f2D8iCzH8cC/lPAYBtcyduUOTCtBC/MEBj8NucXBhGpGhboAyY12MTtlASI2sXLvxplo2cyxMqKq2lQvTnyxz7dFSwQ8cveOy6+siZv44d64X5VWcTV5VW02x481vok7Y0+L5Pvl/DmHoCo9Vhu1X7yNxrqHl0CgYEAjAbnJHZuMppsgQS0In0Cf0MDXKMlxwPOFPJEJRK+OEAitGIgdJbzYQTDqpvwnSJ2gRWD+R6xe4eLXJjStdBDuvdilxHG0ikvvqN0gxnqKh4CCafuZLhaR9HvJPU39jsItcpXTtQKJrWhJZCM22R143w2CHG8tkjTz4Jqv2Cq1eUCgYA8LuRlh/pvjLlnRPUc+NtMso2XQyDGEGoRNUKAhfaRIHi7C/dQUfHQCNDpIytCYvGzIJ/ZyOWu2hWQTqBA5SGo8VBhG2Qzc81vuPJ1rix0bkrZSUKoKbN/G1sp8eG7DSoqHqvSeTLUC37p2Y+tFCewkdeTVBdqXsLe8S2GMIgy/QKBgFAxeR6zlWLBKB6nPAYRJhe/9n6qfFRrmEsNy0JVocJkBT2qfprtilBZCwVOyM5vABYEUwJ6w5IaLe51o3Tp9gXSqWf2WXfImXGM6A7wo3ShIE4VSFh128OfLGGnBIR53Ry3/1nhFT+bExJb/NMOi2q3kIQ1aN0OHW67x4EeDV5A";

    @PostMapping(value = "decrypt", produces = "text/plain;charset=UTF-8")
    @ResponseBody
    public String decrypt(String text) throws GeneralSecurityException {

        System.out.println("加密后：" + text);
        String pwd = decrypt(text, PRIVATE_KEY);
        return "服务器返回：" + pwd;
    }

    static String decrypt(String data, String privateKey) throws GeneralSecurityException {

        PKCS8EncodedKeySpec keySpec = new PKCS8EncodedKeySpec(base64Decode(privateKey));
        KeyFactory keyFactory = KeyFactory.getInstance("RSA");

        Cipher cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
        cipher.init(Cipher.DECRYPT_MODE, keyFactory.generatePrivate(keySpec));
        return new String(cipher.doFinal(base64Decode(data)));
    }

    static byte[] base64Decode(String str) {

        return Base64.getDecoder().decode(str);
    }
}
```

### index.html

```html
<!DOCTYPE html>
<html lang="zh">

<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <script src="https://cdn.bootcdn.net/ajax/libs/jsencrypt/2.3.1/jsencrypt.js"></script>
  <title>加解密</title>
</head>

<body>

  <input type="text" id="text"> <input id="send" type="button" value="发送"><br>
  <span></span>
  <script type="text/javascript">
    let publicKey = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAufbeqTZVe3DOgmPY76zQsYD+HLiayX4JQsHb0MHov85h9kOIc3WLEk5YIwjhm5tfASrs2YvsCrVlYPQqp8ByWtauxyge5zb/ZMKDhP5yweXN+9ABQvsLZIHb5mwqpCKrikf2M8qOUNmtviUwjjuuFYvsAKz1FAnPOl1qIQKVfioNUsrYtHU0Ole+kf3DZqhjkSn98r/nrOosFD3GEr3wfN0iuYhwlK2yjr+6xOQqBWE4lul+d00c9xknyFttjXYUIJvbygORY4pVz9HIGJUVBCPWaG+rpy4SMp4cYlNXxBD4VZJ9p8E5wPpvU7+3C9qGSICqePHkwgnr4lhQlIPeOwIDAQAB";

    let RSAEncrypt = new JSEncrypt();
    RSAEncrypt.setPublicKey(publicKey);

    document.getElementById("send").onclick = function () {
      let text = document.getElementById("text").value;
      console.log("原始内容" + text);

      let encryptedPass = RSAEncrypt.encrypt(text);
      console.log("加密内容" + encryptedPass);
      send(encryptedPass);
    }

    /**
     * 发送请求
     * 
     * */
    function send(text) {

      httpRequest = new XMLHttpRequest();
      httpRequest.onreadystatechange = function (result) {
        if (httpRequest.readyState === XMLHttpRequest.DONE) {
          document.getElementsByTagName("span")[0].textContent = httpRequest.responseText;
        }
      };
      httpRequest.open('POST', 'decrypt');
      httpRequest.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
      httpRequest.send("text=" + encodeURIComponent(text));
    }

  </script>
</body>

</html>
```

### 浏览器显示

部署服务到 tomcat，访问：http://localhost:8080/spring-mvc-rsa/index.html ，看到浏览器有一个输入框。
输入内容进行加密，点击发送，后端可以解密，并将解密的内容返回前端。

其中 PublicKey 和 PrivateKey 的生成请参考 [【Java】RSA 生成密钥对][1]

[1]: 