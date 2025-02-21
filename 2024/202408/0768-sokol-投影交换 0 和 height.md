# 0768-sokol-投影交换 0 和 height

## 目标

在 0760-sokol-正交投影 的想法部分，对于教程中交换了 0 和 height 的位置，再细想了一下。

## 环境

- Time 2025-02-21
- Zig 0.14.0-dev.2851+b074fb7dd

## 参考

1. <https://github.com/floooh/sokol-zig/tree/master/src/examples>

## 想法

为什么投影矩阵的时候，需要交换 0 和 height 的值？
因为标准坐标 NDC，向上为正，即 Y 轴向上，数值越大越靠近上边。
但是屏幕坐标，左上角为原点，Y 轴向下，数值越大，越靠近下边。
所以投影矩阵为了适配屏幕坐标，交换了 0 和 height 的值，这样就满足屏幕坐标，Y 越大，越在下面。

还有一个问题是，我使用的矩阵变换的时候，为什么不需要？
我使用的矩阵库，它默认交换了两个参数，按照顺序传就行。

## DirectXMathMatrix.inl

我去看了 DirectXMathMatrix 的源码，如下：

参数顺序是 left，right，bottom，top，bottom 传 height 比如 480，
top 传 0，top - bottom 得到 -480，负数翻转 Y 轴。

```cpp
...
inline XMMATRIX XM_CALLCONV XMMatrixOrthographicOffCenterLH
(
    float ViewLeft,
    float ViewRight,
    float ViewBottom,
    float ViewTop,
    float NearZ,
    float FarZ
) noexcept
{
    assert(!XMScalarNearEqual(ViewRight, ViewLeft, 0.00001f));
    assert(!XMScalarNearEqual(ViewTop, ViewBottom, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = 1.0f / (FarZ - NearZ);

    XMMATRIX M;
    M.m[0][0] = ReciprocalWidth + ReciprocalWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = ReciprocalHeight + ReciprocalHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;
...
}
...
```

## zmath.zig

我使用的 zmath，参数顺序是 left，right，top，bottom，top 传 0，bottom 传 480，
top - bottom 得到 -480，翻转 Y 轴，所以关键区别在于参数顺序不一样，所以不需要交换两个参数。

```zig
pub fn orthographicOffCenterLh(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) Mat {
    assert(!math.approxEqAbs(f32, far, near, 0.001));

    const r = 1 / (far - near);
    return .{
        f32x4(2 / (right - left), 0.0, 0.0, 0.0),
        f32x4(0.0, 2 / (top - bottom), 0.0, 0.0),
        f32x4(0.0, 0.0, r, 0.0),
        f32x4(-(right + left) / (right - left), -(top + bottom) / (top - bottom), -r * near, 1.0),
    };
}
```

## 附录
