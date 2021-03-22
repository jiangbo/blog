# 【Maven】Maven 配置阿里云仓库


以下内容全部来自阿里云的[官方文档][1]


### 配置指南

### maven 配置指南

打开 maven 的配置文件，windows 机器一般在 maven 安装目录的 conf/settings.xml，
也可以配置在用户 home 的 .m2 目录下，然后在 <mirrors></mirrors> 标签中添加 mirror 子节点:

```xml
<mirror>
    <id>aliyunmaven</id>
    <mirrorOf>*</mirrorOf>
    <name>阿里云公共仓库</name>
    <url>https://maven.aliyun.com/repository/public</url>
</mirror>
```

如果想使用其它代理仓库,可在 <repositories></repositories> 节点中加入对应的仓库使用地址。
以使用 spring 代理仓为例：

```xml
<repository>
    <id>spring</id>
    <url>https://maven.aliyun.com/repository/spring</url>
    <releases>
        <enabled>true</enabled>
    </releases>
    <snapshots>
        <enabled>true</enabled>
    </snapshots>
</repository>
```

### gradle 配置指南

在 build.gradle 文件中加入以下代码:

```groovy
allprojects {
    repositories {
        maven { url 'https://maven.aliyun.com/repository/public/' }
        mavenLocal()
        mavenCentral()
    }
}
```

如果想使用maven.aliyun.com提供的其它代理仓，以使用spring仓为例，代码如下:

```groovy
allProjects {
    repositories {
        maven { url 'https://maven.aliyun.com/repository/public/' }
        maven { url 'https://maven.aliyun.com/repository/spring/'}
        mavenLocal()
        mavenCentral()
    }
}
```

[1]: https://help.aliyun.com/document_detail/102512.html?spm=a2c40.aliyun_maven_repo.0.0.36183054soO3QN