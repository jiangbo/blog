# 【JavaWeb】HttpServletResponse 的输出流是否应该关闭

> 以下内容参考 [stackoverflow][1]

先说结论，通过 HttpServletResponse 得到的 PrintWriter 和 ServletOutputStream 不需要手动关闭。

## 回答

Normally you should not close the stream. 
The servlet container will automatically close the stream 
after the servlet is finished running as part of the servlet request life-cycle.

通常您不应该关闭流。在 servlet 完成生命周期之后，servlet 容器会自动关闭流。

---

For instance, if you closed the stream it would not be available if you implemented a Filter.

举个例子，如果你关闭了流的话，在你实现的 Filter 中就不能再使用了。

---

Having said all that, if you do close it nothing bad will happen as long as you don't try to use it again.

说了这么多，如果你不再使用流了的话，即使关闭了也不会有任何坏影响。

---

更多信息可以参考原文。

[1]: https://stackoverflow.com/questions/1159168/should-one-call-close-on-httpservletresponse-getoutputstream-getwriter

