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

## loom.rs

给 `AtomicPtr` 增加了一个 `with_mut` 方法，该方法是一个闭包。

```rust
pub(crate) mod sync {
    pub(crate) mod atomic {
        pub(crate) use core::sync::atomic::{fence, AtomicPtr, AtomicUsize, Ordering};

        pub(crate) trait AtomicMut<T> {
            fn with_mut<F, R>(&mut self, f: F) -> R
            where
                F: FnOnce(&mut *mut T) -> R;
        }

        impl<T> AtomicMut<T> for AtomicPtr<T> {
            fn with_mut<F, R>(&mut self, f: F) -> R
            where
                F: FnOnce(&mut *mut T) -> R,
            {
                f(self.get_mut())
            }
        }
    }
}
```

## Shared

当字节分配到栈上的时候，为了避免多次分配，需要对字节进行共享。定义了一个共享的结构，`_vec` 是指向真正的数据，`ref_cnt` 是共享时的计数。

```rust
struct Shared {
    _vec: Vec<u8>,
    ref_cnt: AtomicUsize,
}
```

## Vtable

忽略其中的内存序相关内容，这两个方法相当于将 `AtomicPtr<()>` 转换成了 `*mut Shared`。

```rust
static SHARED_VTABLE: Vtable = Vtable {
    clone: shared_clone,
    drop: shared_drop,
};

unsafe fn shared_clone(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Bytes {
    let shared = data.load(Ordering::Relaxed);
    shallow_clone_arc(shared as _, ptr, len)
}

unsafe fn shared_drop(data: &mut AtomicPtr<()>, _ptr: *const u8, _len: usize) {
    data.with_mut(|shared| {
        release_shared(*shared as *mut Shared);
    });
}
```

## shallow_clone_arc

对于引用类型的 `clone`，指需要将引用加一就可以了。

```rust
unsafe fn shallow_clone_arc(shared: *mut Shared, ptr: *const u8, len: usize) -> Bytes {
    let old_size = (*shared).ref_cnt.fetch_add(1, Ordering::Relaxed);

    Bytes {
        ptr,
        len,
        data: AtomicPtr::new(shared as _),
        vtable: &SHARED_VTABLE,
    }
}
```

## release_shared

对于引用类型，如果引用不为 1，直接将引用减一。如果引用为 1，表示是最后一个引用，需要释放数据。
将原始指针转换成了 `Box`，会受到所有权管理，`Box` 超出范围自动释放 `Shared`，里面的字节数据也被释放。

```rust
unsafe fn release_shared(ptr: *mut Shared) {
    if (*ptr).ref_cnt.fetch_sub(1, Ordering::Release) != 1 {
        return;
    }
    atomic::fence(Ordering::Acquire);
    Box::from_raw(ptr);
}
```

## 总结

了解了 `Bytes` 中对于共享的字节的处理方法，和 `Rc` 有点类似。

## 附录
