# Spring：SpringRunner 和 SpringJUnit4ClassRunner

## 环境

1. jdk 7
2. 4.3.24.RELEASE

## 背景

在使用 spring-test 的过程中，有两个 runner 可以选择，分别是 SpringRunner 和 SpringJUnit4ClassRunner。
如果是在 4.3 之前，只能选择 SpringJUnit4ClassRunner，如果是 4.3 之后，建议选择 SpringRunner。
SpringRunner 对 junit 的版本有要求，需要 4.12 及以上。

## 使用示例

### 加入依赖

```xml
<dependency>
    <groupId>org.springframework</groupId>
    <artifactId>spring-test</artifactId>
    <version>4.3.24.RELEASE</version>
</dependency>

<dependency>
    <groupId>junit</groupId>
    <artifactId>junit</artifactId>
    <version>4.12</version>
</dependency>
```

### SpringJUnit4ClassRunner

```java
package jiangbo.springweb;

import static org.junit.Assert.assertTrue;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.context.annotation.Configuration;
import org.springframework.test.context.ContextConfiguration;
import org.springframework.test.context.junit4.SpringJUnit4ClassRunner;

@RunWith(SpringJUnit4ClassRunner.class)
@ContextConfiguration
public class SpringJUnit4ClassRunnerTest {

    @Test
    public void testDemo() throws Exception {

        assertTrue(true);
    }

    @Configuration
    static class config {
    }
}
```

### SpringRunner

```java
package jiangbo.springweb;

import static org.junit.Assert.assertTrue;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.context.annotation.Configuration;
import org.springframework.test.context.ContextConfiguration;
import org.springframework.test.context.junit4.SpringRunner;

@RunWith(SpringRunner.class)
@ContextConfiguration
public class SpringRunnerTest {

    @Test
    public void testDemo() throws Exception {

        assertTrue(true);
    }

    @Configuration
    static class config {
    }
}
```
