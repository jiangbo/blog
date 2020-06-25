# 【JUnit】测试套件（Test Suite）

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 测试套件

测试套件可以将多个测试类组合到一起运行。

## Test Suite 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>06-java-junit-test-suite</artifactId>
    <version>1.0.0</version>
    <description>JUnit Test Suite 示例</description>

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

### CaculatorTest1

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class CaculatorTest1 {

    @Test
    public void testAdd() {

        System.out.println("testAdd");
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }
}
```

### CaculatorTest2

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class CaculatorTest2 {

    @Test
    public void testSubtract() {

        System.out.println("testSubtract");
        int number = Caculator.subtract(1, 1);
        assertEquals(0, number);
    }
}
```

### CaculatorTest

```java
package jiangbo.java.junit;

import org.junit.runner.RunWith;
import org.junit.runners.Suite;
import org.junit.runners.Suite.SuiteClasses;

@RunWith(Suite.class)
@SuiteClasses({ CaculatorTest1.class, CaculatorTest2.class })
public class CaculatorTest {

}
```

### 运行

通过运行 CaculatorTest 测试类，控制台输出：

```text
testAdd
testSubtract
```