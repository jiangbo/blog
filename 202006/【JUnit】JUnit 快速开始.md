# 【JUnit】JUnit 快速开始

## 环境

- JDK 6
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 什么是 JUnit

以下是来自官网的一段话：

    What is JUnit?
    JUnit is a simple, open source framework to write and run repeatable tests. It is an instance of the xUnit architecture for unit testing frameworks. JUnit features include:

      Assertions for testing expected results
      Test fixtures for sharing common test data
      Test runners for running tests
    JUnit was originally written by Erich Gamma and Kent Beck.

JUnit 是一个编写和运行可重复测试的简单并且开源的框架。 它是单元测试框架 xUnit 体系结构的一员。
JUnit 包括以下功能：

- 测试期望结果的断言
- 共享通用测试数据的测试工具
- 运行测试的 Test Runner

JUnit最初由 Erich Gamma 和 Kent Beck 编写。

>JUnit 是一个用于 java 的单元测试的框架

## 单元测试的必要性

以下是 JUnit in action 第二版中关于单元测试必要性的说明，也是使用 JUnit 后的优点：

- 比功能测试更高的测试覆盖
- 提高团队生产力
- 发现错误和减少调试
- 自信地重构
- 改进实现
- 文档化期望的行为
- 更高的测试覆盖率

总的说来，就是进行单元测试之后，能够带来很多的好处。

## JUnit 快速开始示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>01-java-junit-start</artifactId>
    <version>1.0.0</version>
    <description>JUnit 快速开始示例</description>

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

}
```

### CaculatorTest

```java
package jiangbo.java.junit;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class CaculatorTest {

    @Test
    public void testAdd() {

        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

}
```

### 运行

运行编写的第一个单元测试，应该可以看到一个绿条，表示测试通过。如果是红色的，表示测试失败。


![JUnit 快速开始][1]

## JUnit 推荐做法

1. 将原始类和测试放在同一个包名下，不同目录。
2. 测试类以 Test 结尾。
3. 测试方法以 test 开头。

关于第一点，保持包名一致，存放到不同的目录。例如：
原始类放到 src/main/java 目录，代表源码目录，测试类放到 src/test/java 目录，代表测试源码。

[1]:images/01junit-quick-start.png
