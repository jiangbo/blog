# 【Docker】.dockerignore 文件

参考教程：https://docs.docker.com/engine/reference/builder/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## .dockerignore file

Before the docker CLI sends the context to the docker daemon, it looks for a file named `.dockerignore` in the root directory of the context. If this file exists, the CLI modifies the context to exclude files and directories that match patterns in it. This helps to avoid unnecessarily sending large or sensitive files and directories to the daemon and potentially adding them to images using `ADD` or `COPY`.

在 docker CLI 将上下文发送到 docker 守护程序之前，它将在上下文的根目录中查找名为 `.dockerignore` 的文件。如果此文件存在，则 CLI 会修改上下文以排除与其中的模式匹配的文件和目录。这有助于避免不必要地将较大或敏感的文件和目录发送到守护程序，并避免使用 `ADD` 或 `COPY` 将它们添加到镜像中。

The CLI interprets the `.dockerignore` file as a newline-separated list of patterns similar to the file globs of Unix shells. For the purposes of matching, the root of the context is considered to be both the working and the root directory. For example, the patterns `/foo/bar` and `foo/bar` both exclude a file or directory named `bar` in the `foo` subdirectory of `PATH` or in the root of the git repository located at `URL`. Neither excludes anything else.

CLI 将 `.dockerignore` 文件解释为以换行符分隔的模式列表，类似于 Unix shell 的文件组。为了匹配，上下文的根目录被认为是工作目录和根目录。例如，模式 `/foo/bar` 和 `foo/bar` 都排除位于 `PATH` 的 foo 子目录中或位于 URL 的 git 仓库根目录中名为 bar 的文件或目录。两者都不排除其他任何东西。

If a line in `.dockerignore` file starts with `#` in column 1, then this line is considered as a comment and is ignored before interpreted by the CLI.

如果 `.dockerignore` 文件中的行以第 1 列以 `＃` 号开头，则该行被视为注释，并在 CLI 解释之前被忽略。

Here is an example `.dockerignore` file:

这是一个示例 `.dockerignore` 文件：

```text
# comment
*/temp*
*/*/temp*
temp?
```

This file causes the following build behavior:

此文件导致以下生成行为：

| Rule | Behavior |
| --- | --- |
| `# comment` | Ignored. |
| `*/temp*` | Exclude files and directories whose names start with `temp` in any immediate subdirectory of the root. For example, the plain file `/somedir/temporary.txt` is excluded, as is the directory `/somedir/temp`. |
| `*/*/temp*` | Exclude files and directories starting with `temp` from any subdirectory that is two levels below the root. For example, `/somedir/subdir/temporary.txt` is excluded. |
| `temp?` | Exclude files and directories in the root directory whose names are a one-character extension of `temp`. For example, `/tempa` and `/tempb` are excluded. |

|规则|行为|
| --- | --- |
| `# comment` | 忽略 |
| `*/temp*` |在根目录的任何直接子目录中排除名称以 `temp` 开头的文件和目录。例如，排除了纯文件 `/somedir/temporary.txt`，以及目录 `/somedir/temp`。 |
| `*/*/temp*`|从根目录以下两级的任何子目录中排除以 `temp` 开头的文件和目录。例如，排除 `/somedir/subdir/temporary.txt`。 |
| `temp?`|排除根目录中名称为 `temp` 的一个字符扩展名的文件和目录。例如，排除 `/tempa` 和` /tempb`。 |

