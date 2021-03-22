# 【JUnit】JUnit 理论（Theory）测试（一）

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## Theory

这些都是实验性质的，可以不了解。
Theory 相比 Test 更加的灵活。
其中 @DataPoint 的名称和 @FromDataPoints 的名称对应，只会注入对应的值。
如果没有 @FromDataPoints，则会将 @DataPoint 的值进行排列组合，然后注入。
@DataPoints 和 @TestOn 可以注入多个值。

## 理论示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>17-java-junit-theory</artifactId>
    <version>1.0.0</version>
    <description>JUnit 理论测试示例（一）</description>

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

import static org.hamcrest.CoreMatchers.anyOf;
import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.MatcherAssert.assertThat;
import static org.junit.Assert.assertEquals;
import static org.junit.Assume.assumeFalse;

import org.junit.experimental.theories.DataPoint;
import org.junit.experimental.theories.FromDataPoints;
import org.junit.experimental.theories.Theories;
import org.junit.experimental.theories.Theory;
import org.junit.runner.RunWith;

@RunWith(Theories.class)
public class CaculatorTest1 {

    @DataPoint("n1")
    public static int i1 = 1;

    @DataPoint("n2")
    public static int i0 = 0;

    @Theory
    public void testAdd(@FromDataPoints("n1") int n1, @FromDataPoints("n2") int n2) {

        System.out.println(n1);
        System.out.println(n2);
        assertEquals(1, Caculator.add(n1, n2));
    }

    @Theory
    public void testSubtract(int n1, int n2) {

        // 除数不为0
        assumeFalse(n2 == 0);
        assertThat(Caculator.divide(n1, n2), anyOf(is(1), is(0)));
    }
}
```

### CaculatorTest2

```java
package jiangbo.java.junit;

import static org.hamcrest.CoreMatchers.anyOf;
import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.MatcherAssert.assertThat;
import static org.junit.Assume.assumeFalse;

import org.junit.experimental.theories.DataPoints;
import org.junit.experimental.theories.Theories;
import org.junit.experimental.theories.Theory;
import org.junit.experimental.theories.suppliers.TestedOn;
import org.junit.runner.RunWith;

@RunWith(Theories.class)
public class CaculatorTest2 {

    @DataPoints
    public static int[] arr = new int[] { 1, 2, 3 };

    @Theory
    public void testDataPoints(int i) {

        assumeFalse(i == 1);
        assertThat(i, anyOf(is(2), is(3)));
    }

    @Theory
    public void testTestedOn(@TestedOn(ints = { 4, 5, 6 }) int i) {

        assumeFalse(i == 4);
        assertThat(i, anyOf(is(5), is(6)));
    }
}
```
