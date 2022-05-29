# 0042-Bytes-bytes源码阅读

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

## promotable_odd_clone

奇数 `Vtable` 就不用再对指针进行减一的操作了。

```rust
unsafe fn promotable_odd_clone(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Bytes {
    let shared = data.load(Ordering::Acquire);
    let kind = shared as usize & KIND_MASK;

    if kind == KIND_ARC {
        shallow_clone_arc(shared as _, ptr, len)
    } else {
        debug_assert_eq!(kind, KIND_VEC);
        shallow_clone_vec(data, shared, shared as *mut u8, ptr, len)
    }
}
```

## promotable_odd_drop

```rust
unsafe fn promotable_odd_drop(data: &mut AtomicPtr<()>, ptr: *const u8, len: usize) {
    data.with_mut(|shared| {
        let shared = *shared;
        let kind = shared as usize & KIND_MASK;

        if kind == KIND_ARC {
            release_shared(shared as *mut Shared);
        } else {
            debug_assert_eq!(kind, KIND_VEC);

            drop(rebuild_boxed_slice(shared as *mut u8, ptr, len));
        }
    });
}
```

## from box

根据字节地址的奇偶性，分别创建 `PROMOTABLE_EVEN_VTABLE` 和 `PROMOTABLE_ODD_VTABLE`。
其中 data 指向的地址最先都是奇数。

```rust
impl From<Box<[u8]>> for Bytes {
    fn from(slice: Box<[u8]>) -> Bytes {
        if slice.is_empty() {
            return Bytes::new();
        }

        let len = slice.len();
        let ptr = Box::into_raw(slice) as *mut u8;

        if ptr as usize & 0x1 == 0 {
            let data = ptr as  usize| KIND_VEC;
            Bytes {
                ptr,
                len,
                data: AtomicPtr::new(data as *mut _),
                vtable: &PROMOTABLE_EVEN_VTABLE,
            }
        } else {
            Bytes {
                ptr,
                len,
                data: AtomicPtr::new(ptr as *mut _),
                vtable: &PROMOTABLE_ODD_VTABLE,
            }
        }
    }
}
```

## from vec

```rust
impl From<Vec<u8>> for Bytes {
    fn from(vec: Vec<u8>) -> Bytes {
        let slice = vec.into_boxed_slice();
        slice.into()
    }
}
```

## from string

```rust
impl From<String> for Bytes {
    fn from(s: String) -> Bytes {
        Bytes::from(s.into_bytes())
    }
}
```

## 总结

了解了 `Bytes` 中对于共享的字节的处理方法，包括共享和非共享的。

## 附录
