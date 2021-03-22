# Spring：Spring Cloud 2020 年维护路线图

 > 原文地址：[Spring Cloud Roadmap and Hoxton and Greenwich Maintenance and EOL Announcements][1]

With the recent [Spring Framework][2] and [Spring Boot][3] maintenance and roadmap posts, 
the Spring Cloud team is taking the opportunity to provide some insight to our future roadmap as well as the lifetimes of the Greenwich and Hoxton release trains.


随便最近 [Spring Framework][2] 和 [Spring Boot][3] 发布维护和路线图，
Spring Cloud 团队将借此机会对我们的未来路线图提供一些说明，包括 Greenwich 和 Hoxton 版本的生命周期。

## Spring Cloud Ilford

We would like to announce our next major release, Spring Cloud Ilford. 
This will be the first major release since the release of Spring Cloud Finchley, which provided support for Spring Boot 2.x and Spring Framework 5.x. 
By making Ilford a major release, it will allow us to remove modules that have entered [maintenance mode][4] and to complete the simplification of the release train [announced earlier this year][5]. 
It will also allow us to do some API refactoring that may introduce breaking changes.

Ilford will be released following Spring Framework 5.3 and Spring Boot 2.4 sometime in Q4 2020.

## Spring Cloud Ilford

我们想宣布我们的下一个主要版本是 Spring Cloud Ilford。
这将是自 Spring Cloud Finchley 发布以来的第一个主要版本，该版本提供了对 Spring Boot 2.x 和S pring Framework 5.x 的支持。
通过使 Ilford 成为主要发行版，它将使我们能够删除进入[维护模式][4]的模块，并完成对[今年早些时候][5]宣布的发行版的简化。
这还将使我们能够进行一些破坏性的 API 重构。

Ilford 将在 2020 年第四季度的某个时候随 Spring Framework 5.3 和 Spring Boot 2.4 一起发布。

## Spring Cloud Hoxton

According to the [Pivotal Open Source Support Policy][6], major releases are supported for a period of three years. 
Finchley was first released in June 2018. Therefore, Hoxton, a minor release of the Finchley release train, will be supported until the end of June 2021 with regular releases. 
Starting in July 2021 Hoxton will enter a special maintenance period where only critical bug fixes and security patches will be released until the end of December 2021.

This will allow a full year of overlap between major versions.

Spring Boot 2.3.x, scheduled for release in Q2 2020, will be supported by a Hoxton Service Release soon after the Spring Boot release.

根据 [Pivotal 开源支持政策][6]，主要版本的支持期限为三年。 
Finchley于 2018 年 6 月首次发布。因此，Finchley 版本的次要版本 Hoxton 将得到常规版本的支持，直到 2021 年 6 月底。
从 2021 年 7 月开始，Hoxton 将进入一个特殊的维护期，在此期间，将仅发布关键的错误修复程序和安全补丁，直到 2021 年 12 月底。

这样可以让主要版本之间可以有整整一年的重叠时间。

计划于 2020 年第二季度发布的 Spring Boot 2.3.x 将在 Spring Boot 发布后不久获得 Hoxton Service Release 支持。

## Spring Cloud Greenwich

Greenwich, a minor release of the Finchley release train, was released in January 2019. Its last regular Service Release will be in Jan 2020. After that, it will enter a special maintenance period where only critical bug fixes and security patches will be released until the end of December 2020. 
This will be the last release train to support Spring Boot 2.1.x.

Please see the [release train milestones][7] to track future releases and the new [Supported Versions][8] page to see version lifetime.

## Spring Cloud Greenwich

Finchley 版本的次要版本 Hoxton 于 2019 年 1 月发布。其最后的常规服务发行版于 2020 年 1 月。
此后，它将进入一个特殊的维护期，在此期间，仅发行关键的错误修复程序和安全补丁，直到到 2020 年 12 月结束。
这将是最后一个支持 Spring Boot 2.1.x 的发行版本。

请查看[发行里程碑][7]以跟踪将来的发布，并查看新的[支持的版本][8]页面以查看版本的生命周期。

[1]: https://spring.io/blog/2019/12/23/spring-cloud-roadmap-and-hoxton-and-greenwich-maintenance-and-eol-announcements
[2]: https://www.cnblogs.com/jiangbo44/p/12913453.html
[3]: https://www.cnblogs.com/jiangbo44/p/12919971.html
[4]: https://spring.io/blog/2018/12/12/spring-cloud-greenwich-rc1-available-now#spring-cloud-netflix-projects-entering-maintenance-mode
[5]: https://spring.io/blog/2019/07/24/simplifying-the-spring-cloud-release-train
[6]: https://pivotal.io/support/oss
[7]: https://github.com/spring-cloud/spring-cloud-release/milestones?direction=asc&sort=due_date&state=open
[8]: https://github.com/spring-cloud/spring-cloud-release/wiki/Supported-Versions