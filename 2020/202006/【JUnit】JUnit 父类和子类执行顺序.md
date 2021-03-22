# 【JUnit】JUnit 父类和子类执行顺序

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 顺序注解

可以参与到执行过程中的注解一共有四个，分别是：

1. @BeforeClass：在当前测试类的所有测试执行之前执行，比 @Before 更早执行。
2. @AfterClass：在当前测试类的所有测试执行之后执行，比 @After 更晚执行。
3. @Before：在每个测试方法执行前执行。
4. @After：在每个测试方法执行后执行。

具体的执行顺序可以参见：[【JUnit】JUnit 执行顺序][1]

## 继承

除了自身测试类可以参与到 JUnit 执行的过程中，还可以通过继承父类来参与。
父类的 Before 注解比子类先执行，父类的 After 注解比子类后执行。

## JUnit 父类和子类执行顺序示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>03-java-junit-test-fixture</artifactId>
    <version>1.0.0</version>
    <description>JUnit 父类和子类执行顺序示例</description>

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

### AbstractTestSupport

```java
package jiangbo.java.junit;

import org.junit.After;
import org.junit.AfterClass;
import org.junit.Before;
import org.junit.BeforeClass;

public abstract class AbstractTestSupport {

    @BeforeClass
    public static void setUpClassParent() {

        System.out.println("parent @BeforeClass");
    }

    @AfterClass
    public static void tearDownClassParent() {

        System.out.println("parent @AfterClass");
    }

    @Before
    public void setUpParent() {

        System.out.println("parent @Before");
    }

    @After
    public void tearDownParent() {

        System.out.println("parent @After");
    }
}
```

### CaculatorTest

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import java.io.IOException;

import org.junit.After;
import org.junit.AfterClass;
import org.junit.Before;
import org.junit.BeforeClass;
import org.junit.Test;

public class CaculatorTest extends AbstractTestSupport {

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

    @BeforeClass
    public static void setUpClass() {

        System.out.println("BeforeClass");
    }

    @AfterClass
    public static void tearDownClass() throws IOException {

        System.out.println("@AfterClass");
    }

    @Before
    public void setUp() {

        System.out.println("@Before");
    }

    @After
    public void tearDown() throws IOException {

        System.out.println("@After");
    }
}
```

### 运行

通过运行单元测试，控制台输出：

```text
parent @BeforeClass
@BeforeClass
parent @Before
@Before
testAdd
@After
parent @After
parent @Before
@Before
testSubtract
@After
parent @After
@AfterClass
parent @AfterClass
```

[1]: https://www.cnblogs.com/jiangbo44/p/13070314.html