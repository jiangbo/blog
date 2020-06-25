# 【Java】RSA 生成密钥对

## 环境

- JDK 8
- Spring Tool Suite 4.6.1
- Maven 3.6.3

## 概述

使用 RSA 算法生成公钥和私钥。

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.rsa</groupId>
    <artifactId>java-rsa</artifactId>
    <version>1.0.0</version>
    <description>Java 生成 RSA 密钥对示例</description>

    <properties>
        <maven.compiler.source>1.8</maven.compiler.source>
        <maven.compiler.target>1.8</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

</project>
```

### RSADemo

```java
package jiangbo.java.rsa;

import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.NoSuchAlgorithmException;
import java.util.Base64;

public class RSADemo {

    public static void main(String[] args) throws NoSuchAlgorithmException {

        KeyPairGenerator keyGen = KeyPairGenerator.getInstance("RSA");
        keyGen.initialize(2048);
        KeyPair pair = keyGen.generateKeyPair();

        byte[] publicBytes = pair.getPublic().getEncoded();
        byte[] privateBytes = pair.getPrivate().getEncoded();

        System.out.println("public key: " + base64Encode(publicBytes));
        System.out.println("private key: " + base64Encode(privateBytes));
    }

    static String base64Encode(byte[] bytes) {

        return Base64.getEncoder().encodeToString(bytes);
    }
}
```