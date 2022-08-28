pub trait Interface {
    fn new(base_address: usize) -> Self
    where
        Self: Sized;

    fn base_address(&self) -> usize;
}

#[macro_export]
macro_rules! lpcstr {
    ($string:expr) => {
        format!("{}\0", $string).as_ptr() as winapi::um::winnt::LPCSTR
    };
}

#[macro_export]
macro_rules! define_interface {
    ($name:ident) => {
        #[allow(dead_code)]
        #[derive(Copy, Clone, Debug)]
        pub struct $name {
            base_address: usize,
        }

        impl $crate::macros::Interface for $name {
            #[allow(dead_code)]
            fn new(base_address: usize) -> Self {
                Self { base_address }
            }

            #[allow(dead_code)]
            fn base_address(&self) -> usize {
                self.base_address
            }
        }
    };
}

#[macro_export]
macro_rules! vfunc {
    ($vtable_index:expr, fn $name:ident($($arg_name:ident: $arg:ty),*) => $ret_ty:ty) => {
        #[allow(dead_code)]
        pub extern "thiscall" fn $name(&self, $($arg_name: $arg)*) -> $ret_ty {
            unsafe {
                let address = (*(self.base_address as *mut usize) as *mut usize).add($vtable_index) as *mut usize;
                let original = ::std::mem::transmute::<_, fn(*mut usize, $($arg),*) -> $ret_ty>(address.read());
                original(self.base_address as *mut usize, $($arg_name),*)
            }
        }
    };
    ($vtable_index:expr, static fn $name:ident($($arg_name:ident: $arg:ty),*) => $ret_ty:ty) => {
        #[allow(dead_code)]
        pub extern fn $name(&self, $($arg_name: $arg)*) -> $ret_ty {
            unsafe {
                let address = (*(self.base_address as *mut usize) as *mut usize).add($vtable_index) as *mut usize;
                let original = ::std::mem::transmute::<_, fn($($arg),*) -> $ret_ty>(address.read());
                original($($arg_name),*)
            }
        }
    };
}
