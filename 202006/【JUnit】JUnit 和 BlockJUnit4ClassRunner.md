# 【JUnit】JUnit 和 BlockJUnit4ClassRunner

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## IDE 支持

在 IDE 中，比如 NetBeans, Eclipse and IntelliJ IDEA 都内置了图形化的 JUnit Runner 的支持，可以直接运行。
在当前版本，如果不使用 @RunWith 注解指定 Runner，默认使用的是 BlockJUnit4ClassRunner。

## @RunWith 注解

* 当一个类或者其父类标注了 @RunWith 注解后，JUnit 就会使用指定的 Runner 替换内置的默认的 Runner。
* 当前版本，默认的 Runner 是 BlockJUnit4ClassRunner，它替换了过时的 JUnit4ClassRunner。
* @RunWith(JUnit4.class) 通常使用默认的 Runner 运行，当前版本是 BlockJUnit4ClassRunner，不过将来可能会改变。
* 有一类特殊的 Runner 叫 Suite，会在后面介绍。

## JUnit 和 BlockJUnit4ClassRunner 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>04-java-junit-test-runner</artifactId>
    <version>1.0.0</version>
    <description>Block JUnit4 Class Runner 和 JUnit4 示例</description>

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

}
```

### CaculatorTest

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.JUnit4;

//@RunWith(BlockJUnit4ClassRunner.class)
@RunWith(JUnit4.class)
public class CaculatorTest {

    @Test
    public void testAdd() {

        System.out.println("testAdd");
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @Test
    public void testSubtract() {

        System.out.println("testSubtract");
        int number = Caculator.subtract(1, 1);
        assertEquals(0, number);
    }
}
```

### 运行

通过运行单元测试，控制台输出：

```text
testAdd
testSubtract
```