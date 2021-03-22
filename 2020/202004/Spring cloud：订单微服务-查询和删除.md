# Spring cloud：订单微服务-查询和删除

## 环境

1. spring cloud Edgware.SR6
2. jdk 7
3. sts 4.6.0
5. mysql 5.7

## 背景

搭建订单微服务的环境。

## 搭建步骤

### 接口层

接口层新增两个方法，一个根据id查询，一个根据id删除。

```java
    @GetMapping("/{id}")
    public OrderInfo getOrderInfo(@PathVariable long id) {

        return orderInfoService.queryOrderInfo(id);
    }

    @DeleteMapping("/{id}")
    public int deleteOrderInfo(@PathVariable long id) {

        return orderInfoService.deleteOrderInfo(id);
    }
```

### 服务层

```java
    @Override
    public OrderInfo queryOrderInfo(long id) {

        return orderInfoDao.queryOrderInfo(id);
    }

    @Override
    public int deleteOrderInfo(long id) {

        return orderInfoDao.deleteOrderInfo(id);
    }
```

### 数据访问层

```java
    private static final String QUERY_ALL_SQL = "select * from order_info";

    private static final String QUERY_ORDER_INFO_BY_ID_SQL = QUERY_ALL_SQL + " where id = ?";

    private static final String DELETE_ORDER_INFO_BY_ID_SQL = "delete from order_info where id = ?";

    @Override
    public OrderInfo queryOrderInfo(long id) {

        return jdbcTemplate.queryForObject(QUERY_ORDER_INFO_BY_ID_SQL, ROW_MAPPER, id);
    }

    @Override
    public int deleteOrderInfo(long id) {

        return jdbcTemplate.update(DELETE_ORDER_INFO_BY_ID_SQL, id);
    }
```

## 验证

### 建立测试

```java
    @Test
    public void getOrderInfo() throws Exception {

        OrderInfo order = restTemplate.getForObject("/order/1", OrderInfo.class);
        assertEquals("新订单测试", order.getName());
    }
```

### 运行

运行单元测试，通过测试，则订单微服务的接口层环境搭建成功。


