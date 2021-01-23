# 【Kubernetes】构建 Spring boot 测试镜像

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03
4. kubernetes 1.20.2
5. Spring Boot 2.5.0-M1

## 目标

创建一个 Spring Boot 项目，可以返回主机名，然后打包成镜像，并推送到远程仓库。

## 创建 Spring Boot 项目

### pom.xml

其中 image.name 表示生成的镜像名称，需要替换成自己的用户名。

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
    <groupId>jiangbo.spring.docker</groupId>
    <artifactId>spring-docker</artifactId>
    <version>1.0.0</version>
    <name>spring-boot-docker</name>
    <properties>
        <java.version>1.8</java.version>
    </properties>
    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
        </dependency>
    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.springframework.boot</groupId>
                <artifactId>spring-boot-maven-plugin</artifactId>
                <configuration>
                    <image>
                        <name>jiangbo920827/${project.artifactId}</name>
                    </image>
                </configuration>
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

### 主机名

```java
package jiangbo.spring.docker;

import java.net.InetAddress;
import java.net.UnknownHostException;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class SpringDockerApplication {

    private static String hostname;

    public static void main(String[] args) throws UnknownHostException {

        hostname = InetAddress.getLocalHost().getHostName();
        SpringApplication.run(SpringDockerApplication.class, args);
    }

    @GetMapping("/hostname")
    public String hello() {

        return hostname;
    }

}
```

## 打包镜像

### 本地 Docker 环境

```powershell
PS D:\workspace\sts\spring-docker> docker version -f "{{.Client.Version}}"
20.10.2
```

如果没有安装 Docker，可以参考：[Docker：在 wsl2 上安装 docker][1]
安装完成后，最好安装一个桌面使用的 Docker Desktop。

### 构建镜像

