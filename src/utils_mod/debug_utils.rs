

///
/// Prints to STDOUT if the log level is set to ```log_level=debug```.
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

///
/// Prints to STDERR if the log level is set to ```log_level=error```.
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