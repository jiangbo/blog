# Idea：手动安装更新包

## 环境

1. idea 2020.1
2. AdoptOpenJDK 1.8.0_242

## 背景

由于 idea 在无网的环境下，不能在线更新 idea，同时又不希望每次都完全下载 idea，所以需要手动下载更新包进行手动安装。

## 步骤

### 下载更新包

得到更新包的地址，目前没有找到单独下载的页面，可以从相同版本 idea 在线更新时，得到 idea 更新包的地址。比如：

```
https://download.jetbrains.8686c.com/idea/IU-201.6668.13-201.6668.60-patch-jbr11-win.jar
```

下载得到安装包。

### 安装更新

将下载的更新包放到当前目录，执行命令

```cmd
java -jar .\IU-201.6668.13-201.6668.60-patch-jbr11-win.jar "C:\\work\\software\\idea"
```

其中最后一个参数是 idea 的安装路径，需要根据实际情况填写。执行命令后，输出：

```text
PS C:\Users\jiangbo> cd C:\work\ideaupdate
PS C:\work\ideaupdate> java -jar .\IU-201.6668.13-201.6668.60-patch-jbr11-win.jar "C:\\work\\software\\idea"
Extracting patch file...
From IntelliJ IDEA (build 201.6668.13) to IntelliJ IDEA (build 201.6668.60)
Validating installation...
Backing up files...
Applying patch...
Cleaning up...
PS C:\work\ideaupdate>
```

如果没有看到明显的错误提示，安装就成功了。