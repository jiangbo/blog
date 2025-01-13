# 0729-DirectX12-去掉 DirectX 11

## 目标

遇到一个段错误的问题，排查的时候去掉了 DirectX 11 所有的东西。

## 环境

- Time 2025-01-13
- Zig 0.14.0-dev.1911+3bf89f55c

## 参考

<https://www.3dgep.com/learning-directx-12-1/#DirectX_12_Graphics_Pipeline>

## 想法

遇到的这个错误排查了很久，一直以为是代码的问题，结果不是。

## build.zig.zon

```zig
.{
    .name = "demo",
    .version = "0.0.0",
    .dependencies = .{
        .zigwin32 = .{
            .url = "git+https://github.com/marlersoft/zigwin32",
            .hash = "1220adcf9ec0447c6a170ed069ed9d52c999b3dcae3557b3647878bf65ee59a2f5d0",
        },
        .zmath = .{
            .url = "git+https://github.com/zig-gamedev/zmath#24cdd20f9da09bd1ce7b552907eeaba9bafea59d",
            .hash = "1220081d55b58b968d953db1afc2fb01b2f5733929144e69522461ce25fa6450d84e",
        },
    },

    .paths = .{""},
}
```

## build.zig

```zig
const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);

    const win32 = b.dependency("zigwin32", .{});
    exe.root_module.addImport("win32", win32.module("zigwin32"));

    const zmath = b.dependency("zmath", .{});
    exe.root_module.addImport("zm", zmath.module("root"));

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
```

## main.zig

```zig
const std = @import("std");
const System = @import("System.zig");

pub const UNICODE: bool = true;

pub fn main() !void {
    var system = System.initialize();
    defer system.shutdown();

    system.run();
}
```

## System.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Input = @import("Input.zig");
const Graphics = @import("Graphics.zig");

const ui = win32.ui.windows_and_messaging;

var applicationHandle: *@This() = undefined;
window: ?win32.foundation.HWND = null,
input: Input,
graphics: Graphics,

pub fn initialize() @This() {
    const window = initializeWindows(Graphics.WIDTH, Graphics.HEIGHT);

    return .{
        .window = window,
        .input = Input.initialize(),
        .graphics = Graphics.initialize(window),
    };
}

pub fn run(self: *@This()) void {
    applicationHandle = self;
    var message: ui.MSG = std.mem.zeroes(ui.MSG);

    while (true) {
        while (ui.PeekMessage(&message, null, 0, 0, ui.PM_REMOVE) > 0) {
            _ = ui.TranslateMessage(&message);
            _ = ui.DispatchMessage(&message);
        }
        if (message.message == ui.WM_QUIT) break;
        if (!self.frame()) break;
    }
}

pub fn frame(self: *@This()) bool {
    const key = win32.ui.input.keyboard_and_mouse.VK_ESCAPE;
    if (self.input.isKeyDown(@intFromEnum(key))) {
        return false;
    }

    return self.graphics.frame();
}

pub fn shutdown(self: *@This()) void {
    self.graphics.shutdown();
    _ = ui.DestroyWindow(self.window);
}

fn initializeWindows(width: u16, height: u16) ?win32.foundation.HWND {
    const handle = win32.system.library_loader.GetModuleHandle(null).?;

    var windowClass = std.mem.zeroes(ui.WNDCLASSEX);
    const className = win32.zig.L("DirectX12");
    windowClass.cbSize = @sizeOf(ui.WNDCLASSEX);
    windowClass.style = .{ .HREDRAW = 1, .VREDRAW = 1, .OWNDC = 1 };
    windowClass.lpszClassName = className;
    windowClass.lpfnWndProc = windowCallback;
    windowClass.hInstance = handle;

    win32Check(ui.RegisterClassEx(&windowClass));

    // 计算位置
    const posX = @divTrunc(ui.GetSystemMetrics(.CXSCREEN) - width, 2);
    const posY = @divTrunc(ui.GetSystemMetrics(.CYSCREEN) - height, 2);
    var rect: win32.foundation.RECT = .{
        .left = posX,
        .top = posY,
        .right = posX + width,
        .bottom = posY + height,
    };
    const style = ui.WS_OVERLAPPEDWINDOW;
    win32Check(ui.AdjustWindowRect(&rect, style, win32.zig.FALSE));

    //  根据计算的位置创建窗口
    const name = win32.zig.L("DirectX12 学习");
    const window = ui.CreateWindowEx(.{}, className, name, style, rect.left, rect.top, //
        rect.right - rect.left, rect.bottom - rect.top, null, null, handle, null);
    _ = ui.ShowWindow(window, ui.SW_SHOW);
    return window;
}

