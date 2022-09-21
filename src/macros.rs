use std::any::Any;
use std::fmt::Debug;

pub trait Interface: Sync + Send + Any + Debug + Clone + Copy {
    fn new(base_address: usize) -> Self
    where
        Self: Sized;

    fn base_address(&self) -> usize;
    fn as_any(&self) -> &(dyn Any + Send + Sync);
}

#[macro_export]
macro_rules! lpcstr {
    ($string:expr) => {
        format!("{}\0", $string).as_ptr() as winapi::um::winnt::LPCSTR
    };
}
