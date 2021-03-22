# Spring：Spring Framework 2020 年维护路线图


 > 原文地址：[Spring Framework maintenance roadmap in 2020 (including 4.3 EOL)][1]
---
Dear Spring community,

With [Spring Framework 5.2.2 and 5.1.12][2] being available now, 
let me take the opportunity to provide an update on the maintenance roadmap in 2020.

亲爱的 Spring 社区，

随着 [Spring Framework 5.2.2 和 5.1.12][2] 的面世，让我借此机会提供 2020 年维护路线图的更新。

---

Most importantly, Spring Framework 4.3.x and therefore Spring Framework 4 overall 
will reach its end-of-life next year: 
Our EOL cut-off is December 31st, 2020, with no further support on 4.3.x beyond that point. 
At the same time, we are also phasing out 5.0.x and 5.1.x for good.

最重要的一点是，Spring Framework 4.3.x 以及整个 Spring Framework 4 将在明年到期：
我们的截止日期是 2020 年 12 月 31 日，在此之后，我们将不再对 4.3.x 提供任何支持。
同时，我们还将逐步淘汰 5.0.x 和 5.1.x 。

---

As for planned releases, first up is a full round in mid January: 
with 5.2.3 and 5.1.13 accompanied by 5.0.16 and 4.3.26. 
The latter are the last maintenance releases in the 5.0.x and 4.3.x lines. 
We may do critical patches in case of vulnerabilities 
but otherwise no further releases are planned in those lines until the final cut-off at the end of 2020.

至于计划发布的版本，首先是一月中旬的全面发布：
5.2.3 和 5.1.13 以及 5.0.16 和 4.3.26 。
后者是 5.0.x 和 4.3.x 系列中的最后一个维护版本。
如果有漏洞，我们可能会进行重要补丁修复，
否则直到 2020 年底最终截止之前，这些产品都不会计划进一步发布。

---

The 5.1.x line will receive general maintenance throughout 2020 but just with infrequent releases (~ once a quarter). 
The primary active branch is 5.2.x now, with frequent releases planned (~ once a month), 
supporting not only the current Spring Boot 2.2 generation but also the upcoming Spring Boot 2.3 (April 2020) for its entire lifetime.

5.1.x 系列将在 2020 年获得常规维护，但很少发布（每季度一次）。
现在的主要活动分支是 5.2.x，计划进行频繁发布（每月一次），
不仅支持当前的 Spring Boot 2.2 一代，而且还支持即将到来的 Spring Boot 2.3（ 2020 年 4 月）。

---

Last but not least, the next Spring Framework feature release will be 5.3, with GA planned for October 2020, aligned with Spring Boot 2.4.
This is expected to be the last 5.x feature branch, enjoying an extended support life. 
We intend to wrap up all 5.x themes for 5.3, including our runtime tuning efforts (startup performance, memory consumption).

最后并且最重要的是，下一个 Spring Framework 功能版本将是 5.3 ，GA计划在 2020 年 10 月与 Spring Boot 2.4 保持一致。
预计这将是最后的 5.x 功能分支，具有更长的支持寿命。
我们打算完成 5.3 的所有 5.x 主题，包括我们的运行时调整工作（启动性能，内存消耗）。

---

TL;DR: By the end of 2020, the only active Spring Framework branches are going to be 5.2.x and the then-new 5.3.x line (which is expected to receive long-term support, effectively superseding 4.3.x from that perspective). 
Please upgrade to 5.2+ at your earliest convenience.

最后：到 2020 年底，唯一活跃的 Spring 框架分支将是 5.2.x ，和最新的 5.3.x 产品线
（预计将获得长期支持，从此有效取代 4.3.x ）。请尽快升级到 5.2+ 。

Cheers,
Juergen

---

P.S.: See the [versions page][3] for support timeframes and the [milestones page][4] for release dates.
附注：请参阅版本页面以获取支持时间表，并查看里程碑页面以了解发布日期。

[1]: https://spring.io/blog/2019/12/03/spring-framework-maintenance-roadmap-in-2020-including-4-3-eol
[2]: https://spring.io/blog/2019/12/03/spring-framework-5-2-2-and-5-1-12-available-now
[3]: https://github.com/spring-projects/spring-framework/wiki/Spring-Framework-Versions#supported-versions
[4]: https://github.com/spring-projects/spring-framework/milestones