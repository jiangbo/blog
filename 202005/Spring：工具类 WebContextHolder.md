# Spring：工具类 WebContextHolder

## 环境

1. jdk 7
2. 4.3.24.RELEASE

## 源码

```java
package jiangbo.springweb.context;

import javax.servlet.ServletContext;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.servlet.http.HttpSession;

import org.springframework.web.context.request.RequestContextHolder;
import org.springframework.web.context.request.ServletRequestAttributes;

public final class WebContextHolder {

    /**
     * Constructor<br>
     */
    private WebContextHolder() {
    }

    /**
     * get Request<br>
     * 
     * @return HttpServletRequest
     */
    public static HttpServletRequest getRequest() {

        return (((ServletRequestAttributes) RequestContextHolder.currentRequestAttributes()).getRequest());
    }

    /**
     * get Response<br>
     * 
     * @return {@link HttpServletResponse}
     */
    public static HttpServletResponse getResponse() {

        return (((ServletRequestAttributes) RequestContextHolder.currentRequestAttributes()).getResponse());
    }

    /**
     * get Http Session<br>
     * 
     * @return Http Session
     */
    public static HttpSession getSession() {

        return getRequest().getSession();
    }

    /**
     * get Request Attribute<br>
     * 
     * @param name name
     * @return Object
     */
    public static Object getRequestAttribute(String name) {

        return getRequest().getAttribute(name);
    }

    /**
     * get Request Attribute<br>
     * 
     * @param <T>   泛型参数
     * @param name  name
     * @param clazz 值的类型
     * @return 从请求中获取的值
     */
    public static <T> T getRequestAttribute(String name, Class<T> clazz) {

        return clazz.cast(getRequest().getAttribute(name));
    }

    /**
     * get Request String<br>
     * 
     * @param name name
     * @return 从请求中获取的值
     */
    public static String getRequestString(String name) {

        return getRequestAttribute(name, String.class);
    }

    /**
     * get Session Attribute<br>
     * 
     * @param name name
     * @return Object
     */
    public static Object getSessionAttribute(String name) {

        return getSession().getAttribute(name);
    }

    /**
     * get Session Attribute<br>
     * 
     * @param <T>   泛型参数
     * @param name  session中的key
     * @param clazz 获取的类型
     * @return 从session中获取的值
     */
    public static <T> T getSessionAttribute(String name, Class<T> clazz) {

        return clazz.cast(getSession().getAttribute(name));
    }

    /**
     * get Session String<br>
     * 
     * @param name session中的key
     * @return 从session中获取的值
     */
    public static String getSessionString(String name) {

        return getSessionAttribute(name, String.class);
    }

    /**
     * get Servlet Context<br>
     * 
     * @return ServletContext
     */
    public static ServletContext getServletContext() {

        return getSession().getServletContext();
    }

    /**
     * set Request Attribute<br>
     * 
     * @param name  name
     * @param value value
     */
    public static void setRequestAttribute(String name, Object value) {

        getRequest().setAttribute(name, value);
    }

    /**
     * set Session Attribute<br>
     * 
     * @param name  name
     * @param value value
     */
    public static void setSessionAttribute(String name, Object value) {

        getSession().setAttribute(name, value);
    }
}
```