# 【JUnit】JUnit 方法执行顺序

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 方法执行顺序

查看官方[原文][1]

根据设计，JUnit不指定测试方法调用的执行顺序。 到目前为止，仅按反射 API 返回的顺序依次用这些方法。 
但是，使用JVM顺序是不明智的，因为 Java 平台未指定任何特定的顺序，并且实际上 JDK 7 返回了或多或少的随机顺序。 
当然，编写良好的测试代码不会假设任何顺序，但偶尔有，并且在某些平台上，可预测的错误要好于随机错误。

从版本4.11开始，JUnit默认将使用确定但不可预测的顺序（MethodSorters.DEFAULT）。 
要更改测试执行顺序，只需使用 @FixMethodOrder 注解测试类并指定一个可用的 MethodSorters：

* @FixMethodOrder（MethodSorters.JVM）：按照JVM返回的顺序保留测试方法。 此顺序可能会在运行时改变。

* @FixMethodOrder（MethodSorters.NAME_ASCENDING）：根据方法名称，按字典顺序对测试方法进行排序。

## 方法执行顺序示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>14-java-junit-execution-order</artifactId>
    <version>1.0.0</version>
    <description>JUnit 方法执行顺序示例</description>

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

### TestMethodOrder

示例来源于官网

```java
package jiangbo.java.junit;

import org.junit.FixMethodOrder;
import org.junit.Test;
import org.junit.runners.MethodSorters;

@FixMethodOrder(MethodSorters.NAME_ASCENDING)
public class TestMethodOrder {

    @Test
    public void testB() {
        System.out.println("second");
    }

    @Test
    public void testA() {
        System.out.println("first");
    }

    @Test
    public void testC() {
        System.out.println("third");
    }
}
```

### 输出

```text
first
second
third
```

[1]: https://github.com/junit-team/junit4/wiki/Test-execution-order
