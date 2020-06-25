# Git：基础操作

## 前言

本文介绍 Git 最基本的命令和概念。其中包括初始化仓库、跟踪文件、暂存、提交以及和推送远程仓库。本文所有的内容参考:

1. [Git权威指南][1]。

## 仓库目录

>仓库（Repository）：Git 用来保存项目的元数据和对象数据库的地方（ .git 目录）。

有两种取得 Git 项目仓库的方法。 第一种是在本地直接初始化一个仓库；第二种是从一个服务器克隆一个现有的 Git 仓库。

### 直接初始化

如果想使用 Git 来对现有的项目进行管理，只需要进入该项目目录并输入：

```git
git init
```

在 E 盘建立一个空的文件夹 myRepo, 并在该目录下进行初始化。

```git
$ git init
Initialized empty Git repository in E:/myRepo/.git/
```

> `git init` 可以初始化一个仓库

该命令将创建一个名为 .git （看不到可以试试显示隐藏文件）的子目录，这个子目录含有初始化的 Git 仓库中所有的必须文件，这些文件是 Git 仓库的骨干。

### 远程克隆

>如需要代理，使用 `git config --global http.proxy` 命令。  
>例如：`git config --global http.proxy http://172.17.18.80:8080`

```git
$ git clone https://github.com/jiangbo920827/blog.git myblog
Cloning into 'myblog'...
remote: Counting objects: 3, done.
remote: Compressing objects: 100% (2/2), done.
remote: Total 3 (delta 0), reused 0 (delta 0), pack-reused 0
Unpacking objects: 100% (3/3), done.
```

>`git clone` 可以克隆一个远程仓库

克隆的仓库的名称默认为远程仓库名称，即 blog，如果想自定义名称可以在克隆地址后加上自定义名称。

## 工作空间

> 工作空间（Working Tree）：从项目的某个版本独立提取出来的内容（除 .git 的目录）。

在工作空间中做的操作，都能被 git 跟踪到。  
新建一个测试文件

```git
echo "this is a test!" > test.txt
```

查看当前工作空间里文件的状态

```git
$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
Untracked files:
  (use "git add <file>..." to include in what will be committed)

        test.txt

nothing added to commit but untracked files present (use "git add" to track)
```

> `git status` 命令可以查看文件处于什么状态

## 暂存区

> 暂存区（Staging Area）：一个文件，保存了下次将提交的文件列表信息，一般在 Git 仓库目录中。

如果想要将一个文件加入到暂存区，可以使用 `git add` 命令。

```git
$ git add test.txt

$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
Changes to be committed:
  (use "git reset HEAD <file>..." to unstage)

        new file:   test.txt
```

可以看到 test.txt已经加入到了暂存区。

> `git add` 命令可以将文件加入暂存区

## 本地仓库

将文件加入到暂存区后，可以提交到本地仓库进行保存，提交到本地仓库的内容，只包含暂存区的内容。

> 如果提示 *** Please tell me who you are. 则需要配置相应的用户名和邮箱

```git
  git config --global user.email "you@example.com"
  git config --global user.name "Your Name"
```

git 的配置可以分为三种：

1. 系统级别（system）：存在于 Git 安装目录下（config），使用 `git config --system` 配置
2. 全局级别（global）：存在于 用户 Home 目录下（.gitconfig），使用`git config --global`配置
3. 项目级别：存在于该项目的仓库目录下（config），使用 `git config` 配置

```git
$ git commit -m "commit a test file"
[master 368d93e] commit a test file
 Date: Mon Jun 12 14:37:55 2017 +0800
 1 file changed, 1 insertion(+)
 create mode 100644 test.txt
```

> `git commit` 命令可以将暂存的内容提交到本地仓库

## 远程仓库

> 远程仓库一般存在于服务上，不存在工作空间，只有一个裸（bare）仓库

在将本地修改的内容提交到本地仓库后，可以将这些提交推送到远程仓库，与其他人共享这些修改。

```git
$ git push
Counting objects: 3, done.
Delta compression using up to 4 threads.
Compressing objects: 100% (2/2), done.
Writing objects: 100% (3/3), 291 bytes | 0 bytes/s, done.
Total 3 (delta 0), reused 0 (delta 0)
To https://github.com/jiangbo920827/blog.git
   20ae201..368d93e  master -> master
```

> `git push` 命令可以将本地仓库的提交推送至远程仓库

[1]:https://git-scm.com/book/en/v2 "Git权威指南"
