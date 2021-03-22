# 【JUnit】分类测试

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## Category

分类用于在测试中添加元数据，鼓励像下面这样使用分类测试：

* 自动化测试的类型： UnitTests, IntegrationTests, SmokeTests, RegressionTests, PerformanceTests ...
* 测试执行的速度：SlowTests，QuickTests
* 应在 ci 构建的一部分中执行测试：NightlyBuildTests
* 测试状态：UnstableTests，InProgressTests

这还用于添加特定于项目的元数据，例如测试涵盖了项目的哪个功能。

## 分类示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>19-java-junit-category</artifactId>
    <version>1.0.0</version>
    <description>JUnit 分类测试示例</description>

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

### FastTests

```java
package jiangbo.java.junit;

public interface FastTests {
    /* category marker */
}
```

### SlowTests

```java
package jiangbo.java.junit;

public interface SlowTests {
    /* category marker */
}
```

### CaculatorTest1

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import java.util.concurrent.TimeUnit;

import org.junit.Test;
import org.junit.experimental.categories.Category;

public class CaculatorTest1 {

    @Category(FastTests.class)
    @Test
    public void testAdd() {

        System.out.println("testAdd");
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @Category(SlowTests.class)
    @Test
    public void testSubtract() throws InterruptedException {

        TimeUnit.SECONDS.sleep(3);
        System.out.println("testSubtract");
        int number = Caculator.subtract(1, 1);
        assertEquals(0, number);
    }

    @Test
    public void testDivide() throws InterruptedException {

        TimeUnit.SECONDS.sleep(1);
        System.out.println("testDivide");
        int number = Caculator.divide(1, 1);
        assertEquals(1, number);
    }
}
```

### CaculatorTest2

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import java.util.concurrent.TimeUnit;

import org.junit.Test;
import org.junit.experimental.categories.Category;

@Category({ SlowTests.class, FastTests.class })
public class CaculatorTest2 {

    @Category(SlowTests.class)
    @Test
    public void testAdd() throws InterruptedException {

        TimeUnit.SECONDS.sleep(3);
        System.out.println("testAdd");
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @Test
    public void testSubtract() throws InterruptedException {

        TimeUnit.SECONDS.sleep(1);
        System.out.println("testSubtract");
        int number = Caculator.subtract(1, 1);
        assertEquals(0, number);
    }

    @Test
    public void testDivide() {

        System.out.println("testDivide");
        int number = Caculator.divide(1, 1);
        assertEquals(1, number);
    }
}
```

### SlowTestSuite1

```java
package jiangbo.java.junit;

import org.junit.experimental.categories.Categories;
import org.junit.experimental.categories.Categories.IncludeCategory;
import org.junit.runner.RunWith;
import org.junit.runners.Suite.SuiteClasses;

@RunWith(Categories.class)
@IncludeCategory(SlowTests.class)
@SuiteClasses({ CaculatorTest1.class, CaculatorTest2.class }) // Note that Categories is a kind of Suite
public class SlowTestSuite1 {

    // 将会运行 CaculatorTest1.testSubtract CaculatorTest2里面的所有
}
```

### SlowTestSuite2

```java
package jiangbo.java.junit;

import org.junit.experimental.categories.Categories;
import org.junit.experimental.categories.Categories.ExcludeCategory;
import org.junit.experimental.categories.Categories.IncludeCategory;
import org.junit.runner.RunWith;
import org.junit.runners.Suite.SuiteClasses;

@RunWith(Categories.class)
@IncludeCategory(SlowTests.class)
@ExcludeCategory(FastTests.class)
@SuiteClasses({ CaculatorTest1.class, CaculatorTest2.class }) // Note that Categories is a kind of Suite
public class SlowTestSuite2 {

    // 将会运行 CaculatorTest1.testSubtract
}
```
