#[macro_export]
macro_rules! lpcstr {
    ($string:literal) => {
        format!("{}\0", $string).as_ptr() as winapi::um::winnt::LPCSTR
    };
}
