# 0467-OpenGL-创建着色器程序

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

定义顶点着色器和片段着色器，创建并编译这两个着色器。

## vertexShaderSource

```zig
const vertexShaderSource: [:0]const u8 =
    \\#version 330 core
    \\layout (location = 0) in vec4 aPos;
    \\
    \\void main()
    \\{
    \\    gl_Position = aPos;
    \\}
;
```

## fragmentShaderSource

```zig
const fragmentShaderSource: [:0]const u8 =
    \\#version 330 core
    \\out vec4 FragColor;
    \\
    \\void main()
    \\{
    \\    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    \\}
;
```

## createShaderProgram

```zig
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
```

## 效果

和之前一致，创建了着色器程序，还需要顶点信息才能绘制。

## 总结

创建了顶点和片段着色器，然后链接着色器，生成了着色器程序。

## 附录

### 不显示三角形

如果只使用 VBO 进行渲染的话，需要将核心模式（opengl_core_profile）修改为兼容模式。