```powershell
PS D:\workspace\sts\spring-docker> mvn spring-boot:build-image
[INFO] Scanning for projects...
[INFO]
[INFO] ----------------< jiangbo.spring.docker:spring-docker >-----------------
[INFO] Building spring-boot-docker 1.0.0
[INFO] --------------------------------[ jar ]---------------------------------
[INFO]
[INFO] >>> spring-boot-maven-plugin:2.5.0-M1:build-image (default-cli) > package @ spring-docker >>>
[INFO]
[INFO] --- maven-resources-plugin:3.2.0:resources (default-resources) @ spring-docker ---
[INFO] Using 'UTF-8' encoding to copy filtered resources.
[INFO] Using 'UTF-8' encoding to copy filtered properties files.
[INFO] Copying 1 resource
[INFO] Copying 0 resource
[INFO] --- maven-compiler-plugin:3.8.1:compile (default-compile) @ spring-docker ---
[INFO] Nothing to compile - all classes are up to date
[INFO]
[INFO] --- maven-compiler-plugin:3.8.1:testCompile (default-testCompile) @ spring-docker ---
[INFO] Nothing to compile - all classes are up to date
[INFO]
[INFO] --- maven-surefire-plugin:2.22.2:test (default-test) @ spring-docker ---
[INFO]
[INFO] --- maven-jar-plugin:3.2.0:jar (default-jar) @ spring-docker ---
[INFO]
[INFO] --- spring-boot-maven-plugin:2.5.0-M1:repackage (repackage) @ spring-docker ---
[INFO] Replacing main artifact with repackaged archive
[INFO]
[INFO] <<< spring-boot-maven-plugin:2.5.0-M1:build-image (default-cli) < package @ spring-docker <<<
[INFO]
[INFO]
[INFO] --- spring-boot-maven-plugin:2.5.0-M1:build-image (default-cli) @ spring-boot-docker ---
[INFO] Building image 'docker.io/jiangbo920827/spring-docker:1.0.0'
[INFO]
[INFO]  > Pulling builder image 'docker.io/paketobuildpacks/builder:base' 100%
[INFO]  > Pulled builder image 'paketobuildpacks/builder@sha256:c9141bd56f3e837a9d618c9e6748b8d5ab0783728a4924bbca41c7e8fb21fca5'
[INFO]  > Pulling run image 'docker.io/paketobuildpacks/run:base-cnb' 100%
[INFO]  > Pulled run image 'paketobuildpacks/run@sha256:79bd9986ae32d62e8339e602b61ca6288df8434d5e2972bed5d7644d6b8ed0d0'
[INFO]  > Executing lifecycle version v0.10.1
[INFO]  > Using build cache volume 'pack-cache-7107a48ebc7b.build'
[INFO]
[INFO]  > Running creator
[INFO]     [creator]     ===> DETECTING
[INFO]     [creator]     5 of 18 buildpacks participating
[INFO]     [creator]     paketo-buildpacks/ca-certificates   1.0.1
[INFO]     [creator]     paketo-buildpacks/bellsoft-liberica 6.0.0
[INFO]     [creator]     paketo-buildpacks/executable-jar    3.1.3
[INFO]     [creator]     paketo-buildpacks/dist-zip          2.2.2
[INFO]     [creator]     paketo-buildpacks/spring-boot       3.5.0
[INFO]     [creator]     ===> ANALYZING
[INFO]     [creator]     Previous image with name "docker.io/jiangbo920827/spring-docker:1.0.0" not found
[INFO]     [creator]     ===> RESTORING
[INFO]     [creator]     ===> BUILDING
[INFO]     [creator]
[INFO]     [creator]     Paketo CA Certificates Buildpack 1.0.1
[INFO]     [creator]       https://github.com/paketo-buildpacks/ca-certificates
[INFO]     [creator]       Launch Helper: Contributing to layer
[INFO]     [creator]         Creating /layers/paketo-buildpacks_ca-certificates/helper/exec.d/ca-certificates-helper
[INFO]     [creator]         Writing profile.d/helper
[INFO]     [creator]
[INFO]     [creator]     Paketo BellSoft Liberica Buildpack 6.0.0
[INFO]     [creator]       https://github.com/paketo-buildpacks/bellsoft-liberica
[INFO]     [creator]       Build Configuration:
[INFO]     [creator]         $BP_JVM_VERSION              8.*             the Java version
[INFO]     [creator]       Launch Configuration:
[INFO]     [creator]         $BPL_JVM_HEAD_ROOM           0               the headroom in memory calculation
[INFO]     [creator]         $BPL_JVM_LOADED_CLASS_COUNT  35% of classes  the number of loaded classes in memory calculation
[INFO]     [creator]         $BPL_JVM_THREAD_COUNT        250             the number of threads in memory calculation
[INFO]     [creator]         $JAVA_TOOL_OPTIONS                           the JVM launch flags
[INFO]     [creator]       BellSoft Liberica JRE 8.0.275: Contributing to layer
[INFO]     [creator]         Downloading from https://github.com/bell-sw/Liberica/releases/download/8u275+1/bellsoft-jre8u275+1-linux-amd64.tar.gz
[INFO]     [creator]         Verifying checksum
[INFO]     [creator]         Expanding to /layers/paketo-buildpacks_bellsoft-liberica/jre
[INFO]     [creator]         Adding 138 container CA certificates to JVM truststore
[INFO]     [creator]         Writing env.launch/BPI_APPLICATION_PATH.default
[INFO]     [creator]         Writing env.launch/BPI_JVM_CACERTS.default
[INFO]     [creator]         Writing env.launch/BPI_JVM_CLASS_COUNT.default
[INFO]     [creator]         Writing env.launch/BPI_JVM_EXT_DIR.default
[INFO]     [creator]         Writing env.launch/BPI_JVM_SECURITY_PROVIDERS.default
[INFO]     [creator]         Writing env.launch/JAVA_HOME.default
[INFO]     [creator]         Writing env.launch/MALLOC_ARENA_MAX.default
[INFO]     [creator]       Launch Helper: Contributing to layer
[INFO]     [creator]         Creating /layers/paketo-buildpacks_bellsoft-liberica/helper/exec.d/active-processor-count
[INFO]     [creator]         Creating /layers/paketo-buildpacks_bellsoft-liberica/helper/exec.d/java-opts
[INFO]     [creator]         Creating /layers/paketo-buildpacks_bellsoft-liberica/helper/exec.d/link-local-dns
[INFO]     [creator]         Creating /layers/paketo-buildpacks_bellsoft-liberica/helper/exec.d/memory-calculator
[INFO]     [creator]         Creating /layers/paketo-buildpacks_bellsoft-liberica/helper/exec.d/openssl-certificate-loader
[INFO]     [creator]         Creating /layers/paketo-buildpacks_bellsoft-liberica/helper/exec.d/security-providers-configurer
[INFO]     [creator]         Creating /layers/paketo-buildpacks_bellsoft-liberica/helper/exec.d/security-providers-classpath-8
[INFO]     [creator]         Writing profile.d/helper
[INFO]     [creator]       JVMKill Agent 1.16.0: Contributing to layer
[INFO]     [creator]         Downloading from https://github.com/cloudfoundry/jvmkill/releases/download/v1.16.0.RELEASE/jvmkill-1.16.0-RELEASE.so
[INFO]     [creator]         Verifying checksum
[INFO]     [creator]         Copying to /layers/paketo-buildpacks_bellsoft-liberica/jvmkill
[INFO]     [creator]         Writing env.launch/JAVA_TOOL_OPTIONS.append
[INFO]     [creator]         Writing env.launch/JAVA_TOOL_OPTIONS.delim
[INFO]     [creator]       Java Security Properties: Contributing to layer
[INFO]     [creator]         Writing env.launch/JAVA_SECURITY_PROPERTIES.default
[INFO]     [creator]         Writing env.launch/JAVA_TOOL_OPTIONS.append
[INFO]     [creator]         Writing env.launch/JAVA_TOOL_OPTIONS.delim
[INFO]     [creator]
[INFO]     [creator]     Paketo Executable JAR Buildpack 3.1.3
[INFO]     [creator]       https://github.com/paketo-buildpacks/executable-jar
[INFO]     [creator]         Writing env.launch/CLASSPATH.delim
[INFO]     [creator]         Writing env.launch/CLASSPATH.prepend
[INFO]     [creator]       Process types:
[INFO]     [creator]         executable-jar: java org.springframework.boot.loader.JarLauncher
[INFO]     [creator]         task:           java org.springframework.boot.loader.JarLauncher
[INFO]     [creator]         web:            java org.springframework.boot.loader.JarLauncher
[INFO]     [creator]
[INFO]     [creator]     Paketo Spring Boot Buildpack 3.5.0
[INFO]     [creator]       https://github.com/paketo-buildpacks/spring-boot
[INFO]     [creator]       Creating slices from layers index
[INFO]     [creator]         dependencies
[INFO]     [creator]         spring-boot-loader
[INFO]     [creator]         snapshot-dependencies
[INFO]     [creator]         application
[INFO]     [creator]       Launch Helper: Contributing to layer
[INFO]     [creator]         Creating /layers/paketo-buildpacks_spring-boot/helper/exec.d/spring-cloud-bindings
[INFO]     [creator]         Writing profile.d/helper
[INFO]     [creator]       Web Application Type: Contributing to layer
[INFO]     [creator]         Servlet web application detected
[INFO]     [creator]         Writing env.launch/BPL_JVM_THREAD_COUNT.default
[INFO]     [creator]       Spring Cloud Bindings 1.7.0: Contributing to layer
[INFO]     [creator]         Downloading from https://repo.spring.io/release/org/springframework/cloud/spring-cloud-bindings/1.7.0/spring-cloud-bindings-1.7.0.jar
[INFO]     [creator]         Verifying checksum
[INFO]     [creator]         Copying to /layers/paketo-buildpacks_spring-boot/spring-cloud-bindings
[INFO]     [creator]       4 application slices
[INFO]     [creator]       Image labels:
[INFO]     [creator]         org.opencontainers.image.title
[INFO]     [creator]         org.opencontainers.image.version
[INFO]     [creator]         org.springframework.boot.spring-configuration-metadata.json
[INFO]     [creator]         org.springframework.boot.version
[INFO]     [creator]     ===> EXPORTING
[INFO]     [creator]     Adding layer 'paketo-buildpacks/ca-certificates:helper'
[INFO]     [creator]     Adding layer 'paketo-buildpacks/bellsoft-liberica:helper'
[INFO]     [creator]     Adding layer 'paketo-buildpacks/bellsoft-liberica:java-security-properties'
[INFO]     [creator]     Adding layer 'paketo-buildpacks/bellsoft-liberica:jre'
[INFO]     [creator]     Adding layer 'paketo-buildpacks/bellsoft-liberica:jvmkill'
[INFO]     [creator]     Adding layer 'paketo-buildpacks/executable-jar:class-path'
[INFO]     [creator]     Adding layer 'paketo-buildpacks/spring-boot:helper'
[INFO]     [creator]     Adding layer 'paketo-buildpacks/spring-boot:spring-cloud-bindings'
[INFO]     [creator]     Adding layer 'paketo-buildpacks/spring-boot:web-application-type'
[INFO]     [creator]     Adding 5/5 app layer(s)
[INFO]     [creator]     Adding layer 'launcher'
[INFO]     [creator]     Adding layer 'config'
[INFO]     [creator]     Adding layer 'process-types'
[INFO]     [creator]     Adding label 'io.buildpacks.lifecycle.metadata'
[INFO]     [creator]     Adding label 'io.buildpacks.build.metadata'
[INFO]     [creator]     Adding label 'io.buildpacks.project.metadata'
[INFO]     [creator]     Adding label 'org.opencontainers.image.title'
[INFO]     [creator]     Adding label 'org.opencontainers.image.version'
[INFO]     [creator]     Adding label 'org.springframework.boot.spring-configuration-metadata.json'
[INFO]     [creator]     Adding label 'org.springframework.boot.version'
[INFO]     [creator]     Setting default process type 'web'
[INFO]     [creator]     *** Images (ba6729253f7d):
[INFO]     [creator]           docker.io/jiangbo920827/spring-docker:1.0.0
[INFO]
[INFO] Successfully built image 'docker.io/jiangbo920827/spring-docker:1.0.0'
[INFO]
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time:  49:10 min
[INFO] Finished at: 2021-01-23T22:58:53+08:00
[INFO] ------------------------------------------------------------------------
```

使用 Spring Boot 自带的方式是最简单构建镜像的方式，但是由于网络原因，很可能会在构建的过程中，下载文件失败。
可以参考：[【Docker】Spring Boot 和 Docker][2]，里面介绍了生成镜像的方式和生成镜像的大小。

### 查看镜像

```powershell
PS C:\Users\JiangBo> docker image ls jiangbo920827/spring-docker
REPOSITORY                    TAG       IMAGE ID       CREATED        SIZE
jiangbo920827/spring-docker   1.0.0     ba6729253f7d   41 years ago   244MB
```

### 上传到远程仓库

```
PS C:\Users\JiangBo> docker login -u jiangbo920827
PS C:\Users\JiangBo> docker push jiangbo920827/spring-docker:1.0.0
```

除了使用命令，可以直接使用界面推送：
![push-spring-docker.png][3]

### 在远程仓库查看

![spring-docker-remote.png][4]

## 总结

介绍了编写 Spring Boot 返回主机名，并将程序打包成镜像上传到 Docker 远程仓库。

[1]: https://www.cnblogs.com/jiangbo44/p/12637389.html
[2]: https://www.cnblogs.com/jiangbo44/p/14056909.html
[3]: push-spring-docker.png
[4]: spring-docker-remote.png

## 附录