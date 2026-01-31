# 1144-怪物战争-ImGui 框架支持

## 目标

将 ImGui 框架引入到这个教程项目中。

## 环境

- Time 2026-01-31
- Zig 0.15.1

## 参考

1. <https://www.bilibili.com/video/BV1jf9XYQEhW>

## 想法

sokol 默认支持了 ImGui 框架，索引直接加入进来就可以了。

## main.zig

```zig
const std = @import("std");
const builtin = @import("builtin");
const zhu = @import("zhu");

const gui = @import("gui.zig");

var vertexBuffer: []zhu.batch.Vertex = undefined;
var commandBuffer: [16]zhu.batch.Command = undefined;
var soundBuffer: [20]zhu.audio.Sound = undefined;

pub fn init() void {
    zhu.audio.init(44100 / 2, &soundBuffer);

    vertexBuffer = zhu.assets.oomAlloc(zhu.batch.Vertex, 5000);
    zhu.graphics.frameStats(true);
    zhu.batch.init(vertexBuffer, &commandBuffer);
    gui.init();
}

pub fn event(ev: *const zhu.window.Event) void {
    gui.event(ev);
}

pub fn frame(delta: f32) void {
    gui.update(delta);

    zhu.batch.beginDraw(.black);
    gui.draw();
    zhu.batch.endDraw();
}

pub fn deinit() void {
    gui.deinit();
    zhu.assets.free(vertexBuffer);
    zhu.audio.deinit();
}

pub fn main() void {
    var allocator: std.mem.Allocator = undefined;
    var debugAllocator: std.heap.DebugAllocator(.{}) = undefined;
    if (builtin.mode == .Debug) {
        debugAllocator = std.heap.DebugAllocator(.{}).init;
        allocator = debugAllocator.allocator();
    } else {
        allocator = std.heap.c_allocator;
    }

    defer if (builtin.mode == .Debug) {
        _ = debugAllocator.deinit();
    };

    zhu.window.run(allocator, .{
        .title = "怪物战争",
        .size = .xy(1280, 720),
        .scaleEnum = .integer,
    });
}
```

## gui.zig

所有关于 ImGui 的东西，都打算放到 gui 模块。

```zig
const std = @import("std");
const sk = @import("sokol");
const zhu = @import("zhu");

const gui = @import("cimgui");

pub fn init() void {
    sk.imgui.setup(.{ .logger = .{ .func = sk.log.func } });

    const io = gui.igGetIO();
    const font = io.*.Fonts;
    const range = gui.ImFontAtlas_GetGlyphRangesChineseSimplifiedCommon(font);
    const chineseFont = gui.ImFontAtlas_AddFontFromFileTTF(font, //
        "assets/VonwaonBitmap-16px.ttf", 16, null, range);

    if (chineseFont == null) @panic("failed to load font");
    io.*.FontDefault = chineseFont;
}

pub fn event(ev: *const zhu.window.Event) void {
    _ = sk.imgui.handleEvent(ev.*);
}

var flag: bool = true;
pub fn update(delta: f32) void {
    sk.imgui.newFrame(.{
        .width = sk.app.width(),
        .height = sk.app.height(),
        .delta_time = delta,
        .dpi_scale = sk.app.dpiScale(),
    });

    gui.igShowDemoWindow(&flag);

    if (gui.igBegin("怪物战争", &flag, gui.ImGuiWindowFlags_None)) {
        _ = gui.igText("ImGui 版本：%s", gui.IMGUI_VERSION);
    }

    gui.igEnd();
}

pub fn draw() void {
    sk.imgui.render();
}

pub fn deinit() void {
    sk.imgui.shutdown();
}
```

## 效果

![ImGui 框架支持][1]

[1]: images/怪物战争02.png

## 附录
