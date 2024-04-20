# 0469-OpenGL-VAO 显示正方形

## 环境

- Time 2024-04-20
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

定义顶点数组对象（VAO），并使用兼容模式（opengl_compat_profile）显示正方形。

## main

```zig
pub fn main() void {
    const window = initWindow();
    defer deinit(window);

    glfw.makeContextCurrent(window);
    defer glfw.makeContextCurrent(null);
    glfw.swapInterval(1);

    if (!glProcs.init(glfw.getProcAddress)) glfwPanic();

    gl.makeProcTableCurrent(&glProcs);
    defer gl.makeProcTableCurrent(null);

    const program = createShaderProgram();
    defer gl.DeleteProgram(program);

    // VBO 顶点缓冲对象
    var vbo: c_uint = undefined;
    gl.GenBuffers(1, (&vbo)[0..1]);
    gl.BindBuffer(gl.ARRAY_BUFFER, vbo);
    gl.BufferData(gl.ARRAY_BUFFER, @sizeOf(@TypeOf(vertices)), &vertices, gl.STATIC_DRAW);

    gl.EnableVertexAttribArray(0);
    gl.VertexAttribPointer(0, 2, gl.FLOAT, gl.FALSE, 2 * @sizeOf(f32), 0);

    // VAO 顶点数组对象
    var vao: c_uint = undefined;
    gl.GenBuffers(1, (&vao)[0..1]);
    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, vao);
    gl.BufferData(gl.ELEMENT_ARRAY_BUFFER, @sizeOf(@TypeOf(indices)), &indices, gl.STATIC_DRAW);

    gl.UseProgram(program);

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gl.COLOR_BUFFER_BIT);

        gl.DrawElements(gl.TRIANGLES, 6, gl.UNSIGNED_INT, 0);

        window.swapBuffers();
    }
}
```

## 效果

![VAO 显示正方形][1]

## 总结

使用顶点数组对象显示正方形。

[1]: images/opengl03.png

## 附录

### 源码

```zig
const std = @import("std");
const glfw = @import("mach-glfw");
const gl = @import("gl");

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    errorPanic(glfw.getErrorString());
}

fn errorPanic(message: ?[]const u8) noreturn {
    @panic(message orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;
const vertices = [_]f32{ -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5 };
const indices = [_]u32{ 0, 1, 2, 2, 3, 0 };

const vertexShaderSource: [:0]const u8 =
    \\#version 330 core
    \\layout (location = 0) in vec4 aPos;
    \\
    \\void main()
    \\{
    \\    gl_Position = aPos;
    \\}
;

const fragmentShaderSource: [:0]const u8 =
    \\#version 330 core
    \\out vec4 FragColor;
    \\
    \\void main()
    \\{
    \\    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    \\}
;

pub fn main() void {
    const window = initWindow();
    defer deinit(window);

    glfw.makeContextCurrent(window);
    defer glfw.makeContextCurrent(null);
    glfw.swapInterval(1);

    if (!glProcs.init(glfw.getProcAddress)) glfwPanic();

    gl.makeProcTableCurrent(&glProcs);
    defer gl.makeProcTableCurrent(null);

    const program = createShaderProgram();
    defer gl.DeleteProgram(program);

    // VBO 顶点缓冲对象
    var vbo: c_uint = undefined;
    gl.GenBuffers(1, (&vbo)[0..1]);
    gl.BindBuffer(gl.ARRAY_BUFFER, vbo);
    gl.BufferData(gl.ARRAY_BUFFER, @sizeOf(@TypeOf(vertices)), &vertices, gl.STATIC_DRAW);

    gl.EnableVertexAttribArray(0);
    gl.VertexAttribPointer(0, 2, gl.FLOAT, gl.FALSE, 2 * @sizeOf(f32), 0);

    // VAO 顶点数组对象
    var vao: c_uint = undefined;
    gl.GenBuffers(1, (&vao)[0..1]);
    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, vao);
    gl.BufferData(gl.ELEMENT_ARRAY_BUFFER, @sizeOf(@TypeOf(indices)), &indices, gl.STATIC_DRAW);

    gl.UseProgram(program);

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gl.COLOR_BUFFER_BIT);

        gl.DrawElements(gl.TRIANGLES, 6, gl.UNSIGNED_INT, 0);

        window.swapBuffers();
    }
}

fn initWindow() glfw.Window {
    glfw.setErrorCallback(logGlfwError);

    if (!glfw.init(.{})) glfwPanic();

    return glfw.Window.create(640, 480, "学习 OpenGL", null, null, .{
        .context_version_major = gl.info.version_major,
        .context_version_minor = gl.info.version_minor,
        .opengl_profile = .opengl_compat_profile,
    }) orelse glfwPanic();
}

fn createShaderProgram() c_uint {
    var success: c_int = undefined;
    var logBuffer: [512:0]u8 = undefined;
    // 顶点着色器
    const vertexShader = gl.CreateShader(gl.VERTEX_SHADER);
    if (vertexShader == 0) errorPanic("create vertex shader failed");
    defer gl.DeleteShader(vertexShader);
    gl.ShaderSource(vertexShader, 1, (&vertexShaderSource.ptr)[0..1], null);
    gl.CompileShader(vertexShader);
    gl.GetShaderiv(vertexShader, gl.COMPILE_STATUS, &success);
    if (success == gl.FALSE) {
        gl.GetShaderInfoLog(vertexShader, logBuffer.len, null, &logBuffer);
        errorPanic(std.mem.sliceTo(&logBuffer, 0));
    }

    // 片段着色器
    const fragmentShader = gl.CreateShader(gl.FRAGMENT_SHADER);
    if (fragmentShader == 0) errorPanic("create fragment shader failed");
    defer gl.DeleteShader(fragmentShader);
    gl.ShaderSource(fragmentShader, 1, (&fragmentShaderSource.ptr)[0..1], null);
    gl.CompileShader(fragmentShader);
    gl.GetShaderiv(fragmentShader, gl.COMPILE_STATUS, &success);
    if (success == gl.FALSE) {
        gl.GetShaderInfoLog(fragmentShader, logBuffer.len, null, &logBuffer);
        errorPanic(std.mem.sliceTo(&logBuffer, 0));
    }

    // 着色器程序
    const program = gl.CreateProgram();
    if (program == 0) errorPanic("create program failed");
    errdefer gl.DeleteProgram(program);

    gl.AttachShader(program, vertexShader);
    gl.AttachShader(program, fragmentShader);
    gl.LinkProgram(program);
    gl.GetProgramiv(program, gl.LINK_STATUS, &success);
    if (success == gl.FALSE) {
        gl.GetProgramInfoLog(program, logBuffer.len, null, &logBuffer);
        errorPanic(std.mem.sliceTo(&logBuffer, 0));
    }
    return program;
}

fn deinit(window: glfw.Window) void {
    window.destroy();
    glfw.terminate();
}
```
