use core::ops::{Bound, Deref, RangeBounds};
use core::sync::atomic::AtomicPtr;

use core::{ptr, slice};

pub struct Bytes {
    ptr: *const u8,
    len: usize,
    data: AtomicPtr<()>,
    vtable: &'static Vtable,
}

pub(crate) struct Vtable {
    pub clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
    pub drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
}

impl Bytes {
    pub const fn new() -> Bytes {
        const EMPTY: &[u8] = &[];
        Bytes::from_static(EMPTY)
    }

    pub const fn from_static(bytes: &'static [u8]) -> Bytes {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            data: AtomicPtr::new(ptr::null_mut()),
            vtable: &STATIC_VTABLE,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> Bytes {
        let (begin, end) = self.range(range);
        if end == begin {
            return Bytes::new();
        }

        let mut ret = self.clone();

        ret.len = end - begin;
        ret.ptr = unsafe { ret.ptr.offset(begin as isize) };

        ret
    }

    fn range(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let len = self.len();

        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).expect("out of range"),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };

        assert!(
            begin <= end,
            "range start must not be greater than end: {begin:?} <= {end:?}"
        );
        assert!(end <= len, "range end out of bounds: {end:?} <= {len:?}");
        (begin, end)
    }

    pub fn slice_ref(&self, subset: &[u8]) -> Bytes {
        if subset.is_empty() {
            return Bytes::new();
        }

        let bytes_p = self.as_ptr() as usize;
        let bytes_len = self.len();

        let sub_p = subset.as_ptr() as usize;
        let sub_len = subset.len();

        // 子切片的地址大于等于父切片的地址
        assert!(
            sub_p >= bytes_p,
            "subset pointer ({:p}) is smaller than self pointer ({:p})",
            sub_p as *const u8,
            bytes_p as *const u8,
        );
        assert!(
            sub_p + sub_len <= bytes_p + bytes_len,
            "subset is out of bounds: self = ({:p}, {}), subset = ({:p}, {})",
            bytes_p as *const u8,
            bytes_len,
            sub_p as *const u8,
            sub_len,
        );
        // 起地址
        let sub_offset = sub_p - bytes_p;

        self.slice(sub_offset..(sub_offset + sub_len))
    }

    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

const STATIC_VTABLE: Vtable = Vtable {
    clone: |_, ptr, len| {
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        Bytes::from_static(slice)
    },
    drop: |_, _, _| {},
};

impl Drop for Bytes {
    fn drop(&mut self) {
        unsafe { (self.vtable.drop)(&mut self.data, self.ptr, self.len) }
    }
}

impl Clone for Bytes {
    fn clone(&self) -> Bytes {
        unsafe { (self.vtable.clone)(&self.data, self.ptr, self.len) }
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}
