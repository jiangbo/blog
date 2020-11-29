# 【Docker】解析器指令之 escape

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## escape

```Dockerfile
# escape=\ (backslash)
```

Or

```Dockerfile
# escape=` (backtick)
```

The `escape` directive sets the character used to escape characters in a `Dockerfile`. If not specified, the default escape character is `\`.

`escape` 指令设置用于转义 `Dockerfile` 中的字符的字符。如果未指定，则默认转义字符为 `\`。

The escape character is used both to escape characters in a line, and to escape a newline. This allows a `Dockerfile` instruction to span multiple lines. Note that regardless of whether the `escape` parser directive is included in a `Dockerfile`, _escaping is not performed in a `RUN` command, except at the end of a line._

转义字符用于转义一行中的字符和转义换行符。这允许 Dockerfile 指令跨越多行。请注意，无论 Dockerfile 中是否包含 `escape` 解析器指令，都不会在 RUN 命令中执行转义，除非在行末。

Setting the escape character to `` ` `` is especially useful on `Windows`, where `\` is the directory path separator. `` ` `` is consistent with [Windows PowerShell](https://technet.microsoft.com/en-us/library/hh847755.aspx).

在 Windows 上将转义符设置为 `` ` `` 特别有用，其中 `\` 是目录路径分隔符。 `` ` `` 与 [Windows PowerShell](https://technet.microsoft.com/zh-cn/library/hh847755.aspx) 一致。

Consider the following example which would fail in a non-obvious way on `Windows`. The second `\` at the end of the second line would be interpreted as an escape for the newline, instead of a target of the escape from the first `\`. Similarly, the `\` at the end of the third line would, assuming it was actually handled as an instruction, cause it be treated as a line continuation. The result of this dockerfile is that second and third lines are considered a single instruction:

考虑以下示例，该示例将在 Windows 上以非明显的方式失败。第二行末尾的第二个 `\` 将被解释为换行符的转义，而不是第一个 `\` 的转义目标。类似地，假设实际将其作为指令处理，则第三行末尾的 `\` 会导致将其视为行的延续。该 dockerfile 的结果是第二和第三行被视为一条指令：

```Dockerfile
FROM microsoft/nanoserver
COPY testfile.txt c:\\
RUN dir c:\
```

Results in:

```powershell
PS C:\John> docker build -t cmd .
Sending build context to Docker daemon 3.072 kB
Step 1/2 : FROM microsoft/nanoserver
 ---> 22738ff49c6d
Step 2/2 : COPY testfile.txt c:\RUN dir c:
GetFileAttributesEx c:RUN: The system cannot find the file specified.
PS C:\John>
```

One solution to the above would be to use `/` as the target of both the `COPY` instruction, and `dir`. However, this syntax is, at best, confusing as it is not natural for paths on `Windows`, and at worst, error prone as not all commands on `Windows` support `/` as the path separator.

上面情况的一种解决方案是将 `/` 用作 `COPY` 指令和 `dir` 的目标。但是，这种语法充其量是令人困惑的，因为 Windows 上的路径并不自然，更糟糕的是容易出错，因为并非 Windows 上的所有命令都支持 `/` 作为路径分隔符。

By adding the `escape` parser directive, the following `Dockerfile` succeeds as expected with the use of natural platform semantics for file paths on `Windows`:

通过添加 `escape` 解析器指令，以下 `Dockerfile` 可以通过在 Windows 上为文件路径使用自然平台语义而成功完成：

```Dockerfile
# escape=`

FROM microsoft/nanoserver
COPY testfile.txt c:\
RUN dir c:\
```

Results in:

```powershell
PS C:\John> docker build -t succeeds --no-cache=true .
Sending build context to Docker daemon 3.072 kB
Step 1/3 : FROM microsoft/nanoserver
 ---> 22738ff49c6d
Step 2/3 : COPY testfile.txt c:\
 ---> 96655de338de
Removing intermediate container 4db9acbb1682
Step 3/3 : RUN dir c:\
 ---> Running in a2c157f842f5
 Volume in drive C has no label.
 Volume Serial Number is 7E6D-E0F7

 Directory of c:\

10/05/2016  05:04 PM             1,894 License.txt
10/05/2016  02:22 PM    <DIR>          Program Files
10/05/2016  02:14 PM    <DIR>          Program Files (x86)
10/28/2016  11:18 AM                62 testfile.txt
10/28/2016  11:20 AM    <DIR>          Users
10/28/2016  11:20 AM    <DIR>          Windows
           2 File(s)          1,956 bytes
           4 Dir(s)  21,259,096,064 bytes free
 ---> 01c7f3bef04f
Removing intermediate container a2c157f842f5
Successfully built 01c7f3bef04f
PS C:\John>
```

## 总结

介绍了 Dockerfile 指令解析器的 `escape` 用法。