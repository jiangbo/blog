# Java：sha256 摘要

## 环境

1. jdk 8
2. commons-codec 1.14

## 示例

### jdk

```java
package jiangbo.java.lang;

import java.io.IOException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

import javax.xml.bind.DatatypeConverter;

public class ShaDemo {

    public static void main(String[] args) throws IOException, NoSuchAlgorithmException {

        String str = "jiangbo";
        MessageDigest messageDigest = MessageDigest.getInstance("SHA-256");
        byte[] digestBytes = messageDigest.digest(str.getBytes("UTF-8"));
        String shaString = DatatypeConverter.printHexBinary(digestBytes);
        System.out.println(shaString);
    }
}
```

### commons-codec

加入依赖

```xml
<dependency>
    <groupId>commons-codec</groupId>
    <artifactId>commons-codec</artifactId>
    <version>1.14</version>
</dependency>
```

```java
package jiangbo.java.lang;

import java.util.Locale;

import org.apache.commons.codec.digest.DigestUtils;

public class ShaDemo {

    public static void main(String[] args) {

        String str = "jiangbo";
        String hex = DigestUtils.sha256Hex(str).toUpperCase(Locale.US);
        System.out.println(hex);
    }
}
```