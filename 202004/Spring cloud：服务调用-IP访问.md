# Spring cloud：服务调用-IP访问

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

订单微服务通过 IP 和端口访问支付微服务进行订单的支付。

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

    @PostMapping("/payment")
    public OrderInfo paymentOrderInfo(@RequestBody OrderInfo orderInfo) {

        return orderInfoService.payOrderInfo(orderInfo);
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

    OrderInfo payOrderInfo(OrderInfo orderInfo);
}
```

```java
package jiangbo.springcloud.service.impl;

import java.util.List;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import jiangbo.springcloud.dao.OrderInfoDao;
import jiangbo.springcloud.dao.PaymentDao;
import jiangbo.springcloud.dao.dto.PaymentRequest;
import jiangbo.springcloud.dao.dto.PaymentResponse;
import jiangbo.springcloud.entity.OrderInfo;
import jiangbo.springcloud.entity.OrderStatusEnum;
import jiangbo.springcloud.service.OrderInfoService;

@Service
public class OrderInfoServiceImpl implements OrderInfoService {

    private final OrderInfoDao orderInfoDao;

    private final PaymentDao paymentDao;

    public OrderInfoServiceImpl(OrderInfoDao orderInfoDao, PaymentDao paymentDao) {

        this.orderInfoDao = orderInfoDao;
        this.paymentDao = paymentDao;
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

    @Transactional
    @Override
    public OrderInfo payOrderInfo(OrderInfo orderInfo) {

        long id = orderInfoDao.insertOrderInfo(orderInfo);

        PaymentRequest request = new PaymentRequest();
        request.setAmount(orderInfo.getAmount());
        request.setOrderId(id);
        PaymentResponse response = paymentDao.payment(request);

        if ("SUCCESS".equals(response.getStatus())) {

            orderInfoDao.updateStatus(id, OrderStatusEnum.PAID);
        }

        return queryOrderInfo(id);
    }
}
```

### 订单数据访问层

```java
package jiangbo.springcloud.dao;

import java.util.List;

import jiangbo.springcloud.entity.OrderInfo;
import jiangbo.springcloud.entity.OrderStatusEnum;

public interface OrderInfoDao {

    List<OrderInfo> queryAllOrders();

    long insertOrderInfo(OrderInfo orderInfo);

    OrderInfo queryOrderInfo(long id);

    int deleteOrderInfo(long id);

    int updateStatus(long id, OrderStatusEnum orderStatus);
}
```

```java
package jiangbo.springcloud.dao.jdbc;

import java.util.Date;
import java.util.List;

import org.springframework.jdbc.core.BeanPropertyRowMapper;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.jdbc.core.namedparam.BeanPropertySqlParameterSource;
import org.springframework.jdbc.core.simple.SimpleJdbcInsert;
import org.springframework.stereotype.Repository;

import jiangbo.springcloud.dao.OrderInfoDao;
import jiangbo.springcloud.entity.OrderInfo;
import jiangbo.springcloud.entity.OrderStatusEnum;

@Repository
public class OrderInfoDaoImpl implements OrderInfoDao {

    private static final String UPDATE_STATUS_SQL = "update order_info set status = ? where id = ?";

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

    @Override
    public int updateStatus(long id, OrderStatusEnum orderStatus) {

        return jdbcTemplate.update(UPDATE_STATUS_SQL, new Object[] { orderStatus.toString(), id });
    }
}
```

### 支付服务调用层

```java
package jiangbo.springcloud.dao;

import jiangbo.springcloud.dao.dto.PaymentRequest;
import jiangbo.springcloud.dao.dto.PaymentResponse;

public interface PaymentDao {

    PaymentResponse payment(PaymentRequest paymentRequest);
}
```

```java
package jiangbo.springcloud.dao.http;

import org.springframework.stereotype.Component;
import org.springframework.web.client.RestTemplate;

import jiangbo.springcloud.dao.PaymentDao;
import jiangbo.springcloud.dao.dto.PaymentRequest;
import jiangbo.springcloud.dao.dto.PaymentResponse;

@Component
public class PaymentDaoImpl implements PaymentDao {

    private static final String PAYMENT_SERIVCE_URL = "http://localhost:4420/payment";

    private final RestTemplate restTemplate;

    public PaymentDaoImpl(RestTemplate restTemplate) {

        this.restTemplate = restTemplate;
    }

    @Override
    public PaymentResponse payment(PaymentRequest paymentRequest) {

        return restTemplate.postForObject(PAYMENT_SERIVCE_URL, paymentRequest, PaymentResponse.class);
    }

}
```

### 启动类

```java
package jiangbo.springcloud;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.web.client.RestTemplateBuilder;
import org.springframework.cloud.netflix.eureka.EnableEurekaClient;
import org.springframework.context.annotation.Bean;
import org.springframework.web.client.RestTemplate;

@SpringBootApplication
@EnableEurekaClient
public class JiangBoApplication {

    public static void main(String[] args) {

        SpringApplication.run(JiangBoApplication.class, args);
    }

    @Bean
    public RestTemplate restTemplate(RestTemplateBuilder builder) {

        return builder.build();
    }
}
```

## 验证

使用 curl 命令访问订单微服务，查看返回，如果返回的订单已支付，则表示通过 IP 访问成功。

```shell
curl -H "Content-Type: application/json" -X POST --data '{"name":"jiangbo8","amount":"8.88"}', http://localhost:4410/order/payment
{"id":9,"name":"jiangbo8","amount":"8.88","status":"PAID","createTime":1587278913000,"updateTime":1587278914000}     
```