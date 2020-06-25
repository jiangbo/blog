# 【JUnit】JUnit 与 maven 集成

## 环境

- JDK 11
- JUnit 4.13
- Spring Tool Suite 4.6.2
- Maven 3.6.3

## 与 maven 集成

因为已经是 maven 项目了，所以不再需要任何东西，默认就与 maven 集成了。
不过由于 maven-surefire-plugin 自身的[缺陷][1]，导致测试时，如果有中文，则会出现乱码。
下面有两种解决方案，推荐第二种。

### 方案一

在 pom.xml 中新增以下内容

```xml
<build>
    <plugins>
        <plugin>
            <groupId>org.apache.maven.plugins</groupId>
            <artifactId>maven-surefire-plugin</artifactId>
            <configuration>
                <argLine>-Dfile.encoding=${project.build.sourceEncoding}</argLine>
            </configuration>
        </plugin>
    </plugins>
</build>
```

### 方案二（推荐）

因为在 2.15 版本修复的，所以只要升级到 2.15 以上，都可以解决该问题。

```xml
<build>
    <plugins>
        <plugin>
            <groupId>org.apache.maven.plugins</groupId>
            <artifactId>maven-surefire-plugin</artifactId>
            <version>2.22.2</version>
        </plugin>
    </plugins>
</build>
```

## 与 maven 集成示例

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>jiangbo.java.junit</groupId>
    <artifactId>20-java-junit-maven</artifactId>
    <version>1.0.0</version>
    <description>JUnit 集成 maven 示例</description>

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

    <build>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-surefire-plugin</artifactId>
                <version>2.22.2</version>
            </plugin>
        </plugins>
    </build>

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

import org.junit.Test;

public class CaculatorTest {

    @Test
    public void testAdd() {

        System.out.println("testAdd");
        int number = Caculator.add(1, 1);
        assertEquals(2, number);
    }

    @Test
    public void testSubtract() throws InterruptedException {

        System.out.println("中文测试");
        System.out.println("testSubtract");
        int number = Caculator.subtract(1, 1);
        assertEquals(0, number);
    }

    @Test
    public void testDivide() throws InterruptedException {

        System.out.println("testDivide");
        int number = Caculator.divide(1, 1);
        assertEquals(1, number);
    }
}
```

### 运行

```text
[INFO] -------------------------------------------------------
[INFO]  T E S T S
[INFO] -------------------------------------------------------
[INFO] Running jiangbo.java.junit.CaculatorTest
testAdd
中文测试
testSubtract
testDivide
[INFO] Tests run: 3, Failures: 0, Errors: 0, Skipped: 0, Time elapsed: 0.11 s - in jiangbo.java.junit.CaculatorTest
[INFO] 
[INFO] Results:
[INFO] 
[INFO] Tests run: 3, Failures: 0, Errors: 0, Skipped: 0
[INFO] 
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time:  5.937 s
[INFO] Finished at: 2020-06-10T22:46:57+08:00
[INFO] ------------------------------------------------------------------------
```

[1]: https://issues.apache.org/jira/browse/SUREFIRE-951
