use std::fmt::Debug;

pub mod aimbot;
pub mod anti_aim;
pub mod esp;

pub trait Feature: Send + Sync + Debug {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> &str
    where
        Self: Sized;
}

#[macro_export]
macro_rules! feature {
    ($name:ident => $($event:path),*) => {
        #[derive(Debug)]
        pub struct $name;

        impl $crate::features::Feature for $name{
            fn new() -> Self{
                $(
                    event_bus::subscribe_event("main", $event, 1);
                )*
                Self
            }

            fn name(&self) -> &str{
                stringify!($name)
            }
        }

    };
}

#[macro_export]
macro_rules! register_features {
    ($($feature:ident),*) => {
        static FEATURES: once_cell::sync::OnceCell<Vec<Box<dyn crate::features::Feature>>> = once_cell::sync::OnceCell::new();

        pub fn init_features(){
            FEATURES.get_or_init(|| {
                vec![$(Box::new($feature::new())),*]
            });
        }
    };
}