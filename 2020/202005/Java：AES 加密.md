# Java：AES 加密

## 环境

1. jdk 8
2. spring-security-crypto 4.2.12.RELEASE

## 示例

AES （Advanced Encryption Standard）属于对称加密，用来替换 DES 和 3DES 加密。
AES 加密有几种模式可选，下面演示 ECB 和 CBC 模式。
以下涉及到的 Base64Utils 工具类，可以参考 [Java：Base64 编码][1]。

### ECB

```java
package jiangbo.java.lang;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;

import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;

public class AesEcbDemo {

    public static void main(String[] args) throws Exception {

        String name = "jiangbo";
        String seed = "seed";
        String str = encrypt(name, seed);
        System.out.println(str);

        System.out.println(decrypt(str, seed));
    }

    public static final String encrypt(String str, String key) throws Exception {

        KeyGenerator keyGenerator = KeyGenerator.getInstance("AES");
        keyGenerator.init(128, new SecureRandom(key.getBytes(StandardCharsets.UTF_8)));
        SecretKey secretKey = keyGenerator.generateKey();

        Cipher cipher = Cipher.getInstance("AES/ECB/PKCS5Padding");
        cipher.init(Cipher.ENCRYPT_MODE, secretKey);

        byte[] bytes = cipher.doFinal(str.getBytes(StandardCharsets.UTF_8));

        return Base64Utils.encodeToString(bytes);
    }

    public static final String decrypt(String str, String key) throws Exception {

        KeyGenerator keyGenerator = KeyGenerator.getInstance("AES");
        keyGenerator.init(128, new SecureRandom(key.getBytes(StandardCharsets.UTF_8)));
        SecretKey secretKey = keyGenerator.generateKey();

        Cipher cipher = Cipher.getInstance("AES/ECB/PKCS5Padding");
        cipher.init(Cipher.DECRYPT_MODE, secretKey);

        byte[] bytes = cipher.doFinal(Base64Utils.decodeString(str));

        return new String(bytes, StandardCharsets.UTF_8);
    }
}
```

### CBC

```java
package jiangbo.java.lang;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;

import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import javax.crypto.spec.IvParameterSpec;

public class AesCbcDemo {

    public static void main(String[] args) throws Exception {

        String name = "jiangbo";
        String seed = "seed";
        String str = encrypt(name, seed);
        System.out.println(str);

        System.out.println(decrypt(str, seed));

    }

    public static final String encrypt(String str, String key) throws Exception {

        KeyGenerator keyGenerator = KeyGenerator.getInstance("AES");
        SecureRandom secureRandom = new SecureRandom(key.getBytes(StandardCharsets.UTF_8));
        keyGenerator.init(128, secureRandom);
        SecretKey secretKey = keyGenerator.generateKey();

        byte[] iv = new byte[16];
        secureRandom.nextBytes(iv);
        IvParameterSpec ivParameterSpec = new IvParameterSpec(iv);

        Cipher cipher = Cipher.getInstance("AES/CBC/PKCS5Padding");
        cipher.init(Cipher.ENCRYPT_MODE, secretKey, ivParameterSpec);

        byte[] bytes = cipher.doFinal(str.getBytes(StandardCharsets.UTF_8));

        return Base64Utils.encodeToString(bytes);
    }

    public static final String decrypt(String str, String key) throws Exception {

        KeyGenerator keyGenerator = KeyGenerator.getInstance("AES");
        SecureRandom secureRandom = new SecureRandom(key.getBytes(StandardCharsets.UTF_8));
        keyGenerator.init(128, secureRandom);
        SecretKey secretKey = keyGenerator.generateKey();

        byte[] iv = new byte[16];
        secureRandom.nextBytes(iv);
        IvParameterSpec ivParameterSpec = new IvParameterSpec(iv);

        Cipher cipher = Cipher.getInstance("AES/CBC/PKCS5Padding");
        cipher.init(Cipher.DECRYPT_MODE, secretKey, ivParameterSpec);

        byte[] bytes = cipher.doFinal(Base64Utils.decodeString(str));

        return new String(bytes, StandardCharsets.UTF_8);
    }
}
```

### spring

加入依赖

```xml
<dependency>
    <groupId>org.springframework.security</groupId>
    <artifactId>spring-security-crypto</artifactId>
    <version>4.2.12.RELEASE</version>
</dependency>
```

代码的示例参考的[这里][2]。

```java
package jiangbo.spring.security;

import org.springframework.security.crypto.encrypt.Encryptors;
import org.springframework.security.crypto.encrypt.TextEncryptor;
import org.springframework.security.crypto.keygen.KeyGenerators;

public class EncryptorsDemo {

    public static void main(String[] args) {

        final String password = "I AM SHERLOCKED";
        final String salt = KeyGenerators.string().generateKey();
        System.out.println(salt);

        TextEncryptor encryptor = Encryptors.text(password, salt);
        System.out.println("Salt: \"" + salt + "\"");

        String textToEncrypt = "*royal secrets*";
        System.out.println("Original text: \"" + textToEncrypt + "\"");

        String encryptedText = encryptor.encrypt(textToEncrypt);
        System.out.println("Encrypted text: \"" + encryptedText + "\"");

        // Could reuse encryptor but wanted to show reconstructing TextEncryptor
        TextEncryptor decryptor = Encryptors.text(password, salt);
        String decryptedText = decryptor.decrypt(encryptedText);
        System.out.println("Decrypted text: \"" + decryptedText + "\"");

        if (textToEncrypt.equals(decryptedText)) {
            System.out.println("Success: decrypted text matches");
        } else {
            System.out.println("Failed: decrypted text does not match");
        }
    }
}
```

### 工具类

```java
package jiangbo.utils;

import org.springframework.security.crypto.encrypt.Encryptors;
import org.springframework.security.crypto.encrypt.TextEncryptor;

public class AesUtils {

    private static final String PASSWORD = "jiangbo";

    // 使用 KeyGenerators.string().generateKey() 生成 salt
    private static final String SALT = "99653a8359ed31e1";

    private static final TextEncryptor TEXT_ENCRYPTOR = Encryptors.text(PASSWORD, SALT);

    public static final String encrypt(String str) {

        return TEXT_ENCRYPTOR.encrypt(str);
    }

    public static final String decrypt(String str) {

        return TEXT_ENCRYPTOR.decrypt(str);
    }
}
```

[1]:https://www.cnblogs.com/jiangbo44/p/12808207.html
[2]:https://stackoverflow.com/questions/992019/java-256-bit-aes-password-based-encryption


