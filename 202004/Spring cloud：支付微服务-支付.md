# Spring cloud：支付微服务-支付

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

搭建支付微服务的环境。

## 搭建步骤

### 数据层

```java
package jiangbo.springcloud.dao;

import java.util.List;

import org.springframework.jdbc.core.BeanPropertyRowMapper;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.jdbc.core.namedparam.BeanPropertySqlParameterSource;
import org.springframework.jdbc.core.simple.SimpleJdbcInsert;
import org.springframework.stereotype.Repository;

import jiangbo.springcloud.entity.PaymentInfo;

@Repository
public class PaymentDaoImpl implements PaymentDao {

    private static final String QUERY_ALL_PAYMENT_SQL = "select * from payment_info";

    private static final String QUERY_PAYMENT_BY_ID_SQL = QUERY_ALL_PAYMENT_SQL + " where id = ?";

    private static final RowMapper<PaymentInfo> ROW_MAPPER = new BeanPropertyRowMapper<>(PaymentInfo.class);

    private final JdbcTemplate jdbcTemplate;

    public PaymentDaoImpl(JdbcTemplate jdbcTemplate) {

        this.jdbcTemplate = jdbcTemplate;
    }

    @Override
    public List<PaymentInfo> queryAllPayments() {

        return jdbcTemplate.query(QUERY_ALL_PAYMENT_SQL, ROW_MAPPER);
    }

    @Override
    public long insertPaymentInfo(PaymentInfo paymentInfo) {

        paymentInfo.setStatus("SUCCESS");
        return new SimpleJdbcInsert(jdbcTemplate)

                .withTableName("payment_info")
                // 指定主键
                .usingGeneratedKeyColumns("id")
                // 更新的列
                .usingColumns("order_id", "amount", "status")
                // 参数
                .executeAndReturnKey(new BeanPropertySqlParameterSource(paymentInfo)).longValue();
    }

    @Override
    public PaymentInfo queryPaymentInfo(long id) {

        return jdbcTemplate.queryForObject(QUERY_PAYMENT_BY_ID_SQL, ROW_MAPPER, id);
    }
}
```

### 服务层

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

    @Override
    public PaymentInfo insertPaymentInfo(PaymentInfo paymentInfo) {

        long id = paymentDao.insertPaymentInfo(paymentInfo);
        return paymentDao.queryPaymentInfo(id);
    }
}
```

### 控制层

```java
package jiangbo.springcloud.controller;

import java.util.List;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
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

    @PostMapping
    public PaymentInfo newPaymentInfo(@RequestBody PaymentInfo paymentInfo) {

        return paymentService.insertPaymentInfo(paymentInfo);
    }

    @GetMapping
    public List<PaymentInfo> allPayemtns() {

        return paymentService.queryAllPayments();
    }
}
```

## 验证

使用命名进行数据的新增，看到如下的结果，则证明成功：

```shell
curl -H "Content-Type: application/json" -X POST  --data '{"orderId":8,"amount":"8.88"}', http://localhost:4420/payment
{"id":2,"orderId":8,"amount":"8.88","status":"SUCCESS","createTime":1587224718000}  
```
