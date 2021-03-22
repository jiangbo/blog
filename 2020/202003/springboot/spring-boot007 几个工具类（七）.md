## 环境

1. jdk 6
2. tomcat 6.0.53
3. sts 4.4.2
4. maven 3.2.5
5. mysql 5.7

## SpringContextHolder

SpringContextHolder 可以很方便地获取 spring 的环境信息。

```java
package jiangbo.demo.core;

import javax.servlet.ServletContext;

import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.config.BeanFactoryPostProcessor;
import org.springframework.beans.factory.config.ConfigurableListableBeanFactory;
import org.springframework.context.ApplicationContext;
import org.springframework.context.ApplicationContextAware;
import org.springframework.core.env.Environment;
import org.springframework.stereotype.Component;
import org.springframework.web.context.WebApplicationContext;

@Component
public final class SpringContextHolder implements BeanFactoryPostProcessor, ApplicationContextAware {

    private static ApplicationContext context;

    private static ServletContext servletContext;

    private static Environment environment;

    private SpringContextHolder() {
    }

    public ApplicationContext getContext() {

        return context;
    }

    public static <T> T getBean(Class<T> requiredType) {

        return context.getBean(requiredType);
    }

    public static <T> T getBean(String name, Class<T> requiredType) {

        return context.getBean(name, requiredType);
    }

    public static Environment getEnviroment() {

        return environment;
    }

    public static ServletContext getServletContext() {

        return servletContext;
    }

    @Override
    public void postProcessBeanFactory(ConfigurableListableBeanFactory beanFactory) {

        // 实现BeanFactoryPostProcessor使其提前初始化，因为在其它bean初始化的时候，可能会使用SpringContextHolder
        LoggerFactory.getLogger(getClass()).info("Spring context holder initialized successful");
    }

    @Override
    public void setApplicationContext(ApplicationContext applicationContext) {

        init(applicationContext);
    }

    private void init(ApplicationContext applicationContext) {

        context = applicationContext;
        environment = applicationContext.getEnvironment();

        if (applicationContext instanceof WebApplicationContext) {

            servletContext = ((WebApplicationContext) applicationContext).getServletContext();
        }
    }
}
```

## AbstractJdbcDao

AbstractJdbcDao 是 sql 操作时的支持类。

```java
package jiangbo.demo.core;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.jdbc.core.JdbcTemplate;

public abstract class AbstractJdbcDao {

    protected final Logger logger = LoggerFactory.getLogger(getClass());

    protected final JdbcTemplate jdbcTemplate = SpringContextHolder.getBean(JdbcTemplate.class);
}

```

## AbstractJdbcInsertDao

AbstractJdbcInsertDao 在插入数据的时候比较有帮助。

```java
package jiangbo.demo.core;

import org.springframework.jdbc.core.simple.SimpleJdbcInsert;

public abstract class AbstractJdbcInsertDao extends AbstractJdbcDao {

    private static final String SQL_TEMPLATE_FIND_ALL = "SELECT * FROM ";

    protected final String findAllSql;

    protected final SimpleJdbcInsert jdbcInsert;

    public AbstractJdbcInsertDao(String tableName) {

        jdbcInsert = new SimpleJdbcInsert(jdbcTemplate).withTableName(tableName);
        findAllSql = SQL_TEMPLATE_FIND_ALL + tableName;
    }
}

```
