# 0491-OpenGL-资源管理器

## 环境

- Time 2024-04-28
- Zig 0.12.0-dev.3180+83e578a18
- WSL-Ubuntu 22.04.3 LTS
- OpenGL 3.3

## 前言

### 说明

参考资料：

1. <https://learnopengl-cn.github.io/01%20Getting%20started/03%20Hello%20Window/>

### 目标

将资源的加载和卸载统一管理，并且进行缓存。

## resource.zig

```zig
const std = @import("std");
const engine = @import("engine.zig");
const zstbi = @import("zstbi");
const gl = @import("gl");

var textures: std.StringHashMap(engine.Texture) = undefined;
var shaders: std.StringHashMap(engine.Shader) = undefined;

pub fn init(allocator: std.mem.Allocator) void {
    textures = std.StringHashMap(engine.Texture).init(allocator);
    shaders = std.StringHashMap(engine.Shader).init(allocator);
}

const cstr = [:0]const u8;
pub fn loadShader(name: []const u8, vs: cstr, fs: cstr) !engine.Shader {
    const shader = engine.Shader.init(vs, fs);
    try shaders.put(name, shader);
    return shader;
}

pub fn getShader(name: []const u8) engine.Shader {
    return shaders.get(name).?;
}

pub fn loadTexture(name: []const u8, file: cstr, alpha: bool) !engine.Texture {
    var image = try zstbi.Image.loadFromFile(file, 0);
    defer image.deinit();

    var texture = engine.Texture.init(image.data);
    texture.width = @intCast(image.width);
    texture.height = @intCast(image.height);

    const internal: c_int = if (alpha) gl.RGBA else gl.RGB;
    const format: c_uint = if (alpha) gl.RGBA else gl.RGB;
    texture.generate(internal, format);

    try textures.put(name, texture);
    return texture;
}

pub fn getTexture(name: []const u8) engine.Texture {
    return textures.get(name).?;
}

pub fn deinit() void {
    var textureIterator = textures.valueIterator();
    while (textureIterator.next()) |texture| texture.deinit();
    var shaderIterator = shaders.valueIterator();
    while (shaderIterator.next()) |shader| shader.deinit();
    textures.deinit();
    shaders.deinit();
}
```

## 效果

只进行了资源管理封装，不改变之前的功能。

![旋转的立方体][1]

## 总结

对资源管理进行了封装。

[1]: images/opengl13.gif

## 附录

### main.zig

```zig
const std = @import("std");
const glfw = @import("mach-glfw");
const gl = @import("gl");
// const shader = @import("shader.zig");
const zstbi = @import("zstbi");
const zlm = @import("zlm");
const engine = @import("engine.zig");
const resource = @import("resource.zig");

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
const indices = [_]u32{ 0, 1, 2, 0, 2, 3 };
const texCoords = [_]f32{ 0, 0, 1, 0, 1, 1, 0, 1 };

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

    resource.init(gpa.allocator());
    defer resource.deinit();

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

    const vs: [:0]const u8 = @embedFile("vertex.glsl");
    const fs: [:0]const u8 = @embedFile("fragment.glsl");
    const shader = try resource.loadShader("shader", vs, fs);
    defer shader.deinit();
    shader.use();

    gl.ActiveTexture(gl.TEXTURE0);
    const name = "container.jpg";
    var texture1 = try resource.loadTexture(name, "assets/" ++ name, false);
    defer texture1.deinit();
    texture1.bind();
    shader.setUniform1i("texture1", 0);

    gl.ActiveTexture(gl.TEXTURE1);
    const face = "awesomeface.png";
    var texture2 = try resource.loadTexture(face, "assets/" ++ face, true);
    defer texture2.deinit();
    texture2.bind();
    shader.setUniform1i("texture2", 1);

    const view = zlm.Mat4.createTranslationXYZ(0, 0, -3);
    shader.setUniformMatrix4fv("view", &view.fields[0][0]);

    const projection = zlm.Mat4.createPerspective(zlm.toRadians(45.0), 640 / 480, 0.1, 100);
    shader.setUniformMatrix4fv("projection", &projection.fields[0][0]);

    while (!window.shouldClose()) {
        glfw.pollEvents();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        gl.Clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

        const angle: f32 = @floatCast(glfw.getTime() * zlm.toRadians(50.0));
        const model = zlm.Mat4.createAngleAxis(zlm.Vec3.new(0.5, 1.0, 0.0), angle);
        shader.setUniformMatrix4fv("model", &model.fields[0][0]);
        gl.DrawArrays(gl.TRIANGLES, 0, 36);

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
