# 0550-GStreamer-C 语言环境

## 环境

- Time 2024-06-18
- WSL2 Ubuntu 22.04.4 LTS
- GNU Make 4.3

## 前言

### 说明

参考资料：

1. <https://gstreamer.freedesktop.org/documentation/installing/on-linux.html>
2. <https://gstreamer.freedesktop.org/documentation/tutorials/basic/hello-world.html>

### 目标

建立 GStreamer C 语言环境。

## 安装依赖

```sh
apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio
```

## CMakeLists.txt

```txt
cmake_minimum_required(VERSION 3.12)

project(gstreamer)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GST REQUIRED gstreamer-1.0)

add_executable(gstreamer main.c)
target_include_directories(gstreamer PRIVATE ${GST_INCLUDE_DIRS})
target_link_libraries(gstreamer ${GST_LIBRARIES})
```

## main.c

```c
#include <gst/gst.h>

#ifdef __APPLE__
#include <TargetConditionals.h>
#endif

int tutorial_main(int argc, char *argv[])
{
    GstElement *pipeline;
    GstBus *bus;
    GstMessage *msg;

    /* Initialize GStreamer */
    g_print("argc %d\n", argc);
    for (int i = 0; i < argc; i++)
    {
        g_print("%s", argv[i]);
    }
    gst_init(&argc, &argv);

    /* Build the pipeline */
    pipeline =
        gst_parse_launch("playbin uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm",
                         NULL);

    /* Start playing */
    gst_element_set_state(pipeline, GST_STATE_PLAYING);

    /* Wait until error or EOS */
    bus = gst_element_get_bus(pipeline);
    msg =
        gst_bus_timed_pop_filtered(bus, GST_CLOCK_TIME_NONE,
                                   GST_MESSAGE_ERROR | GST_MESSAGE_EOS);

    /* See next tutorial for proper error message handling/parsing */
    if (GST_MESSAGE_TYPE(msg) == GST_MESSAGE_ERROR)
    {
        g_printerr("An error occurred! Re-run with the GST_DEBUG=*:WARN "
                   "environment variable set for more details.\n");
    }

    /* Free resources */
    gst_message_unref(msg);
    gst_object_unref(bus);
    gst_element_set_state(pipeline, GST_STATE_NULL);
    gst_object_unref(pipeline);
    return 0;
}

int main(int argc, char *argv[])
{
#if defined(__APPLE__) && TARGET_OS_MAC && !TARGET_OS_IPHONE
    return gst_macos_main((GstMainFunc)tutorial_main, argc, argv, NULL);
#else
    return tutorial_main(argc, argv);
#endif
}
```

## 构建和运行

```sh
mkdir build && cd build
cmake ..
make -j4
./gstreamer
```

## 效果

![播放视频][1]

## 总结

使用 GStreamer 播放视频。

[1]: images/gstreamer01.png

## 附录
