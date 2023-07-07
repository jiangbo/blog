# 0294-Nand-算术逻辑单元ALU

## 环境

- Time 2023-07-07

## 前言

### 说明

参考：<https://www.nand2tetris.org/>    
参考：《编码：隐匿在计算机背后的语言》

### 目标

接上一节，实现 HalfAdder、FullAdder、Add16、Inc16、ALU。

## HalfAdder

```hdl
/**
 * Computes the sum of two bits.
 */

CHIP HalfAdder {
    IN a, b;    // 1-bit inputs
    OUT sum,    // Right bit of a + b 
        carry;  // Left bit of a + b

    PARTS:
    // Put you code here:
    Xor(a = a, b = b, out = sum);
    And(a = a, b = b, out = carry);
}
```

## FullAdder

```hdl
/**
 * Computes the sum of three bits.
 */

CHIP FullAdder {
    IN a, b, c;  // 1-bit inputs
    OUT sum,     // Right bit of a + b + c
        carry;   // Left bit of a + b + c

    PARTS:
    // Put you code here:
    HalfAdder(a = a, b = b, sum = s, carry= c1);
    HalfAdder(a = s, b = c, sum = sum, carry= c2);
    Or(a = c1, b = c2, out = carry);
}
```

## Add16

```hdl
/**
 * Adds two 16-bit values.
 * The most significant carry bit is ignored.
 */

CHIP Add16 {
    IN a[16], b[16];
    OUT out[16];

    PARTS:
    // Put you code here:
    FullAdder(a = a[0], b = b[0], c = false, sum=out[0], carry=c0);
    FullAdder(a = a[1], b = b[1], c = c0, sum = out[1], carry = c1);
    FullAdder(a = a[2], b = b[2], c = c1, sum = out[2], carry = c2);
    FullAdder(a = a[3], b = b[3], c = c2, sum = out[3], carry = c3);
    FullAdder(a = a[4], b = b[4], c = c3, sum = out[4], carry = c4);
    FullAdder(a = a[5], b = b[5], c = c4, sum = out[5], carry = c5);
    FullAdder(a = a[6], b = b[6], c = c5, sum = out[6], carry = c6);
    FullAdder(a = a[7], b = b[7], c = c6, sum = out[7], carry = c7);
    FullAdder(a = a[8], b = b[8], c = c7, sum = out[8], carry = c8);
    FullAdder(a = a[9], b = b[9], c = c8, sum = out[9], carry = c9);
    FullAdder(a = a[10], b = b[10], c = c9, sum= out[10], carry= c10);
    FullAdder(a = a[11], b = b[11], c = c10, sum = out[11], carry = c11);
    FullAdder(a = a[12], b = b[12], c = c11, sum = out[12], carry = c12);
    FullAdder(a = a[13], b = b[13], c = c12, sum = out[13], carry = c13);
    FullAdder(a = a[14], b = b[14], c = c13, sum = out[14], carry = c14);
    FullAdder(a = a[15], b = b[15], c = c14, sum = out[15], carry = c15);
}
```

## Inc16

```hdl
/**
 * 16-bit incrementer:
 * out = in + 1 (arithmetic addition)
 */

CHIP Inc16 {
    IN in[16];
    OUT out[16];

    PARTS:
    // Put you code here:
    Add16(a = in, b[0] = true, b[1..15] = false, out = out);
}
```

## ALU

```hdl
/**
 * The ALU (Arithmetic Logic Unit).
 * Computes one of the following functions:
 * x+y, x-y, y-x, 0, 1, -1, x, y, -x, -y, !x, !y,
 * x+1, y+1, x-1, y-1, x&y, x|y on two 16-bit inputs, 
 * according to 6 input bits denoted zx,nx,zy,ny,f,no.
 * In addition, the ALU computes two 1-bit outputs:
 * if the ALU output == 0, zr is set to 1; otherwise zr is set to 0;
 * if the ALU output < 0, ng is set to 1; otherwise ng is set to 0.
 */

// Implementation: the ALU logic manipulates the x and y inputs
// and operates on the resulting values, as follows:
// if (zx == 1) set x = 0        // 16-bit constant
// if (nx == 1) set x = !x       // bitwise not
// if (zy == 1) set y = 0        // 16-bit constant
// if (ny == 1) set y = !y       // bitwise not
// if (f == 1)  set out = x + y  // integer 2's complement addition
// if (f == 0)  set out = x & y  // bitwise and
// if (no == 1) set out = !out   // bitwise not
// if (out == 0) set zr = 1
// if (out < 0) set ng = 1

CHIP ALU {
    IN  
        x[16], y[16],  // 16-bit inputs        
        zx, // zero the x input?
        nx, // negate the x input?
        zy, // zero the y input?
        ny, // negate the y input?
        f,  // compute out = x + y (if 1) or x & y (if 0)
        no; // negate the out output?

    OUT 
        out[16], // 16-bit output
        zr, // 1 if (out == 0), 0 otherwise
        ng; // 1 if (out < 0),  0 otherwise

    PARTS:
    // Put you code here:
    /** 处理输入 x */
    Mux16(a = x, b = false, sel = zx, out = xOr0);
    Not16(in = xOr0, out = znx);
    Mux16(a = xOr0, b = znx, sel = nx, out = x1);
    /** 处理输入 y */
    Mux16(a = y, b = false, sel = zy, out = yOr0);
    Not16(in = yOr0, out = zny);
    Mux16(a = yOr0, b = zny, sel = ny, out = y1);
    /** 根据函数不同，选择布尔或者算数逻辑 */
    Add16(a = x1, b = y1, out = sum);
    And16(a = x1, b = y1, out = and);
    Mux16(a = and, b = sum, sel = f, out = o);
    /** 取反和输出out和符号 */
    Not16(in = o, out = noto);
    Mux16(a = o, b = noto, sel = no, out = out, out[15] = ng,
        out[0..7] = low, out[8..15] = high);
    /** 判断是否为 0 */
    Or8Way(in = low, out = a);
    Or8Way(in = high, out = b);
    Or(a = a, b = b, out = nzr);
    Not(in = nzr, out = zr);
}
```

## 总结

通过之前建立起来的逻辑门，实现了算术逻辑单元 ALU。

## 附录
