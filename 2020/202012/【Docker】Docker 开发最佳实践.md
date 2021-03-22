# 【Docker】Docker 开发最佳实践

参考教程：https://docs.docker.com/develop/dev-best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## 如何保持镜像较小

Small images are faster to pull over the network and faster to load into memory when starting containers or services. There are a few rules of thumb to keep image size small:

小镜像在启动容器或服务时更快地通过网络传输，更快地加载到内存中。有一些经验法则可以使镜像尺寸较小：

- Start with an appropriate base image. For instance, if you need a JDK, consider basing your image on the official `openjdk` image, rather than starting with a generic `ubuntu` image and installing `openjdk` as part of the Dockerfile.

- 从适当的基础镜像开始。例如，如果您需要 JDK，请考虑将镜像基于正式的 `openjdk` 镜像，而不是从通过 `ubuntu` 镜像开始并在 Dockerfile 中安装 `openjdk`。
    
- [Use multistage builds](https://docs.docker.com/develop/develop-images/multistage-build/). For instance, you can use the `maven` image to build your Java application, then reset to the `tomcat` image and copy the Java artifacts into the correct location to deploy your app, all in the same Dockerfile. This means that your final image doesn’t include all of the libraries and dependencies pulled in by the build, but only the artifacts and the environment needed to run them.

- [使用多阶段构建](https://docs.docker.com/develop/develop-images/multistage-build/)。例如，您可以使用 `maven` 镜像来构建 Java 应用程序，然后重置为 `tomcat` 镜像并将 Java 构件复制到正确的位置以部署您的应用程序，所有这些都在同一 Dockerfile 中。这意味着您的最终映像不包括构建所引入的所有库和依赖项，而仅包括运行它们所需的工件和环境。
    
    - If you need to use a version of Docker that does not include multistage builds, try to reduce the number of layers in your image by minimizing the number of separate `RUN` commands in your Dockerfile. You can do this by consolidating multiple commands into a single `RUN` line and using your shell’s mechanisms to combine them together. Consider the following two fragments. The first creates two layers in the image, while the second only creates one.

    - 如果您需要使用不包含多阶段构建的 Docker 版本，请尝试通过最小化 Dockerfile 中单独的 `RUN` 命令的数量来减少镜像中的层数。为此，您可以将多个命令合并到一条 RUN 行中，并使用 Shell 的机制将它们组合在一起。考虑以下两个片段。第一层在图像中创建两层，而第二层仅创建一层。
        
        ```Dockerfile
        RUN apt-get -y update
        RUN apt-get install -y python
        ```
        
        ```Dockerfile
        RUN apt-get -y update && apt-get install -y python
        ```
        
- If you have multiple images with a lot in common, consider creating your own [base image](https://docs.docker.com/develop/develop-images/baseimages/) with the shared components, and basing your unique images on that. Docker only needs to load the common layers once, and they are cached. This means that your derivative images use memory on the Docker host more efficiently and load more quickly.

- 如果多个镜像有许多共同点，请考虑使用共享组件创建自己的[基础镜像](https://docs.docker.com/develop/develop-images/baseimages/)，然后定制你自己的镜像。 Docker 只需要加载一次公共层，然后将它们缓存。这意味着您的派生镜像将更有效地使用 Docker 主机上的内存，并更快地加载。
    
- To keep your production image lean but allow for debugging, consider using the production image as the base image for the debug image. Additional testing or debugging tooling can be added on top of the production image.

- 为使生产镜像保持精简但允许进行调试，请考虑将生产镜像用作调试镜像的基础镜像。可以在生产镜像的顶部添加其他测试或调试工具。
    
- When building images, always tag them with useful tags which codify version information, intended destination (`prod` or `test`, for instance), stability, or other information that is useful when deploying the application in different environments. Do not rely on the automatically-created `latest` tag.
    
- 构建镜像时，请始终使用有用的标签对其进行标记，这些标签可将版本信息，预期的目标（例如 `prod` 或者 `test`），稳定性或其他在不同环境中部署应用程序时有用的信息进行编码。不要依赖自动创建的 `latest` 标签。
    

## 在哪里以及如何保存应用程序数据

- **Avoid** storing application data in your container’s writable layer using [storage drivers](https://docs.docker.com/storage/storagedriver/select-storage-driver/). This increases the size of your container and is less efficient from an I/O perspective than using volumes or bind mounts.
- **避免**使用[存储驱动程序](https://docs.docker.com/storage/storagedriver/select-storage-driver/)将应用程序数据存储在容器的可写层中。这会增加容器的大小，并且从 I/O 角度来看，效率不如使用卷或绑定挂载。

- Instead, store data using [volumes](https://docs.docker.com/storage/volumes/).
- 而是使用 [volumes](https://docs.docker.com/storage/volumes/) 存储数据。

- One case where it is appropriate to use [bind mounts](https://docs.docker.com/storage/bind-mounts/) is during development, when you may want to mount your source directory or a binary you just built into your container. For production, use a volume instead, mounting it into the same location as you mounted a bind mount during development.
- 一种在开发过程中适合使用 [bind mounts](https://docs.docker.com/storage/bind-mounts/) 的情况，这时您可能想挂载源目录或刚刚构建的二进制文件放入您的容器中。对于生产，请改用卷，将其安装到与开发期间安装绑定安装结构相同的位置。

- For production, use [secrets](https://docs.docker.com/engine/swarm/secrets/) to store sensitive application data used by services, and use [configs](https://docs.docker.com/engine/swarm/configs/) for non-sensitive data such as configuration files. If you currently use standalone containers, consider migrating to use single-replica services, so that you can take advantage of these service-only features.
- 对于生产，请使用 [secrets](https://docs.docker.com/engine/swarm/secrets/) 存储服务使用的敏感应用程序数据，并使用 [configs](https://docs.docker.com/engine/swarm/configs/) 获取非敏感数据，例如配置文件。如果当前使用独立容器，请考虑迁移以使用单一副本服务，以便可以利用这些仅服务功能。

## 使用 CI/CD 进行测试和部署

- When you check in a change to source control or create a pull request, use [Docker Hub](https://docs.docker.com/docker-hub/builds/) or another CI/CD pipeline to automatically build and tag a Docker image and test it.
- 当您签入对源代码管理的更改或创建请求请求时，请使用 [Docker Hub](https://docs.docker.com/docker-hub/builds/) 或其他 CI/CD 管道自动生成并标记 Docker 镜像并对其进行测试。
    
- Take this even further by requiring your development, testing, and security teams to [sign images](https://docs.docker.com/engine/reference/commandline/trust/) before they are deployed into production. This way, before an image is deployed into production, it has been tested and signed off by, for instance, development, quality, and security teams.
- 通过要求您的开发，测试和安全团队在部署到生产中之前，对它们进行[签名](https://docs.docker.com/engine/reference/commandline/trust/)来进一步做到这一点。这样，在将镜像部署到生产中之前，它已由开发，质量和安全团队进行了测试和签名。
    

## 开发和生产环境中的差异

| Development | Production |
| --- | --- |
| Use bind mounts to give your container access to your source code. | Use volumes to store container data. |
| Use Docker Desktop for Mac or Docker Desktop for Windows. | Use Docker Engine, if possible with [userns mapping](https://docs.docker.com/engine/security/userns-remap/) for greater isolation of Docker processes from host processes. |
| Don’t worry about time drift. | Always run an NTP client on the Docker host and within each container process and sync them all to the same NTP server. If you use swarm services, also ensure that each Docker node syncs its clocks to the same time source as the containers. |

## 总结

介绍了在开发过程中，使用 Docker 的最佳实践。