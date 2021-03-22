# Maven：org/codehaus/plexus/interpolation/InterpolationException

## 环境

1. jdk 6
2. maven 3.2.5

## 原因

在项目打包的过程中，由于版本问题出错了错误，提示：

```text
Failed to execute goal org.apache.maven.plugins:maven-jar-plugin:2.3.2:jar (default-jar) on project xxx: Execution default-jar of goal org.apache.maven.plugins:maven-jar-plugin:2.3.2:jar failed: A required class was missing while executing org.apache.maven.plugins:maven-jar-plugin:2.3.2:jar: org/codehaus/plexus/interpolation/InterpolationException
```

## 分析

从错误提示来看，不是很清楚发生了什么问题。

### 删除重下

首先试试将 maven-jar-plugin 这个插件在本地仓库中删除，再重新下载一次，看看是否解决问题。

### 调整版本

从错误提示来看，使用的版本是 2.3.2，将起升级到 2.4 版本后，该错误不再发生。

## 详细错误

### 未开启 debug

```text
[INFO] --- maven-resources-plugin:2.6:testResources (default-testResources) @ xxx ---
[INFO] Not copying test resources
[INFO] 
[INFO] --- maven-compiler-plugin:3.3:testCompile (default-testCompile) @ xxx ---
[INFO] Not compiling test sources
[INFO] 
[INFO] --- maven-surefire-plugin:2.9:test (default-test) @ xxx ---
[INFO] Tests are skipped.
[INFO] 
[INFO] --- maven-jar-plugin:2.3.2:jar (default-jar) @ xxx ---
[INFO] ------------------------------------------------------------------------
[INFO] BUILD FAILURE
[INFO] ------------------------------------------------------------------------
[INFO] Total time: 20.063 s
[INFO] Finished at: 2020-04-02T12:31:46+08:00
[INFO] Final Memory: 38M/501M
[INFO] ------------------------------------------------------------------------
[ERROR] Failed to execute goal org.apache.maven.plugins:maven-jar-plugin:2.3.2:jar (default-jar) on project xxx: Execution default-jar of goal org.apache.maven.plugins:maven-jar-plugin:2.3.2:jar failed: A required class was missing while executing org.apache.maven.plugins:maven-jar-plugin:2.3.2:jar: org/codehaus/plexus/interpolation/InterpolationException
[ERROR] -----------------------------------------------------
[ERROR] realm =    plugin>org.apache.maven.plugins:maven-jar-plugin:2.3.2
[ERROR] strategy = org.codehaus.plexus.classworlds.strategy.SelfFirstStrategy
[ERROR] urls[0] = file:/D:/repository/maven/org/apache/maven/plugins/maven-jar-plugin/2.3.2/maven-jar-plugin-2.3.2.jar
[ERROR] urls[1] = file:/D:/repository/maven/junit/junit/3.8.1/junit-3.8.1.jar
[ERROR] urls[2] = file:/D:/repository/maven/org/apache/maven/maven-archiver/2.4.2/maven-archiver-2.4.2.jar
[ERROR] urls[3] = file:/D:/repository/maven/org/codehaus/plexus/plexus-archiver/2.0.1/plexus-archiver-2.0.1.jar
[ERROR] urls[4] = file:/D:/repository/maven/org/codehaus/plexus/plexus-io/2.0.1/plexus-io-2.0.1.jar
[ERROR] urls[5] = file:/D:/repository/maven/commons-lang/commons-lang/2.1/commons-lang-2.1.jar
[ERROR] urls[6] = file:/D:/repository/maven/org/codehaus/plexus/plexus-utils/3.0/plexus-utils-3.0.jar
[ERROR] Number of foreign imports: 1
[ERROR] import: Entry[import  from realm ClassRealm[maven.api, parent: null]]
[ERROR] 
[ERROR] -----------------------------------------------------: org.codehaus.plexus.interpolation.InterpolationException
[ERROR] -> [Help 1]
[ERROR] 
[ERROR] To see the full stack trace of the errors, re-run Maven with the -e switch.
[ERROR] Re-run Maven using the -X switch to enable full debug logging.
[ERROR] 
[ERROR] For more information about the errors and possible solutions, please read the following articles:
[ERROR] [Help 1] http://cwiki.apache.org/confluence/display/MAVEN/PluginContainerException
```

### 开启 debug

