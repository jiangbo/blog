# 0476-OpenGL-三角形纹理

## 环境

- Time 2024-04-25
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

实现三角形纹理。

## vertex.glsl

```glsl
#version 330 core

layout(location = 0) in vec3 vertexPosition; // 顶点位置
layout(location = 1) in vec2 texCoords; // 顶点纹理坐标
out vec2 uv;

void main()
{
    gl_Position = vec4(vertexPosition, 1.0); // 设置顶点位置
    uv = texCoords;
}
```

## fragment.glsl

```glsl
#version 330 core
in vec2 uv;
out vec4 FragColor;
uniform sampler2D ourTexture;

void main()
{
    FragColor = texture(ourTexture, uv);
}
```

## main.zig

```zig
const std = @import("std");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const shader = @import("shader.zig");
const zstbi = @import("zstbi");

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;
const vertices = [_]f32{ -0.5, -0.5, 0.5, -0.5, 0, 0.5 };
const indices = [_]u32{ 0, 1, 2 };
const texCoords = [_]f32{ 0, 0, 1, 0, 0.5, 1 };

const vertexSource: [:0]const u8 = @embedFile("vertex.glsl");
const fragmentSource: [:0]const u8 = @embedFile("fragment.glsl");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const window = initWindow();
    defer deinit(window);

    zstbi.init(gpa.allocator());
    defer zstbi.deinit();
    zstbi.setFlipVerticallyOnLoad(true);

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
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[0]);
    gl.BufferData(gl.ARRAY_BUFFER, @sizeOf(@TypeOf(vertices)), &vertices, gl.STATIC_DRAW);
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[1]);
    gl.BufferData(gl.ARRAY_BUFFER, @sizeOf(@TypeOf(texCoords)), &texCoords, gl.STATIC_DRAW);

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
    gl.VertexAttribPointer(1, 2, gl.FLOAT, gl.FALSE, 2 * @sizeOf(f32), 0);

    gl.BindBuffer(gl.ELEMENT_ARRAY_BUFFER, ebo);
    gl.UseProgram(program);

    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    var image = try zstbi.Image.loadFromFile("assets/wall.jpg", 0);
    var texture: c_uint = undefined;
    gl.GenTextures(1, (&texture)[0..1]);
    gl.BindTexture(gl.TEXTURE_2D, texture);
    const w: c_int = @intCast(image.width);
    const h: c_int = @intCast(image.height);
    gl.TexImage2D(gl.TEXTURE_2D, 0, gl.RGB, w, h, 0, gl.RGB, gl.UNSIGNED_BYTE, image.data.ptr);
    gl.GenerateMipmap(gl.TEXTURE_2D);
    image.deinit();

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gl.COLOR_BUFFER_BIT);

        gl.DrawElements(gl.TRIANGLES, indices.len, gl.UNSIGNED_INT, 0);

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

![三角形纹理][1]

## 总结

显示了三角形纹理。

[1]: images/opengl06.png

## 附录
