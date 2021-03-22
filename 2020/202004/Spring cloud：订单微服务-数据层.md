# Spring cloud：订单微服务-数据层

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

搭建订单微服务的环境。

## 搭建步骤

### 建立数据库和数据表

新建如下的表结构：

![spring-cloud-order][1]

建表语句：

```sql
-- spring_cloud_order.order_info definition

CREATE TABLE `order_info` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(100) NOT NULL,
  `amount` varchar(100) NOT NULL,
  `status` enum('CREATED','PAID','DELETED') NOT NULL,
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4;
```

插入数据：

```sql
INSERT INTO spring_cloud_order.order_info (name,amount,status,create_time,update_time) VALUES 
('新订单测试','4.88','CREATED','2020-04-13 06:22:14.0','2020-04-13 14:22:00.0');
```

### 新增 pom.xml 依赖

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-jdbc</artifactId>
</dependency>

<dependency>
    <groupId>mysql</groupId>
    <artifactId>mysql-connector-java</artifactId>
</dependency>

<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-test</artifactId>
    <scope>test</scope>
</dependency>
```

### 定义实体类

```java
package jiangbo.springcloud.entity;

import java.util.Date;

public class OrderInfo {

    private long id;

    private String name;

    private String amount;

    private OrderStatusEnum status;

    private Date createTime;

    private Date updateTime;
}
```

```java
package jiangbo.springcloud.entity;

public enum OrderStatusEnum {

    /** 新建 */
    CREATED,

    /** 已支付 */
    PAID,

    /** 已删除 */
    DELETED
}
```

### 定义数据访问层

```java
package jiangbo.springcloud.dao;

import java.util.List;

import org.springframework.jdbc.core.BeanPropertyRowMapper;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.stereotype.Repository;

import jiangbo.springcloud.entity.OrderInfo;

@Repository
public class OrderInfoDao {

    private static final RowMapper<OrderInfo> ROW_MAPPER = new BeanPropertyRowMapper<>(OrderInfo.class);

    private static final String QUERY_ALL_SQL = "select * from order_info";

    private JdbcTemplate jdbcTemplate;

    public OrderInfoDao(JdbcTemplate jdbcTemplate) {

        this.jdbcTemplate = jdbcTemplate;
    }

    public List<OrderInfo> queryAllOrders() {

        return jdbcTemplate.query(QUERY_ALL_SQL, ROW_MAPPER);
    }
}
```

### 配置信息

```yml
server:
  port: 4410

spring:
  application:
    name: order
  
  datasource:
    url: jdbc:mysql://localhost/spring_cloud_order?useUnicode=true&characterEncoding=utf-8&useSSL=false
    driver-class-name: com.mysql.jdbc.Driver
    username: root
    password: jiangbo
```

## 验证

### 建立测试

```java
package jiangbo.springcloud.dao;

import static org.junit.Assert.assertFalse;

import java.util.List;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;

import jiangbo.springcloud.entity.OrderInfo;

@RunWith(SpringRunner.class)
@SpringBootTest
public class OrderInfoDaoTest {

    @Autowired
    private OrderInfoDao orderInfoDao;

    @Test
    public void testQueryAllOrders() {

        List<OrderInfo> allOrders = orderInfoDao.queryAllOrders();
        assertFalse(allOrders.isEmpty());
    }
}

```

### 运行

运行单元测试，通过测试，则订单微服务的数据层环境搭建成功。


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
            <artifactId>spring-boot-starter-jdbc</artifactId>
        </dependency>

        <dependency>
            <groupId>mysql</groupId>
            <artifactId>mysql-connector-java</artifactId>
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

[1]:../../images/spring-cloud-order.png