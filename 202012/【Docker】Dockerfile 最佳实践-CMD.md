# 【Docker】Dockerfile 最佳实践-CMD

参考教程：https://docs.docker.com/develop/develop-images/dockerfile_best-practices/

## 环境

1. virtual box 6.1
2. centos 7.8
3. docker 19.03

## CMD

The `CMD` instruction should be used to run the software contained in your image, along with any arguments. `CMD` should almost always be used in the form of `CMD ["executable", "param1", "param2"…]`. Thus, if the image is for a service, such as Apache and Rails, you would run something like `CMD ["apache2","-DFOREGROUND"]`. Indeed, this form of the instruction is recommended for any service-based image.

应该使用 `CMD` 指令来运行镜像中包含的软件以及所有参数。`CMD` 几乎应始终以 `CMD ["executable", "param1", "param2"…]` 的形式使用。因此，如果镜像用于服务，例如 Apache 和 Rails，则将运行诸如 `CMD ["apache2","-DFOREGROUND"]`之类的内容。实际上，建议将这种形式的指令用于任何基于服务的镜像。

In most other cases, `CMD` should be given an interactive shell, such as bash, python and perl. For example, `CMD ["perl", "-de0"]`, `CMD ["python"]`, or `CMD ["php", "-a"]`. Using this form means that when you execute something like `docker run -it python`, you’ll get dropped into a usable shell, ready to go. `CMD` should rarely be used in the manner of `CMD ["param", "param"]` in conjunction with [`ENTRYPOINT`](https://docs.docker.com/engine/reference/builder/#entrypoint), unless you and your expected users are already quite familiar with how `ENTRYPOINT` works.

在大多数其他情况下，应该给 `CMD` 一个交互式外壳，例如 bash，python 和 perl。例如，`CMD ["perl", "-de0"]`，`CMD ["python"]` 或 `CMD ["php", "-a"]`。使用此种格式意味着执行 `docker run -it python` 之类的操作时，您将进入可用的 shell 中，随时可以使用。除非您和您的预期用户已经非常熟悉 `ENTRYPOINT` 的工作原理，否则 `CMD` 很少以 `CMD ["param", "param"]` 的形式与 `ENTRYPOINT` 结合使用。

## 总结

介绍了 Dockerfile 的 CMD 指令的最佳实践。