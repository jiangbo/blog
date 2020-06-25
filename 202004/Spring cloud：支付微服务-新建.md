# Spring cloud：支付微服务-新建

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

搭建支付微服务的环境。

## 搭建步骤

### 新增数据库和数据表

新增一个 spring_cloud_payment 的数据库，再新建 payment_info 数据库表。建表语句如下：

```sql
-- spring_cloud_payment.payment_info definition

CREATE TABLE `payment_info` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `order_id` bigint(20) NOT NULL,
  `amount` varchar(100) NOT NULL,
  `status` varchar(100) NOT NULL,
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

### 实体

```java
package jiangbo.springcloud.entity;

import java.util.Date;

public class PaymentInfo {

    private long id;

    private long orderId;

    private String amount;

    private String status;

    private Date createTime;
}
```

### 数据层

```java
package jiangbo.springcloud.dao;

import java.util.List;

import jiangbo.springcloud.entity.PaymentInfo;

public interface PaymentDao {

    List<PaymentInfo> queryAllPayments();
}
```

```java
package jiangbo.springcloud.dao;

import java.util.List;

import org.springframework.jdbc.core.BeanPropertyRowMapper;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.stereotype.Repository;

import jiangbo.springcloud.entity.PaymentInfo;

@Repository
public class PaymentDaoImpl implements PaymentDao {

    private static final String QUERY_ALL_PAYMENT_SQL = "select * from payment_info";

    private static final RowMapper<PaymentInfo> ROW_MAPPER = new BeanPropertyRowMapper<>(PaymentInfo.class);

    private final JdbcTemplate jdbcTemplate;

    public PaymentDaoImpl(JdbcTemplate jdbcTemplate) {

        this.jdbcTemplate = jdbcTemplate;
    }

    @Override
    public List<PaymentInfo> queryAllPayments() {

        return jdbcTemplate.query(QUERY_ALL_PAYMENT_SQL, ROW_MAPPER);
    }
}
```

### 服务层

```java
package jiangbo.springcloud.service;

import java.util.List;

import jiangbo.springcloud.entity.PaymentInfo;

public interface PaymentService {

    List<PaymentInfo> queryAllPayments();
}

```

```java
package jiangbo.springcloud.service.impl;

import java.util.List;

import org.springframework.stereotype.Service;

import jiangbo.springcloud.dao.PaymentDao;
import jiangbo.springcloud.entity.PaymentInfo;
import jiangbo.springcloud.service.PaymentService;

@Service
public class PaymentServiceImpl implements PaymentService {

    private final PaymentDao paymentDao;

    public PaymentServiceImpl(PaymentDao paymentDao) {

        this.paymentDao = paymentDao;

    }

    @Override
    public List<PaymentInfo> queryAllPayments() {

        return paymentDao.queryAllPayments();
    }
}
```

### 控制层

```java
package jiangbo.springcloud.controller;

import java.util.List;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import jiangbo.springcloud.entity.PaymentInfo;
import jiangbo.springcloud.service.PaymentService;

@RestController
@RequestMapping("/payment")
public class PaymentContrller {

    private final PaymentService paymentService;

    public PaymentContrller(PaymentService paymentService) {

        this.paymentService = paymentService;
    }

    @GetMapping
    List<PaymentInfo> allPayemtns() {

        return paymentService.queryAllPayments();
    }
}
```

### 配置

```yml
server:
  port: 4420

spring:
  application:
    name: payment

  datasource:
    url: jdbc:mysql://localhost/spring_cloud_payment?useUnicode=true&characterEncoding=utf-8&useSSL=false
    driver-class-name: com.mysql.jdbc.Driver
    username: root
    password: jiangbo
```

### 启动类

```java
package jiangbo.springcloud;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.netflix.eureka.EnableEurekaClient;

@SpringBootApplication
public class JiangBoApplication {

    public static void main(String[] args) {

        SpringApplication.run(JiangBoApplication.class, args);
    }
}
```

## 验证

访问 http://localhost:4420/ ，未看到明显的报错信息，则证明支付微服务环境搭建成功。


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
    <artifactId>07spring-cloud-payment</artifactId>
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


