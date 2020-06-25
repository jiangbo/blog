# 【JUnit】JUnit Rules（一）

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## Rules

官网描述如下：

    Rules allow very flexible addition or redefinition of the behavior of each test method in a test class. 
    Testers can reuse or extend one of the provided Rules below, or write their own.

规则允许非常灵活地添加或重新定义测试类中每个测试方法的行为。测试者可以重用或扩展已提供的规则，或者编写自己的规则。

## 类型

在当前版本，共有 11 中 TestRule，分别是：

1. DisableOnDebug
2. ExpectedException
3. ExternalResource
4. TemporaryFolder
5. RuleChain
6. Stopwatch
7. TestWatcher
8. TestName
9. Timeout
10. Verifier
11. ErrorCollector

其中 TestName 是 TestWatcher 的子类，ErrorCollector 是 Verifier 的子类。
而注入规则的方式有两种，一种是通过 @Rule 注解标注成员属性或者方法；
另一种是使用 @ClassRule 标注类变量或者方法。
这两种注解都提供了一个 order 属性，可以用来表示每个规则应用的顺序，越大表示越在里面。

## Rules 示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>08-java-junit-rule</artifactId>
    <version>1.0.0</version>
    <description>JUnit Rules 示例（一）</description>

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

### DisableOnDebug

DisableOnDebug 可以在 debug 时禁用某些 Rule。下面是一个超时的规则，不过如果是 debug 模式，则不会生效。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import java.util.concurrent.TimeUnit;

import org.junit.ClassRule;
import org.junit.Test;
import org.junit.rules.DisableOnDebug;
import org.junit.rules.TestRule;
import org.junit.rules.Timeout;

public class DisableOnDebugTest {

    @ClassRule
    public static TestRule RULE = new DisableOnDebug(Timeout.millis(100));

    @Test
    public void testAdd() throws InterruptedException {

        // TimeUnit.SECONDS.sleep(1);
        System.out.println("testAdd");
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }
}
```

### ExpectedException

ExpectedException 在当前版本（4.13）已经不建议使用，而是推荐 assertThrows，这个之后学。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;

public class ExpectedExceptionTest {

    // 使用 ClassRule 不生效，不清楚是否是缺陷，不过这个已经过时了，
    // 在 4.13 版本建议使用 assertThrows，所以可能 ExpectedException 不再支持了。
    //
    // @SuppressWarnings("deprecation")
    // @ClassRule
    // public static final ExpectedException THROWN = ExpectedException.none();

    @SuppressWarnings("deprecation")
    @Rule
    public ExpectedException thrown = ExpectedException.none();

    @Test
    public void testAdd() {

        System.out.println("testAdd");
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @Test
    public void testDivide() {

        // THROWN.expect(ArithmeticException.class);
        // THROWN.expectMessage("/ by zero");

        thrown.expect(ArithmeticException.class);
        thrown.expectMessage("/ by zero");

        System.out.println("testDivide");
        Caculator.divide(1, 0);
    }
}
```

### TemporaryFolder

TemporaryFolder 可以创建临时文件，在测试执行完成后，会自动删除。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertTrue;

import java.io.File;
import java.io.IOException;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.TemporaryFolder;

public class TemporaryFolderTest {

    @Rule
    public final TemporaryFolder folder = new TemporaryFolder();

    @Test
    public void testUsingTempFolder() throws IOException {

        File file = folder.newFile("myfile.txt");
        assertTrue(file.exists());
    }
}
```

### TestWatcher

TestWatcher 可以监控测试执行的状态，比如测试成功，测试失败，测试跳过等。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.TestWatcher;
import org.junit.runner.Description;

public class TestWatcherTest {

    @Rule
    public TestWatcher watchman = new TestWatcher() {

        @Override
        protected void succeeded(Description description) {

            System.out.println("success");
        };
    };

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

### TestName

TestName 可以很方便获取当前测试执行的方法的名称。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.TestName;

public class TestNameTest {

    @Rule
    public final TestName name = new TestName();

    @Test
    public void testAdd() {

        System.out.println(name.getMethodName());
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @Test
    public void testSubtract() {

        System.out.println(name.getMethodName());
        int number = Caculator.subtract(1, 1);
        assertEquals(0, number);
    }
}
```

### RuleSuiteTest

由于有好几个测试类，使用 Test Suite 将它们组合在一起运行。

```java
package jiangbo.java.junit;

import org.junit.runner.RunWith;
import org.junit.runners.Suite;
import org.junit.runners.Suite.SuiteClasses;

@RunWith(Suite.class)
@SuiteClasses({
        DisableOnDebugTest.class,
        ExpectedExceptionTest.class,
        ExternalResourceTest.class,
        TemporaryFolderTest.class,
        TestNameTest.class,
        TestWatcherTest.class
})
public class RuleSuiteTest {

}
```

### 运行

在 IDE 上使用右键运行 Suite，获得如下的结果：
![Rule 测试一][1]

[1]: images/03junit-rules01.png