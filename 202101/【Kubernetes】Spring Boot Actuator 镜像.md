# 【Kubernetes】Spring Boot Actuator 镜像

## 环境

1. kubernetes 1.20.2
2. Spring Boot 2.5.0-M1

## 目标

构建一个 Spring Boot Actuator 镜像，并在 Kubernetes 环境上运行，查看与普通环境上的区别。

## Spring Boot Actuator

### pom.xml

```xml
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>2.5.0-M1</version>
        <relativePath /> <!-- lookup parent from repository -->
    </parent>
    <groupId>jiangbo.spring.demo</groupId>
    <artifactId>spring-demo</artifactId>
    <version>actuator</version>
    <name>spring-boot-demo</name>
    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-actuator</artifactId>
        </dependency>
    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
                <executions>
                    <execution>
                        <id>build-info</id>
                        <goals>
                            <goal>build-info</goal>
                        </goals>
                    </execution>
                </executions>
            </plugin>
        </plugins>
    </build>
    <repositories>
        <repository>
            <id>spring-milestones</id>
            <name>Spring Milestones</name>
            <url>https://repo.spring.io/milestone</url>
        </repository>
    </repositories>
    <pluginRepositories>
        <pluginRepository>
            <id>spring-milestones</id>
            <name>Spring Milestones</name>
            <url>https://repo.spring.io/milestone</url>
        </pluginRepository>
    </pluginRepositories>

</project>
```

### SpringDemoApplication

```java
package jiangbo.spring.docker;

import java.net.InetAddress;
import java.net.UnknownHostException;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.info.BuildProperties;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class SpringDemoApplication {

    private final BuildProperties buildProperties;

    public SpringDemoApplication(BuildProperties buildProperties) {

        this.buildProperties = buildProperties;
    }

    private static String hostname;

    public static void main(String[] args) throws UnknownHostException {

        hostname = InetAddress.getLocalHost().getHostName();
        SpringApplication.run(SpringDemoApplication.class, args);
    }

    @GetMapping("/hostname")
    public String hello() {

        return hostname + " " + buildProperties.getVersion();
    }

}
```

### application.yaml

```yaml
management:
  endpoint:
    shutdown:
      enabled: true
  endpoints:
    web:
      exposure:
        include: "*"
```

### Dockerfile

```Dockerfile
FROM openjdk:8-jre-alpine as builder
ARG JAR_FILE=target/*.jar
COPY ${JAR_FILE} application.jar
RUN java -Djarmode=layertools -jar application.jar extract

FROM openjdk:8-jre-alpine
COPY --from=builder dependencies/ ./
COPY --from=builder snapshot-dependencies/ ./
COPY --from=builder spring-boot-loader/ ./
COPY --from=builder application/ ./
ENTRYPOINT ["java", "org.springframework.boot.loader.JarLauncher"]
```

## 构建镜像

### 打包

```powershell
PS D:\workspace\sts\spring-demo> mvn clean package
[INFO] Scanning for projects...
[INFO]
[INFO] ------------------< jiangbo.spring.demo:spring-demo >-------------------
[INFO] Building spring-boot-demo actuator
[INFO] --------------------------------[ jar ]---------------------------------
[INFO]
[INFO] --- maven-clean-plugin:3.1.0:clean (default-clean) @ spring-demo ---
[INFO] Deleting D:\workspace\sts\spring-demo\target
[INFO]
[INFO] --- spring-boot-maven-plugin:2.5.0-M1:build-info (build-info) @ spring-demo ---
[INFO]
[INFO] --- maven-resources-plugin:3.2.0:resources (default-resources) @ spring-demo ---
[INFO] Using 'UTF-8' encoding to copy filtered resources.
[INFO] Using 'UTF-8' encoding to copy filtered properties files.
[INFO] Copying 1 resource
[INFO] Copying 0 resource
[INFO]
[INFO] --- maven-compiler-plugin:3.8.1:compile (default-compile) @ spring-demo ---
[INFO] Changes detected - recompiling the module!
[INFO] Compiling 1 source file to D:\workspace\sts\spring-demo\target\classes
[INFO]
[INFO] --- maven-resources-plugin:3.2.0:testResources (default-testResources) @ spring-demo ---
[INFO] Using 'UTF-8' encoding to copy filtered resources.
[INFO] Using 'UTF-8' encoding to copy filtered properties files.
[INFO] skip non existing resourceDirectory D:\workspace\sts\spring-demo\src\test\resources
[INFO]
[INFO] --- maven-compiler-plugin:3.8.1:testCompile (default-testCompile) @ spring-demo ---
[INFO] Changes detected - recompiling the module!
[INFO]
[INFO] --- maven-surefire-plugin:2.22.2:test (default-test) @ spring-demo ---
[INFO]
[INFO] --- maven-jar-plugin:3.2.0:jar (default-jar) @ spring-demo ---
[INFO] Building jar: D:\workspace\sts\spring-demo\target\spring-demo-actuator.jar
[INFO]
[INFO] --- spring-boot-maven-plugin:2.5.0-M1:repackage (repackage) @ spring-demo ---
[INFO] Replacing main artifact with repackaged archive
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time:  7.433 s
[INFO] Finished at: 2021-01-25T21:47:17+08:00
[INFO] ------------------------------------------------------------------------
```

### 构建镜像

