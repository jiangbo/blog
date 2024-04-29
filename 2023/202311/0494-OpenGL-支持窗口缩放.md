# 0494-OpenGL-支持窗口缩放

## 环境

- Time 2024-04-29
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://learnopengl-cn.github.io/01%20Getting%20started/03%20Hello%20Window/>

### 目标

支持窗口的缩放，对 main.zig 进行了重构。

## main.zig

```zig
const std = @import("std");
const zlm = @import("zlm");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const resource = @import("resource.zig");
const zstbi = @import("zstbi");

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    glfw.setErrorCallback(logGlfwError);
    if (!glfw.init(.{})) glfwPanic();
    defer glfw.terminate();
    const window = glfw.Window.create(800, 600, "学习 OpenGL", null, null, .{
        .context_version_major = gl.info.version_major,
        .context_version_minor = gl.info.version_minor,
        .opengl_profile = .opengl_core_profile,
    }) orelse glfwPanic();
    defer window.destroy();

    glfw.makeContextCurrent(window);
    defer glfw.makeContextCurrent(null);
    glfw.swapInterval(1);
    glfw.Window.setFramebufferSizeCallback(window, windowChange);

    if (!glProcs.init(glfw.getProcAddress)) glfwPanic();

    gl.makeProcTableCurrent(&glProcs);
    defer gl.makeProcTableCurrent(null);
    gl.Enable(gl.BLEND);
    gl.BlendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    zstbi.init(gpa.allocator());
    defer zstbi.deinit();
    resource.init(gpa.allocator());
    defer resource.deinit();

    const vs: [:0]const u8 = @embedFile("shader/vertex.glsl");
    const fs: [:0]const u8 = @embedFile("shader/fragment.glsl");
    const shader = try resource.loadShader("shader", vs, fs);

    var renderer = @import("renderer.zig").Renderer{ .shader = shader };
    renderer.initRenderData();

    const face = "awesomeface.png";
    const texture = try resource.loadTexture(face, "assets/" ++ face);

    const projection = zlm.Mat4.createOrthogonal(0, 1, 1, 0, -1, 1);
    shader.setUniformMatrix4fv("projection", &projection.fields[0][0]);
    shader.setUniform1i("image", 0);

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0, 0, 0, 0);
        gl.Clear(gl.COLOR_BUFFER_BIT);

        renderer.draw(texture);

        window.swapBuffers();
    }
}

fn windowChange(_: glfw.Window, w: u32, h: u32) void {
    gl.Viewport(0, 0, @as(c_int, @intCast(w)), @as(c_int, @intCast(h)));
}
```

## 效果

![封装窗口][1]

## 总结

实现了窗口的缩放时，纹理一起进行缩放。

[1]: images/opengl20.gif

## 附录
