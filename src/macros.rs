#[macro_export]
macro_rules! lpcstr {
    ($string:literal) => {
        format!("{}\0", $string).as_ptr() as winapi::um::winnt::LPCSTR
    };
}

#[macro_export]
macro_rules! define_interface {
    ($name:ident) => {
        #[allow(dead_code)]
        pub struct $name {
            class_base: usize,
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new(class_base: usize) -> Self {
                Self { class_base }
            }

            #[allow(dead_code)]
            pub fn class_base(&self) -> usize {
                self.class_base
            }
        }
    };
}

#[macro_export]
macro_rules! vfunc {
    ($vtable_index:expr, fn $name:ident($($arg_name:ident: $arg:ty),*) => $ret_ty:ty) => {
        #[allow(dead_code)]
        pub fn $name(&self, $($arg_name: $arg)*) -> $ret_ty{
            unsafe{
                let original = ::std::mem::transmute::<_, fn($($arg),*) -> $ret_ty>((self.class_base as *const u8).add($vtable_index * ::std::mem::size_of::<*const ::std::ffi::c_void>()));
                original($($arg_name),*)
            }
        }

    };
}
