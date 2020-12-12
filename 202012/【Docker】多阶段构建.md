# 【Docker】多阶段构建

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 多阶段构建

### 定义 Dockerfile

```Dockerfile
FROM  maven:3.6.3-openjdk-8-slim as builder
LABEL author=jiangbo version=1.0.3
COPY . /
#RUN mvn clean package

FROM openjdk:8-jdk-alpine
ARG JAR_FILE=*.jar
COPY --from=builder /${JAR_FILE} app.jar
EXPOSE 8080
ENTRYPOINT ["java","-jar","/app.jar"]
```

### 准备 spring boot 应用

可以在官网生成一个。

### 构建

```sh
[root@master multi]# docker build -t demo:1.0.3 .
Sending build context to Docker daemon  17.01MB
Step 1/8 : FROM  maven:3.6.3-openjdk-8-slim as builder
 ---> 05187405e310
Step 2/8 : LABEL author=jiangbo version=1.0.3
 ---> Using cache
 ---> e93004b2e682
Step 3/8 : COPY . /
 ---> 48b9b04102d8
Step 4/8 : FROM openjdk:8-jdk-alpine
 ---> a3562aa0b991
Step 5/8 : ARG JAR_FILE=*.jar
 ---> Running in a88ae571ffe5
Removing intermediate container a88ae571ffe5
 ---> ae5612bd07e8
Step 6/8 : COPY --from=builder /${JAR_FILE} app.jar
 ---> 99022253d767
Step 7/8 : EXPOSE 8080
 ---> Running in aec2d6a48621
Removing intermediate container aec2d6a48621
 ---> cbf4c735bb9b
Step 8/8 : ENTRYPOINT ["java","-jar","/app.jar"]
 ---> Running in a06af9a28f44
Removing intermediate container a06af9a28f44
 ---> 9747915cf8a3
Successfully built 9747915cf8a3
Successfully tagged demo:1.0.3
```

### 查看镜像大小

```sh
[root@master multi]# docker image ls
REPOSITORY              TAG                    IMAGE ID       CREATED         SIZE
demo                    1.0.3                  9747915cf8a3   8 minutes ago   122MB
maven                   3.6.3-openjdk-8-slim   05187405e310   17 hours ago    306MB
openjdk                 8-jdk-alpine           a3562aa0b991   19 months ago   105MB
[root@master multi]#
```

可以看到第一次引入的镜像构建完成后，就没有使用了。

## 总结

介绍了 Docker 的多阶段构建。