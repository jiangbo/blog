# Spring cloud：订单微服务-接口层

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

搭建订单微服务的环境。

## 搭建步骤

### 新增 pom.xml 依赖

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-web</artifactId>
</dependency>
```

### 定义服务层

```java
package jiangbo.springcloud.service;

import java.util.List;

import jiangbo.springcloud.entity.OrderInfo;

public interface OrderInfoService {

    List<OrderInfo> queryAllOrders();
}
```

```java
package jiangbo.springcloud.service.impl;

import java.util.List;

import org.springframework.stereotype.Service;

import jiangbo.springcloud.dao.OrderInfoDao;
import jiangbo.springcloud.entity.OrderInfo;
import jiangbo.springcloud.service.OrderInfoService;

@Service
public class OrderInfoServiceImpl implements OrderInfoService {

    private final OrderInfoDao orderInfoDao;

    public OrderInfoServiceImpl(OrderInfoDao orderInfoDao) {

        this.orderInfoDao = orderInfoDao;
    }

    @Override
    public List<OrderInfo> queryAllOrders() {

        return orderInfoDao.queryAllOrders();
    }
}
```

### 定义接口层

```java
package jiangbo.springcloud.controller;

import java.util.List;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import jiangbo.springcloud.entity.OrderInfo;
import jiangbo.springcloud.service.OrderInfoService;

@RestController
@RequestMapping("/order")
public class OrderInfoController {

    private final OrderInfoService orderInfoService;

    public OrderInfoController(OrderInfoService orderInfoService) {

        this.orderInfoService = orderInfoService;
    }

    @GetMapping
    public List<OrderInfo> allOrders() {

        return orderInfoService.queryAllOrders();
    }
}
```

## 验证

### 建立测试

```java
package jiangbo.springcloud.controller;

import static org.junit.Assert.assertFalse;

import java.util.List;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.boot.test.context.SpringBootTest.WebEnvironment;
import org.springframework.boot.test.web.client.TestRestTemplate;
import org.springframework.test.context.junit4.SpringRunner;

import jiangbo.springcloud.entity.OrderInfo;

@RunWith(SpringRunner.class)
@SpringBootTest(webEnvironment = WebEnvironment.RANDOM_PORT)
public class OrderInfoControllerTest {

    @Autowired
    private TestRestTemplate restTemplate;

    @SuppressWarnings("unchecked")
    @Test
    public void testAllOrders() {

        List<OrderInfo> orders = restTemplate.getForObject("/order", List.class);
        assertFalse(orders.isEmpty());
    }
}
```

### 运行

运行单元测试，通过测试，则订单微服务的接口层环境搭建成功。


## 附录

### pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.cloud</groupId>
        <artifactId>spring-cloud-starter-parent</artifactId>
        <version>Edgware.SR6</version>
    </parent>

    <groupId>jiangbo.springcloud</groupId>
    <artifactId>04spring-cloud-order</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>

    <properties>
        <java.version>1.7</java.version>
    </properties>

    <dependencies>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-jdbc</artifactId>
        </dependency>

        <dependency>
            <groupId>mysql</groupId>
            <artifactId>mysql-connector-java</artifactId>
            <scope>runtime</scope>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-test</artifactId>
            <scope>test</scope>
        </dependency>

        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-devtools</artifactId>
            <scope>provided</scope>
        </dependency>

    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
            </plugin>
        </plugins>
    </build>
</project>
```