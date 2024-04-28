# 0488-OpenGL-鼠标控制摄像机

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

通过鼠标来控制摄像机，WSL 环境的鼠标禁用有点问题，没有设置鼠标的禁用。

## 无变化

vertex.glsl，fragment.glsl 和 shader.zig 文件都没有变化。

## 变量定义

```zig
var eye = zlm.Vec3.new(0, 0, 3);
var front = zlm.Vec3.new(0, 0, -1);
var up = zlm.Vec3.new(0, 1, 0);
var lastFrame: f64 = 0.0; // 上一帧的时间
var firstMouse = true;
var lastX: f32 = 0;
var lastY: f32 = 0;
var yaw: f32 = -90;
var pitch: f32 = 0;
var fov: f32 = 0;
```

## 鼠标移动回调

```zig
fn mouseCallback(_: glfw.Window, mouseX: f64, mouseY: f64) void {
    const xpos: f32 = @floatCast(mouseX);
    const ypos: f32 = @floatCast(mouseY);
    if (firstMouse) {
        lastX = xpos;
        lastY = ypos;
        firstMouse = false;
    }

    var xoffset = xpos - lastX;
    var yoffset = lastY - ypos;
    lastX = xpos;
    lastY = ypos;

    const sensitivity = 0.05;
    xoffset *= sensitivity;
    yoffset *= sensitivity;

    yaw += xoffset;
    pitch += yoffset;

    if (pitch > 89.0) pitch = 89.0;
    if (pitch < -89.0) pitch = -89.0;

    front.x = @cos(zlm.toRadians(yaw)) * @cos(zlm.toRadians(pitch));
    front.y = @sin(zlm.toRadians(pitch));
    front.z = @sin(zlm.toRadians(yaw)) * @cos(zlm.toRadians(pitch));
    front = front.normalize();
}
```

## 鼠标滚轮回调

```zig
fn scrollCallback(_: glfw.Window, _: f64, y: f64) void {
    const yoffset: f32 = @floatCast(y);
    if (fov >= 1.0 and fov <= 45.0) fov -= yoffset;
    if (fov <= 1.0) fov = 1.0;
    if (fov >= 45.0) fov = 45.0;
}
```

## 输入处理

```zig
const view = zlm.Mat4.createLookAt(eye, eye.add(front), up);
gl.UniformMatrix4fv(viewPosition, 1, gl.FALSE, &view.fields[0][0]);

const projection = zlm.Mat4.createPerspective(zlm.toRadians(fov), 640 / 480, 0.1, 100);
gl.UniformMatrix4fv(projectionPosition, 1, gl.FALSE, &projection.fields[0][0]);
```

## 效果

![鼠标控制摄像机][1]

## 总结

通过鼠标来控制摄像机。

[1]: images/opengl17.gif

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
    // glfw.Window.setInputModeCursor(window, .normal);
    glfw.Window.setCursorPosCallback(window, mouseCallback);
    glfw.Window.setScrollCallback(window, scrollCallback);

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

    while (!window.shouldClose()) {
        glfw.pollEvents();
        processInput(window);
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

        const view = zlm.Mat4.createLookAt(eye, eye.add(front), up);
        gl.UniformMatrix4fv(viewPosition, 1, gl.FALSE, &view.fields[0][0]);

        const projection = zlm.Mat4.createPerspective(zlm.toRadians(fov), 640 / 480, 0.1, 100);
        gl.UniformMatrix4fv(projectionPosition, 1, gl.FALSE, &projection.fields[0][0]);

        window.swapBuffers();
    }
}

var eye = zlm.Vec3.new(0, 0, 3);
var front = zlm.Vec3.new(0, 0, -1);
var up = zlm.Vec3.new(0, 1, 0);
var lastFrame: f64 = 0.0; // 上一帧的时间
var firstMouse = true;
var lastX: f32 = 0;
var lastY: f32 = 0;
var yaw: f32 = -90;
var pitch: f32 = 0;
var fov: f32 = 0;

fn initWindow() glfw.Window {
    glfw.setErrorCallback(logGlfwError);

    if (!glfw.init(.{})) glfwPanic();

    return glfw.Window.create(640, 480, "学习 OpenGL", null, null, .{
        .context_version_major = gl.info.version_major,
        .context_version_minor = gl.info.version_minor,
        .opengl_profile = .opengl_core_profile,
    }) orelse glfwPanic();
}

fn processInput(window: glfw.Window) void {
    const currentFrame = glfw.getTime();
    defer lastFrame = currentFrame;
    const speed: f32 = @floatCast(2.5 * (currentFrame - lastFrame));

    if (window.getKey(.escape) == .press) window.setShouldClose(true);
    if (window.getKey(.w) == .press) eye = eye.add(front.scale(speed));
    if (window.getKey(.s) == .press) eye = eye.sub(front.scale(speed));
    if (window.getKey(.a) == .press)
        eye = eye.sub(front.cross(up).normalize().scale(speed));
    if (window.getKey(.d) == .press)
        eye = eye.add(front.cross(up).normalize().scale(speed));
}

fn mouseCallback(_: glfw.Window, mouseX: f64, mouseY: f64) void {
    const xpos: f32 = @floatCast(mouseX);
    const ypos: f32 = @floatCast(mouseY);
    if (firstMouse) {
        lastX = xpos;
        lastY = ypos;
        firstMouse = false;
    }

    var xoffset = xpos - lastX;
    var yoffset = lastY - ypos;
    lastX = xpos;
    lastY = ypos;

    const sensitivity = 0.05;
    xoffset *= sensitivity;
    yoffset *= sensitivity;

    yaw += xoffset;
    pitch += yoffset;

    if (pitch > 89.0) pitch = 89.0;
    if (pitch < -89.0) pitch = -89.0;

    front.x = @cos(zlm.toRadians(yaw)) * @cos(zlm.toRadians(pitch));
    front.y = @sin(zlm.toRadians(pitch));
    front.z = @sin(zlm.toRadians(yaw)) * @cos(zlm.toRadians(pitch));
    front = front.normalize();
}

fn scrollCallback(_: glfw.Window, _: f64, y: f64) void {
    const yoffset: f32 = @floatCast(y);
    if (fov >= 1.0 and fov <= 45.0) fov -= yoffset;
    if (fov <= 1.0) fov = 1.0;
    if (fov >= 45.0) fov = 45.0;
}

fn deinit(window: glfw.Window) void {
    window.destroy();
    glfw.terminate();
}
```
