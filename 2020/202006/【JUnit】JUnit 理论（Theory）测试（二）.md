# 【JUnit】JUnit 理论（Theory）测试（二）

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## Theory

这些都是实验性质的，可以不了解。
@ParametersSuppliedBy 可以编写自己提供数据的方式，不过需要实现 ParameterSupplier 接口。

## 理论示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>18-java-junit-theory</artifactId>
    <version>1.0.0</version>
    <description>JUnit 理论测试示例（二）</description>

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
import static org.junit.Assume.assumeFalse;

import org.junit.experimental.theories.Theories;
import org.junit.experimental.theories.Theory;
import org.junit.runner.RunWith;

@RunWith(Theories.class)
public class CaculatorTest {

    @Theory
    public void testDivide(@Between(first = -1, last = 1) int i) {

        assumeFalse(i == 0);
        assertEquals(1, Caculator.divide(i, i));
    }

}
```

### Between

```java
package jiangbo.java.junit;

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

import org.junit.experimental.theories.ParametersSuppliedBy;

@Retention(RetentionPolicy.RUNTIME)
@ParametersSuppliedBy(BetweenSupplier.class)
public @interface Between {

    int first();

    int last();
}
```

### BetweenSupplier

```java
package jiangbo.java.junit;

import java.util.ArrayList;
import java.util.List;

import org.junit.experimental.theories.ParameterSignature;
import org.junit.experimental.theories.ParameterSupplier;
import org.junit.experimental.theories.PotentialAssignment;

public class BetweenSupplier extends ParameterSupplier {

    @Override
    public List<PotentialAssignment> getValueSources(ParameterSignature sig) {

        List<PotentialAssignment> list = new ArrayList<PotentialAssignment>();

        Between annotation = sig.findDeepAnnotation(Between.class);

        for (int i = annotation.first(); i <= annotation.last(); i++) {

            list.add(PotentialAssignment.forValue("ints", i));
        }

        return list;

    }
}
```
