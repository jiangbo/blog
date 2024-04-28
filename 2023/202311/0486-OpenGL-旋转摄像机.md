# 0486-OpenGL-旋转摄像机

## 环境

- Time 2024-04-28
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://github.com/hexops/mach-glfw-opengl-example/tree/main>
2. <https://learnopengl-cn.github.io/01%20Getting%20started/03%20Hello%20Window/>
3. <https://www.bilibili.com/video/BV1CZ421e7YB>

### 目标

在之前的基础上，去掉了立方体随着时间旋转的功能，增加摄像机随着时间旋转。

## 无变化

vertex.glsl，fragment.glsl 和 shader.zig 文件都没有变化。

## 摄像机

```zig
    const cameraX: f32 = @floatCast(@sin(glfw.getTime()) * 10);
    const cameraZ: f32 = @floatCast(@cos(glfw.getTime()) * 10);
    const eye = zlm.Vec3.new(cameraX, 0.0, cameraZ);
    const center = zlm.Vec3.new(0.0, 0.0, 0.0);
    const up = zlm.Vec3.new(0.0, 1.0, 0.0);
    const view = zlm.Mat4.createLookAt(eye, center, up);
    gl.UniformMatrix4fv(viewPosition, 1, gl.FALSE, &view.fields[0][0]);
```

## 效果

![旋转的摄像机][1]

## 总结

摄像机随着时间旋转，从不同的角度观察立方体。

[1]: images/opengl15.gif

## 附录

### main.zig

