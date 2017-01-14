#[unstable(feature = "steed", issue = "0")]
#[macro_export]
macro_rules! print {
   ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[unstable(feature = "steed", issue = "0")]
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
