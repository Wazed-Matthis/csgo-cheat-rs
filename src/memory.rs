use hook_rs_lib::signature_scan::{Signature, SignatureComponent};
use vtables::VTable;
use winapi::ctypes::c_char;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winnt::{IMAGE_DOS_SIGNATURE, PIMAGE_DOS_HEADER, PIMAGE_NT_HEADERS};

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

/// # Safety
/// This function is safe because bytes can never be a nullptr.
/// all reads performed are checked and will not fail.
pub unsafe fn scan_for_signature(
    signature: &Signature,
    module: *const c_char,
) -> Option<*const usize> {
    let module = GetModuleHandleA(module);

    if module.is_null() {
        return None;
    }

    let dos_headers = (module as PIMAGE_DOS_HEADER).read();

    if dos_headers.e_magic != IMAGE_DOS_SIGNATURE {
        return None;
    }

    let nt_headers = (module as usize + dos_headers.e_lfanew as usize) as PIMAGE_NT_HEADERS;

    let size_of_image = (*nt_headers).OptionalHeader.SizeOfImage as usize;

    let bytes = module as *mut u8;

    let len = signature.sig.len();
    for i in 0..(size_of_image - len) {
        let mut found = true;
        for (sig_index, pat) in signature.sig.iter().enumerate().take(len) {
            match pat {
                SignatureComponent::Value(val) => {
                    if *bytes.add(i + sig_index) != *val {
                        found = false;
                        break;
                    }
                }
                SignatureComponent::Mask => {}
            }
        }
        if found {
            return Some(bytes.add(i) as _);
        }
    }
    None
}
