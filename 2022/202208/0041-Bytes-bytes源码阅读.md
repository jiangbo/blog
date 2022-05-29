# 0040-Bytes-bytes源码阅读

## 环境

- Time 2022-05-29
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：

1. <https://github.com/tokio-rs/bytes>
2. <https://zhuanlan.zhihu.com/p/109977513>

### 目标

之前阅读的部分，都是关于静态的字节，后面开始涉及到动态。其中有很多关于原子类型的操作，来实现无锁并发。
这里不深入，先简单理解，之后有机会单独学原子操作和无锁数据结构和并发。
实现 `bytes.rs` 中的动态字节部分的方法。
前面实现了共享的 `Vtable`，还有一种复杂的是独占，并且在 `clone` 时转为共享。

## Shared

保证指向 Shared 的指针能被 2 整除，因为会使用指针的奇偶性来判断底层字节当前时共享还是独占的。

```rust
const _: [(); 0 - mem::align_of::<Shared>() % 2] = [];
```

## 奇偶性

```rust
// 偶数表示共享
const KIND_ARC: usize = 0b0;
// 奇数表示不共享
const KIND_VEC: usize = 0b1;
const KIND_MASK: usize = 0b1;
```

## Vtable

定义了两个 `Vtable`，一个奇数，一个偶数的。

```rust
static PROMOTABLE_EVEN_VTABLE: Vtable = Vtable {
    clone: promotable_even_clone,
    drop: promotable_even_drop,
};

static PROMOTABLE_ODD_VTABLE: Vtable = Vtable {
    clone: promotable_odd_clone,
    drop: promotable_odd_drop,
};
```

## promotable_even_clone

对指向 `u8` 的偶数地址 `Vtable` 实现 `clone`。

```rust
unsafe fn promotable_even_clone(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Bytes {
    let shared = data.load(Ordering::Acquire);
    let kind = shared as usize & KIND_MASK;

    if kind == KIND_ARC {
        // 如果是共享的，直接走之前看的共享的增加引用计数的逻辑
        shallow_clone_arc(shared as _, ptr, len)
    } else {
        // 非共享的，需要转成共享
        debug_assert_eq!(kind, KIND_VEC);
        // 创建 share 指针的时候，加上了1，所以这里需要减去1，得到字节的起始地址
        let buf = (shared as usize & !KIND_MASK) as *mut u8;
        shallow_clone_vec(data, shared, buf, ptr, len)
    }
}
```

## shallow_clone_vec

```rust
unsafe fn shallow_clone_vec(atom: &AtomicPtr<()>, ptr: *const (),
    buf: *mut u8, offset: *const u8, len: usize,
) -> Bytes {

    let vec = rebuild_boxed_slice(buf, offset, len).into_vec();
    let shared = Box::new(Shared {
        _vec: vec,
        // 原始的一个，clone的一个引用，所以现在是两个
        ref_cnt: AtomicUsize::new(2),
    });

    let shared = Box::into_raw(shared);
    // 如果有的话，需要将其它Bytes的data指向shared指针，无锁进行修改
    match atom.compare_exchange(ptr as _, shared as _, Ordering::AcqRel, Ordering::Acquire) {
        Ok(actual) => {
            Bytes {
                // ptr 指向的是字节的起始地址
                ptr: offset,
                len,
                // 指向 share，需要保证share指针是偶数，表示已经共享
                data: AtomicPtr::new(shared as _),
                vtable: &SHARED_VTABLE,
            }
        }
        Err(actual) => {
            let shared = Box::from_raw(shared);
            mem::forget(*shared);
            // 已经转成共享的了，直接增加引用计数
            shallow_clone_arc(actual as _, offset, len)
        }
    }
}
```

## rebuild_boxed_slice

```rust
unsafe fn rebuild_boxed_slice(buf: *mut u8, offset: *const u8, len: usize) -> Box<[u8]> {
    let cap = (offset as usize - buf as usize) + len;
    Box::from_raw(slice::from_raw_parts_mut(buf, cap))
}
```

## promotable_even_drop

```rust
unsafe fn promotable_even_drop(data: &mut AtomicPtr<()>, ptr: *const u8, len: usize) {
    data.with_mut(|shared| {
        let shared = *shared;
        let kind = shared as usize & KIND_MASK;

        if kind == KIND_ARC {
            // 共享的继续走共享的逻辑
            release_shared(shared as *mut Shared);
        } else {
            // 不共享的直接删除数据
            debug_assert_eq!(kind, KIND_VEC);
            let buf = (shared as usize & !KIND_MASK) as *mut u8;
            drop(rebuild_boxed_slice(buf, ptr, len));
        }
    });
}
```

## 总结

了解了 `Bytes` 中对于共享的字节的处理方法，包括共享和非共享的。

## 附录
