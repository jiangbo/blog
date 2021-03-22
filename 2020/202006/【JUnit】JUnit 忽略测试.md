# 【JUnit】JUnit 忽略测试

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## @Ignore 注解

使用 @Ignore 注解可以忽略一个或者一组单元测试，并且可以提供忽略的说明。

## Ignore 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>07-java-junit-ignore</artifactId>
    <version>1.0.0</version>
    <description>JUnit Ignore 示例</description>

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

import org.junit.Ignore;
import org.junit.Test;

//@Ignore("这里可以忽略当前类中的所有单元测试")
public class CaculatorTest {

    @Test
    public void testAdd() {

        System.out.println("testAdd");
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @Test
//    @Ignore
    @Ignore("忽略除法测试，因为会抛异常")
    public void testDivide() {

        System.out.println("testDivide");
        Caculator.divide(1, 0);
    }
}
```

### 运行

通过运行 CaculatorTest 测试类，控制台输出：

```text
testAdd
```
并且在 IDE 上可以看到被忽略的测试，和其它的颜色不一样。
![忽略测试][1]

[1]: images/02junit-ignore.png