```text
realm =    plugin>org.apache.maven.plugins:maven-jar-plugin:2.3.2
strategy = org.codehaus.plexus.classworlds.strategy.SelfFirstStrategy
urls[0] = file:/D:/repository/maven/org/apache/maven/plugins/maven-jar-plugin/2.3.2/maven-jar-plugin-2.3.2.jar
urls[1] = file:/D:/repository/maven/junit/junit/3.8.1/junit-3.8.1.jar
urls[2] = file:/D:/repository/maven/org/apache/maven/maven-archiver/2.4.2/maven-archiver-2.4.2.jar
urls[3] = file:/D:/repository/maven/org/codehaus/plexus/plexus-archiver/2.0.1/plexus-archiver-2.0.1.jar
urls[4] = file:/D:/repository/maven/org/codehaus/plexus/plexus-io/2.0.1/plexus-io-2.0.1.jar
urls[5] = file:/D:/repository/maven/commons-lang/commons-lang/2.1/commons-lang-2.1.jar
urls[6] = file:/D:/repository/maven/org/codehaus/plexus/plexus-utils/3.0/plexus-utils-3.0.jar
Number of foreign imports: 1
import: Entry[import  from realm ClassRealm[maven.api, parent: null]]

at org.apache.maven.plugin.DefaultBuildPluginManager.executeMojo(DefaultBuildPluginManager.java:166)
at org.apache.maven.lifecycle.internal.MojoExecutor.execute(MojoExecutor.java:208)
... 19 more
Caused by: org.apache.maven.plugin.PluginContainerException: A required class was missing while executing org.apache.maven.plugins:maven-jar-plugin:2.3.2:jar: org/codehaus/plexus/interpolation/InterpolationException
realm =    plugin>org.apache.maven.plugins:maven-jar-plugin:2.3.2
strategy = org.codehaus.plexus.classworlds.strategy.SelfFirstStrategy
urls[0] = file:/D:/repository/maven/org/apache/maven/plugins/maven-jar-plugin/2.3.2/maven-jar-plugin-2.3.2.jar
urls[1] = file:/D:/repository/maven/junit/junit/3.8.1/junit-3.8.1.jar
urls[2] = file:/D:/repository/maven/org/apache/maven/maven-archiver/2.4.2/maven-archiver-2.4.2.jar
urls[3] = file:/D:/repository/maven/org/codehaus/plexus/plexus-archiver/2.0.1/plexus-archiver-2.0.1.jar
urls[4] = file:/D:/repository/maven/org/codehaus/plexus/plexus-io/2.0.1/plexus-io-2.0.1.jar
urls[5] = file:/D:/repository/maven/commons-lang/commons-lang/2.1/commons-lang-2.1.jar
urls[6] = file:/D:/repository/maven/org/codehaus/plexus/plexus-utils/3.0/plexus-utils-3.0.jar
Number of foreign imports: 1
import: Entry[import  from realm ClassRealm[maven.api, parent: null]]

at org.apache.maven.plugin.DefaultBuildPluginManager.executeMojo(DefaultBuildPluginManager.java:164)
... 20 more
Caused by: java.lang.NoClassDefFoundError: org/codehaus/plexus/interpolation/InterpolationException
    at org.apache.maven.plugin.jar.AbstractJarMojo.createArchive(AbstractJarMojo.java:188)
    at org.apache.maven.plugin.jar.AbstractJarMojo.execute(AbstractJarMojo.java:235)
    at org.apache.maven.plugin.DefaultBuildPluginManager.executeMojo(DefaultBuildPluginManager.java:132)
    ... 20 more
Caused by: java.lang.ClassNotFoundException: org.codehaus.plexus.interpolation.InterpolationException
    at org.codehaus.plexus.classworlds.strategy.SelfFirstStrategy.loadClass(SelfFirstStrategy.java:50)
    at org.codehaus.plexus.classworlds.realm.ClassRealm.unsynchronizedLoadClass(ClassRealm.java:271)
    at org.codehaus.plexus.classworlds.realm.ClassRealm.loadClass(ClassRealm.java:254)
    at org.codehaus.plexus.classworlds.realm.ClassRealm.loadClass(ClassRealm.java:239)
    ... 23 more
[ERROR] 
[ERROR] 
[ERROR] For more information about the errors and possible solutions, please read the following articles:
[ERROR] [Help 1] http://cwiki.apache.org/confluence/display/MAVEN/PluginContainerException
```