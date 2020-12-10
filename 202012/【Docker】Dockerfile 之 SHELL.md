# 【Docker】Dockerfile 之 SHELL

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## SHELL

```Dockerfile
SHELL ["executable", "parameters"]
```

The `SHELL` instruction allows the default shell used for the _shell_ form of commands to be overridden. The default shell on Linux is `["/bin/sh", "-c"]`, and on Windows is `["cmd", "/S", "/C"]`. The `SHELL` instruction _must_ be written in JSON form in a Dockerfile.

`SHELL` 指令允许覆盖用于 _shell_ 形式的命令的默认 shell。在 Linux 上，默认 shell 是 `["/bin/sh", "-c"]`，而在 Windows 上，默认 shell 是 `["cmd", "/S", "/C"]`。 `SHELL` 指令必须以 JSON 格式写入 Dockerfile 中。

The `SHELL` instruction is particularly useful on Windows where there are two commonly used and quite different native shells: `cmd` and `powershell`, as well as alternate shells available including `sh`.

`SHELL` 指令在 Windows 上特别有用，在 Windows 上有两个常用且完全不同的本机shell：`cmd` 和 `powershell`，以及可用的替代 shell，包括 `sh`。

The `SHELL` instruction can appear multiple times. Each `SHELL` instruction overrides all previous `SHELL` instructions, and affects all subsequent instructions. For example:

`SHELL` 指令可以出现多次。每条 `SHELL` 指令都将覆盖所有先前的 `SHELL` 指令，并影响所有后续指令。例如：

```Dockerfile
FROM microsoft/windowsservercore

# Executed as cmd /S /C echo default
RUN echo default

# Executed as cmd /S /C powershell -command Write-Host default
RUN powershell -command Write-Host default

# Executed as powershell -command Write-Host hello
SHELL ["powershell", "-command"]
RUN Write-Host hello

# Executed as cmd /S /C echo hello
SHELL ["cmd", "/S", "/C"]
RUN echo hello
```

The following instructions can be affected by the `SHELL` instruction when the _shell_ form of them is used in a Dockerfile: `RUN`, `CMD` and `ENTRYPOINT`.

当它们的 _shell_ 形式在 Dockerfile 中使用时，以下指令可能会受到 `SHELL` 指令的影响：`RUN`，`CMD` 和 `ENTRYPOINT`。

The following example is a common pattern found on Windows which can be streamlined by using the `SHELL` instruction:

以下示例是 Windows 上常见的模式，可以通过使用 `SHELL` 指令进行精简：

```Dockerfile
RUN powershell -command Execute-MyCmdlet -param1 "c:\foo.txt"
```

The command invoked by docker will be:
docker 调用的命令将是：

```Dockerfile
cmd /S /C powershell -command Execute-MyCmdlet -param1 "c:\foo.txt"
```

This is inefficient for two reasons. First, there is an un-necessary cmd.exe command processor (aka shell) being invoked. Second, each `RUN` instruction in the _shell_ form requires an extra `powershell -command` prefixing the command.

这效率低下有两个原因。首先，有一个不必要的 cmd.exe 命令处理器（也称为 shell）被调用。其次，_shell_ 格式的每条 `RUN` 指令都需要在命令前面加上一个额外的 `powershell -command`。

To make this more efficient, one of two mechanisms can be employed. One is to use the JSON form of the RUN command such as:

为了使其更有效，可以采用两种机制之一。一种是使用 RUN 命令的 JSON 形式，例如：

```Dockerfile
RUN ["powershell", "-command", "Execute-MyCmdlet", "-param1 \"c:\\foo.txt\""]
```

While the JSON form is unambiguous and does not use the un-necessary cmd.exe, it does require more verbosity through double-quoting and escaping. The alternate mechanism is to use the `SHELL` instruction and the _shell_ form, making a more natural syntax for Windows users, especially when combined with the `escape` parser directive:

尽管 JSON 形式是明确的，并且不使用不必要的 cmd.exe，但它确实需要通过双引号和转义来实现更多的详细信息。另一种机制是使用 `SHELL` 指令和 _shell_ 形式，使 Windows 用户的语法更自然，尤其是与  `escape` 解析器指令结合使用时：

```Dockerfile
# escape=`

FROM microsoft/nanoserver
SHELL ["powershell","-command"]
RUN New-Item -ItemType Directory C:\Example
ADD Execute-MyCmdlet.ps1 c:\example\
RUN c:\example\Execute-MyCmdlet -sample 'hello world'
```

Resulting in:

```powershell
PS E:\docker\build\shell> docker build -t shell .
Sending build context to Docker daemon 4.096 kB
Step 1/5 : FROM microsoft/nanoserver
 ---> 22738ff49c6d
Step 2/5 : SHELL powershell -command
 ---> Running in 6fcdb6855ae2
 ---> 6331462d4300
Removing intermediate container 6fcdb6855ae2
Step 3/5 : RUN New-Item -ItemType Directory C:\Example
 ---> Running in d0eef8386e97


    Directory: C:\


Mode                LastWriteTime         Length Name
----                -------------         ------ ----
d-----       10/28/2016  11:26 AM                Example


 ---> 3f2fbf1395d9
Removing intermediate container d0eef8386e97
Step 4/5 : ADD Execute-MyCmdlet.ps1 c:\example\
 ---> a955b2621c31
Removing intermediate container b825593d39fc
Step 5/5 : RUN c:\example\Execute-MyCmdlet 'hello world'
 ---> Running in be6d8e63fe75
hello world
 ---> 8e559e9bf424
Removing intermediate container be6d8e63fe75
Successfully built 8e559e9bf424
PS E:\docker\build\shell>
```

The `SHELL` instruction could also be used to modify the way in which a shell operates. For example, using `SHELL cmd /S /C /V:ON|OFF` on Windows, delayed environment variable expansion semantics could be modified.

`SHELL` 指令还可用于修改外壳的运行方式。例如，在 Windows 上使用 `SHELL cmd /S /C /V:ON|OFF`，可以修改延迟的环境变量扩展语义。

The `SHELL` instruction can also be used on Linux should an alternate shell be required such as `zsh`, `csh`, `tcsh` and others.

如果需要备用外壳，例如 `zsh`，`csh`，`tcsh`等，也可以在 Linux 上使用 `SHELL` 指令。

## 总结

介绍了 Dockerfile 中 SHELL 指令的用法和注意事项。