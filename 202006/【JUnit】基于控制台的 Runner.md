# 【JUnit】基于控制台的 Runner

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## main 方法运行测试

除了和 IDE 继承，右键运行之外，还可以通过 main 运行测试，并且得到运行的结果数据。
官网上将这种方式叫做基于控制台的 Runner(Console based Test runner)。


## Console based Test runner 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>05-java-junit-test-runner</artifactId>
    <version>1.0.0</version>
    <description>基于控制台的 JUnit 示例</description>

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

### CaculatorMain

```java
package jiangbo.java.junit;

import org.junit.runner.JUnitCore;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;

public class CaculatorMain {

    public static void main(String[] args) {

        Result result = JUnitCore.runClasses(CaculatorTest.class);

        for (Failure failure : result.getFailures()) {

            System.out.println(failure.getMessage());
        }

        System.out.println(result.getRunCount());
        System.out.println(result.getRunTime());
        System.out.println(result.wasSuccessful());
    }
}
```

### 运行

通过运行 main 方法，控制台输出：

```text
testAdd
testSubtract
2
35
true
```