Matching is done using Go’s [filepath.Match](http://golang.org/pkg/path/filepath#Match) rules. A preprocessing step removes leading and trailing whitespace and eliminates `.` and `..` elements using Go’s [filepath.Clean](http://golang.org/pkg/path/filepath/#Clean). Lines that are blank after preprocessing are ignored.

使用 Go 的 [filepath.Match](http://golang.org/pkg/path/filepath#Match) 规则进行匹配。预处理步骤使用 Go 的 [filepath.Clean](http://golang.org/pkg/path/filepath/#Clean) 删除前导和尾随空格，并消除 `.` 和 `..` 元素。预处理后空白的行将被忽略。

Beyond Go’s filepath.Match rules, Docker also supports a special wildcard string `**` that matches any number of directories (including zero). For example, `**/*.go` will exclude all files that end with `.go` that are found in all directories, including the root of the build context.

除了 Go 的 filepath.Match 规则之外，Docker 还支持特殊的通配符字符串 `**`，该字符串可匹配任意数量的目录（包括零个）。例如，`**/*.go` 将排除在所有目录（包括构建上下文的根目录）中找到的所有以 `.go` 结尾的文件。

Lines starting with `!` (exclamation mark) can be used to make exceptions to exclusions. The following is an example `.dockerignore` file that uses this mechanism:

以 `!`（感叹号）开头的行可用于排除例外。以下是使用此机制的示例 `.dockerignore` 文件：

```gitignore
*.md
!README.md
```

All markdown files _except_ `README.md` are excluded from the context.

除了 `README.md` 之外的所有 markdown 文件均从上下文中排除。

The placement of `!` exception rules influences the behavior: the last line of the `.dockerignore` that matches a particular file determines whether it is included or excluded. Consider the following example:

例外规则的位置会影响行为：匹配特定文件的 `.dockerignore` 的最后一行确定是包含还是排除该文件。考虑以下示例：

```gitignore
*.md
!README*.md
README-secret.md
```

No markdown files are included in the context except README files other than `README-secret.md`.

忽略除了 README 开头的所有 .md 文件，但是 `README-secret.md` 文件例外。

Now consider this example:

现在考虑以下示例：

```gitignore
*.md
README-secret.md
!README*.md
```

All of the README files are included. The middle line has no effect because `!README*.md` matches `README-secret.md` and comes last.

忽略除了 README 开头的所有 .md 文件，`README-secret.md` 文件也会忽略。

You can even use the `.dockerignore` file to exclude the `Dockerfile` and `.dockerignore` files. These files are still sent to the daemon because it needs them to do its job. But the `ADD` and `COPY` instructions do not copy them to the image.

您甚至可以使用 `.dockerignore` 文件来排除 `Dockerfile` 和 `.dockerignore` 文件。这些文件仍被发送到守护程序，因为它需要它们来完成其工作。但是，`ADD` 和 `COPY` 指令不会将它们复制到镜像中。

Finally, you may want to specify which files to include in the context, rather than which to exclude. To achieve this, specify `*` as the first pattern, followed by one or more `!` exception patterns.

最后，您可能想要指定要包含在上下文中的文件，而不是要排除的文件。为此，将 `*` 指定为第一个模式，然后指定一个或多个 `!` 异常模式。

> **Note**
> 
> For historical reasons, the pattern `.` is ignored.

> **注意**
>
>由于历史原因，模式 `.` 将被忽略。


## 练习

### 建立 Dockerfile

```Dockerfile
FROM busybox
COPY tex*.txt /
```

### 建立 .dockerignore 文件

```sh
[root@master env]# cat .dockerignore
text.txt
```

### 文件目录结构

```sh
[root@master env]# tree .
.
├── Dockerfile
├── text2.txt
└── text.txt

0 directories, 3 files
```

### 构建镜像并查看结果

```sh
[root@master env]# docker build -t jiangbo:0.0.1 .
Sending build context to Docker daemon  3.584kB
Step 1/2 : FROM busybox
 ---> dc3bacd8b5ea
Step 2/2 : COPY tex*.txt /
 ---> 53183cf1cf16
Successfully built 53183cf1cf16
Successfully tagged jiangbo:0.0.1
[root@master env]# docker run -it jiangbo:0.0.1
/ # ls
bin        dev        etc        home       proc       root       sys        text2.txt  tmp        usr        var
/ #
```

## 总结

介绍了 Dockerfile 中 `.dockerignore` 文件的作用以及使用方式。