```powershell
PS D:\workspace\sts\spring-demo> docker build -t jiangbo920827/spring-demo:actuator .
Sending build context to Docker daemon   19.2MB
Step 1/10 : FROM openjdk:8-jre-alpine as builder
 ---> f7a292bbb70c
Step 2/10 : ARG JAR_FILE=target/*.jar
 ---> Using cache
 ---> 061b93704007
Step 3/10 : COPY ${JAR_FILE} application.jar
 ---> 20e5f2fa61e3
Step 4/10 : RUN java -Djarmode=layertools -jar application.jar extract
 ---> Running in e5028a5d2546
Removing intermediate container e5028a5d2546
 ---> 6cfa89d4fe04
Step 5/10 : FROM openjdk:8-jre-alpine
 ---> f7a292bbb70c
Step 6/10 : COPY --from=builder dependencies/ ./
 ---> Using cache
 ---> 94c069dd1619
Step 7/10 : COPY --from=builder snapshot-dependencies/ ./
 ---> Using cache
 ---> 1aba6579c120
Step 8/10 : COPY --from=builder spring-boot-loader/ ./
 ---> Using cache
 ---> 1f67b70f1647
Step 9/10 : COPY --from=builder application/ ./
 ---> 6143bdef4fd0
Step 10/10 : ENTRYPOINT ["java", "org.springframework.boot.loader.JarLauncher"]
 ---> Running in 3bee9efaf294
Removing intermediate container 3bee9efaf294
 ---> 27b1db723cf2
Successfully built 27b1db723cf2
Successfully tagged jiangbo920827/spring-demo:actuator
SECURITY WARNING: You are building a Docker image from Windows against a non-Windows Docker host. 
All files and directories added to build context will have '-rwxr-xr-x' permissions. 
It is recommended to double check and reset permissions for sensitive files and directories.
```

### 推送到远程仓库

```powershell
PS D:\workspace\sts\spring-demo> docker push jiangbo920827/spring-demo:actuator
The push refers to repository [docker.io/jiangbo920827/spring-demo]
da6a673c5c81: Pushed
b06e2e5a3a34: Mounted from jiangbo920827/spring-docker
7d97b13df67a: Mounted from jiangbo920827/spring-docker
edd61588d126: Mounted from jiangbo920827/spring-docker
9b9b7f3d56a0: Mounted from jiangbo920827/spring-docker
f1b5933fe4b5: Mounted from library/openjdk
actuator: digest: sha256:fef2dd74c274e783e4cf2f270da15cadbc0766c8a0d24dad31dd0258e2eb4722 size: 1576
```

## 创建 Pod

### pod.yaml

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-demo

spec:
  containers:
    - name: pod-demo
      image: jiangbo920827/spring-demo:actuator
      ports:
        - containerPort: 8080

```

### 创建

```sh
[root@master pod]# kubectl apply  -f pod.yaml 
pod/pod-demo created
```

### 查看

```sh
[root@master pod]# kubectl get -f pod.yaml -o wide
NAME       READY   STATUS    RESTARTS   AGE     IP            NODE    NOMINATED NODE   READINESS GATES
pod-demo   1/1     Running   0          5m12s   10.244.1.16   node1   <none>           <none>
```

### 比较

同时在本地和 Kubernetes 环境上访问 http://localhost:8080/actuator/health 。

### 本地访问

```json
{
    "status": "UP"
}
```

### Kubernetes 访问

URL 根据 Pod 的实际 IP 填写。

```json
{
  "status": "UP",
  "groups": [
    "liveness",
    "readiness"
  ]
}
```

## 总结

介绍了创建 Spring Boot Actuator 的镜像，并以 Pod 的方式启动。
比较了本地和 Kubernetes 的环境暴露健康端点的不通。

## 附录

### Spring Boot 判断 Kubernetes 环境

Spring Boot 判断 Kubernetes 环境的源码如下：

```java
/**
  * Kubernetes platform.
  */
KUBERNETES {

  private static final String KUBERNETES_SERVICE_HOST = "KUBERNETES_SERVICE_HOST";

  private static final String KUBERNETES_SERVICE_PORT = "KUBERNETES_SERVICE_PORT";

  private static final String SERVICE_HOST_SUFFIX = "_SERVICE_HOST";

  private static final String SERVICE_PORT_SUFFIX = "_SERVICE_PORT";

  @Override
  public boolean isDetected(Environment environment) {
    if (environment instanceof ConfigurableEnvironment) {
      return isAutoDetected((ConfigurableEnvironment) environment);
    }
    return false;
  }

  private boolean isAutoDetected(ConfigurableEnvironment environment) {
    PropertySource<?> environmentPropertySource = environment.getPropertySources()
        .get(StandardEnvironment.SYSTEM_ENVIRONMENT_PROPERTY_SOURCE_NAME);
    if (environmentPropertySource != null) {
      if (environmentPropertySource.containsProperty(KUBERNETES_SERVICE_HOST)
          && environmentPropertySource.containsProperty(KUBERNETES_SERVICE_PORT)) {
        return true;
      }
      if (environmentPropertySource instanceof EnumerablePropertySource) {
        return isAutoDetected((EnumerablePropertySource<?>) environmentPropertySource);
      }
    }
    return false;
  }

  private boolean isAutoDetected(EnumerablePropertySource<?> environmentPropertySource) {
    for (String propertyName : environmentPropertySource.getPropertyNames()) {
      if (propertyName.endsWith(SERVICE_HOST_SUFFIX)) {
        String serviceName = propertyName.substring(0,
            propertyName.length() - SERVICE_HOST_SUFFIX.length());
        if (environmentPropertySource.getProperty(serviceName + SERVICE_PORT_SUFFIX) != null) {
          return true;
        }
      }
    }
    return false;
  }

};
```

### 判断 Kubernetes 环境

```java
if (CloudPlatform.getActive(environment) == CloudPlatform.KUBERNETES) {
  return ConditionOutcome.match(message.because("running on Kubernetes"));
}
```