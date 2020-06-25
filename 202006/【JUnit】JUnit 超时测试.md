# 【JUnit】JUnit 超时测试

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 说明

测试超时有两种方式，一种是使用 @Test 注解的 timeout 属性，另一种是使用 @Rule。
两种方式有一点区别，其中  @Test 方式运行的，测试方法和 @Before @After 不在同一线程内；
@Rule 方式的，都是在同一线程之内运行。如果对 Rules 不熟悉，可以参考前面的 JUnit Rules 文章。

## 超时测试示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>10-java-junit-timeout</artifactId>
    <version>1.0.0</version>
    <description>JUnit 超时测试示例</description>

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

### CaculatorTest1

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import java.io.IOException;

import org.junit.After;
import org.junit.AfterClass;
import org.junit.Before;
import org.junit.BeforeClass;
import org.junit.Test;

public class CaculatorTest1 {

    @Test(timeout = 1000)
    public void testAdd() {

        System.out.println("testAdd: " + Thread.currentThread());
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @BeforeClass
    public static void setUpClass() {

        System.out.println("BeforeClass: " + Thread.currentThread());
    }

    @AfterClass
    public static void tearDownClass() throws IOException {

        System.out.println("@AfterClass: " + Thread.currentThread());
    }

    @Before
    public void setUp() {

        System.out.println("@Before: " + Thread.currentThread());
    }

    @After
    public void tearDown() {

        System.out.println("@After: " + Thread.currentThread());
    }
}
```

### CaculatorTest2

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import java.io.IOException;

import org.junit.After;
import org.junit.AfterClass;
import org.junit.Before;
import org.junit.BeforeClass;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.TestRule;
import org.junit.rules.Timeout;

public class CaculatorTest2 {

    @Rule
    public TestRule rule = Timeout.seconds(1);

    @Test
    public void testAdd() {

        System.out.println("testAdd: " + Thread.currentThread());
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @BeforeClass
    public static void setUpClass() {

        System.out.println("BeforeClass: " + Thread.currentThread());
    }

    @AfterClass
    public static void tearDownClass() throws IOException {

        System.out.println("@AfterClass: " + Thread.currentThread());
    }

    @Before
    public void setUp() {

        System.out.println("@Before: " + Thread.currentThread());
    }

    @After
    public void tearDown() {

        System.out.println("@After: " + Thread.currentThread());
    }
}
```

### 运行

通过运行 CaculatorTest1 测试类，控制台输出：

```text
BeforeClass: Thread[main,5,main]
@Before: Thread[main,5,main]
testAdd: Thread[Time-limited test,5,FailOnTimeoutGroup]
@After: Thread[main,5,main]
@AfterClass: Thread[main,5,main]
```

通过运行 CaculatorTest2 测试类，控制台输出：

```text
BeforeClass: Thread[main,5,main]
@Before: Thread[Time-limited test,5,FailOnTimeoutGroup]
testAdd: Thread[Time-limited test,5,FailOnTimeoutGroup]
@After: Thread[Time-limited test,5,FailOnTimeoutGroup]
@AfterClass: Thread[main,5,main]
```
