# 【JUnit】JUnit 异常测试

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 说明

异常测试在 4.13 之前也有，都有一点缺点，不推荐使用，推荐使用 4.13 之后的 assertThrows。

## 异常测试示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>11-java-junit-exception</artifactId>
    <version>1.0.0</version>
    <description>JUnit 异常测试示例</description>

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

### Caculator

```java
package jiangbo.java.junit;

public class Caculator {

    public static int add(int number1, int number2) {

        return number1 + number2;
    }

    public static int subtract(int number1, int number2) {

        return number1 - number2;
    }

    public static int divide(int number1, int number2) {

        return number1 / number2;
    }
}
```

### CaculatorTest

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertThrows;
import static org.junit.Assert.fail;

import org.junit.Rule;
import org.junit.Test;
import org.junit.function.ThrowingRunnable;
import org.junit.rules.ExpectedException;

public class CaculatorTest {

    @SuppressWarnings("deprecation")
    @Rule
    public ExpectedException thrown = ExpectedException.none();

    @Test(expected = ArithmeticException.class)
    public void testDivide1() {

        System.out.println("testDivide1");
        Caculator.divide(1, 0);

    }

    @Test
    public void testDivide2() {

        try {
            System.out.println("testDivide2");
            Caculator.divide(1, 0);
            fail("Expected an ArithmeticException to be thrown");

        } catch (ArithmeticException e) {

            assertEquals(e.getMessage(), "/ by zero");
        }
    }

    @Test
    public void testDivide3() {

        thrown.expect(ArithmeticException.class);
        thrown.expectMessage("/ by zero");

        System.out.println("testDivide3");
        Caculator.divide(1, 0);
    }

    @Test
    public void testDivide4() {

        // 推荐使用这种方式，特别是 JDK 大于等于 8 的时候
        ArithmeticException thrown = assertThrows(ArithmeticException.class, new ThrowingRunnable() {

            @Override
            public void run() throws Throwable {

                System.out.println("testDivide2");
                Caculator.divide(1, 0);
            }
        });

        // java 8
        // ArithmeticException thrown = assertThrows(ArithmeticException.class, () -> Caculator.divide(1, 0));

        assertEquals("/ by zero", thrown.getMessage());
    }
}
```

### 运行

通过运行 CaculatorTest 测试类，控制台输出：

```text
testDivide1
testDivide2
testDivide3
testDivide2
```
