# spring-boot 连接数据库（六）

## 环境

1. jdk 6
2. tomcat 6.0.53
3. sts 4.4.2
4. maven 3.2.5
5. mysql 5.7

## 准备

接下来的数据库操作基于 mysql，所以需要一套可用的 mysql 环境。

## 引入 jdbc 依赖

spring boot 有配置的 jdbc starter，可以直接使用。

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-jdbc</artifactId>
</dependency>
```

## 删除 tomcat-juli

因为默认的数据库连接池依赖的是 tomcat-jdbc，已经依赖进来了 tomcat-juli，所以这个依赖可用直接删除。
删除如下依赖：

```xml
<dependency>
    <groupId>org.apache.tomcat</groupId>
    <artifactId>tomcat-juli</artifactId>
    <version>${tomcat.version}</version>
    <scope>provided</scope>
</dependency
```

## 引入 mysql 连接驱动

```xml
<dependency>
    <groupId>mysql</groupId>
    <artifactId>mysql-connector-java</artifactId>
    <scope>runtime</scope>
</dependency>
```

## 增加数据库连接信息

在 application.properties 中，增加如下信息：

```properteis
spring.datasource.url=jdbc:mysql://localhost/spring-boot-demo?useUnicode=true&characterEncoding=UTF-8&useSSL=false
spring.datasource.username=root
spring.datasource.password=123456
```

## 数据库对象

省略了 get/set 方法。

```java
public class User {

    private Integer id;

    private String username;

    private String userpwd;

    private String fullname;

    private Integer age;
}
```

## api 访问层

增加 UserApi 控制层，并增加获得旅客列表的方法。增加构造函数注入访问数据库的 userDao。
修改完成后，如下：

```java
package jiangbo.demo.user.api;

@RestController
@RequestMapping("/users")
public class UserApi {

    private final UserService userService;

    public UserApi(UserService userService) {

        this.userService = userService;
    }

    @GetMapping
    public List<User> allUsers() {

        return userService.findAll();
    }
}

```

## service 层

暂时还没有任何的业务，可以直接调用 dao 层，可以增加一个接口来隔离实现。

```java
package jiangbo.demo.user.service;

public interface UserService {

    List<User> allUsers();

}
```

```java
package jiangbo.demo.user.service.impl;

@Service
public class UserServiceImpl implements UserService {

    private final UserDao userDao;

    public UserServiceImpl(UserDao userDao) {

        this.userDao = userDao;
    }

    @Override
    public List<User> allUsers() {

        return userDao.findAll();
    }
}
```

## 增加 dao

先定义 dao 访问的接口：

```java
package jiangbo.demo.dao;

public interface UserDao {

    List<User> findAll();
}
```

在增加 jdbc 的实现。

```java
package jiangbo.demo.user.dao.jdbc;

@Repository
public class JdbcUserDao implements UserDao {

    private static final RowMapper<User> USER_ROW_MAPPER = BeanPropertyRowMapper.newInstance(User.class);

    private final JdbcTemplate jdbcTemplate;

    public JdbcUserDao(JdbcTemplate jdbcTemplate) {

        this.jdbcTemplate = jdbcTemplate;
    }

    @Override
    public List<User> findAll() {

        return jdbcTemplate.query("SELECT * FROM user", USER_ROW_MAPPER);
    }
}
```

## 运行并访问

访问 localhost:8080/demo/users，可以看到页面显示了数据库中存在的数据。

```json
[
{
"id": 1,
"username": "zhangdaming",
"userpwd": "123456",
"fullname": "张大明",
"age": 66
},
{
"id": 2,
"username": "jiangbo",
"userpwd": "123456",
"fullname": "jiangbo",
"age": 44
},
{
"id": 3,
"username": "zhangsan",
"userpwd": "123456",
"fullname": "张三",
"age": 55
}
]
```

## 附录

### 完整 pom.xml

```xml
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>1.5.22.RELEASE</version>
    </parent>

    <groupId>jiangbo.demo</groupId>
    <artifactId>demo</artifactId>
    <version>1.0.0</version>
    <packaging>war</packaging>

    <properties>
        <tomcat.version>7.0.59</tomcat.version>
        <jackson.version>2.6.7.3</jackson.version>
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
            <artifactId>spring-boot-legacy</artifactId>
            <version>1.1.0.RELEASE</version>
        </dependency>

        <!-- provided -->
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-tomcat</artifactId>
            <scope>provided</scope>
        </dependency>

    </dependencies>

</project>
```