```zig
const std = @import("std");
const glfw = @import("mach-glfw");
const gl = @import("gl");
const shader = @import("shader.zig");
const zstbi = @import("zstbi");
const zlm = @import("zlm");

fn logGlfwError(code: glfw.ErrorCode, description: [:0]const u8) void {
    std.log.err("{}: {s}\n", .{ code, description });
}

fn glfwPanic() noreturn {
    @panic(glfw.getErrorString() orelse "unknown error");
}

var glProcs: gl.ProcTable = undefined;
const vertices = [_]f32{
    -0.5, -0.5, -0.5, 0.0, 0.0, //
    0.5,  -0.5, -0.5, 1.0, 0.0,
    0.5,  0.5,  -0.5, 1.0, 1.0,
    0.5,  0.5,  -0.5, 1.0, 1.0,
    -0.5, 0.5,  -0.5, 0.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 0.0,

    -0.5, -0.5, 0.5,  0.0, 0.0,
    0.5,  -0.5, 0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5, 0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5, 0.5,  0.0, 0.0,

    -0.5, 0.5,  0.5,  1.0, 0.0,
    -0.5, 0.5,  -0.5, 1.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 1.0,
    -0.5, -0.5, 0.5,  0.0, 0.0,
    -0.5, 0.5,  0.5,  1.0, 0.0,

    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  -0.5, 1.0, 1.0,
    0.5,  -0.5, -0.5, 0.0, 1.0,
    0.5,  -0.5, -0.5, 0.0, 1.0,
    0.5,  -0.5, 0.5,  0.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5, 0.0, 1.0,
    0.5,  -0.5, -0.5, 1.0, 1.0,
    0.5,  -0.5, 0.5,  1.0, 0.0,
    0.5,  -0.5, 0.5,  1.0, 0.0,
    -0.5, -0.5, 0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5, 0.0, 1.0,

    -0.5, 0.5,  -0.5, 0.0, 1.0,
    0.5,  0.5,  -0.5, 1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5, 0.5,  0.5,  0.0, 0.0,
    -0.5, 0.5,  -0.5, 0.0, 1.0,
};

const cubePositions = [_]zlm.Vec3{
    zlm.Vec3.new(0.0, 0.0, 0.0), //
    zlm.Vec3.new(2.0, 5.0, -15.0),
    zlm.Vec3.new(-1.5, -2.2, -2.5),
    zlm.Vec3.new(-3.8, -2.0, -12.3),
    zlm.Vec3.new(2.4, -0.4, -3.5),
    zlm.Vec3.new(-1.7, 3.0, -7.5),
    zlm.Vec3.new(1.3, -2.0, -2.5),
    zlm.Vec3.new(1.5, 2.0, -2.5),
    zlm.Vec3.new(1.5, 0.2, -1.5),
    zlm.Vec3.new(-1.3, 1.0, -1.5),
};

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
    gl.Enable(gl.DEPTH_TEST);

    const program = shader.init(vertexSource, fragmentSource);
    defer gl.DeleteProgram(program);

    // VBO 顶点缓冲对象
    var vbos: [1]c_uint = undefined;
    gl.GenBuffers(vbos.len, &vbos);
    defer gl.DeleteBuffers(vbos.len, &vbos);
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[0]);
    gl.BufferData(gl.ARRAY_BUFFER, @sizeOf(@TypeOf(vertices)), &vertices, gl.STATIC_DRAW);

    // VAO 顶点数组对象
    var vao: c_uint = undefined;
    gl.GenVertexArrays(1, (&vao)[0..1]);
    gl.BindVertexArray(vao);
    gl.BindBuffer(gl.ARRAY_BUFFER, vbos[0]);
    gl.EnableVertexAttribArray(0);
    gl.VertexAttribPointer(0, 3, gl.FLOAT, gl.FALSE, 5 * @sizeOf(f32), 0);

    gl.EnableVertexAttribArray(1);
    gl.VertexAttribPointer(1, 2, gl.FLOAT, gl.FALSE, 5 * @sizeOf(f32), 3 * @sizeOf(f32));

    gl.UseProgram(program);

    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    var textures: [2]c_uint = undefined;
    gl.GenTextures(textures.len, &textures);
    defer gl.DeleteTextures(textures.len, &textures);

    gl.ActiveTexture(gl.TEXTURE0);
    gl.BindTexture(gl.TEXTURE_2D, textures[0]);
    var image = try zstbi.Image.loadFromFile("assets/container.jpg", 0);
    var w: c_int = @intCast(image.width);
    var h: c_int = @intCast(image.height);
    gl.TexImage2D(gl.TEXTURE_2D, 0, gl.RGB, w, h, 0, gl.RGB, gl.UNSIGNED_BYTE, image.data.ptr);
    gl.GenerateMipmap(gl.TEXTURE_2D);
    image.deinit();
    gl.Uniform1i(gl.GetUniformLocation(program, "texture1"), 0);

    gl.ActiveTexture(gl.TEXTURE1);
    gl.BindTexture(gl.TEXTURE_2D, textures[1]);
    image = try zstbi.Image.loadFromFile("assets/awesomeface.png", 0);
    w, h = .{ @intCast(image.width), @intCast(image.height) };
    gl.TexImage2D(gl.TEXTURE_2D, 0, gl.RGBA, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, image.data.ptr);
    gl.GenerateMipmap(gl.TEXTURE_2D);
    image.deinit();
    gl.Uniform1i(gl.GetUniformLocation(program, "texture2"), 1);

    const modelPosition = gl.GetUniformLocation(program, "model");
    const viewPosition = gl.GetUniformLocation(program, "view");
    const projectionPosition = gl.GetUniformLocation(program, "projection");

    const projection = zlm.Mat4.createPerspective(zlm.toRadians(45.0), 640 / 480, 0.1, 100);
    gl.UniformMatrix4fv(projectionPosition, 1, gl.FALSE, &projection.fields[0][0]);

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

        gl.BindVertexArray(vao);
        for (cubePositions, 0..) |cube, i| {
            const index: f64 = @floatFromInt(i);
            const angle: f32 = @floatCast(zlm.toRadians(20 * index));
            const rotate = zlm.Mat4.createAngleAxis(zlm.Vec3.new(1, 0.3, 0.5), angle);
            const model = rotate.mul(zlm.Mat4.createTranslation(cube));
            gl.UniformMatrix4fv(modelPosition, 1, gl.FALSE, &model.fields[0][0]);

            gl.DrawArrays(gl.TRIANGLES, 0, 36);
        }

        const cameraX: f32 = @floatCast(@sin(glfw.getTime()) * 10);
        const cameraZ: f32 = @floatCast(@cos(glfw.getTime()) * 10);
        const eye = zlm.Vec3.new(cameraX, 0.0, cameraZ);
        const center = zlm.Vec3.new(0.0, 0.0, 0.0);
        const up = zlm.Vec3.new(0.0, 1.0, 0.0);
        const view = zlm.Mat4.createLookAt(eye, center, up);
        gl.UniformMatrix4fv(viewPosition, 1, gl.FALSE, &view.fields[0][0]);

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
