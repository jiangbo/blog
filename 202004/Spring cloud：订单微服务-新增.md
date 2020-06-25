# Spring cloud：订单微服务-新增

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

搭建订单微服务的环境。

## 搭建步骤

### 接口层

```java
package jiangbo.springcloud.controller;

import java.util.List;

import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
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

    @GetMapping("/{id}")
    public OrderInfo getOrderInfo(@PathVariable long id) {

        return orderInfoService.queryOrderInfo(id);
    }

    @DeleteMapping("/{id}")
    public int deleteOrderInfo(@PathVariable long id) {

        return orderInfoService.deleteOrderInfo(id);
    }

    @PostMapping
    public OrderInfo newOrderInfo(@RequestBody OrderInfo orderInfo) {

        return orderInfoService.insertOrderInfo(orderInfo);
    }
}
```

### 服务层

```java
package jiangbo.springcloud.service;

import java.util.List;

import jiangbo.springcloud.entity.OrderInfo;

public interface OrderInfoService {

    List<OrderInfo> queryAllOrders();

    OrderInfo insertOrderInfo(OrderInfo orderInfo);

    OrderInfo queryOrderInfo(long id);

    int deleteOrderInfo(long id);
}
```

```java
package jiangbo.springcloud.service.impl;

import java.util.List;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

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

    @Transactional
    @Override
    public OrderInfo insertOrderInfo(OrderInfo orderInfo) {

        long id = orderInfoDao.insertOrderInfo(orderInfo);
        return orderInfoDao.queryOrderInfo(id);
    }

    @Override
    public OrderInfo queryOrderInfo(long id) {

        return orderInfoDao.queryOrderInfo(id);
    }

    @Override
    public int deleteOrderInfo(long id) {

        return orderInfoDao.deleteOrderInfo(id);
    }
}
```

### 数据访问层

```java
package jiangbo.springcloud.dao;

import java.util.List;

import jiangbo.springcloud.entity.OrderInfo;

public interface OrderInfoDao {

    List<OrderInfo> queryAllOrders();

    long insertOrderInfo(OrderInfo orderInfo);

    OrderInfo queryOrderInfo(long id);

    int deleteOrderInfo(long id);
}
```

```java
package jiangbo.springcloud.dao;

import java.util.Date;
import java.util.List;

import org.springframework.jdbc.core.BeanPropertyRowMapper;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.jdbc.core.namedparam.BeanPropertySqlParameterSource;
import org.springframework.jdbc.core.simple.SimpleJdbcInsert;
import org.springframework.stereotype.Repository;

import jiangbo.springcloud.entity.OrderInfo;

@Repository
public class OrderInfoDaoImpl implements OrderInfoDao {

    private static final RowMapper<OrderInfo> ROW_MAPPER = new BeanPropertyRowMapper<>(OrderInfo.class);

    private static final String QUERY_ALL_SQL = "select * from order_info";

    private static final String QUERY_ORDER_INFO_BY_ID_SQL = QUERY_ALL_SQL + " where id = ?";

    private static final String DELETE_ORDER_INFO_BY_ID_SQL = "delete from order_info where id = ?";

    private JdbcTemplate jdbcTemplate;

    public OrderInfoDaoImpl(JdbcTemplate jdbcTemplate) {

        this.jdbcTemplate = jdbcTemplate;
    }

    @Override
    public List<OrderInfo> queryAllOrders() {

        return jdbcTemplate.query(QUERY_ALL_SQL, ROW_MAPPER);
    }

    @Override
    public long insertOrderInfo(OrderInfo orderInfo) {

        orderInfo.setUpdateTime(new Date());
        return new SimpleJdbcInsert(jdbcTemplate)
                // 插入订单表
                .withTableName("order_info")
                // 指定主键
                .usingGeneratedKeyColumns("id")
                // 更新的列
                .usingColumns("name", "amount", "update_time")
                // 参数
                .executeAndReturnKey(new BeanPropertySqlParameterSource(orderInfo)).longValue();

    }

    @Override
    public OrderInfo queryOrderInfo(long id) {

        return jdbcTemplate.queryForObject(QUERY_ORDER_INFO_BY_ID_SQL, ROW_MAPPER, id);
    }

    @Override
    public int deleteOrderInfo(long id) {

        return jdbcTemplate.update(DELETE_ORDER_INFO_BY_ID_SQL, id);
    }
}
```

## 验证

启动服务，使用

`curl -H "Content-Type: application/json" -X POST  --data '{"name":"jiangbo4","amount":"8.88"}', http://localhost:4410/order`

进行验证，插入数据库成功，则增删查功能都已实现。

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


