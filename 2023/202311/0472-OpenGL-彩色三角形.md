# 0472-OpenGL-彩色三角形

## 环境

- Time 2024-04-21
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach-glfw-opengl-example/tree/main>
2. <https://learnopengl-cn.github.io/01%20Getting%20started/03%20Hello%20Window/>
3. <https://www.bilibili.com/video/BV1Ni4y1o7Au>

### 目标

显示一个彩色三角形。

## vertex.glsl

```glsl
#version 330 core
layout (location = 0) in vec4 aPos;
layout (location = 1) in vec3 aColor;
out vec4 Color;
void main()
{
    Color = vec4(aColor, 1.0);
    gl_Position = aPos;
}
```

## fragment.glsl

```glsl
#version 330 core
out vec4 FragColor;

in vec4 Color;

void main()
{
    FragColor = Color;
}
```

## shader.zig

无变化

## main.zig

```zig
const std = @import("std");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const shader = @import("shader.zig");

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;
const vertices = [_]f32{ -0.5, -0.5, 0.5, -0.5, 0, 0.5 };
const colors = [_]f32{
    1.0, 0.0, 0.0, //
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0,
};
const indices = [_]u32{ 0, 1, 2 };

const vertexSource: [:0]const u8 = @embedFile("vertex.glsl");
const fragmentSource: [:0]const u8 = @embedFile("fragment.glsl");

pub fn main() void {
    const window = initWindow();
    defer deinit(window);

    glfw.makeContextCurrent(window);
    defer glfw.makeContextCurrent(null);
    glfw.swapInterval(1);

    if (!glProcs.init(glfw.getProcAddress)) glfwPanic();

    gl.makeProcTableCurrent(&glProcs);
    defer gl.makeProcTableCurrent(null);

    const program = shader.init(vertexSource, fragmentSource);
    defer gl.DeleteProgram(program);

    // VBO 顶点缓冲对象
    var vbos: [2]c_uint = undefined;
    gl.GenBuffers(vbos.len, &vbos);
    defer gl.DeleteBuffers(vbos.len, &vbos);

    // 位置
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[0]);
    gl.BufferData(gl.ARRAY_BUFFER, @sizeOf(@TypeOf(vertices)), &vertices, gl.STATIC_DRAW);
    // 颜色
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[1]);
    gl.BufferData(gl.ARRAY_BUFFER, @sizeOf(@TypeOf(colors)), &colors, gl.STATIC_DRAW);

    // EBO 索引缓冲对象
    var ebo: c_uint = undefined;
    gl.GenBuffers(1, (&ebo)[0..1]);
    defer gl.DeleteBuffers(1, (&ebo)[0..1]);
    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, ebo);
    gl.BufferData(gl.ELEMENT_ARRAY_BUFFER, @sizeOf(@TypeOf(indices)), &indices, gl.STATIC_DRAW);

    // VAO 顶点数组对象
    var vao: c_uint = undefined;
    gl.GenVertexArrays(1, (&vao)[0..1]);
    gl.BindVertexArray(vao);
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[0]);
    gl.EnableVertexAttribArray(0);
    gl.VertexAttribPointer(0, 2, gl.FLOAT, gl.FALSE, 2 * @sizeOf(f32), 0);

    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[1]);
    gl.EnableVertexAttribArray(1);
    gl.VertexAttribPointer(1, 3, gl.FLOAT, gl.FALSE, 3 * @sizeOf(f32), 0);

    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, ebo);

    gl.UseProgram(program);

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gl.COLOR_BUFFER_BIT);

        gl.DrawElements(gl.TRIANGLES, 3, gl.UNSIGNED_INT, 0);

        window.swapBuffers();
    }
}

fn initWindow() glfw.Window {
    glfw.setErrorCallback(logGlfwError);

    if (!glfw.init(.{})) glfwPanic();

    return glfw.Window.create(640, 480, "学习 OpenGL", null, null, .{
        .context_version_major = gl.info.version_major,
        .context_version_minor = gl.info.version_minor,
        .opengl_profile = .opengl_core_profile,
    }) orelse glfwPanic();
}

fn deinit(window: glfw.Window) void {
    window.destroy();
    glfw.terminate();
}
```

## 效果

![彩色三角形][1]

## 总结

绘制一个彩色的三角形。

[1]: images/opengl04.png

## 附录
