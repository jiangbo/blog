# 【JUnit】JUnit Rules（二）

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
    <artifactId>09-java-junit-rule</artifactId>
    <version>1.0.0</version>
    <description>JUnit Rules 示例（二）</description>

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

### RuleChain

RuleChain 可以将多个规则组合起来，在 4.13 之前，是为了 rule 的顺序，在 4.13 之后，建议使用 oder 属性进行排序。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertTrue;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExternalResource;
import org.junit.rules.RuleChain;
import org.junit.rules.TestRule;

public class RuleChainTest {
    @Rule
    public final TestRule chain = RuleChain
            .outerRule(new LoggingRule("outer rule"))
            .around(new LoggingRule("middle rule"))
            .around(new LoggingRule("inner rule"));

    @Test
    public void testRuleChain() {
        assertTrue(true);
    }

    static class LoggingRule extends ExternalResource {

        private final String str;

        public LoggingRule(String str) {

            this.str = str;
        }

        @Override
        protected void before() throws Throwable {

            System.out.println("starting " + str);
        }

        @Override
        protected void after() {

            System.out.println("finished " + str);
        }
    }
}
```

### Stopwatch

Stopwatch 用来监控每个测试执行的时间。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;
import static org.junit.Assume.assumeTrue;

import java.util.concurrent.TimeUnit;

import org.junit.AssumptionViolatedException;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.Stopwatch;
import org.junit.runner.Description;

public class StopwatchTest {

    private static void logInfo(Description description, String status, long nanos) {
        String testName = description.getMethodName();
        System.out.println(String.format("Test %s %s, spent %d microseconds",
                testName, status, TimeUnit.NANOSECONDS.toMicros(nanos)));
    }

    @Rule
    public Stopwatch stopwatch = new Stopwatch() {
        @Override
        protected void succeeded(long nanos, Description description) {
            logInfo(description, "succeeded", nanos);
        }

        @Override
        protected void skipped(long nanos, AssumptionViolatedException e, Description description) {
            logInfo(description, "skipped", nanos);
        }

        @Override
        protected void finished(long nanos, Description description) {
            logInfo(description, "finished", nanos);
        }
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

    @Test
    public void skips() {

        // assume 之后学习
        assumeTrue(false);
    }

}
```

### Timeout

Timeout 用来断言超时。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.TestRule;
import org.junit.rules.Timeout;

public class TimeoutTest {

    @Rule
    public TestRule rule = Timeout.millis(20);

    @Test
    public void testAdd() throws InterruptedException {

        // TimeUnit.SECONDS.sleep(1);

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

### Verifier

Verifier 可以补充自己验证逻辑。

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.TestRule;
import org.junit.rules.Verifier;

public class VerifierTest {

    @Rule
    public TestRule rule = new Verifier() {
        @Override
        protected void verify() throws Throwable {

            assertTrue(true);
        };
    };

    @Test
    public void testAdd() throws InterruptedException {

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

### ErrorCollector

ErrorCollector 可以在出错的时候，收集所有的出错信息，并打印异常堆栈。

```java
package jiangbo.java.junit;

import org.junit.Ignore;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ErrorCollector;

public class ErrorCollectorTest {

    @Rule
    public ErrorCollector collector = new ErrorCollector();

    @Test
    @Ignore("注释出错的测试；测试出错时，会打印所有的出错信息，一般的测试只能看到一个异常")
    public void testThrowError() {

        collector.addError(new Throwable("first thing went wrong"));
        collector.addError(new Throwable("second thing went wrong"));

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
        ErrorCollectorTest.class,
        StopwatchTest.class,
        TimeoutTest.class,
        VerifierTest.class,
        RuleChainTest.class
})
public class RuleSuiteTest {

}
```

### 运行

在 IDE 上使用右键运行 Suite，获得如下的结果：
![Rule 测试二][1]

[1]: images/04junit-rules.png