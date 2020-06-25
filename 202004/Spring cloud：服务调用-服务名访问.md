# Spring cloud：服务调用-服务名访问

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

通过 IP 访问需要知道具体的地址和端口，使用了服务注册后，可以通过服务名进行访问。

## 搭建步骤

只需要修改支付服务调用层和一些配置就可以实现。

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

    private static final String PAYMENT_SERIVCE_URL = "http://payment/payment";

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
    @LoadBalanced
    public RestTemplate restTemplate(RestTemplateBuilder builder) {

        return builder.build();
    }
}
```

### 增加日志

因为有三台支付微服务，为了方便看到访问了哪一台，增加端口的日志记录。

```java
package jiangbo.springcloud.controller;

import java.util.List;

import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
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

    private final int port;

    public PaymentContrller(PaymentService paymentService, @Value("${server.port}") int port) {

        this.paymentService = paymentService;
        this.port = port;
    }

    @PostMapping
    public PaymentInfo newPaymentInfo(@RequestBody PaymentInfo paymentInfo) {

        LoggerFactory.getLogger(getClass()).info("invoke payment port: {}", port);
        return paymentService.insertPaymentInfo(paymentInfo);
    }

    @GetMapping
    public List<PaymentInfo> allPayemtns() {

        return paymentService.queryAllPayments();
    }
}
```

## 验证

使用 curl 命令访问订单微服务，查看返回，如果返回的订单已支付，则表示通过服务名访问成功。

```shell
curl -H "Content-Type: application/json" -X POST --data '{"name":"jiangbo10","amount":"8.88"}', http://localhost:4410/order/payment
{"id":11,"name":"jiangbo10","amount":"8.88","status":"PAID","createTime":1587303467000,"updateTime":1587303467000}    
```

多访问几次，能看到支付微服务被轮询调用。到此，客户端的负载均衡也一并实现了。