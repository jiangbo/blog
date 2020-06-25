# 【JUnit】JUnit Assert 断言

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 断言

> 在程序设计中，断言（assertion）是一种放在程序中的一阶逻辑（如一个结果为真或是假的逻辑判断式），目的是为了标示与验证程序开发者预期的结果－当程序运行到断言的位置时，对应的断言应该为真。若断言不为真时，程序会中止运行，并给出错误消息。 --来源于维基百科

## Assert 断言示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>12-java-junit-assertions</artifactId>
    <version>1.0.0</version>
    <description>JUnit Assertions 示例</description>

    <properties>
        <maven.compiler.source>1.6</maven.compiler.source>
        <maven.compiler.target>1.6</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

    <dependencies>

        <dependency>
            <groupId>junit</groupId>
            <artifactId>junit</artifactId>
            <version>4.13</version>
            <scope>test</scope>
        </dependency>

    </dependencies>

</project>
```

### CaculatorTest

示例来源于官网

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertNotSame;
import static org.junit.Assert.assertNull;
import static org.junit.Assert.assertSame;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

public class CaculatorTest {

    @Test
    public void testAssertArrayEquals() {

        byte[] expected = "trial".getBytes();
        byte[] actual = "trial".getBytes();
        assertArrayEquals("failure - byte arrays not same", expected, actual);
    }

    @Test
    public void testAssertEquals() {

        assertEquals("failure - strings are not equal", "text", "text");
    }

    @Test
    public void testAssertFalse() {

        assertFalse("failure - should be false", false);
    }

    @Test
    public void testAssertNotNull() {

        assertNotNull("should not be null", new Object());
    }

    @Test
    public void testAssertNotSame() {

        assertNotSame("should not be same Object", new Object(), new Object());
    }

    @Test
    public void testAssertNull() {

        assertNull("should be null", null);
    }

    @Test
    public void testAssertSame() {

        Integer aNumber = Integer.valueOf(768);
        assertSame("should be same", aNumber, aNumber);
    }

    @Test
    public void testAssertTrue() {

        assertTrue("failure - should be true", true);
    }
}
```
