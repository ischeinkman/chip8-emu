

#[macro_export]
macro_rules! debug_log {
    ($fmt:expr) => {
        if cfg!(feature = "log_level=debug") {
            println!($fmt);
        }
    };
    ($fmt:expr, $($args:tt)*) => {
        if cfg!(feature = "log_level=debug") {
            println!($fmt, $($args)*);
        }
    };
}

#[macro_export]
macro_rules! error_log {
    ($fmt:expr) => {
        if cfg!(feature = "log_level=error") {
            eprintln!($fmt);
        }
    };
    ($fmt:expr, $($args:tt)*) => {
        if cfg!(feature = "log_level=error") {
            eprintln!($fmt, $($args)*);
        }
    };
}