# 【JUnit】JUnit 假设测试

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 假设

Assumptions 即假设，JUnit 用它来确定条件满足的情况下才执行测试，否则就直接忽略。
如果是在 @Before 或者 @BeforeClass 方法中，和放在 @Test 方法内一样的效果。

## 假设示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>16-java-junit-assume</artifactId>
    <version>1.0.0</version>
    <description>JUnit 假设测试示例</description>

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
import static org.junit.Assume.assumeFalse;

import org.junit.Test;

public class CaculatorTest {

    @Test
    public void testAdd() {

        int i = 2;
        assumeFalse(1 + 1 == i); // 这世界肯定疯了，下面的测试就不要执行了吧
        assertEquals(i, Caculator.add(1, 1));
    }

    @Test
    public void testSubtract() {

        assertEquals(0, Caculator.subtract(1, 1));
    }

}
```

### 运行

通过运行测试类，得到如下的结果，可以看到第一个测试被忽略掉了。

![假设测试][1]

[1]: images/06junit-assume.png