fn windowCallback(
    w: win32.foundation.HWND,
    message: u32,
    wParam: win32.foundation.WPARAM,
    lParam: win32.foundation.LPARAM,
) callconv(std.os.windows.WINAPI) win32.foundation.LRESULT {
    switch (message) {
        ui.WM_DESTROY => {
            std.log.info("WM_DESTROY", .{});
            ui.PostQuitMessage(0);
        },
        ui.WM_KEYDOWN => applicationHandle.input.keyDown(wParam),
        ui.WM_KEYUP => applicationHandle.input.keyUp(wParam),
        else => {},
    }
    return ui.DefWindowProc(w, message, wParam, lParam);
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## Graphics.zig

```zig
const std = @import("std");
const win32 = @import("win32");
const Direct3D = @import("Direct3D.zig");
const Model = @import("Model.zig");
const Shader = @import("Shader.zig");
const Camera = @import("Camera.zig");
const Texture = @import("Texture.zig");

pub const WIDTH: u16 = 800;
pub const HEIGHT: u16 = 600;

direct3D: Direct3D,
// model: Model,
// shader: Shader,
// camera: Camera,
// texture: Texture,

pub fn initialize(window: ?win32.foundation.HWND) @This() {
    var direct = Direct3D{};

    direct.initialize(WIDTH, HEIGHT, window);
    return .{
        .direct3D = direct,
        // .model = Model.initialize(direct.device),
        // .shader = Shader.initialize(direct.device),
        // .camera = Camera.init(direct.device, WIDTH, HEIGHT),
        // .texture = Texture.init(direct.device, "assets/player32.bmp"),
    };
}

pub fn frame(self: *@This()) bool {
    return self.render();
}

pub fn render(self: *@This()) bool {
    self.direct3D.beginScene(0, 0, 0, 1);

    // // self.shader.render(self.direct3D.deviceContext);
    // // self.model.render(self.direct3D.deviceContext);
    // // self.texture.draw(self.direct3D.deviceContext);
    // // self.camera.render(self.direct3D.deviceContext, self.texture.model);
    self.direct3D.render();

    self.direct3D.endScene();
    return true;
}

pub fn shutdown(self: *@This()) void {
    // self.shader.shutdown();
    // self.model.shutdown();
    // self.texture.deinit();
    // self.camera.deinit();
    self.direct3D.shutdown();
}
```

## Direct3D.zig

```zig
const std = @import("std");
const win32 = @import("win32");

const dxgi = win32.graphics.dxgi;
const d12 = win32.graphics.direct3d12;

var d12Debug: *d12.ID3D12Debug5 = undefined;
var dxgiDebug: *dxgi.IDXGIDebug1 = undefined;

factory: *dxgi.IDXGIFactory7 = undefined,
device: *d12.ID3D12Device9 = undefined,

commandQueue: *d12.ID3D12CommandQueue = undefined,
commandAllocator: *d12.ID3D12CommandAllocator = undefined,
commandList: *d12.ID3D12GraphicsCommandList = undefined,

swapChain: *dxgi.IDXGISwapChain4 = undefined,
descriptorHeap: *d12.ID3D12DescriptorHeap = undefined,
targetView: *d12.ID3D12Resource = undefined,
backBuffers: [2]*d12.ID3D12Resource = undefined,

pub fn initialize(self: *@This(), w: u16, h: u16, window: ?win32.foundation.HWND) void {
    initDebug();

    self.initDevice();
    self.initCommand();
    self.initSwapChain(w, h, window);
}

fn initDebug() void {
    win32Check(d12.D3D12GetDebugInterface(d12.IID_ID3D12Debug5, @ptrCast(&d12Debug)));
    d12Debug.ID3D12Debug.EnableDebugLayer();
    d12Debug.ID3D12Debug3.SetEnableGPUBasedValidation(win32.zig.TRUE);

    win32Check(dxgi.DXGIGetDebugInterface1(0, dxgi.IID_IDXGIDebug1, @ptrCast(&dxgiDebug)));
    dxgiDebug.EnableLeakTrackingForThread();
}

fn initDevice(self: *@This()) void {
    const flags = dxgi.DXGI_CREATE_FACTORY_DEBUG;
    var id = dxgi.IID_IDXGIFactory7;
    win32Check(dxgi.CreateDXGIFactory2(flags, id, @ptrCast(&self.factory)));

    id = d12.IID_ID3D12Device9;
    win32Check(d12.D3D12CreateDevice(null, .@"12_1", id, @ptrCast(&self.device)));
}

fn initCommand(self: *@This()) void {
    var queueDesc = std.mem.zeroes(d12.D3D12_COMMAND_QUEUE_DESC);
    queueDesc.Flags = d12.D3D12_COMMAND_QUEUE_FLAG_NONE;
    queueDesc.Type = .DIRECT;

    win32Check(self.device.ID3D12Device.CreateCommandQueue(
        &queueDesc,
        d12.IID_ID3D12CommandQueue,
        @ptrCast(&self.commandQueue),
    ));

    win32Check(self.device.ID3D12Device.CreateCommandAllocator(
        .DIRECT,
        d12.IID_ID3D12CommandAllocator,
        @ptrCast(&self.commandAllocator),
    ));

    win32Check(self.device.ID3D12Device.CreateCommandList(0, .DIRECT, self.commandAllocator, //
        null, d12.IID_ID3D12GraphicsCommandList, @ptrCast(&self.commandList)));
}

fn initSwapChain(self: *@This(), w: u16, h: u16, window: ?win32.foundation.HWND) void {
    var desc = std.mem.zeroes(dxgi.DXGI_SWAP_CHAIN_DESC1);
    desc.Width = w;
    desc.Height = h;
    desc.Format = .R8G8B8A8_UNORM;
    desc.Stereo = win32.zig.FALSE;
    desc.SampleDesc = .{ .Count = 1, .Quality = 0 };
    desc.BufferUsage = dxgi.DXGI_USAGE_RENDER_TARGET_OUTPUT;
    desc.BufferCount = 2;
    desc.Scaling = .STRETCH;
    desc.SwapEffect = .FLIP_DISCARD;
    desc.AlphaMode = .UNSPECIFIED;
    desc.Flags = @intFromEnum(dxgi.DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING);

    win32Check(self.factory.IDXGIFactory2.CreateSwapChainForHwnd(&self.commandQueue.IUnknown, //
        window, &desc, null, null, @ptrCast(&self.swapChain)));

    win32Check(self.factory.IDXGIFactory.MakeWindowAssociation(window, dxgi.DXGI_MWA_NO_ALT_ENTER));
}

pub fn beginScene(self: *@This(), red: f32, green: f32, blue: f32, alpha: f32) void {
    // const color = [_]f32{ red, green, blue, alpha };
    // self.deviceContext.ClearRenderTargetView(self.targetView, @ptrCast(&color));
    _ = self;
    _ = red;
    _ = green;
    _ = blue;
    _ = alpha;
}

pub fn render(self: *@This()) void {
    _ = self;
}

pub fn endScene(self: *@This()) void {
    // win32Check(self.swapChain.IDXGISwapChain.Present(1, 0));
    _ = self;
}

pub fn shutdown(self: *@This()) void {
    _ = self.factory.IUnknown.Release();
    _ = self.device.IUnknown.Release();

    _ = self.commandQueue.IUnknown.Release();
    _ = self.commandAllocator.IUnknown.Release();
    _ = self.swapChain.IUnknown.Release();
    _ = self.descriptorHeap.IUnknown.Release();
    for (self.backBuffers) |back| {
        _ = back.IUnknown.Release();
    }

    _ = d12Debug.ID3D12Debug.IUnknown.Release();
    const flags = dxgi.DXGI_DEBUG_RLO_ALL;
    win32Check(dxgiDebug.IDXGIDebug.ReportLiveObjects(dxgi.DXGI_DEBUG_ALL, flags));
    _ = dxgiDebug.IUnknown.Release();
}

fn win32Check(result: win32.foundation.HRESULT) void {
    if (win32.zig.SUCCEEDED(result)) return;
    @panic(@tagName(win32.foundation.GetLastError()));
}
```

## 效果

![DirectX 12][1]

[1]: images/directx065.png

## 附录
