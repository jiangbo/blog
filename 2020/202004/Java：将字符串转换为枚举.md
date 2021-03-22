# Java：将字符串转换为枚举

## 环境

1. jdk 8

## 示例

```java
package jiangbo.java.lang;

import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

public enum SeasonEnum {

    SPRING("春"),

    SUMMER("夏"),

    AUTUMN("秋"),

    WINTER("冬");

    private static final Map<String, SeasonEnum> MAP = new HashMap<>();

    static {

        for (SeasonEnum season : values()) {
            MAP.put(season.name, season);
        }
    }

    private final String name;

    private SeasonEnum(String name) {

        this.name = name;
    }

    public static SeasonEnum valueOfName(String name) {

        return MAP.get(name);
    }

    public String toLocale() {

        if (Locale.CHINA.equals(Locale.getDefault())) {
            return name;
        }
        return toString();
    }

    public static void main(String[] args) {

        // 如果转换不成功，抛出 java.lang.IllegalArgumentException
        SeasonEnum spring = SeasonEnum.valueOf("SPRING");
        System.out.println(spring);

        SeasonEnum summer = SeasonEnum.valueOfName("夏");
        System.out.println(summer);

        System.out.println(SeasonEnum.AUTUMN.toLocale());
    }
}
```

