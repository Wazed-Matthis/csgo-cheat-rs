use vtables::VTable;

#[repr(transparent)]
#[derive(Debug)]
pub struct NotNull<T: VTable> {
    ptr: T,
}

impl<T: VTable> NotNull<T> {
    /// Returns `None` if the contained value is `null()`,
    /// if not it returns `Some(T)`.
    pub fn get(self) -> Option<T> {
        if !self.ptr.as_ptr().is_null() {
            return Some(self.ptr);
        }

        None
    }

    /// Even if the contained value is `null()` a new `T`
    /// with the value will be created and returned.
    pub fn unwrap(self) -> T {
        self.ptr
    }
}

/// # Safety
///
/// This is not safe lmao
pub unsafe fn read<T>(address: usize) -> T {
    core::ptr::read::<T>(address as *const T)
}
