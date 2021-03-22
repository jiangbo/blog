# Java：Base64 编码

## 环境

1. jdk 8 或者 6

## 示例

### jdk 6

```java
package jiangbo.java.lang;

import java.io.IOException;
import java.nio.charset.Charset;

import javax.xml.bind.DatatypeConverter;

import sun.misc.BASE64Decoder;
import sun.misc.BASE64Encoder;

public class Base64Demo {

    public static void main(String[] args) throws IOException {

        String name = "jiangbo";
        Charset utf8 = Charset.forName("UTF-8");

        BASE64Encoder base64Encoder = new sun.misc.BASE64Encoder();

        String BASE64EncoderString = base64Encoder.encode(name.getBytes(utf8));
        System.out.println(BASE64EncoderString);

        BASE64Decoder base64Decoder = new sun.misc.BASE64Decoder();
        byte[] decodeBuffer = base64Decoder.decodeBuffer(BASE64EncoderString);
        System.out.println(new String(decodeBuffer, utf8));

        String base64String = DatatypeConverter.printBase64Binary(name.getBytes(utf8));
        System.out.println(base64String);

        byte[] base64Binary = DatatypeConverter.parseBase64Binary(base64String);
        System.out.println(new String(base64Binary, utf8));
    }
}
```

### jdk 8

```java
package jiangbo.java.lang;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Base64;

public class Base64Demo {

    public static void main(String[] args) throws IOException {

        String name = "Base64 编码";
        String string = Base64.getEncoder().encodeToString(name.getBytes(StandardCharsets.UTF_8));
        System.out.println(string);

        byte[] bytes = Base64.getDecoder().decode(string);
        System.out.println(new String(bytes, StandardCharsets.UTF_8));

        // url safe：不会出现 + /
        string = Base64.getUrlEncoder().encodeToString(name.getBytes(StandardCharsets.UTF_8));
        System.out.println(string);

        bytes = Base64.getUrlDecoder().decode(string);
        System.out.println(new String(bytes, StandardCharsets.UTF_8));

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

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Base64;

public class Base64Demo {

    public static void main(String[] args) throws IOException {

        string = org.apache.commons.codec.binary.Base64.encodeBase64String(name.getBytes(StandardCharsets.UTF_8));
        System.out.println(string);

        bytes = org.apache.commons.codec.binary.Base64.decodeBase64(string);
        System.out.println(new String(bytes, StandardCharsets.UTF_8));

        // url safe：不会出现 + /
        string = org.apache.commons.codec.binary.Base64
                .encodeBase64URLSafeString(name.getBytes(StandardCharsets.UTF_8));
        System.out.println(string);

        bytes = org.apache.commons.codec.binary.Base64.decodeBase64(string);
        System.out.println(new String(bytes, StandardCharsets.UTF_8));
    }
}
```

### 工具类

```java
package jiangbo.java.lang;

import java.nio.charset.StandardCharsets;
import java.util.Base64;

public final class Base64Utils {

    private Base64Utils() {
    }

    public static final String encodeString(String str) {

        return Base64.getEncoder().encodeToString(str.getBytes(StandardCharsets.UTF_8));
    }

    public static final String decodeString(String str) {

        return new String(Base64.getDecoder().decode(str), StandardCharsets.UTF_8);
    }

    public static final String encodeUrlSafeString(String str) {

        return Base64.getUrlEncoder().encodeToString(str.getBytes(StandardCharsets.UTF_8));
    }

    public static final String decodeUrlSafeString(String str) {

        return new String(Base64.getUrlDecoder().decode(str), StandardCharsets.UTF_8);
    }
}
```


