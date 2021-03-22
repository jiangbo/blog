# Java：md5 摘要

## 环境

1. jdk 8
2. commons-codec 1.14
3. 4.3.24.RELEASE

## 示例

### jdk

```java
package jiangbo.java.lang;

import java.io.IOException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

import javax.xml.bind.DatatypeConverter;

public class Md5Demo {

    public static void main(String[] args) throws IOException, NoSuchAlgorithmException {

        String str = "jiangbo";
        MessageDigest messageDigest = MessageDigest.getInstance("MD5");
        byte[] digestBytes = messageDigest.digest(str.getBytes("UTF-8"));
        String md5String = DatatypeConverter.printHexBinary(digestBytes);
        System.out.println(md5String);
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

public class Md5Demo {

    public static void main(String[] args) {

        String str = "jiangbo";
        String md5Hex = DigestUtils.md5Hex(str).toUpperCase(Locale.US);
        System.out.println(md5Hex);
    }
}
```

### spring

加入依赖

```xml
<dependency>
    <groupId>org.springframework</groupId>
    <artifactId>spring-core</artifactId>
    <version>4.3.24.RELEASE</version>
</dependency>
```

```java
package jiangbo.java.lang;

import java.nio.charset.StandardCharsets;
import java.util.Locale;

import org.springframework.util.DigestUtils;

public class Md5Demo {

    public static void main(String[] args) {

        String str = "jiangbo";

//      jdk 6
//      Charset utf8 = Charset.forName("utf8");
//      String hex = DigestUtils.md5DigestAsHex(str.getBytes(utf8));

        String hex = DigestUtils.md5DigestAsHex(str.getBytes(StandardCharsets.UTF_8));
        System.out.println(hex.toUpperCase(Locale.US));
    }
}
```

### 工具类

```java
package jiangbo.java.lang;

import java.nio.charset.StandardCharsets;
import java.util.Locale;

import org.springframework.util.DigestUtils;

public class Md5Utils {

    public static final String digestAsHex(String str) {

        return DigestUtils.md5DigestAsHex(str.getBytes(StandardCharsets.UTF_8));
    }

    public static final String digestAsUpperCaseHex(String str) {

        return digestAsHex(str).toUpperCase(Locale.US);
    }
}
```


